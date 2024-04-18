use indicatif::ProgressBar;

mod color;
mod vector;

fn main() {
    // Image
    let image_width = 256u32;
    let image_height = 256u32;

    // Render
    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    let bar = ProgressBar::new(u64::from(image_width * image_height));
    bar.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("Pixels Rendered: {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .unwrap()
            .progress_chars("#>-"),
    );

    for i in 0..image_height {
        for j in 0..image_width {
            bar.inc(1);
            let pixel = {
                let red = f64::from(i) / f64::from(image_width - 1);
                let green = f64::from(j) / f64::from(image_height - 1);
                let blue = 0f64;
                color::RgbReal::new(red, green, blue)
            };

            let ppm_pixel: color::RgbInt = pixel.into();
            println!("{}", ppm_pixel.to_ppm_pixel());
        }
    }
    bar.finish();
}
