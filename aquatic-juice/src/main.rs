extern crate nannou;

use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run();
}

fn view(app: &App, frame: Frame) {
    // Handy constants.
    let t = app.duration.since_start.secs() as f32;
    let win = app.window_rect();
    let mid_to_corner = win.xy().distance(win.top_right());
    let pi2 = PI * 2.0;

    // Prepare to draw.
    let draw = app.draw();

    // Clear the background to pink.
    draw.background().color(BLACK);

    // Some sweet circles.
    let rad_hz = 0.1;
    let a_hz = 0.3;
    let n = 20;
    for i in 0..n {
        let f = i as f32 / n as f32;
        let t = t + (0.3 * t).sin() * 0.5;
        let rad = ((f + t * rad_hz) % 1.0) * mid_to_corner;
        let a = ((f + a_hz * t) * pi2).sin() * 0.1 + 0.11;
        let a_fade_in = rad / mid_to_corner;
        let a_fade_out = (mid_to_corner - rad) / mid_to_corner;
        let hue = (t + f * 3.0) % 1.0;
        let sat = 0.5;
        let lum = 0.5;
        draw.ellipse()
            .resolution(200)
            .radius(rad)
            .hsla(hue, sat, lum, a_fade_in.powi(i) * a * a_fade_out);
    }

    // Weird squares in the middle.
    let rect = Rect::from_w_h(200.0, 200.0);
    for (j, r) in rect.subdivisions_iter().enumerate() {
        let jf = j as f32 / 4.0;
        for (i, r) in r.subdivisions_iter().enumerate() {
            let f = jf + i as f32 / 16.0;
            let offset = win.w() * 0.1;
            let x = r.x() + (t + f % 1.0).sin() * offset;
            let y = r.y() + (1.0 + t - f % 1.0).cos() * offset;
            let p = pt2(x, y) * 1.2;
            let from_mid = win.xy().distance(p);
            let pad = rect.w() * 0.5 * (from_mid / mid_to_corner);
            let r = r.pad(pad);
            let hue = (t + f * 0.3) % 1.0;
            let sat = 0.5;
            let lum = 0.5;
            let a = 1.0;
            draw.rect()
                .xy(p)
                .wh(r.wh())
                .hsla(hue, sat, lum, a);
        }
    }

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}
