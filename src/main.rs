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

    // create the window, initial width/height
    let window_size = ( 1200, 1200 );
    let window = video_subsystem.window("rust fractal", window_size.0, window_size.1)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

/*
 * experiment with spawning threads
 *
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

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut col : i32 = 0;
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

        // fractal routine
        if drawing {
            let start_lambda = 3.5;
            let end_lambda = 4.0;

            let lambda = col as f64 * (end_lambda - start_lambda) / window_size.0 as f64 + start_lambda;
            let mut x : f64 = 0.5; // initial condition

            // draw up to this many points
            let mut points_drawn = 0;
            let max_points_drawn = 1000;

            // iterate this many times, maximum
            let iters : i32 = 1000000;
            for _i in 0 .. iters {
                x = x * lambda * (1.0 - x);

                /*
                // discard first few generations
                if _i < 1000 {
                    continue;
                }
                */

                // compute the integer row position
                let row = ((1.0 - x) * window_size.1 as f64) as i32;

                // compute the color (few different algorithms)

                //let color = (i as f64 / iters as f64) * 256.0;
                //let color = points_drawn as f64 / max_points_drawn as f64;

                // compute based on number of iterations, clamped to 1.0
                let color = (_i as f64 * 0.001).min(1.0);

                let color_r = 0.0;
                let color_g = 1.0 - color;
                let color_b = color;
                //println!("color {} r {} g {} b {}", color, color_r, color_g, color_b);

                let color_r = (color_r * 256.0) as u8;
                let color_g = (color_g * 256.0) as u8;
                let color_b = (color_b * 256.0) as u8;
                canvas.set_draw_color(Color::RGB(color_r, color_g, color_b));

                // draw a point
                use sdl2::rect::Point;
                let p = Point::new(col, row);
                canvas.draw_point(p).unwrap();
                //println!("lambda {} col {} x {}", lambda, col, x);

                // stop drawing after we've plotted enough points
                points_drawn += 1;
                if points_drawn >= max_points_drawn {
                    break;
                }
            }

            // next column, stop if we hit the end
            col += 1;
            if col == window_size.0 as i32 {
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
