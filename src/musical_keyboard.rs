use crossterm::event::KeyCode;

pub fn frequency_from_keycode(c: KeyCode, octave: f32) -> Option<f32> {
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
pub fn calc_frequency(octave: f32, note: f32) -> f32 {
    return 440.0 * 2.0_f32.powf(((octave - 4.0) * 12.0 + note) / 12.0);
}

#[derive(Copy, Clone)]
pub enum NoteEvent {
    Press(f32),
    Hold,
    Up,
}
