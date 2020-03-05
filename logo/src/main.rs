extern crate nannou;

use nannou::app;
use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run();
}

const RADIUS: f32 = 100.0;
const CHAR_W: f32 = RADIUS * 0.6;
const CHAR_H: f32 = RADIUS * 0.8;
const LINE_THICKNESS: f32 = RADIUS * 0.03;
const VERTEX_RADIUS: f32 = LINE_THICKNESS * 2.5;
const CHAR_PAD: f32 = CHAR_W * 0.5;
const N_CHARS: usize = 6;
const WORD_W: f32 = CHAR_W * N_CHARS as f32 + CHAR_PAD * 5.0;

/////////////
// Helpers //
/////////////

fn rect_corners(r: &Rect) -> (Point2<f32>, Point2<f32>, Point2<f32>, Point2<f32>) {
    let bl = r.bottom_left();
    let tl = r.top_left();
    let br = r.bottom_right();
    let tr = r.top_right();
    (bl, br, tl, tr)
}

fn hue(t: f32) -> f32 {
    (t * 0.0033).sin() * 0.1 - 0.325
}

fn lum(t: f32) -> f32 {
    (t * 0.5).sin() * 0.25 + 0.75
}

fn lines(draw: &app::Draw, t: f32, ls: &[(Point2<f32>, Point2<f32>)]) {
    for &(pa, pb) in ls {
        let x = (pa[0] + pb[0]) * 0.5;
        let y = (pa[1] + pb[1]) * 0.5;
        let a = if y < 0.0 {
            (t * 0.015 * x).sin().max(0.0)
        } else {
            (t * 0.01 * x).sin() * 0.5 + 0.5
        };
        let h = hue(t);
        let l = lum(t);
        draw.line()
            .points(pa, pb)
            .hsla(h, 1.0, l, a)
            .weight(LINE_THICKNESS);
    }
}

fn verts(draw: &app::Draw, t: f32, vs: &[Point2<f32>]) {
    for &v in vs {
        let h = hue(t);
        let l = lum(t);
        draw.ellipse()
            .radius(VERTEX_RADIUS)
            .xy(v)
            .hsla(h, 1.0, l, 1.0);
    }
}

////////////////
// Characters //
////////////////

fn n_lines(draw: &app::Draw, t: f32, r: &Rect) {
    let (bl, br, tl, tr) = rect_corners(r);
    let ls = [(bl, tl), (tl, br), (br, tr)];
    lines(draw, t, &ls);
}

fn n_verts(draw: &app::Draw, t: f32, r: &Rect) {
    let (bl, br, tl, tr) = rect_corners(r);
    let vs = [bl, tl, br, tr];
    verts(draw, t, &vs);
}

fn n(draw: &app::Draw, t: f32, r: &Rect) {
    n_lines(draw, t, r);
    n_verts(draw, t, r);
}

fn a_lines(draw: &app::Draw, t: f32, r: &Rect) {
    let (bl, br, tl, tr) = rect_corners(r);
    let ml = pt2(bl[0], bl[1] + r.h() * 0.5);
    let mr = pt2(br[0], br[1] + r.h() * 0.5);
    let ls = [(ml, mr), (bl, tl), (tl, tr), (tr, br)];
    lines(draw, t, &ls);
}

fn a_verts(draw: &app::Draw, t: f32, r: &Rect) {
    let (bl, br, tl, tr) = rect_corners(r);
    let vs = [bl, br, tl, tr];
    verts(draw, t, &vs);
}

fn a(draw: &app::Draw, t: f32, r: &Rect) {
    a_lines(draw, t, r);
    a_verts(draw, t, r);
}

fn o_lines(draw: &app::Draw, t: f32, r: &Rect) {
    let (bl, br, tl, tr) = rect_corners(r);
    let ls = [(bl, tl), (tl, tr), (tr, br), (br, bl)];
    lines(draw, t, &ls);
}

fn o_verts(draw: &app::Draw, t: f32, r: &Rect) {
    let (bl, br, tl, tr) = rect_corners(r);
    let vs = [bl, br, tl, tr];
    verts(draw, t, &vs);
}

fn o(draw: &app::Draw, t: f32, r: &Rect) {
    o_lines(draw, t, r);
    o_verts(draw, t, r);
}

fn u_lines(draw: &app::Draw, t: f32, r: &Rect) {
    let (bl, br, tl, tr) = rect_corners(r);
    let ls = [(tl, bl), (bl, br), (br, tr)];
    lines(draw, t, &ls);
}

fn u_verts(draw: &app::Draw, t: f32, r: &Rect) {
    let (bl, br, tl, tr) = rect_corners(r);
    let vs = [bl, br, tl, tr];
    verts(draw, t, &vs);
}

fn u(draw: &app::Draw, t: f32, r: &Rect) {
    u_lines(draw, t, r);
    u_verts(draw, t, r);
}

//////////
// View //
//////////

fn view(app: &App, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();
    let win = app.window_rect();
    let t = app.time;

    // Clear the background to white.
    draw.background().color(WHITE);
    draw.rect().w_h(win.w(), win.h() * 0.5).y(-win.h() * 0.25).color(BLACK);

    /////////////
    // Upper N //
    /////////////

    let single_n_rect = Rect::from_x_y_w_h(0.0, win.h() * 0.25, CHAR_W, CHAR_H);
    draw.ellipse().y(single_n_rect.y()).radius(RADIUS).color(BLACK);
    n(&draw, t, &single_n_rect);

    ////////////
    // NANNOU //
    ////////////

    let y = -win.h() * 0.25;
    let word_rect = Rect::from_w_h(WORD_W, CHAR_H).shift_y(y);
    let first_x = word_rect.left() + CHAR_W * 0.5;
    let char_x_offset = CHAR_W + CHAR_PAD;
    let r = Rect::from_w_h(CHAR_W, CHAR_H).shift(vec2(first_x, y));
    let char_rects: Vec<_> = (0..6)
        .map(|i| r.shift_x(i as f32 * char_x_offset))
        .collect();
    n_lines(&draw, t, &char_rects[0]);
    a_lines(&draw, t, &char_rects[1]);
    n_lines(&draw, t, &char_rects[2]);
    n_lines(&draw, t, &char_rects[3]);
    o_lines(&draw, t, &char_rects[4]);
    u_lines(&draw, t, &char_rects[5]);

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}
