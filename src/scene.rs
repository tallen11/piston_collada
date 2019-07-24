use vecmath::Matrix4;

#[derive(Debug, Clone)]
pub struct Camera {
    pub transform: Matrix4<f32>,
}

#[derive(Debug, Clone)]
pub struct Light {
    pub transform: Matrix4<f32>,
}
