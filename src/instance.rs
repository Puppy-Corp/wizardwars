use crate::matrix::Matrix4x4;


#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Instance {
    model: Matrix4x4,
}

impl Instance {
    pub fn new(model: Matrix4x4) -> Self {
        Self {
            model
        }
    }
}


// /**
//  * [[1, 1, 1]
//  * [1, 1, 1]
//  * [1, 1, 1]]
//  * 
//  */