use sdl2::event::Event;

use crate::winsdl::Winsdl;

mod winsdl;

fn main() {
    let mut winsdl = Winsdl::new(800, 600).unwrap();

    let mut hue = 0;

    'running: loop {
        for event in winsdl.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                _ => { }
            }
        }
        
        unsafe {
            let c = 1.0_f32;
            let x = c * (1.0 - (hue as f32 / 60.0 % 2.0 - 1.0).abs());
            let (r, g, b) =
            if hue < 60 {
                (c, x, 0.0)
            } else if hue < 120 {
                (x, c, 0.0)
            } else if hue < 180 {
                (0.0, c, x)
            } else if hue < 240 {
                (0.0, x, c)
            } else if hue < 300 {
                (x, 0.0, c)
            } else {
                (c, 0.0, x)
            };
            
            hue += 1;
            hue %= 360;

            gl::ClearColor(r, g, b, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        winsdl.window.gl_swap_window();
    }
}
