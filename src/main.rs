use image::buffer::ConvertBuffer;
use image::GrayImage;
use image::Rgb;
use imageproc::edges::canny;
use imageproc::filter::gaussian_blur_f32;
use imageproc::hough::{detect_lines, PolarLine};
use imageproc::hough::draw_polar_lines;
use imageproc::hough::LineDetectionOptions;
use imageproc::point::Point;
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
    let mut cartesian_line: Vec<Point<f32>>;

    lines.iter().for_each(|line|{
        println!("{line:?}");
        let cartesian = polar_to_cartesian(line);
        println!("Cartesian x: {},y: {}",cartesian.x,cartesian.y);
        cartesian_line.push(cartesian);
    });
    
    let with_lines = draw_polar_lines(&source.to_rgb8(), &lines, Rgb([255, 0, 0]));

    canny.save("assets/canny.png")?;
    with_lines.save("assets/with_lines.png")?;
    
    Ok(())
}

fn polar_to_cartesian(line: &PolarLine) -> Point<f32> {
    let angle_in_radian = f32::to_radians(line.angle_in_degrees as f32);
    let x = line.r * f32::cos(angle_in_radian);
    let y = line.r * f32::sin(angle_in_radian);

    Point{x,y}
}

//fn line_intersection(line1: )
