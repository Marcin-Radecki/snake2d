//! Game controller.

use piston::input::{GenericEvent, Button, Key};
use crate::{game_logic, Direction};

/// Handles events for snake2d game.
pub struct GameController {
    /// Stores the gameboard state.
    pub game_logic: game_logic::GameLogic,
    /// Current direction where snake moves,
    pub direction: Option<Direction>,
}

impl GameController {
    /// Creates a new game logic controller.
    pub fn new(game_logic: game_logic::GameLogic) -> GameController {
        GameController {
            game_logic,
            direction: None,
        }
    }

    /// Handles events.
    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            if key == Key::Right {
                self.direction = Some(Direction::Right);
            } else if key == Key::Left {
                self.direction = Some(Direction::Left);
            }  else if key == Key::Up {
                self.direction = Some(Direction::Up);
            }  else if key == Key::Down {
                self.direction = Some(Direction::Down);
            }
        };

        if let Some(_args) = e.update_args() {
            self.game_logic.main_loop(self.direction);
        }
    }
}