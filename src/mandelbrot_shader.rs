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
        let (vert_path, frag_path) = (Path::new(vert_path), Path::new(frag_path));
        let (mut vert_file, mut frag_file) = (File::open(vert_path).unwrap(), File::open(frag_path).unwrap());
        let (mut vert_src, mut frag_src) = (String::new(), String::new());

        vert_file.read_to_string(&mut vert_src);
        frag_file.read_to_string(&mut frag_src);

        Shaders {
            program: Program::from_source(display, &vert_src, &frag_src, None).unwrap()
        }
    }
}
