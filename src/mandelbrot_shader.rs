use vertex;

use glium::Program;
use glium::backend::glutin_backend::GlutinFacade;
use std::path::Path;
use std::fs::File;
use std::io::Read;

pub struct Shaders {
    pub program: Program,
}

impl Shaders {
    pub fn from_sources(display: &GlutinFacade, vert_path: &str, frag_path: &str) -> Shaders {
        Shaders {
            program: Program::from_source(display, include_str!("vert.glsl"), include_str!("frag.glsl"), None).unwrap()
        }
    }
}
