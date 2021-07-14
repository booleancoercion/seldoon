use crate::Grid;
use crate::Vec2;

use pipeframe::pixels::Rgb;
use pipeframe::Frame;

use std::ops::RangeInclusive;

pub trait Drawable {
    /// This function takes a point in *pixel space*, and calculates the distance
    /// from that point to this object in *pixel space*. The distinction between spaces is important!
    fn pixel_dist(&self, p: Vec2) -> f64;

    /// This function returns a bounding box that fully contains this object,
    /// in the form of inclusive pixel limits.
    fn bounding_box(&self) -> (RangeInclusive<usize>, RangeInclusive<usize>);
}

pub fn draw<D: Drawable>(object: &D, grid: &Grid, frame: &mut Frame<Rgb>) {
    let (xres, yres) = grid.get_res();
    let (xrange, yrange) = object.bounding_box();

    let (xmin, xmax) = xrange.into_inner();
    let (ymin, ymax) = yrange.into_inner();

    let xmin = xmin.saturating_sub(3);
    let xmax = xmax.saturating_add(3).min(xres - 1);

    let ymin = ymin.saturating_sub(3);
    let ymax = ymax.saturating_add(3).min(yres - 1);

    for px in xmin..=xmax {
        for py in ymin..=ymax {
            let p = grid.centered_pixel(px, py);
            let dist = object.pixel_dist(p);

            let red = if dist < 1. {
                255
            } else if frame[(px, py)].vals != [0, 0, 0] {
                continue;
            } else {
                let alpha = (2. - dist).max(0.);

                (255. * alpha) as u8
            };

            frame[(px, py)] = Rgb::from_bytes([red, 0, 0]);
        }
    }
}

pub fn rangify(a: usize, b: usize) -> RangeInclusive<usize> {
    if a <= b {
        a..=b
    } else {
        b..=a
    }
}

pub struct Line {
    p0: Vec2,
    p1: Vec2,
    xrange: RangeInclusive<usize>,
    yrange: RangeInclusive<usize>,
}

impl Line {
    /// Creates a new Line from two points in coordinate space.
    pub fn new(p0: Vec2, p1: Vec2, grid: &Grid) -> Self {
        let pix0 = grid.get_clamped_pixel(p0);
        let pix1 = grid.get_clamped_pixel(p1);

        let xrange = rangify(pix0.0, pix1.0);
        let yrange = rangify(pix0.1, pix1.1);

        Self {
            p0: grid.in_pixel_space(p0),
            p1: grid.in_pixel_space(p1),
            xrange,
            yrange,
        }
    }

    /// Performs a linear interpolation of the two inner points,
    /// returning another point in pixel space.
    /// `t` should be between 0 and 1 for this to make sense.
    fn lerp(&self, t: f64) -> Vec2 {
        self.p0 * (1. - t) + self.p1 * t
    }

    /// Takes in a point in pixel space, and returns the closest point
    /// on the line.
    fn closest(&self, p: Vec2) -> Vec2 {
        let Vec2(x0, y0) = self.p0;
        let Vec2(x1, y1) = self.p1;
        let Vec2(x, y) = p;

        let enumerator = (x0 - x1) * (x0 - x) + (y0 - y1) * (y0 - y);
        let demominator = (x1 - x0).powi(2) + (y1 - y0).powi(2);

        let t = (enumerator / demominator).clamp(0., 1.);
        self.lerp(t)
    }
}

impl Drawable for Line {
    fn pixel_dist(&self, p: Vec2) -> f64 {
        self.closest(p).dist(&p)
    }

    fn bounding_box(&self) -> (RangeInclusive<usize>, RangeInclusive<usize>) {
        (self.xrange.clone(), self.yrange.clone())
    }
}
