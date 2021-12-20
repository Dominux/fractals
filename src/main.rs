use std::convert::TryInto;

use image;
use num_complex;

/// https://www.rapidtables.com/convert/color/hsv-to-rgb.html
#[allow(non_upper_case_globals)]
fn hue_to_rgd(hue: u8) -> [u8; 3] {
    // Expecting to be S and V in HSV 1
    const c: i32 = 1;
    let hh = hue as f32 / 60.0;
    let x = c * (1.0 - (hh % 2.0 - 1.0).abs()) as i32;

    let result = match hh as i8 {
        0 => [c, x, 0],
        1 => [x, c, 0],
        2 => [0, c, x],
        3 => [0, x, c],
        4 => [x, 0, c],
        _ => [c, 0, x],
    };

    result
        .into_iter()
        .map(|x| (x * 255) as u8)
        .collect::<Vec<_>>()
        .try_into()
        .expect("lol")
}

fn main() {
    let imgx = 800;
    let imgy = 800;

    let scalex = 3.0 / imgx as f32;
    let scaley = 3.0 / imgy as f32;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // Iterate over the coordinates and pixels of the image
    // for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
    //     *pixel = image::Rgb([0, 0, 1]);
    // }

    // A redundant loop to demonstrate reading image data
    for x in 0..imgx {
        for y in 0..imgy {
            let cx = y as f32 * scalex - 1.5;
            let cy = x as f32 * scaley - 1.5;

            let c = num_complex::Complex::new(-0.7269, 0.1889);
            let mut z = num_complex::Complex::new(cx, cy);

            let mut i: u8 = 0;
            while i < 255 && z.norm() <= 2.0 {
                z = z * z + c;
                i += 1;
            }
            // println!("{}", i);

            let pixel = imgbuf.get_pixel_mut(x, y);
            let image::Rgb(data) = *pixel;
            *pixel = image::Rgb(hue_to_rgd(i));
        }
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save("renders/fractal.png").unwrap();
}
