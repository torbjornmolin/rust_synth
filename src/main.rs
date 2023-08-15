use std::sync::mpsc;

use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use envvelope::Envelope;
use math::round;
use musical_keyboard::frequency_from_keycode;
use rodio::{source::Source, OutputStream};
use saw_wave_oscilator::SawWaveOscilator;
use saw_wave_oscilator_band_limited::SawWaveOscilatorBandLimited;
use wave_table_oscilator::WavetableOscillator;

pub mod envvelope;
mod musical_keyboard;
mod saw_wave_oscilator;
mod saw_wave_oscilator_band_limited;
mod wave_table_oscilator;

fn main() {
    let (tx, rx) = mpsc::channel();

    //let oscillator = WavetableOscillator::new(44100, wave_table, rx);
    let oscillator = SawWaveOscilatorBandLimited::new(44100, rx);

    let envelope = Envelope::new::<SawWaveOscilatorBandLimited>(oscillator);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let _result = stream_handle.play_raw(envelope.convert_samples());

    listen_for_keyboard(tx);
}

fn listen_for_keyboard(mut tx: mpsc::Sender<f32>) {
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
                        Some(f) => tx.send(f).unwrap(),
                        None => (),
                    }
                }
            }

            _ => (),
        }
    }
    disable_raw_mode().unwrap();
}
