
pub struct TriangleWaveGenerator {
    // The current sample in the cycle
    sample_number: f64,
    // Samples per second
    period_samples: f64,
    /// generated frequency in Hz
    freq: f64,
    /// magnitude of generated signal
    volume: f64,
}

impl TriangleWaveGenerator {
    pub fn new(freq: f64, volume: f64) -> Self {
        TriangleWaveGenerator {
            sample_number: 0.,
            period_samples: 44_100. / freq,

            freq,
            volume,
        }
    }
}

impl Iterator for TriangleWaveGenerator {
    type Item = f32;
    fn next(&mut self) -> Option<f32> {
        let t = self.sample_number / self.period_samples;
        let output = (2. * self.volume * (t - (t + 0.5).floor()).abs()) as f32;
        self.sample_number = (self.sample_number + 1.) % self.period_samples;
        Some(output)
    }
}
