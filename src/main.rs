use clap::Parser;

use bokeh::{Blur, params::KERNEL9_PARAM_SET};
use image::{DynamicImage, GenericImageView, imageops::FilterType, io::Reader as ImageReader};
use tracing::{Level, error, info};
use tracing_subscriber::FmtSubscriber;

/// Resize image to fit into dimensions and fill empty space with a bokeh blurred version of the
/// image.
#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Input image path
    #[arg(long)]
    image: String,
    /// Output image path
    #[arg(long)]
    output: String,
    /// Width of the output image
    #[arg(long, default_value = "1920")]
    width: u32,
    /// Height of the output image
    #[arg(long, default_value = "1080")]
    height: u32,
    /// Radius for the bokeh blur
    #[arg(long, default_value = "20")]
    radius: f64,
}

fn main() {
    tracing::subscriber::set_global_default(
        FmtSubscriber::builder()
            .with_max_level(Level::TRACE)
            .finish(),
    )
    .expect("setting default subscriber failed");

    let args = Args::parse();

    info!(args.image, "Reading image");
    let mut img: DynamicImage = match ImageReader::open(args.image) {
        Ok(buf) => match buf.decode() {
            Ok(i) => i,
            Err(e) => {
                error!("Could not decode image: {e:?}");
                return;
            }
        },
        Err(e) => {
            error!("Could not read image: {e:?}");
            return;
        }
    };
    let mut background: DynamicImage = img.clone();

    info!(
        args.width,
        args.height, "Resizing image to specified dimensions"
    );
    background = background.resize_to_fill(args.width, args.height, FilterType::Lanczos3);
    img = img.resize(args.width, args.height, FilterType::Lanczos3);

    info!(args.radius, "Blurring background");
    background.bokeh_blur(args.radius, &KERNEL9_PARAM_SET, 3.0);

    info!("Overlay image in the center");
    let background_size = background.dimensions();
    let img_size = img.dimensions();
    let x = (img_size.0 as i64 - background_size.0 as i64) / 2;
    let y = (img_size.1 as i64 - background_size.1 as i64) / 2;
    image::imageops::overlay(&mut background, &img, -x, -y);

    info!(args.output, "Saving image");
    if let Err(e) = background.save(args.output) {
        error!("Failed to save image: {e:?}");
    }
}
