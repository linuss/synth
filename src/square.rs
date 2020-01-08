
pub struct SquareWaveGenerator {
    // The current sample in the cycle
    sample_number: f64,
    // Samples per second
    period_samples: f64,
    /// generated frequency in Hz
    freq: f64,
    /// magnitude of generated signal
    volume: f64,
}

impl SquareWaveGenerator {
    pub fn new(freq: f64, volume: f64) -> Self {
        SquareWaveGenerator {
            sample_number: 0.,
            period_samples: 44_100. / freq,
            freq,
            volume,
        }
    }
}

impl Iterator for SquareWaveGenerator {
    type Item = f32;
    fn next(&mut self) -> Option<f32> {
        let output;
        if self.sample_number < (self.period_samples / 2.) {
            output = self.volume as f32;
        } else {
            output = (-1. * self.volume) as f32;
        }
        self.sample_number = (self.sample_number + 1.) % self.period_samples;
        return Some(output);
    }
}
