extern crate nannou;

use nannou::app;
use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run();
}

const DEPTH: usize = 7;

fn draw_subdivisions(
    draw: &app::Draw,
    r: Rect,
    depth: usize,
    t: f32,
    hue_hz: f32,
) {
    if depth > 0 {
        for r in r.subdivisions_iter() {
            draw_subdivisions(draw, r, depth - 1, t, hue_hz);
        }
        return;
    }

    let middle = pt2(0.0, 0.0);
    let from_middle = r.xy().distance(middle);
    let moving_point = pt2(t.sin() * (300.0 * from_middle), t.cos() * (300.0 * from_middle));
    let from_moving_point = r.xy().distance(moving_point);
    let hue_range = 0.3 * ((t * 0.07).sin() * 0.15) + 0.15;
    let hue = (t * hue_hz + (from_middle / 1_000.0) + from_moving_point * 0.001) % hue_range + t * 0.01;
    let sat = (t * 0.25).sin() * 0.3 + 0.4;
    let lum = hue_range;
    draw.rect()
        .wh(r.wh())
        .xy(r.xy())
        .hsla(hue, sat, lum, 0.1 + (from_middle / 1_000.0).powi(2));
}

fn view(app: &App, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();

    // Clear the background to pink.
    let win_rect = app.window_rect();
    draw.rect().wh(win_rect.wh()).rgba(0.0, 0.0, 0.0, 0.005);

    // Crazy subdivisions.
    let t = app.duration.since_start.secs() as f32;
    let max_side = win_rect.w().max(win_rect.h());
    let r = Rect::from_w_h(max_side, max_side);
    let hue_hz = 0.1;
    draw_subdivisions(&draw, r, DEPTH, t, hue_hz);

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}
