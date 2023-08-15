use ::num::clamp;
use math::round;
use rodio::Source;
use std::{sync::mpsc::Receiver, time::Duration};
pub struct WavetableOscillator {
    sample_rate: u32,
    wave_table: Vec<f32>,
    index: f32,
    index_increment: f32,
    receiver: Receiver<f32>,
    amplitude: f32,
    current_frequency: f32,
}

impl WavetableOscillator {
    pub fn new_sinwave(sample_rate: u32, receiver: Receiver<f32>) -> WavetableOscillator {
        let wave_table_size = 64;
        let mut wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);

        for n in 0..wave_table_size {
            let sample: f32 = (2.0
                * (2.0 * round::floor(n as f64 / wave_table_size as f64, 0)
                    - round::floor(2.0 * n as f64 / wave_table_size as f64, 0))
                + 1.0) as f32;
            wave_table.push(sample);
        }

        Self::new(sample_rate, wave_table, receiver)
    }
    pub fn new(
        sample_rate: u32,
        wave_table: Vec<f32>,
        receiver: Receiver<f32>,
    ) -> WavetableOscillator {
        return WavetableOscillator {
            sample_rate,
            wave_table,
            index: 0.0,
            index_increment: 0.0,
            receiver,
            amplitude: 0.0,
            current_frequency: 0.0,
        };
    }

    fn set_frequency(&mut self, frequency: f32) {
        if frequency != self.current_frequency {
            self.index_increment =
                frequency * self.wave_table.len() as f32 / self.sample_rate as f32;
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
        let sample = self.lerp() * self.amplitude;
        self.index += self.index_increment;
        self.index %= self.wave_table.len() as f32;
        self.amplitude = clamp(self.amplitude - 0.00001, 0.0, 1.0);
        return sample;
    }

    fn lerp(&self) -> f32 {
        let truncated_index = self.index as usize;
        let next_index = (truncated_index + 1) % self.wave_table.len();

        let next_index_weight = self.index - truncated_index as f32;
        let truncated_index_weight = 1.0 - next_index_weight;

        return truncated_index_weight * self.wave_table[truncated_index]
            + next_index_weight * self.wave_table[next_index];
    }
}

impl Source for WavetableOscillator {
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

impl Iterator for WavetableOscillator {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        return Some(self.get_sample());
    }
}
