
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Matrix4x4 {
    pub data: [[f32; 4]; 4],
}

impl Matrix4x4 {
    pub fn identity() -> Self {
        Matrix4x4 { 
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 1.0],
                [0.0, 0.0, 1.0, 2.0],
                [0.0, 0.0, 0.0, 1.0],
            ]
        }
    }

    pub fn from_translation(t: &[f32; 3]) -> Self {
        Matrix4x4 { 
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [t[0], t[1], t[2], 1.0],
            ]
        }
    }

    pub fn scale(&mut self, scale: &[f32; 3]) -> Self {
        Self { 
            data: [
                [self.data[0][0] * scale[0], self.data[0][1], self.data[0][2], self.data[0][3]],
                [self.data[1][0], self.data[1][1] * scale[1], self.data[1][2], self.data[1][3]],
                [self.data[2][0], self.data[2][1], self.data[2][2] * scale[2], self.data[2][3]],
                [self.data[3][0], self.data[3][1], self.data[3][2], self.data[3][3]],
            ]
        }
    }

    pub fn translate(self, t: &[f32; 3]) -> Self {
        Matrix4x4 { 
            data: [
                [self.data[0][0], self.data[0][1], self.data[0][2], self.data[0][3]],
                [self.data[1][0], self.data[1][1], self.data[1][2], self.data[1][3]],
                [self.data[2][0], self.data[2][1], self.data[2][2], self.data[2][3]],
                // Only the last row changes, because we are using row-major order
                [self.data[3][0] + t[0], self.data[3][1] + t[1], self.data[3][2] + t[2], self.data[3][3]],
            ]
        }
    }
}