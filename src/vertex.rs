#[derive(Copy, Clone)]
pub struct Vertex {
    position: [f32; 2]
}

implement_vertex!(Vertex, position);

pub const SCREEN_VERTS: [Vertex; 4] = [
    Vertex { position: [-1.0, -1.0] },
    Vertex { position: [-1.0,  1.0] },
    Vertex { position: [ 1.0, -1.0] },
    Vertex { position: [ 1.0,  1.0] },
];
