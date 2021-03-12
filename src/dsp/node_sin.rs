use crate::nodes::NodeAudioContext;
use crate::dsp::SAtom;

/// A sine oscillator
#[derive(Debug, Clone)]
pub struct Sin {
    /// Sample rate
    srate: f32,
    /// Oscillator phase
    phase: f32,
}

impl Sin {
    pub fn outputs() -> usize { 1 }

    pub fn new() -> Self {
        Self {
            srate: 44100.0,
            phase: 0.0,
        }
    }

    pub fn set_sample_rate(&mut self, srate: f32) {
        self.srate = srate;
    }

    pub fn reset(&mut self) {
        self.phase = 0.0;
    }

    #[inline]
    pub fn process<T: NodeAudioContext>(&mut self, ctx: &mut T, atoms: &[SAtom], inputs: &[f32], outputs: &mut [f32]) {
        use crate::dsp::denorm;
        use crate::dsp::out;

        let freq = denorm::Sin::freq(inputs);

        out::Sin::sig(outputs,
            (self.phase * 2.0 * std::f32::consts::PI).sin());

        self.phase += freq / self.srate;
        self.phase = self.phase.fract();
    }

    pub const freq : &'static str =
        "Sin freq\nFrequency of the oscillator.\n\nRange: (-1..1)\n";
    pub const sig : &'static str =
        "Sin sig\nOscillator signal output.\n\nRange: (-1..1)\n";
}
