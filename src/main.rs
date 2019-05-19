extern crate sdl2;

use sdl2::event::Event;
use sdl2::pixels;
use sdl2::keyboard::Keycode;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

// SDL2
// Trying to figure out terminology. Some of this seems to originate in OpenGL, some in SDL.
// - get a window
// - get a renderer for (from) that window
// - render stuff to the window
//   - stuff is {surface, texture, rectangles, ..?}
//     - SDL_RenderDraw{Line,Rect,Point}
// - "surface" seems to roughly mean bitmap
//   - stored in main memory
// - "texture" also means bitmap
//   - stored in GPU memory

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsys = sdl_context.video()?;
    let window = video_subsys.window("trying to hand draw a line with opengl", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut lastx = 0;
    let mut lasty = 0;

    let mut events = sdl_context.event_pump()?;
    'main: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} => break 'main,

                Event::KeyDown {keycode: Some(keycode), ..} => {
                    if keycode == Keycode::Escape {
                        break 'main
                    } else if keycode == Keycode::Space {
                        for i in 0..400 {
                            canvas.set_draw_color(pixels::Color::RGB(255, 0, 0));
                            canvas.draw_point((i, i))?;
                        }
                        canvas.present();
                    }
                }

                Event::MouseButtonDown {x, y, ..} => {
                    let color = pixels::Color::RGB(x as u8, y as u8, 255);
                    canvas.set_draw_color(color);
                    canvas.draw_line((lastx, lasty), (x, y))?;
                    lastx = x;
                    lasty = y;
                    println!("mouse btn down at ({},{})", x, y);
                    canvas.present();
                }

                _ => {}
            }
        }
    }

    Ok(())
}
