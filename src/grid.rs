use crate::Vec2;

use std::ops::RangeInclusive;

#[derive(Clone)]
pub struct Grid {
    resolution: (usize, usize),
    xrange: RangeInclusive<f64>,
    yrange: RangeInclusive<f64>,
    origin: (i32, i32),
    unit_x: f64,
    unit_y: f64,
}

impl Grid {
    pub fn new(
        resolution: (usize, usize),
        xrange: RangeInclusive<f64>,
        yrange: RangeInclusive<f64>,
    ) -> Self {
        let (minx, maxx) = xrange.clone().into_inner();
        let (miny, maxy) = yrange.clone().into_inner();

        assert!(minx < maxx);
        assert!(miny < maxy);

        let (xpixels, ypixels) = resolution;
        let unit_x = (maxx - minx) / xpixels as f64;
        let unit_y = (maxy - miny) / ypixels as f64;

        let origin = (-(minx / unit_x) as i32, (maxy / unit_y) as i32);

        Self {
            resolution,
            xrange,
            yrange,
            origin,
            unit_x,
            unit_y,
        }
    }

    /// Converts coordinates on the grid to coordinates in the pixel space, while
    /// retaining sub-pixel values.  
    /// When you round the values from this function accordingly, they line up with
    /// the results of calling `Grid::get_pixel`.
    pub fn in_pixel_space(&self, p: Vec2) -> Vec2 {
        let Vec2(x, y) = p;

        let xpixels = x / self.unit_x;
        let ypixels = y / self.unit_y;

        let xpos = self.origin.0 as f64 + xpixels;
        let ypos = self.origin.1 as f64 - ypixels;

        Vec2(xpos, ypos)
    }

    /// Converts coordinates on the grid to coordinates in the pixel space, rounding accordingly.
    /// This function is the lossy version of `Grid::in_pixel_space`.
    pub fn get_pixel(&self, p: Vec2) -> Result<(usize, usize), (i32, i32)> {
        let Vec2(x, y) = self.in_pixel_space(p);
        let (xpos, ypos) = (x as i32, y as i32);

        if xpos < 0
            || xpos >= self.resolution.0 as i32
            || ypos < 0
            || ypos >= self.resolution.1 as i32
        {
            Err((xpos, ypos))
        } else {
            Ok((xpos as usize, ypos as usize))
        }
    }

    pub fn get_clamped_pixel(&self, p: Vec2) -> (usize, usize) {
        let p = self.get_pixel(p);

        match p {
            Ok(p) => p,
            Err((x, y)) => (
                x.clamp(0, self.resolution.0 as i32 - 1) as usize,
                y.clamp(0, self.resolution.1 as i32 - 1) as usize,
            ),
        }
    }

    /// Converts a given pixel into coordinates in coordinate space.
    pub fn get_coords(&self, xpix: usize, ypix: usize) -> Vec2 {
        let Vec2(x, y) = self.centered_pixel(xpix, ypix);

        let x = (x - self.origin.0 as f64) * self.unit_x;
        let y = (-y + self.origin.1 as f64) * self.unit_y;

        Vec2(x, y)
    }

    /// Converts a given pixel into the coordinates of its center in pixel space.
    pub fn centered_pixel(&self, xpix: usize, ypix: usize) -> Vec2 {
        Vec2(xpix as f64 + 0.5, ypix as f64 + 0.5)
    }

    pub fn get_res(&self) -> (usize, usize) {
        self.resolution
    }

    pub fn get_units(&self) -> (f64, f64) {
        (self.unit_x, self.unit_y)
    }

    pub fn get_ranges(&self) -> (RangeInclusive<f64>, RangeInclusive<f64>) {
        (self.xrange.clone(), self.yrange.clone())
    }
}
