//! An example of generating julia fractals.
use image;
use num_complex;

/// https://www.rapidtables.com/convert/color/hsv-to-rgb.html
fn hue_to_rgd(hue: u8) -> [u8; 3] {
	// Expecting to be S and V in HSV 1
	let c = 1;
	let x = c * (1 - )
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
            let image::(data) = *pixel;
            *pixel = image::Rgb([data[0], i, data[2]]);
        }
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save("renders/fractal.png").unwrap();
}
