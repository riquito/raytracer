use minifb::{Key, Window, WindowOptions};
use std::error::Error;

const WIDTH: usize = 640;
const HEIGHT: usize = 400;

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut window = Window::new("Test", WIDTH, HEIGHT, WindowOptions::default())?;
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    window.limit_update_rate(Some(std::time::Duration::from_micros(1000000)));

    while window.is_open() && !window.is_key_down(Key::Enter) {
        for (i, bi) in buffer.iter_mut().enumerate() {
            let r = (i % WIDTH) as f64 / WIDTH as f64;
            let g = 1.0 - (i as f64 / WIDTH as f64 / HEIGHT as f64);
            let b = 0.2 as f64;
            let ir = (255.0 * r) as u32;
            let ig = (255.0 * g) as u32;
            let ib = (255.0 * b) as u32;
            *bi = (ir << 16) | (ig << 8) | ib;

            //*bi = if (WIDTH * HEIGHT) / 2 > i { 0 } else { 255 };
        }
        window.update_with_buffer(&buffer, WIDTH, HEIGHT)?;
    }
    Ok(())
}
