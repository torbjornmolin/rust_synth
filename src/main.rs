use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use math::round;
use rodio::{source::Source, OutputStream};
use saw_wave_oscilator::SawWaveOscilator;
use saw_wave_oscilator_band_limited::SawWaveOscilatorBandLimited;
use wave_table_oscilator::WavetableOscillator;

use std::sync::mpsc::{self};
mod saw_wave_oscilator;
mod saw_wave_oscilator_band_limited;
mod wave_table_oscilator;
///    Calculate the frequency of any note!
/// frequency = 440×(2^(n/12))
///
/// N=0 is A4
/// N=1 is A#4
/// etc...
///
/// notes go like so...
/// 0  = A
/// 1  = A#
/// 2  = B
/// 3  = C
/// 4  = C#
/// 5  = D
/// 6  = D#
/// 7  = E
/// 8  = F
/// 9  = F#
/// 10 = G
/// 11 = G#
fn calc_frequency(octave: f32, note: f32) -> f32 {
    return 440.0 * 2.0_f32.powf(((octave - 4.0) * 12.0 + note) / 12.0);
}

fn main() {
    let wave_table_size = 64;
    let mut wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);

    for n in 0..wave_table_size {
        let sample: f32 = (2.0
            * (2.0 * round::floor(n as f64 / wave_table_size as f64, 0)
                - round::floor(2.0 * n as f64 / wave_table_size as f64, 0))
            + 1.0) as f32;
        wave_table.push(sample);
    }

    let (tx, rx) = mpsc::channel();

    //let oscillator = WavetableOscillator::new(44100, wave_table, rx);
    let oscillator = SawWaveOscilatorBandLimited::new(44100, rx);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let _result = stream_handle.play_raw(oscillator.convert_samples());

    listen_for_keyboard(tx);
}

fn listen_for_keyboard(tx: mpsc::Sender<f32>) {
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

fn frequency_from_keycode(c: KeyCode, octave: f32) -> Option<f32> {
    match c {
        KeyCode::Char('a') => Some(calc_frequency(octave, 3.0)), // C
        KeyCode::Char('w') => Some(calc_frequency(octave, 4.0)), // C#
        KeyCode::Char('s') => Some(calc_frequency(octave, 5.0)), // D
        KeyCode::Char('e') => Some(calc_frequency(octave, 6.0)), // D#
        KeyCode::Char('d') => Some(calc_frequency(octave, 7.0)), // E
        KeyCode::Char('f') => Some(calc_frequency(octave, 8.0)), // F
        KeyCode::Char('t') => Some(calc_frequency(octave, 9.0)), // F#
        KeyCode::Char('g') => Some(calc_frequency(octave, 10.0)), // G
        KeyCode::Char('y') => Some(calc_frequency(octave, 11.0)), // G#
        KeyCode::Char('h') => Some(calc_frequency(octave + 1.0, 0.0)), // A
        KeyCode::Char('u') => Some(calc_frequency(octave + 1.0, 1.0)), // A#
        KeyCode::Char('j') => Some(calc_frequency(octave + 1.0, 2.0)), // B
        KeyCode::Char('k') => Some(calc_frequency(octave + 1.0, 3.0)), // C

        _ => None,
    }
}
