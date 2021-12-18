use crate::{Vertex, all_faces};

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub center: [f32;3],
    pub faces: [[Vertex; 6]; 6],
}

impl Block {

    pub fn vertices(&self) -> Vec<Vertex> {
        self.faces.concat()
    }
    pub fn new(center: [f32; 3]) -> Block {
        let vertices = all_faces(center);
        Block {
            center,
            faces: vertices,
        }
    }
}