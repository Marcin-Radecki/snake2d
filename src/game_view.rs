//! Game view.

use graphics::types::Color;
use graphics::{Context, Graphics};

use crate::game_controller;

/// Stores game view settings.
pub struct GameViewSettings {
    /// Background color
    pub background_color: Color,
    /// Border color
    pub border_color: Color,
    /// Snake body color
    pub snake_body_color : Color,
    /// Snake head color
    pub snake_head_color : Color,
    /// Obstacle color
    pub obstacles_color: [Color; 3],
}

impl GameViewSettings {
    /// Creates new game view settings.
    pub fn new() -> GameViewSettings {
        GameViewSettings {
            background_color: [1.0, 1.0, 1.0, 1.0],
            border_color: [1.0, 1.0, 1.0, 1.0],
            snake_body_color: [1.0, 0.0, 0.0, 1.0],
            snake_head_color: [0.8, 1.0, 0.0, 1.0],
            obstacles_color: [
                [0.6, 0.4, 0.2, 1.0],
                [0.75, 0.75, 0.75, 1.0],
                [0.9, 0.9, 0.0, 1.0],
                ],
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
        let &snake_head_segment = controller.game_logic.get_snake_segments().front().unwrap();

        for &segment in controller.game_logic.get_snake_segments().iter() {
            let color = if segment == snake_head_segment {
                self.settings.snake_head_color
            } else {
                self.settings.snake_body_color
            };
            let square = graphics::rectangle::square(segment.x as f64 * segment_size,
                                           segment.y as f64 * segment_height, segment_size);
            graphics::rectangle(color, square, c.transform, g);
        }

        for obstacle in controller.game_logic.get_obstacles() {
            let square = graphics::rectangle::square(obstacle.0 as f64 * segment_size,
                                           obstacle.1 as f64 * segment_height, segment_size);
            let points = obstacle.2 as u8 - 1;
            let obstacle_color = self.settings.obstacles_color[points as usize];
            graphics::rectangle(obstacle_color, square, c.transform, g);
        }
    }
}