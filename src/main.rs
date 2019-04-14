extern crate sdl2;

use sdl2::event::Event;
use sdl2::gfx::primitives::DrawRenderer;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

fn draw_points(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    points: &[ControlPoint],
) -> Result<(), String> {
    for point in points {
        point.draw(canvas)?;
    }
    Ok(())
}

fn draw_linear_bezier_curve(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    p0: &ControlPoint,
    p1: &ControlPoint,
) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(255, 0, 0));

    let mut t: f64 = 0.0;
    loop {
        if t >= 1.0 {
            break;
        }

        let x = (1.0 - t) * f64::from(p0.position.x) + t * f64::from(p1.position.x);
        let y = (1.0 - t) * f64::from(p0.position.y) + t * f64::from(p1.position.y);

        canvas.draw_point(sdl2::rect::Point::new(x as i32, y as i32))?;

        t += 0.0001;
    }
    Ok(())
}

fn draw_quadratic_bezier_curve(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    p0: &ControlPoint,
    p1: &ControlPoint,
    p2: &ControlPoint,
) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(0, 255, 0));

    let mut t: f64 = 0.0;
    loop {
        if t >= 1.0 {
            break;
        }

        let x = (1.0 - t).powi(2) * f64::from(p0.position.x)
            + 2.0 * (1.0 - t) * t * f64::from(p1.position.x)
            + t.powi(2) * f64::from(p2.position.x);
        let y = (1.0 - t).powi(2) * f64::from(p0.position.y)
            + 2.0 * (1.0 - t) * t * f64::from(p1.position.y)
            + t.powi(2) * f64::from(p2.position.y);

        canvas.draw_point(sdl2::rect::Point::new(x as i32, y as i32))?;

        t += 0.0001;
    }
    Ok(())
}

fn draw_cubic_bezier_curve(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    p0: &ControlPoint,
    p1: &ControlPoint,
    p2: &ControlPoint,
    p3: &ControlPoint,
) -> Result<(), String> {
    canvas.set_draw_color(Color::RGB(0, 0, 255));

    let mut t: f64 = 0.0;
    loop {
        if t >= 1.0 {
            break;
        }

        let x = (1.0 - t).powi(3) * f64::from(p0.position.x)
            + 3.0 * (1.0 - t).powi(2) * t * f64::from(p1.position.x)
            + 3.0 * (1.0 - t) * t.powi(2) * f64::from(p2.position.x)
            + t.powi(3) * f64::from(p3.position.x);
        let y = (1.0 - t).powi(3) * f64::from(p0.position.y)
            + 3.0 * (1.0 - t).powi(2) * t * f64::from(p1.position.y)
            + 3.0 * (1.0 - t) * t.powi(2) * f64::from(p2.position.y)
            + t.powi(3) * f64::from(p3.position.y);

        canvas.draw_point(sdl2::rect::Point::new(x as i32, y as i32))?;

        t += 0.0001;
    }
    Ok(())
}

struct ControlPoint {
    position: sdl2::rect::Point,
    draw_radius: i16,
    draw_color: sdl2::pixels::Color,
    selected: bool,
}

impl ControlPoint {
    fn new(position: sdl2::rect::Point) -> ControlPoint {
        ControlPoint {
            position,
            draw_radius: 6,
            draw_color: sdl2::pixels::Color {
                r: 255,
                g: 0,
                b: 0,
                a: 255,
            },
            selected: false,
        }
    }

    fn is_selected(&self, x: i32, y: i32) -> bool {
        let distance =
            (f64::from((x - self.position.x).pow(2) + (y - self.position.y).pow(2)).sqrt()) as i16;
        distance <= self.draw_radius
    }

    fn mark_selected(&mut self) {
        self.selected = true;
        self.draw_color = sdl2::pixels::Color {
            r: 0,
            g: 255,
            b: 0,
            a: 255,
        };
    }

    fn mark_unselected(&mut self) {
        self.selected = false;
        self.draw_color = sdl2::pixels::Color {
            r: 255,
            g: 0,
            b: 0,
            a: 255,
        };
    }

    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<(), String> {
        canvas.filled_circle(
            self.position.x as i16,
            self.position.y as i16,
            self.draw_radius,
            self.draw_color,
        )?;
        Ok(())
    }

    fn move_to(&mut self, x: i32, y: i32) {
        self.position.x = x;
        self.position.y = y;
    }
}

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("bezier_curves", 1600, 900)
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    let mut points = vec![
        ControlPoint::new(sdl2::rect::Point::new(100, 50)),
        ControlPoint::new(sdl2::rect::Point::new(250, 800)),
        ControlPoint::new(sdl2::rect::Point::new(1300, 200)),
        ControlPoint::new(sdl2::rect::Point::new(1500, 800)),
    ];

    let mut event_pump = sdl_context.event_pump().unwrap();

    'main_loop: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        draw_linear_bezier_curve(&mut canvas, &points[0], &points[1])?;
        draw_quadratic_bezier_curve(&mut canvas, &points[0], &points[1], &points[2])?;
        draw_cubic_bezier_curve(&mut canvas, &points[0], &points[1], &points[2], &points[3])?;
        draw_points(&mut canvas, &points)?;

        canvas.present();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main_loop,
                Event::MouseButtonDown { x, y, .. } => {
                    for point in points.iter_mut() {
                        if point.is_selected(x, y) {
                            point.mark_selected();
                        }
                    }
                }
                Event::MouseButtonUp { .. } => {
                    for point in points.iter_mut().filter(|p| p.selected) {
                        point.mark_unselected();
                    }
                }
                Event::MouseMotion { x, y, .. } => {
                    for point in points.iter_mut().filter(|p| p.selected) {
                        point.move_to(x, y);
                    }
                }
                _ => {}
            }
        }
    }

    Ok(())
}
