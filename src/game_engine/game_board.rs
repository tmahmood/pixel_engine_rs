use std::collections::HashMap;
use crate::game_engine::shapes::Block;
use std::mem::swap;
use mint::Point2;
use graphics::types::Color;

pub type PixelMap = HashMap<String, Pixel>;

fn make_key(x: f32, y: f32) -> String {
    format!("{},{}", x, y)
}

#[derive(Debug)]
pub struct Pixel {
    pub point: Point2<f32>,
    pub color: Color,
}

pub struct GameBoard {
    pub pixels: PixelMap
}

impl GameBoard {
    pub fn new() -> Self {
        GameBoard {
            pixels: HashMap::new()
        }
    }
}

pub fn set_pixel(pixels: &mut PixelMap, x: f32, y: f32, color: Color) {
    pixels.insert(make_key(x, y), Pixel {
        point: Point2{x, y},
        color,
    });
}

pub fn get_pixel(pixels: &PixelMap, x: f32, y: f32) -> Option<&Pixel> {
    pixels.get(&make_key(x, y))
}

/// draw rectangle
pub fn draw_rectangle(x: f32, y: f32, w: f32, h: f32, pixels: &mut PixelMap, color: Color) {
    let mut i = x;
    loop {
        let mut j = y;
        if i >= x + w { break; }
        loop {
            if j >= y + h { break; }
            set_pixel(pixels, i, j, color);
            j = j + 1.0;
        }
        i = i + 1.0;
    }
}

/// Drawing ellipse with pixels
pub fn draw_ellipse(cx: f32, cy: f32, a: f32, b: f32, points: &mut PixelMap, color: Color) {
    let mut plot_ellipse = |x: f32, y: f32| {
        set_pixel(points, cx + x, cy + y, color);
        set_pixel(points, cx + x, cy - y, color);
        set_pixel(points, cx - x, cy + y, color);
        set_pixel(points, cx - x, cy - y, color);
    };
    let a2 = a * a;
    let b2 = b * b;
    let mut x = 0.0;
    let mut y = b;
    let mut p = b2 + (a2 * (1.0 - 4.0 * b) - 2.0) / 4.0;
    let mut d_pe = 3.0 * b2;
    let mut d2_pe = 2.0 * b2;
    let mut d_pse = d_pe - 2.0 * a2 * (b - 1.0);
    let mut d2_pse = d2_pe + 2.0 * a2;
    plot_ellipse(x, y);
    while d_pse < 2.0 * a2 + 3.0 * b2 {
        if p < 0.0 {
            p = p + d_pe;
            d_pe = d_pe + d2_pe;
            d_pse = d_pse + d2_pe;
        } else {
            p = p + d_pse;
            d_pe = d_pe + d2_pe;
            d_pse = d_pse + d2_pse;
            y -= 1.0;
        }
        x += 1.0;
        plot_ellipse(x, y);
    }
    // let mut d_pse = d_pe - 2.0 * a2 * (b - 1.0);
    // let mut d2_pse = d2_pe + 2.0 * a2;
    p = p - (a2 * (4.0 * y - 3.0) + b2 * (4.0 * x + 3.0) + 2.0) / 4.0;
    let mut d2_ps = 2.0 * a2;
    d_pse = 2.0 * b2 + 3.0 * a2;
    while y > 0.0 {
        if p > 0.0 {
            p = p + d_pe;
            d_pe = d_pe + d2_ps;
            d_pse = d_pse + d2_ps;
        } else {
            p = p + d_pse;
            d_pe = d_pe + d2_ps;
            d_pse = d_pse + d2_pse;
            x += 1.0;
        }
        y -= 1.0;
        plot_ellipse(x, y);
    }
}

/// drawing line using (Bresenham's_line_algorithm)[https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm]
/// will draw straight lines without using the algorithm if x1/x2 or y1,y2 are in same line
pub fn draw_line(_x1: f32, _y1: f32, _x2: f32, _y2: f32, points: &mut PixelMap, color: Color) {
    let mut x1 = _x1;
    let mut x2 = _x2;
    let mut y1 = _y1;
    let mut y2 = _y2;
    let dx = (x2 - x1).abs();
    let dy = -(y2 - y1).abs();
    if dx == 0.0 {
        return draw_straight_line(x1, y1, y2, points, color, true);
    }
    if dy == 0.0 {
        return draw_straight_line(y1, x1, x2, points, color, false);
    }
    // which way we are moving?
    let sx = if x1 < x2 { 1.0 } else { -1.0 };
    let sy = if y1 < y2 { 1.0 } else { -1.0 };
    // err values that needs correcting
    let mut err = dx + dy;  /* error value e_xy */
    let mut e2 = 0.0;
    loop {
        set_pixel(points, x1, y1, color);
        // we have reached our target point
        if x1 >= x2 && y1 >= y2 { break; }
        //
        e2 = 2.0 * err;
        if e2 >= dy {
            err += dy;
            x1 += sx;
        }
        if e2 <= dx {
            err += dx;
            y1 += sy;
        }
    }
}

fn rol() -> u32 {
    let mut pattern: u32 = 0xFFFFFFFF;
    pattern = (pattern << 1) | (pattern >> 31);
    return pattern & 1;
}

fn draw_straight_line(p_fix: f32, _p1: f32, _p2: f32, points: &mut PixelMap, color: Color, if_vertical: bool) {
    let mut p1 = _p1;
    let mut p2 = _p2;
    if _p2 < _p1 {
        p1 = _p2;
        p2 = _p1;
    }
    let mut y = p1;
    while y <= p2 {
        if rol() >= 1 {
            if if_vertical {
                set_pixel(points, p_fix, y, color);
            } else {
                set_pixel(points, y, p_fix, color);
            }
        }
        y = y + 1.0;
    }
    return;
}

pub fn draw_circle(x: f32, y: f32, r: f32, points: &mut PixelMap, color: Color) {
    let mut x0 = 0.0;
    let mut y0 = r;
    let mut d = 3.0 - 2.0 * r;
    while y0 >= x0 {
        set_pixel(points, x + x0, y - y0, color);
        set_pixel(points, x + y0, y - x0, color);
        set_pixel(points, x + y0, y + x0, color);
        set_pixel(points, x + x0, y + y0, color);
        set_pixel(points, x - x0, y + y0, color);
        set_pixel(points, x - y0, y + x0, color);
        set_pixel(points, x - y0, y - x0, color);
        set_pixel(points, x - x0, y - y0, color);
        x0 += 1.0;
        if d < 0.0 {
            d += 4.0 * x0 + 6.0;
        } else {
            y0 -= 1.0;
            d += 4.0 * (x0 - y0) + 10.0;
        }
    }
}