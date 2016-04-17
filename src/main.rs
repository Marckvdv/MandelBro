#[macro_use]
extern crate glium;

mod mandelbrot_window;
mod vertex;
mod mandelbrot_shader;
mod event_handler;

fn main() {
    let mut window = mandelbrot_window::MandelbrotWindow::new(1280, 720);
    window.run();
}
