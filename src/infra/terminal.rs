use terminal_size::{terminal_size, Width};

pub fn get_term_width() -> usize {
    let (Width(width), _) = terminal_size().expect("Can not get terminal size");
    width as usize
}
