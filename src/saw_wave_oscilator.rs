use std::{sync::mpsc::Receiver, time::Duration};

use num::clamp;
use rodio::Source;

pub struct SawWaveOscilator {
    sample_rate: u32,
    receiver: Receiver<f32>,
    amplitude: f32,
    current_frequency: f32,
    phase: f32,
}

impl SawWaveOscilator {
    pub fn new(sample_rate: u32, receiver: Receiver<f32>) -> SawWaveOscilator {
        return SawWaveOscilator {
            sample_rate,
            receiver,
            amplitude: 0.0,
            current_frequency: 0.0,
            phase: 0.0,
        };
    }

    fn set_frequency(&mut self, frequency: f32) {
        if frequency != self.current_frequency {
            self.current_frequency = frequency;
        }
    }

    fn get_sample(&mut self) -> f32 {
        match self.receiver.try_recv() {
            Ok(f) => {
                self.amplitude = 1.0;
                self.set_frequency(f);
            }
            Err(_) => (),
        }

        self.phase = self.phase + self.current_frequency / self.sample_rate as f32;

        while self.phase > 1.0 {
            self.phase = self.phase - 1.0;
        }

        while self.phase < 0.0 {
            self.phase = self.phase + 1.0;
        }

        self.amplitude = clamp(self.amplitude - 0.00001, 0.0, 1.0);
        return ((self.phase * 2.0) - 1.0) * self.amplitude;
    }
}

impl Source for SawWaveOscilator {
    fn channels(&self) -> u16 {
        return 1;
    }

    fn sample_rate(&self) -> u32 {
        return self.sample_rate;
    }

    fn current_frame_len(&self) -> Option<usize> {
        return None;
    }

    fn total_duration(&self) -> Option<Duration> {
        return None;
    }
}

impl Iterator for SawWaveOscilator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        return Some(self.get_sample());
    }
}
