use num::clamp;
use rodio::Source;
use std::{sync::mpsc::Receiver, time::Duration};

use crate::musical_keyboard::NoteEvent;

pub struct SawWaveOscilatorBandLimited {
    sample_rate: u32,
    receiver: Receiver<NoteEvent>,
    amplitude: f32,
    current_frequency: f32,
    phase: f32,
    current_event: Option<NoteEvent>,
}

impl SawWaveOscilatorBandLimited {
    pub fn new(sample_rate: u32, receiver: Receiver<NoteEvent>) -> SawWaveOscilatorBandLimited {
        return SawWaveOscilatorBandLimited {
            sample_rate,
            receiver,
            amplitude: 0.0,
            current_frequency: 0.0,
            phase: 0.0,
            current_event: None,
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
                match f {
                    NoteEvent::Press(frequency) => self.set_frequency(frequency),
                    NoteEvent::Hold => {}
                    NoteEvent::Up => {}
                }
                self.current_event = Some(f);
                //self.set_frequency(f);
            }
            Err(_) => (),
        }

        //advance the phase
        self.phase = self.phase
            + 2.0 * std::f32::consts::PI * self.current_frequency / self.sample_rate as f32;

        while self.phase >= 2.0 * std::f32::consts::PI {
            self.phase -= 2.0 * std::f32::consts::PI;
        }

        while self.phase < 0.0 {
            self.phase += 2.0 * std::f32::consts::PI;
        }

        let mut number_of_harmonics = 0;
        //if num harmonics is zero, calculate how many max harmonics we can do
        //without going over the nyquist frequency (half of sample rate frequency)
        if number_of_harmonics == 0 && self.current_frequency != 0.0 {
            let mut temporary_frequency = self.current_frequency;

            while temporary_frequency < self.sample_rate as f32 * 0.5 {
                number_of_harmonics = number_of_harmonics + 1;
                temporary_frequency = temporary_frequency * 2.0;
            }
        }

        //calculate the saw wave sample
        let mut result = 0.0;
        for current_harmonic in 1..number_of_harmonics {
            result =
                result + (self.phase * current_harmonic as f32).sin() / current_harmonic as f32;
        }

        //adjust the volume
        result = result * 2.0 / std::f32::consts::PI;

        self.amplitude = clamp(self.amplitude - 0.00001, 0.0, 1.0); // decay

        return result * self.amplitude;
    }
}

impl Source for SawWaveOscilatorBandLimited {
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

impl Iterator for SawWaveOscilatorBandLimited {
    type Item = crate::musicdata::MusicData;

    fn next(&mut self) -> Option<Self::Item> {
        return Some(crate::musicdata::MusicData {
            current_event: self.current_event,
            wave_data: self.get_sample(),
        });
    }
}
