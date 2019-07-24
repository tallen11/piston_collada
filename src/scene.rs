use vecmath::Matrix4;

#[derive(Debug)]
pub struct Camera {
    pub transform: Matrix4<f32>,
}

#[derive(Debug)]
pub struct Light {
    pub transform: Matrix4<f32>,
}
