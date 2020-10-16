//TODO
//#![deny(missing_docs)]

//! A snake2d game.

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent};
use piston::window::{WindowSettings, AdvancedWindow};

use snake2d::*;

fn dump_event_settings(events_settings: &EventSettings) {
    println!("{}", events_settings.bench_mode);
    println!("{}", events_settings.lazy);
    println!("{}", events_settings.max_fps);
    println!("{}", events_settings.swap_buffers);
    println!("{}", events_settings.ups);
    println!("{}", events_settings.ups_reset);
}

fn main() {
    let opengl = OpenGL::V3_2;
    let settings =
        WindowSettings::new("snake", [480, 480]).
            graphics_api(opengl).
            exit_on_esc(true);
    let mut window:Window = settings.build().expect("Could not create window");


    let mut event_settings = EventSettings::new();
    const FPS:u64= 15;
    event_settings.ups = FPS;
    event_settings.max_fps = FPS;
    dump_event_settings(&event_settings);
    let mut events = Events::new(event_settings);

    let mut gl = GlGraphics::new(opengl);

    const WIDTH: usize = 25;
    const HEIGHT: usize = 25;
    const STARTING_SEGMENT : Segment = Segment::new(5, 6);
    let game_logic = game_logic::GameLogic::new(WIDTH, HEIGHT, STARTING_SEGMENT);
    let mut game_controller = game_controller::GameController::new(game_logic);
    let game_view_settings = game_view::GameViewSettings::new();
    let mut game_view = game_view::GameView::new(game_view_settings);
    game_view.load_textures();

    while let Some(e) = events.next(&mut window) {
        game_controller.event(&e);
        window.set_title(String::from(format!("snake! Points: {}", game_controller.game_logic.get_points())));

        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                game_view.draw(&game_controller, &c, g, args.window_size);
            });
        }
    }
}
