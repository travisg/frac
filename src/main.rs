extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
//use std::thread;
//use std::sync::Arc;
//use std::sync::Mutex;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    //let event_subsystem = sdl_context.event().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 1000, 1000)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

/*
    let mut threads = vec![];

    for i in 0..10 {
        let event_sender = event_subsystem.event_sender();
        let t = thread::spawn(move || {
            let e  = Event::User {
                timestamp: 0,
                window_id: 0,
                type_: i + 1,
                code: 0,
                data1: ::std::ptr::null_mut(),
                data2: ::std::ptr::null_mut(),
            };

            event_sender.push_event(e);
        });
        threads.push(t);
    }
*/

    canvas.set_draw_color(Color::RGB(255, 255, 255));

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut col = 0;
    let mut drawing = true;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => { /* println!("{:?}", event); */ }
            }
        }

        if drawing {
            let lambda = col as f64 * 1.5 / 1000.0 + 2.5;
            let mut x = 0.9; // initial condition
            let mut points_drawn = 0;

            for i in 0 .. 1000000 {
                //println!("x before {}", x);
                x = x * lambda * (1.0 - x);
                //println!("x after {}", x);

                // warm up generations
                if i < 1000 {
                    continue;
                }

                // compute the integer row position
                let row = ((1.0 - x) * 1000.0) as i32;

                // draw a point
                use sdl2::rect::Point;
                let p = Point::new(col, row);
                canvas.draw_point(p).unwrap();
                //println!("lambda {} col {} x {}", lambda, col, x);
                points_drawn += 1;
                if points_drawn >= 1000 {
                    break;
                }
            }

            // next column, stop if we hit the end
            col += 1;
            if col == 1000 {
                drawing = false;
            }

            canvas.present();
        } else {
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }

/*
    // wait for all the threads to end
    for t in threads {
        t.join().unwrap();
    }
*/
}
