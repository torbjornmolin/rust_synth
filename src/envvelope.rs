use std::time::Duration;

use rodio::Source;

use crate::music_data::MusicData;

pub enum EnvelopeState {
    Flat,
    Attack,
    Hold,
    Release,
}
pub struct Envelope<I>
where
    I: Iterator<Item = MusicData> + Source,
{
    upstream_source: I,
    current_multiplier: f32,
    state: EnvelopeState,
    attack_rate: f32,
    release_rate: f32,
}

impl<T> Envelope<T>
where
    T: Iterator<Item = MusicData> + Source,
{
    pub(crate) fn new<I>(upstream: T) -> Envelope<T>
    where
        I: Iterator,
    {
        Envelope {
            upstream_source: upstream,
            current_multiplier: 0.0,
            state: EnvelopeState::Flat,
            attack_rate: (1.0 / 44100.0) * 1.0, // value per sample added to multiplier. 0.2s attack
            release_rate: -(1.0 / 44100.0) * 1.0, // value per sample added to multiplier. TODO: dynamic with sample rate. 1s release
        }
    }
}

impl<T> Iterator for Envelope<T>
where
    T: Iterator<Item = MusicData> + Source,
{
    type Item = MusicData;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.upstream_source.next();

        if let Some(music_data) = value {
            let mut result = MusicData {
                current_event: music_data.current_event,
                wave_data: music_data.wave_data,
            };

            if let Some(event) = music_data.current_event {
                match event {
                    crate::musical_keyboard::NoteEvent::Press(_) => {
                        self.current_multiplier = 0.0;
                        self.state = EnvelopeState::Attack
                    }
                    crate::musical_keyboard::NoteEvent::Hold => (),
                    crate::musical_keyboard::NoteEvent::Up => self.state = EnvelopeState::Release,
                }
            }

            match self.state {
                EnvelopeState::Flat => {
                    // print!(
                    //     "Flat: {}, {}\r\n",
                    //     self.current_multiplier, self.attack_rate
                    // );
                    result.wave_data = 0.0
                }

                EnvelopeState::Attack => {
                    self.current_multiplier += self.attack_rate;
                    //print!("Attack: {}, {}", self.current_multiplier, self.attack_rate);
                    if self.current_multiplier >= 1.0 {
                        // print!("Attack: {}\r\n", self.current_multiplier);
                        self.current_multiplier = 1.0;
                        self.state = EnvelopeState::Hold
                    }
                    result.wave_data = result.wave_data * self.current_multiplier;
                }
                EnvelopeState::Hold => {
                    // print!("Hold\r\n");
                    // leave the amplitude unchanged.
                }
                EnvelopeState::Release => {
                    // print!("Release\r\n");
                    self.current_multiplier += self.release_rate;
                    if self.current_multiplier <= 0.0 {
                        self.current_multiplier = 0.0;
                        self.state = EnvelopeState::Flat
                    }
                    result.wave_data = result.wave_data * self.current_multiplier;
                }
            }
            return Some(result);
        } else {
            return value;
        }
    }
}

impl<T> Source for Envelope<T>
where
    T: Iterator<Item = MusicData> + Source,
{
    fn channels(&self) -> u16 {
        self.upstream_source.channels()
    }

    fn sample_rate(&self) -> u32 {
        self.upstream_source.sample_rate()
    }

    fn current_frame_len(&self) -> Option<usize> {
        self.upstream_source.current_frame_len()
    }

    fn total_duration(&self) -> Option<Duration> {
        self.upstream_source.total_duration()
    }
}
