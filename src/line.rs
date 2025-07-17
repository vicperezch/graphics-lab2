use crate::framebuffer::{self, Framebuffer};
use raylib::prelude::*;

pub fn line(framebuffer: &mut Framebuffer, start: Vector2, end: Vector2) {
    let mut x0 = start.x as i32;
    let mut y0 = start.y as i32;
    let x1 = end.x as i32;
    let y1 = end.y as i32;

    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        if x0 >= 0 && y0 >= 0 && x0 < framebuffer.width && y0 < framebuffer.height {
            framebuffer.set_pixel(x0, y0);
        }

        if x0 == x1 && y0 == y1 {
            break;
        }

        let e2 = 2 * err;

        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}

pub fn draw_polygon(framebuffer: &mut Framebuffer, points: &Vec<Vector2>) {
    for i in 0..points.len() - 1 {
        line(framebuffer, points[i], points[i + 1]);
    }

    line(framebuffer, points[points.len() - 1], points[0]);
}

pub fn fill_polygon(framebuffer: &mut Framebuffer, points: &Vec<Vector2>) {
    let height = framebuffer.height as i32;

    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;
    for p in points {
        let y = p.y as i32;
        if y < min_y {
            min_y = y;
        }
        if y > max_y {
            max_y = y;
        }
    }

    for y in min_y.max(0)..=max_y.min(height - 1) {
        let mut intersections = Vec::new();

        for i in 0..points.len() {
            let p1 = points[i];
            let p2 = points[(i + 1) % points.len()];

            let (x1, y1) = (p1.x as f32, p1.y as f32);
            let (x2, y2) = (p2.x as f32, p2.y as f32);

            if (y1 <= y as f32 && y2 > y as f32) || (y2 <= y as f32 && y1 > y as f32) {
                let t = (y as f32 - y1) / (y2 - y1);
                let x = x1 + t * (x2 - x1);
                intersections.push(x as i32);
            }
        }

        intersections.sort_unstable();

        for i in (0..intersections.len()).step_by(2) {
            if i + 1 >= intersections.len() {
                break;
            }

            let x_start = intersections[i].max(0);
            let x_end = intersections[i + 1].min(framebuffer.width as i32 - 1);

            for x in x_start..=x_end {
                framebuffer.set_pixel(x, y);
            }
        }
    }
}
