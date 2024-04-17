mod color;

fn main() {
    // Image
    let image_width = 256;
    let image_height = 256;

    // Render
    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    for i in 0..image_height {
        for j in 0..image_width {
            let pixel = {
                let r = f64::from(i) / f64::from(image_width - 1);
                let g = f64::from(j) / f64::from(image_height - 1);
                let b = 0f64;
                color::RgbReal {
                    red: r,
                    green: g,
                    blue: b,
                }
            };

            let ppm_pixel: color::RgbInt = pixel.into();
            println!("{}", ppm_pixel.to_ppm_pixel());
        }
    }
}
