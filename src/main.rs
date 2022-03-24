use std::fmt::Display;

use image::buffer::ConvertBuffer;
use image::GrayImage;
use image::Rgb;
use imageproc::drawing::draw_cross;
use imageproc::edges::canny;
use imageproc::filter::gaussian_blur_f32;
use imageproc::hough::draw_polar_lines;
use imageproc::hough::LineDetectionOptions;
use imageproc::hough::{detect_lines, PolarLine};
use imageproc::point::Point;
use jane_eyre::Result;

#[derive(Debug)]
pub struct Line {
    pub a: f32,
    pub b: f32,
}

impl Line {
    pub fn intersections(&self, lines: &Vec<Line>, bounds: &Point<u32>) -> Vec<Point<u32>> {
        lines
            .iter()
            .flat_map(|line| {
                self.intersection(line)
                    .filter(|i| i.x <= bounds.x && i.y <= bounds.y)
            })
            .collect::<Vec<_>>()
    }

    pub fn intersection(&self, line: &Line) -> Option<Point<u32>> {
        println!("{line}");

        todo!()
    }
}

impl From<&PolarLine> for Line {
    fn from(line: &PolarLine) -> Self {
        let angle = ((line.angle_in_degrees % 90) as f32).to_radians();

        let (a, b) = if line.r == 0f32 {
            let a = angle.tan();
            let b = 0f32;

            (a, b)
        } else {
            let x0 = line.r * angle.sin();
            let y0 = line.r * angle.cos();

            let a = x0 / y0;
            let b = y0;

            (a, b)
        };

        println!(
            "angle {} r {} - > a {} b {}",
            line.angle_in_degrees, line.r, a, b
        );

        Line { a, b }
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "y = {} * x + {}", self.a, self.b)
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

    let lines = polar_lines.iter().map(Line::from).collect::<Vec<_>>();

    lines.iter().for_each(|line| println! {"{line}"});

    let bounds = Point {
        x: grayscale.width(),
        y: grayscale.height(),
    };
    let intersections = lines
        .iter()
        .map(|line| line.intersections(&lines, &bounds))
        .collect::<Vec<_>>();

    intersections
        .iter()
        .for_each(|intersection| println! {"{intersection:?}"});

    let with_lines = draw_polar_lines(&source.to_rgb8(), &polar_lines, Rgb([255, 0, 0]));

    canny.save("assets/canny.png")?;
    with_lines.save("assets/with_lines.png")?;

    draw_cross(&source.to_rgb8(), Rgb([255, 0, 0]), 100, 100).save("assets/with_cross.png")?;

    // from_control_points(from: [(f32, f32); 4], to: [(f32, f32); 4])

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_intersection() {
        let line_a = &Line { a: 1f32, b: 0f32 };
        let line_b = &Line { a: 0f32, b: 1f32 };
        let line_c = &Line { a: 1f32, b: 1f32 };

        let intersection = line_a.intersection(&line_b);
        assert_eq!(intersection, Some(Point{x: 1, y: 1}));

        let intersection = line_a.intersection(&line_c);
        assert_eq!(intersection, None);
    }
}
