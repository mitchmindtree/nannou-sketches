extern crate nannou;

use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();

    let win = app.window_rect();
    let t = app.duration.since_start.secs() as f32;
    let from_corner = pt2(0.0, 0.0).distance(win.top_right());

    // Clear the background to pink.
    let strobe_hz = (t * 0.05).sin() * 10.0;
    let bg = ((t * 2.0 * PI * strobe_hz).sin() * 0.5 + 0.5).powi(4);//.round();
    draw.rect()
        .wh(win.wh())
        .rgb(bg, bg, bg);

    let n_lines = 250;
    for i in 0..n_lines {
        let f = i as f32 / n_lines as f32;
        let tf = t * 0.1 + f;
        //let tf = (t * 0.1) + f + f * (t.sin() * 0.5 + 0.5);
        let x = (tf * PI * 2.0).sin();
        let y = (-tf * PI * 2.0).cos();
        let start = pt2(x, y) * from_corner * 1.2;
        let end = pt2(0.0, 0.0);
        let hue = tf * 0.1 % 0.3 + t * 0.01;
        let sat = 0.5;
        let lum = (t * 0.7).abs().powi(2) * 0.5 * (-bg + 1.0);
        let a_hz = 0.05;
        let a = (t * 0.1 + f) % (0.001 + ((t * PI * 2.0 * a_hz).sin() * 0.01 + 0.02));
        let tail_thick = 50.0;
        let radius = f.powi(2) * tail_thick;

        draw.line()
            .weight(3.0)
            .points(start, end)
            .hsla(hue, sat, lum, a);

        let hue = ((t * 0.1 + f) * 0.3 + t * 0.1) % 1.0;
        let sat = bg;
        let lum = 0.5;
        let a = (t * 5.0 + f).sin() * 0.5 + 0.5;
        draw.ellipse()
            .radius(radius)
            .xy(start * f.powi(2))
            .hsla(hue, sat, lum, a);
    }

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}
