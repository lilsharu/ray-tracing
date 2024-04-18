use indicatif::ProgressBar;

use crate::color::RgbReal;
use crate::vector::ray::Ray;
use crate::vector::vector::{Point3d, Vector3d};

mod color;
mod vector;

fn ray_color(ray: &Ray) -> RgbReal {
    RgbReal::new(0.0, 0.0, 0.0)
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;

    // Image
    let image_width = 400;
    let image_height = (f64::from(image_width) / aspect_ratio).round().max(1.0) as i32;

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * f64::from(image_width) / f64::from(image_height);
    let camera_center = Point3d::new(0.0, 0.0, 0.0);

    // Viewport Spanning Vectors
    let viewport_u = Vector3d::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vector3d::new(-viewport_height, 0.0, 0.0);
    let pixel_delta_u = viewport_u / f64::from(image_width);
    let pixel_delta_v = viewport_v / f64::from(image_height);

    // Upper Left Pixel
    let viewport_upper_left =
        camera_center - Vector3d::new(0.0, 0.0, focal_length) - (viewport_u + viewport_v) / 2.0;
    let pixel00_location = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    // Render
    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    let bar = ProgressBar::new(u64::try_from(image_width * image_height).unwrap());
    bar.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("Pixels Rendered: {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .unwrap()
            .progress_chars("#>-"),
    );

    for i in 0..image_height {
        for j in 0..image_width {
            bar.inc(1);
            let pixel_center =
                pixel00_location + pixel_delta_u * f64::from(i) + pixel_delta_v * f64::from(j);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);
            let pixel_color = ray_color(&ray);
            let ppm_pixel: color::RgbInt = pixel_color.into();
            println!("{}", ppm_pixel.to_ppm_pixel());
        }
    }
    bar.finish();
}
