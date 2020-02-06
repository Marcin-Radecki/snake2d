//! Game view.

use graphics::types::Color;
use graphics::{Context, Graphics};

use crate::game_controller;

/// Stores game view settings.
pub struct GameViewSettings {
    /// Size of snake board along horizontal and vertical edge
    pub size: f64,
    /// Background color
    pub background_color: Color,
    /// Border color
    pub border_color: Color,
    /// Snake body color
    pub snake_body_color : Color,
    /// Snake head color
    pub snake_head_color : Color,
    /// Obstacle color
    pub obstacle_color : Color,
}

impl GameViewSettings {
    /// Creates new game view settings.
    pub fn new() -> GameViewSettings {
        GameViewSettings {
            size: 640.0,
            background_color: [0.0, 1.0, 0.0, 1.0],
            border_color: [1.0, 1.0, 1.0, 1.0],
            snake_body_color: [1.0, 0.0, 0.0, 1.0],
            snake_head_color: [0.8, 0.5, 0.0, 1.0],
            obstacle_color: [0.0, 0.0, 1.0, 1.0],
        }
    }
}

/// Stores visual information about a game.
pub struct GameView {
    /// Stores game view settings.
    pub settings: GameViewSettings,
}

impl GameView {
    /// Creates a new game view.
    pub fn new(settings: GameViewSettings) -> GameView {
        GameView {
            settings,
        }
    }

    /// Draw game view.
    pub fn draw<G: Graphics>(&self, controller: &game_controller::GameController, c: &Context, g: &mut G, screen_size: [f64;2]) {
        let board_size = controller.game_logic.get_board_size();
        let (segment_size, segment_height) = (screen_size[0] / board_size.0 as f64, screen_size[1] / board_size.1 as f64);
        graphics::clear(self.settings.background_color, g);
        for &segment in controller.game_logic.get_snake_segments().iter() {
            let square = graphics::rectangle::square(segment.x as f64 * segment_size,
                                           segment.y as f64 * segment_height, segment_size);
            graphics::rectangle(self.settings.snake_body_color, square, c.transform, g);
        }

        for obstacle in controller.game_logic.get_obstacles() {
            let square = graphics::rectangle::square(obstacle.0 as f64 * segment_size,
                                           obstacle.1 as f64 * segment_height, segment_size);
            graphics::rectangle(self.settings.obstacle_color, square, c.transform, g);
        }
    }
}