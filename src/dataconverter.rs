use std::time::Duration;

use rodio::Source;

use crate::musicdata::MusicData;
pub struct DataConverter<I>
where
    I: Iterator<Item = MusicData> + Source,
{
    upstream_source: I,
}

impl<T> DataConverter<T>
where
    T: Iterator<Item = MusicData> + Source,
{
    pub(crate) fn new<I>(upstream: T) -> DataConverter<T>
    where
        I: Iterator,
    {
        DataConverter {
            upstream_source: upstream,
        }
    }
}

impl<T> Iterator for DataConverter<T>
where
    T: Iterator<Item = MusicData> + Source,
{
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.upstream_source.next();
        //println!("{}\n\r", value.unwrap());
        if value.is_some() {
            return Some(value.unwrap().wave_data);
        }
        None
    }
}

impl<T> Source for DataConverter<T>
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
