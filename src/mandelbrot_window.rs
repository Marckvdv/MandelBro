use vertex;
use mandelbrot_shader::Shaders;
use event_handler::EventHandler;

use glium;
use glium::{DisplayBuild, Surface};
use glium::backend::glutin_backend::GlutinFacade;
use std::default::Default;

const MAX_ITER: i32 = 1000;

pub struct MandelbrotWindow {
    display: GlutinFacade,
    shaders: Shaders,
    res: [f32; 2],
    event_handler: EventHandler,
    center: [f32; 2],
    zoom: f32,
}

impl MandelbrotWindow {
    pub fn new(width: u32, height: u32) -> MandelbrotWindow {
        let display = glium::glutin::WindowBuilder::new()
            .with_dimensions(width, height)
            .with_title("Mandelbrot viewer".to_string())
            .build_glium().expect("Failed to build glium");
        let shaders = Shaders::from_sources(&display, "vert.glsl", "frag.glsl");

        MandelbrotWindow {
            display: display,
            shaders: shaders,
            res: [width as f32, height as f32],
            event_handler: EventHandler::new(),
            center: [0f32, 0f32],
            zoom: 1.0,
        }
    }

    fn draw_mandelbrot(&self, center: [f32; 2], zoom: f32) {
        let mut target = self.display.draw();
        target.clear_color(1.0, 0.0, 0.0, 0.0);

        let vertex_buffer = glium::VertexBuffer::new(&self.display, &vertex::SCREEN_VERTS).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);
        let uniforms = uniform! {
            res: self.res,
            MAX_ITER: MAX_ITER,
            center: center,
            zoom: zoom,
        };

        target.draw(
            &vertex_buffer,
            &indices,
            &self.shaders.program,
            &uniforms,
            &Default::default()).expect("Failed to draw frame to display");

        target.finish().expect("Failed to display the new frame!");
    }

    pub fn run(&mut self) {
        let mut frame = 0;

        while self.handle_events() {
            self.draw_mandelbrot(self.center, self.zoom);

            frame += 1;
        }
    }

    fn handle_events(&mut self) -> bool {
        for ev in self.display.poll_events() {
            self.event_handler.handle_event(ev);
            if self.event_handler.is_mouse_pressed(0) {
                let start = self.event_handler.mouse_position_prev;
                let current = self.event_handler.mouse_position;

                let delta = [start.0 - current.0, -start.1 + current.1];
                let delta = [delta[0] as f32 / self.res[0] / self.zoom, delta[1] as f32 / self.res[1] / self.zoom];
                self.center = [self.center[0] + delta[0], self.center[1] + delta[1]];
            }

            self.zoom *= 1.0 + self.event_handler.scroll_amount * 0.05;
        }

        return !self.event_handler.is_closed();
    }
}
