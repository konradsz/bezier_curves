extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;

fn draw_linear_bezier_curve(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    p0: &sdl2::rect::Point,
    p1: &sdl2::rect::Point,
) {
    canvas.set_draw_color(Color::RGB(0, 255, 0));

    let mut t: f64 = 0.0;
    loop {
        if t >= 1.0 {
            break;
        }

        let x = f64::from(p0.x) * (1.0 - t) + f64::from(p1.x) * t;
        let y = f64::from(p0.y) * (1.0 - t) + f64::from(p1.y) * t;
        canvas.draw_point(sdl2::rect::Point::new(x as i32, y as i32));

        t += 0.0001;
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("bezier_curves", 1600, 900)
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.clear();
    canvas.set_draw_color(Color::RGB(255, 0, 0));

    let p0 = sdl2::rect::Point::new(200, 100);
    let p1 = sdl2::rect::Point::new(250, 300);

    draw_linear_bezier_curve(&mut canvas, &p0, &p1);

    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    'main_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main_loop,
                _ => {}
            }
        }
    }
}
