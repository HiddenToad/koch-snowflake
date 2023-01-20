use nannou::winit::event::VirtualKeyCode;
use nannou::{prelude::rgb::Srgb, prelude::*};

const START_SIDE_LENGTH: f32 = 0.5;
const LINE_THICKNESS: f32 = 1.;
const START_DEPTH: u32 = 0;

fn main() {
    nannou::app(model).update(update).run();
}

struct Pen {
    pos: Point2,
    angle: f32,
    color: Srgb<u8>,
    points: Vec<Point2>,
}

impl Pen {
    fn forward(&mut self, amount: f32) {
        self.points.push(self.pos);
        self.pos.x += deg_to_rad(self.angle).cos() * amount * 1200.;
        self.pos.y += deg_to_rad(self.angle).sin() * amount * 1200.;
    }

    fn right(&mut self, amount: f32) {
        self.angle += amount;
    }

    fn left(&mut self, amount: f32) {
        self.angle -= amount;
    }
}

struct Model {
    window: window::Id,
    pen: Pen,
    result_points: Vec<Point2>,
    ran_yet: bool,
    depth: u32,
}

fn model(app: &App) -> Model {
    let window = app.new_window().view(view).event(event).build().unwrap();
    Model {
        window,
        pen: Pen {
            pos: pt2(-153., -80.),
            angle: 0.,
            color: BLACK,
            points: vec![],
        },
        result_points: vec![],
        ran_yet: false,
        depth: START_DEPTH,
    }
}

fn event(_: &App, model: &mut Model, event: WindowEvent) {
    match event {
        WindowEvent::KeyPressed(key) => match key {
            VirtualKeyCode::Up if model.depth < 8 => {
                model.depth += 1;
                model.ran_yet = false;
            }
            VirtualKeyCode::Down if model.depth > 0 => {
                model.depth -= 1;
                model.ran_yet = false;
            }

            _ => {}
        },
        _ => {}
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    if !model.ran_yet {
        for _ in 0..3 {
            make_koch_snowflake(model, START_SIDE_LENGTH, model.depth);
            model.pen.right(120.);
        }
        model.result_points = std::mem::take(&mut model.pen.points);
        model.result_points.push(model.result_points[0]);
        model.ran_yet = true;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    if model.ran_yet {
        draw.polyline()
            .color(model.pen.color)
            .stroke_weight(LINE_THICKNESS)
            .xy(model.result_points[0])
            .join_miter()
            .points(model.result_points.iter().cloned())
            .finish();
    }
    draw.to_frame(app, &frame).unwrap();
}

fn make_koch_snowflake(model: &mut Model, side_length: f32, depth: u32) {
    if depth == 0 {
        model.pen.forward(side_length);
    } else {
        let side_length = side_length / 3.;
        make_koch_snowflake(model, side_length, depth - 1);
        model.pen.left(60.);
        make_koch_snowflake(model, side_length, depth - 1);
        model.pen.right(120.);
        make_koch_snowflake(model, side_length, depth - 1);
        model.pen.left(60.);
        make_koch_snowflake(model, side_length, depth - 1);
    }
}
