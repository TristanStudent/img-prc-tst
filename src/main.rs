use image::buffer::ConvertBuffer;
use image::GrayImage;
use image::Rgb;
use imageproc::drawing::draw_cross;
use imageproc::edges::canny;
use imageproc::filter::gaussian_blur_f32;
use imageproc::hough::draw_polar_lines;
use imageproc::hough::LineDetectionOptions;
use imageproc::hough::{detect_lines, PolarLine};
use jane_eyre::Result;

#[derive(Debug)]
pub struct Line {
    pub a: f32,
    pub b: f32,
}

impl From<&PolarLine> for Line {
    fn from(line: &PolarLine) -> Self {
        let angle_in_radian = f32::to_radians(line.angle_in_degrees as f32);
        
        let y0 = if line.angle_in_degrees  > 90 {
            line.r / f32::cos(angle_in_radian)
        } else {
            line.r / f32::sin(angle_in_radian)
        };

        let (a, b) = match (line.r == 0f32, line.angle_in_degrees  > 90) {
            (true, _) => {
                (f32::tan(angle_in_radian), 0f32)
            },
            (false, true) => {
                let x0 = line.r / f32::cos(angle_in_radian);
                (y0 as f32 / x0 as f32, y0 as f32)
            },
            (false, false) => {
                let x0 = line.r / f32::sin(angle_in_radian);
                (y0 as f32 / x0 as f32, y0 as f32)
            },
        };
        
        println!("angle {} r {} - > a {} b {}", line.angle_in_degrees, line.r, a, b);

        Line { a, b }
    }
}

fn main() -> Result<()> {
    // let source = image::open("assets/image.png")?;
    let source = image::open("assets/source03.png")?;
    let blurred = gaussian_blur_f32(&source.grayscale().to_rgb32f(), 2f32);
    let grayscale: GrayImage = blurred.convert();
    let canny = canny(&grayscale, 50.0, 100.0);

    let polar_lines = detect_lines(
        &canny,
        LineDetectionOptions {
            vote_threshold: 100,
            suppression_radius: 45,
        },
    );

    let _lines = polar_lines.iter().map(Line::from).collect::<Vec<_>>();

    // let mut cartesian_line: Vec<Point<f32>>;

    // lines.iter().for_each(|line|{
    //     println!("{line:?}");
    //     let cartesian = polar_to_cartesian(line);
    //     println!("Cartesian x: {},y: {}",cartesian.x,cartesian.y);
    //     // cartesian_line.push(cartesian);
    // });

    let last_line = [*polar_lines.get(3).unwrap()];

    let with_lines = draw_polar_lines(&source.to_rgb8(), &last_line, Rgb([255, 0, 0]));

    canny.save("assets/canny.png")?;
    with_lines.save("assets/with_lines.png")?;

    draw_cross(&source.to_rgb8(), Rgb([255, 0, 0]), 100, 100).save("assets/with_cross.png")?;

    // from_control_points(from: [(f32, f32); 4], to: [(f32, f32); 4])

    Ok(())
}

//fn line_intersection(line1: )
