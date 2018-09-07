extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate sdl2_window;

use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use sdl2_window::Sdl2Window as Window;

mod app;
use app::App;

const OPENGL_VERSION: OpenGL = OpenGL::V3_2;

const WINDOW_NAME: &str = "Sort Visualization";
const WINDOW_SIZE: (u32, u32) = (640, 480);

fn main() {
  let mut window: Window = WindowSettings::new(WINDOW_NAME, WINDOW_SIZE)
    .opengl(OPENGL_VERSION)
    .exit_on_esc(true)
    .build()
    .expect("couldn't create window");
  let mut gl = GlGraphics::new(OPENGL_VERSION);

  let mut app = App::new();

  let mut events = Events::new(EventSettings::new());
  while let Some(event) = events.next(&mut window) {
    if let Some(render_args) = event.render_args() {
      app.render(&mut gl, render_args);
    }

    if let Some(update_args) = event.update_args() {
      app.update(update_args);
    }
  }
}
