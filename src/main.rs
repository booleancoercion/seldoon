use seldoon::draw::{self, Line};
use seldoon::{Grid, VField};

use pipeframe::pixels::Rgb;
use pipeframe::{Frame, Video};

fn main() {
    let mut video: Video<Rgb> = Video::new((1024, 1024), 60, "output");

    let (x, y) = video.get_resolution();
    let grid = Grid::new((x, y), -6.1..=6.1, -6.1..=6.1);

    fn dx(x: f64, y: f64) -> f64 {
        (2. + x) * (y - x)
    }

    fn dy(x: f64, y: f64) -> f64 {
        (4. - x) * (y + x)
    }
    let vfield = VField::new(dx, dy, 0.02);

    let mut points = grid.border_points(0.1, 20);
    for pt in &mut points {
        *pt = vfield.rewind(*pt, 1000, 0.5, &grid);
    }

    for i in 0..20 * 60 {
        let frame = video.get_frame_mut();

        if i % 3 < 2 {
            fade_frame(frame, x, y);
        }

        for pt in &mut points {
            let prev = *pt;
            let now = vfield.advance(*pt);

            let line = Line::new(prev, now, &grid);
            draw::draw(&line, &grid, frame);

            *pt = now;
        }

        video.save_frame();
    }

    video.finish();
}

fn fade_frame(frame: &mut Frame<Rgb>, x: usize, y: usize) {
    for px in 0..x {
        for py in 0..y {
            let p = &mut frame[(px, py)];
            if p.vals[0] == 0 {
                continue;
            }
            p.vals[0] -= 1;
        }
    }
}
