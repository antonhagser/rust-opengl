extern crate nalgebra as na;

pub struct Camera {
    proj: na::Matrix4::<f32>,
    view: na::Matrix4::<f32>,
}

impl Camera {
    // pub fn new_perspective() -> Camera {
    //     let proj = na::Matrix4::<f32>::identity();
    //     let view = na::Matrix4::<f32>::identity();

    //     Camera {
    //         proj,
    //         view
    //     }
    // }

    pub fn ortho(left: f32, right: f32, bottom: f32, top: f32, znear: f32, zfar: f32) -> Camera {
        let mut proj = na::Matrix4::<f32>::identity();
        let two: f32 = num::cast(2.0).unwrap();
        proj[(0, 0)] = two / (right - left);
        proj[(1, 1)] = two / (top - bottom);
        proj[(2, 2)] = -two / (zfar - znear);
        proj[(0, 3)] = -(right + left) / (right - left);
        proj[(1, 3)] = -(top + bottom) / (top - bottom);
        proj[(2, 3)] = -(zfar + znear) / (zfar - znear);
        proj[(3, 3)] = 1.0;

        let view = na::Matrix4::<f32>::identity();

        Camera {
            proj,
            view
        }
    }
}