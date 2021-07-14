use seldoon::draw::{self, Line};
use seldoon::grid::Grid;
use seldoon::vec2::Vec2;

use pipeframe::Video;

fn main() {
    let mut video = Video::new((1024, 1024), 60, "output");

    let (x, y) = video.get_resolution();
    let grid = Grid::new((x, y), -6.1..=6.1, -6.1..=6.1);

    let mut points = vec![];
    for i in 0..24 {
        let i = i as f64;
        points.push(Vec2(6. - i * 0.5, 6.));
        points.push(Vec2(6. - i * 0.5, -6.));
        points.push(Vec2(6., 6. - i * 0.5));
        points.push(Vec2(-6., 6. - i * 0.5));
    }

    for pt in &mut points {
        *pt = rewind(*pt, &grid);
    }

    for _ in 1..=20 * 60 {
        let frame = video.get_frame_mut();

        for pt in &mut points {
            let prev = *pt;
            let now = advance(*pt);

            let line = Line::new(prev, now, &grid);
            draw::draw(&line, &grid, frame);

            *pt = now;
        }

        video.save_frame();
    }

    video.finish();
}

fn get_dir(pt: Vec2) -> Vec2 {
    let Vec2(x, y) = pt;

    let dx = (2. + x) * (y - x);
    let dy = (4. - x) * (y + x);

    Vec2(dx, dy)
}

fn advance(pt: Vec2) -> Vec2 {
    let dir = get_dir(pt).normalize();
    pt + dir * 0.02
}

fn rewind(mut pt: Vec2, grid: &Grid) -> Vec2 {
    let (xrange, yrange) = grid.get_ranges();

    for _ in 0..1000 {
        if !xrange.contains(&pt.0) || !yrange.contains(&pt.1) {
            break;
        }
        let dir = get_dir(pt);

        if dir.norm() <= 0.5 {
            break;
        }
        pt -= dir.normalize() * 0.02;
    }

    pt
}
