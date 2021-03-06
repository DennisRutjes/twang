use fon::{mono::Mono64, Audio, Sink};
use twang::{Mix, Pink, Synth, Signal, Fc};

mod wav;

// Target sample rate set to 48 KHz
const S_RATE: u32 = 48_000;

fn main() {
    fn brass(pink: &mut Pink, fc: Fc) -> Signal {
        let pink = pink.noise();
        let tone = fc.freq(220.0).gain(12.0).clamp().gain(0.1);
        let airy = tone.abs().gain(pink.abs());

        let pone = fc.freq(220.0).gain(12.0).clamp().abs();
        let ptwo = fc.freq(220.0).triangle();
        let main = pone.gain(ptwo);

        [airy, main].mix()
    }

    // Initialize audio with five seconds of silence.
    let mut audio = Audio::<Mono64>::with_silence(S_RATE, S_RATE as usize * 5);
    // Create the synthesizer.
    let mut synth = Synth::new(Pink::new(), brass);
    // Generate audio samples.
    audio.sink(..).stream(&mut synth);

    // Write synthesized audio to WAV file.
    wav::write(audio, "brass.wav").expect("Failed to write WAV file");
}
