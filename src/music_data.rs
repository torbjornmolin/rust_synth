use std::sync::Arc;

use crate::musical_keyboard::NoteEvent;
use rodio::Sample;

#[derive(Copy, Clone)]
pub struct MusicData {
    pub current_event: Option<NoteEvent>,
    pub wave_data: f32,
}

unsafe impl rodio::cpal::Sample for MusicData {
    const FORMAT: rodio::cpal::SampleFormat = rodio::cpal::SampleFormat::F32;

    fn to_f32(&self) -> f32 {
        self.wave_data
    }

    fn to_i16(&self) -> i16 {
        self.wave_data as i16
    }

    fn to_u16(&self) -> u16 {
        self.wave_data as u16
    }

    fn from<S>(s: &S) -> Self
    where
        S: rodio::cpal::Sample,
    {
        todo!()
    }
}

impl Sample for MusicData {
    fn lerp(first: Self, second: Self, numerator: u32, denominator: u32) -> Self {
        let wave_data = first.wave_data
            + (second.wave_data - first.wave_data) * numerator as f32 / denominator as f32;
        MusicData {
            current_event: first.current_event,
            wave_data,
        }
    }

    fn amplify(self, value: f32) -> Self {
        MusicData {
            current_event: self.current_event,
            wave_data: self.wave_data * value,
        }
    }

    fn saturating_add(self, other: Self) -> Self {
        MusicData {
            current_event: self.current_event,
            wave_data: self.wave_data + other.wave_data,
        }
    }

    fn zero_value() -> Self {
        MusicData {
            current_event: None,
            wave_data: 0.0,
        }
    }
}
