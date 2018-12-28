//
// this is an example and not very interesting
//


use std::f32::consts::PI;

pub struct Triangle {
    angle: f32,
    //vertices: [f32; 9],
}

impl Triangle {
    pub fn new() -> Triangle {
        let r = ((2.0 * PI) / 3.0) as f32;
        Triangle {
            angle: 0.0,
        }
    }
    pub fn rotate(&mut self, add: f32) {
        self.angle += add;
    }
    pub fn get_vertices(&self) -> [f32; 9] {
        let a = self.angle;
        let r = ((2.0 * PI) / 3.0) as f32;
        // return the vertices of our triangle
        [
            a.cos() * 0.7, a.sin() * 0.7, 0.0,
            (a + r).cos() * 0.7, (a + r).sin() * 0.7, 0.0,
            (a + r * 2.0).cos() * 0.7, (a + r * 2.0).sin() * 0.7, 0.0
        ]
    }
}