use std::time::Duration;

use rodio::Source;
pub struct Envelope<I>
where
    I: Iterator<Item = f32> + Source,
{
    upstream_source: I,
}

impl<T> Envelope<T>
where
    T: Iterator<Item = f32> + Source,
{
    pub(crate) fn new<I>(upstream: T) -> Envelope<T>
    where
        I: Iterator,
    {
        Envelope {
            upstream_source: upstream,
        }
    }
}

impl<T> Iterator for Envelope<T>
where
    T: Iterator<Item = f32> + Source,
{
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let value = self.upstream_source.next();
        //println!("{}\n\r", value.unwrap());
        value
    }
}

impl<T> Source for Envelope<T>
where
    T: Iterator<Item = f32> + Source,
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
