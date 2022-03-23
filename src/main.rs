use image::buffer::ConvertBuffer;
use image::GrayImage;
use image::Rgb;
use imageproc::edges::canny;
use imageproc::filter::gaussian_blur_f32;
use imageproc::hough::detect_lines;
use imageproc::hough::draw_polar_lines;
use imageproc::hough::LineDetectionOptions;
use jane_eyre::Result;

fn main() -> Result<()> {
    let source = image::open("assets/image.png")?;
    let blurred = gaussian_blur_f32(&source.grayscale().to_rgb32f(), 2f32);
    let grayscale: GrayImage = blurred.convert();
    let canny = canny(&grayscale, 50.0, 100.0);
   
    let lines = detect_lines(
        &canny,
        LineDetectionOptions {
            vote_threshold: 100,
            suppression_radius: 45,
        },
    );

    lines.iter().for_each(|line| println!("{line:?}"));
    
    let with_lines = draw_polar_lines(&source.to_rgb8(), &lines, Rgb([255, 0, 0]));

    canny.save("assets/canny.png")?;
    with_lines.save("assets/with_lines.png")?;

    Ok(())
}
