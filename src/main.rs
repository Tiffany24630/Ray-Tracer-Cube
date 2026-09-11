mod camera;
mod color;
mod cube;
mod framebuffer;
mod light;
mod ray_intersect;

use minifb::{Key, Window, WindowOptions};
use nalgebra_glm::{dot, normalize, Vec3};
use std::f32::consts::PI;
use std::time::Duration;

use crate::camera::Camera;
use crate::color::Color;
use crate::cube::Cube;
use crate::framebuffer::Framebuffer;
use crate::light::Light;
use crate::ray_intersect::{Intersect, Material, RayIntersect};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const BACKGROUND_COLOR: u32 = 0x040C24;

const FOV: f32 = PI / 3.0;

const ROTATION_SPEED: f32 = PI / 60.0;

pub fn reflect(incident: &Vec3, normal: &Vec3) -> Vec3 {
    incident - normal * (2.0 * dot(incident, normal))
}

fn mix_colors(base: Color, accent: Color, factor: f32) -> Color {
    let influence = factor.clamp(0.0, 1.0);
    let r = ((base.r as f32 * (1.0 - influence) + accent.r as f32 * influence).round() as i32)
        .clamp(0, 255) as u8;
    let g = ((base.g as f32 * (1.0 - influence) + accent.g as f32 * influence).round() as i32)
        .clamp(0, 255) as u8;
    let b = ((base.b as f32 * (1.0 - influence) + accent.b as f32 * influence).round() as i32)
        .clamp(0, 255) as u8;

    Color::new(r, g, b)
}

fn wood_texture(point: &Vec3) -> Color {
    let radial = point.x * 6.0 + point.z * 4.0 + point.y * 1.5;
    let grain = (radial * 1.8).sin();
    let rings = ((point.x * 4.5 + point.z * 3.0 + point.y * 2.0).sin() * 0.5 + 0.5);
    let variation = (grain * 0.6 + rings * 0.4 + 1.0) * 0.5;

    let dark = Color::new(75, 46, 20);
    let light = Color::new(189, 133, 78);
    mix_colors(dark, light, variation)
}

pub fn shade(intersect: &Intersect, ray_origin: &Vec3, light: &Light) -> Color {
    let light_direction = (light.position - intersect.point).normalize();
    let view_direction = (ray_origin - intersect.point).normalize();

    let diffuse_intensity = dot(&intersect.normal, &light_direction).max(0.0);
    let wood_color = wood_texture(&intersect.point);
    let diffuse = wood_color * (diffuse_intensity * intersect.material.albedo[0] * light.intensity);

    let reflect_direction = reflect(&-light_direction, &intersect.normal);
    let specular_intensity = dot(&view_direction, &reflect_direction)
        .max(0.0)
        .powf(intersect.material.specular);

    let specular =
        light.color * (specular_intensity * intersect.material.albedo[1] * light.intensity);

    diffuse + specular
}

pub fn cast_ray(
    ray_origin: &Vec3,
    ray_direction: &Vec3,
    objects: &[Box<dyn RayIntersect>],
    light: &Light,
) -> Color {
    let mut closest: Option<Intersect> = None;

    for object in objects {
        if let Some(intersect) = object.ray_intersect(ray_origin, ray_direction) {
            if closest.is_none_or(|current| intersect.distance < current.distance) {
                closest = Some(intersect);
            }
        }
    }

    match closest {
        Some(intersect) => shade(&intersect, ray_origin, light),
        None => Color::from_hex(BACKGROUND_COLOR),
    }
}

pub fn render(
    framebuffer: &mut Framebuffer,
    objects: &[Box<dyn RayIntersect>],
    camera: &Camera,
    light: &Light,
) {
    let width = framebuffer.width as f32;
    let height = framebuffer.height as f32;
    let aspect_ratio = width / height;

    let perspective_scale = (FOV / 2.0).tan();

    for y in 0..framebuffer.height {
        for x in 0..framebuffer.width {
            let screen_x = (2.0 * x as f32) / width - 1.0;
            let screen_y = -(2.0 * y as f32) / height + 1.0;

            let screen_x = screen_x * aspect_ratio * perspective_scale;
            let screen_y = screen_y * perspective_scale;

            let ray_direction = normalize(&Vec3::new(screen_x, screen_y, -1.0));
            let ray_direction = camera.basis_change(&ray_direction);

            framebuffer
                .set_current_color(cast_ray(&camera.eye, &ray_direction, objects, light).to_hex());
            framebuffer.point(x, y);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wood_texture_changes_with_position() {
        let a = wood_texture(&Vec3::new(0.0, 0.0, 0.0));
        let b = wood_texture(&Vec3::new(0.75, 0.2, -0.3));
        assert_ne!(a.to_hex(), b.to_hex());
    }
}

fn main() {
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);

    let mut window = Window::new("Lakitu", WIDTH, HEIGHT, WindowOptions::default()).unwrap();

    let cube_material = Material::new(Color::new(120, 80, 45), 8.0, [0.95, 0.05]);

    let objects: Vec<Box<dyn RayIntersect>> = vec![Box::new(Cube::new(
        Vec3::new(-1.0, -1.0, -1.0),
        Vec3::new(1.0, 1.0, 1.0),
        cube_material,
    ))];

    let light = Light::new(Vec3::new(-6.0, 6.0, 8.0), Color::new(255, 255, 255), 1.5);

    let mut camera = Camera::new(
        Vec3::new(0.0, 0.0, 5.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );

    let mut camera_moved = true;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let orbit = [
            (Key::Left, ROTATION_SPEED, 0.0),
            (Key::Right, -ROTATION_SPEED, 0.0),
            (Key::Up, 0.0, -ROTATION_SPEED),
            (Key::Down, 0.0, ROTATION_SPEED),
        ];

        for (key, delta_yaw, delta_pitch) in orbit {
            if window.is_key_down(key) {
                camera.orbit(delta_yaw, delta_pitch);
                camera_moved = true;
            }
        }

        if camera_moved {
            render(&mut framebuffer, &objects, &camera, &light);
            camera_moved = false;
        }

        window
            .update_with_buffer(&framebuffer.buffer, WIDTH, HEIGHT)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
