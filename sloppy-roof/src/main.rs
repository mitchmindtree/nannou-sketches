extern crate nannou;

use nannou::prelude::*;

fn main() {
    nannou::app(model).event(event).simple_window(view).run();
}

const STARS_PER_100_PIXELS: usize = 500;
const STARS_PER_PATH: usize = 50;
const CLOSEST: usize = STARS_PER_PATH / 10;

struct Model {
    // The location of each star.
    points: Vec<Point2<f32>>,
    // The path through the stars.
    path: Vec<usize>,
}

fn random_star_points(win: Rect) -> Vec<Point2<f32>> {
    let w = win.w();
    let h = win.h();
    let n_stars = (w * h / (STARS_PER_100_PIXELS as f32).powi(2)) as usize * STARS_PER_100_PIXELS;
    let half_w = w * 0.5;
    let half_h = h * 0.5;
    (0..n_stars)
        .map(|_| {
            let coord = || {
                let v: f32 = random();
                let sign: bool = random();
                if sign { v } else { -v }
            };
            pt2(coord() * half_w, coord() * half_h)
        })
        .collect()
}

fn distance_order(origin: Point2<f32>, a: Point2<f32>, b: Point2<f32>) -> std::cmp::Ordering {
    origin.distance2(a).partial_cmp(&origin.distance2(b)).unwrap()
}

fn random_star_path(points: &[Point2<f32>]) -> Vec<usize> {

    let mut points_by_distance: Vec<_> = points.iter().cloned().enumerate().collect();
    let o = pt2(0.0, 0.0);
    points_by_distance.sort_by(|&(_, a), &(_, b)| distance_order(o, a, b));
    let mut origin = points_by_distance[0].0;
    let mut path = vec![origin];
    while path.len() < STARS_PER_PATH {
        let o = points_by_distance[origin].1;
        points_by_distance = points_by_distance.iter().cloned().take(CLOSEST).collect();
        points_by_distance.sort_by(|&(_, a), &(_, b)| distance_order(o, a, b));
        origin = random_range(0, CLOSEST);
        path.push(origin);
    }
    path
}

fn model(app: &App) -> Model {
    let points = random_star_points(app.window_rect());
    let path = random_star_path(&points);
    Model { points, path }
}

fn event(app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent { simple: Some(event), .. } => match event {
            KeyPressed(_) | Resized(..) => {
                let points = random_star_points(app.window_rect());
                let path = random_star_path(&points);
                model.points = points;
                model.path = path;
            },
            _other => (),
        },

        _ => (),
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    // Prepare to draw.
    let draw = app.draw();
    let win = app.window_rect();
    let t = app.time;

    // Clear the background to pink.
    draw.background().color(BLACK);

    // Path.
    let mut iter = model.path.iter().peekable();
    let mut i = 0;
    while let Some(&a) = iter.next() {
        if let Some(&&b) = iter.peek() {
            let start = model.points[a];
            let end = model.points[b];
            let a = (t + i as f32 * 0.2).sin().max(0.0).powi(6);
            draw.line()
                .points(start, end)
                .hsla(1.0, 1.0, 1.0, a);
            i += 1;
        }
    }

    // Stars.
    for (i, &p) in model.points.iter().enumerate() {
        let i_f = i as f32 / model.points.len() as f32;
        let x_f = p.x / win.w();
        let a = (t + i as f32 * 0.01).sin().max(0.0).powi(4);
        let r = i_f % 2.0 % 1.0;
        let h = (x_f + i_f + t * 0.2) % 1.0;
        draw.ellipse()
            .resolution(8)
            //.radius(1.0)
            .radius(r)
            .xy(p)
            .hsla(h, 0.5, 0.9, a);
    }

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}
