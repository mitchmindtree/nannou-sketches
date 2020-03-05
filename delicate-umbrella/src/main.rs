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
    let diagonal = win.top_left().distance(win.bottom_right());

    // Clear the background to pink.
    draw.background().color(BLACK);

    let n = ((0.1 * t * 2.0 * PI).sin() * 100.0 + 100.0) as usize;
    //let n = 100;
    for i in 0..n {
        let f = i as f32 / n as f32;
        let max_weight = (1.0 / n as f32) * win.w();
        let x = win.x.lerp(f);
        let hz = 0.125;
        let tx = (t * hz * 2.0 * PI).sin() * win.right();
        let d = (tx - x).abs();
        let dn = d / win.w();
        let weight = max_weight * dn;
        let hue = 1.0;

        // Linear.
        // let pa = pt2(x, win.top());
        // let pb = pt2(x, win.bottom());

        // Radial.
        let rad = (t * 0.05 + f) * 2.0 * PI;
        let mag = diagonal;
        let pa = pt2(rad.cos() * mag, rad.sin() * mag);
        let pb = pt2(rad.cos() * -mag, rad.sin() * -mag);

        //let hue = t * 0.1 + dn * 0.3;
        draw.line()
            .weight(weight)
            .points(pa, pb)
            .hsla(hue, 1.0, 1.0, dn);
    }

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}
