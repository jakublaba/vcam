use std::f32::consts::PI;

use cgmath::{Matrix4, Vector3};

pub fn translation(v: Vector3<f64>) -> Matrix4<f64> {
    Matrix4::new(
        1., 0., 0., v.x,
        0., 1., 0., v.y,
        0., 0., 1., v.z,
        0., 0., 0., 1.,
    )
}

pub fn scaling(s: f64) -> Matrix4<f64> {
    Matrix4::new(
        s, 0., 0., 0.,
        0., s, 0., 0.,
        0., 0., s, 0.,
        0., 0., 0., 1.,
    )
}

pub fn x_rotation(rot: f64) -> Matrix4<f64> {
    Matrix4::new(
        1., 0., 0., 0.,
        0., rot.cos(), -rot.sin(), 0.,
        0., rot.sin(), rot.cos(), 0.,
        0., 0., 0., 1.,
    )
}

pub fn y_rotation(rot: f64) -> Matrix4<f64> {
    Matrix4::new(
        rot.cos(), 0., rot.sin(), 0.,
        0., 1., 0., 0.,
        -rot.sin(), 0., rot.cos(), 0.,
        0., 0., 0., 1.,
    )
}

pub fn z_rotation(rot: f64) -> Matrix4<f64> {
    Matrix4::new(
        rot.cos(), -rot.sin(), 0., 0.,
        rot.sin(), rot.cos(), 0., 0.,
        0., 0., 1., 0.,
        0., 0., 0., 1.,
    )
}

pub fn projection(fov: f64, ar: f64, near: f64, far: f64) -> Matrix4<f64> {
    Matrix4::new(
        1. / ((0.5 * fov).tan() * ar), 0., 0., 0.,
        0., 1. / (0.5 * fov).tan(), 0., 0.,
        0., 0., (far + near) / (near - far), 2. * far * near / (near - far),
        0., 0., -1., 0.,
    )
}
