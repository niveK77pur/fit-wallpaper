use std::path::Path;

use bokeh::{Blur, params::KERNEL9_PARAM_SET};
use image::{ImageError, io::Reader as ImageReader};

fn blur(path: impl AsRef<Path>) -> Result<(), ImageError> {
    // read the image
    let mut img = ImageReader::open(path)?.decode()?;
    // as the `bokeh::Blur` trait is imported
    img.bokeh_blur(10.0, &KERNEL9_PARAM_SET, 3.0);
    // save the image
    img.save("output.png")?;
    Ok(())
}

fn main() {
    blur("image.jpg").unwrap();
}
