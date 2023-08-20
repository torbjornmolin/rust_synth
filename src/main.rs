use std::sync::mpsc;

use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use dataconverter::DataConverter;
use musical_keyboard::{frequency_from_keycode, NoteEvent};
use rodio::{OutputStream, Source};
use saw_wave_oscilator_band_limited::SawWaveOscilatorBandLimited;
mod dataconverter;
pub mod envvelope;
mod music_data;
mod musical_keyboard;
mod saw_wave_oscilator;
mod saw_wave_oscilator_band_limited;
mod wave_table_oscilator;

fn main() {
    let (tx, rx) = mpsc::channel();

    //let oscillator = WavetableOscillator::new(44100, wave_table, rx);
    let oscillator = SawWaveOscilatorBandLimited::new(44100, rx);

    // let envelope = Envelope::new::<SawWaveOscilatorBandLimited>(oscillator);
    let data_converter = DataConverter::new::<SawWaveOscilatorBandLimited>(oscillator);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let _result = stream_handle.play_raw(data_converter.convert_samples());

    listen_for_keyboard(tx);
}

fn listen_for_keyboard(mut tx: mpsc::Sender<NoteEvent>) {
    enable_raw_mode().unwrap();
    let mut current_octave = 1.0;
    loop {
        match read().unwrap() {
            Event::Key(KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            }) => break,
            Event::Key(KeyEvent {
                code: c,
                modifiers: KeyModifiers::NONE,
                kind: KeyEventKind::Press,
                state: KeyEventState::NONE,
            }) => {
                if c == KeyCode::Char('9') {
                    current_octave += 1.0;
                } else if c == KeyCode::Char('8') {
                    current_octave -= 1.0;
                } else {
                    let f = frequency_from_keycode(c, current_octave);
                    match f {
                        Some(f) => tx.send(NoteEvent::Press(f)).unwrap(),
                        None => (),
                    }
                }
            }

            _ => (),
        }
    }
    disable_raw_mode().unwrap();
}
