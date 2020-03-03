use minifb::{Key, Window, WindowOptions};
use std::error::Error;

pub fn run_window(buffer: &Vec<u32>, width: usize, height: usize) -> Result<(), Box<dyn Error>> {
    let mut window = Window::new("Raytracer", width, height, WindowOptions::default())?;
    window.limit_update_rate(Some(std::time::Duration::from_micros(1_000_000)));

    while window.is_open() && !window.is_key_down(Key::Enter) {
        window.update_with_buffer(&buffer, width, height)?;
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }

    Ok(())
}
