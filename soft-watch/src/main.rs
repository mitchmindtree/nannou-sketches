extern crate jen_rx;
extern crate nannou;

use jen_rx::Instrument::*;
use jen_rx::Measure::*;
use nannou_osc as osc;
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

const PORT: u16 = 9002;

struct Model {
    osc_rx: osc::Receiver,
    jen: jen_rx::State,
}

fn model(_app: &App) -> Model {
    let osc_rx = osc::receiver(PORT).unwrap();
    let jen = jen_rx::State::default();
    Model { osc_rx, jen }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    for (packet, _addr) in model.osc_rx.try_iter() {
        model.jen.update_by_osc_packet(packet);
    }
}

fn transient(model: &Model, inst: jen_rx::Instrument, secs: f64) -> f32 {
    match model.jen.secs_since_note_on(inst) {
        None => 0.0,
        Some(f) => map_range(f, 0.0, secs, 1.0, 0.0).max(0.0).powf(0.25),
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let kick = transient(model, Kick, 0.2).powf(2.0);
    let snare = transient(model, Snare, 0.3).powf(0.6);
    let ghost = transient(model, Ghost, 0.032);
    let phrase = model.jen.playhead_position(Phrase).unwrap_or(0.0);
    let bar = model.jen.playhead_position(Bar).unwrap_or(0.0);
    let beat = model.jen.playhead_position(Beat).unwrap_or(0.0);
    let semi_q = model.jen.playhead_position(SemiQuaver).unwrap_or(0.0);

    // Prepare to draw.
    let draw = app.draw();

    let win_rect = app.window_rect();
    let win_w = win_rect.w();
    let win_h = win_rect.h();

    draw.rect()
        .w(snare * win_w)
        .h(snare * win_h)
        .rotate(((phrase*4.0)%1.0) * TAU)
        .hsl(0.6 + phrase*0.2, 0.5, snare);
    draw.ellipse()
        .w(kick * win_w * ((phrase*2.0)%1.0).powf(0.5))
        .h(kick * win_h * ((phrase*2.0)%1.0).powf(0.5))
        .resolution(100)
        .hsl(bar*0.2, 0.5, kick);
    let dist = (win_w*0.125 + win_w*0.875*phrase)*0.5;
    draw.ellipse()
        .rotate(bar * TAU)
        .resolution(3)
        .wh([beat*win_h*0.5; 2].into())
        .color(BLACK);
    draw.line()
        .points(pt2(-dist, 0.0), pt2(dist, 0.0))
        .weight(ghost * win_h * ((phrase * 4.0) % 1.0).powf(3.0))
        .hsl(0.3 + semi_q*0.2, 0.5, 0.6*ghost);
    draw.line()
        .points(pt2(0.0, -dist), pt2(0.0, dist))
        .weight(kick * win_h * 0.25)
        .hsl(0.3 + semi_q*0.2, 0.5, 0.6*ghost);

    draw.to_frame(app, &frame).unwrap();
}
