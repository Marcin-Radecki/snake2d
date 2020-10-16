//! Game view.

use graphics::types::Color;
use graphics::{Context, Graphics, Transformed, ImageSize};
use opengl_graphics::{Filter, Texture, TextureSettings};

use crate::game_controller;
use graphics::math::Scalar;
use std::collections::{HashMap};
use glob::glob;

/// Stores game view settings.
pub struct GameViewSettings {
    /// Background color
    background_color: Color,
    /// Border color
    border_color: Color,
    /// Snake body color
    snake_body_color: Color,
    /// Snake head color
    snake_head_color: Color,
    /// scoring of obstacles
    scores_lookup: HashMap<u8, String>,
}

impl GameViewSettings {
    /// Creates new game view settings.
    pub fn new() -> GameViewSettings {
        let mut scores: HashMap<u8, String> = HashMap::new();
        scores.insert(1, String::from("apple"));
        scores.insert(2, String::from("banana"));
        scores.insert(3, String::from("cherry"));
        GameViewSettings {
            background_color: [1.0, 1.0, 1.0, 1.0],
            border_color: [1.0, 1.0, 1.0, 1.0],
            snake_body_color: [1.0, 0.0, 0.0, 1.0],
            snake_head_color: [0.8, 1.0, 0.0, 1.0],
            scores_lookup: scores,
        }
    }
}

/// Stores visual information about a game.
pub struct GameView {
    /// Stores game view settings.
    pub settings: GameViewSettings,
    /// lookup for textures
    pub textures: HashMap<String, Texture>,
}

impl GameView {
    /// Creates a new game view.
    pub fn new(settings: GameViewSettings) -> GameView {
        GameView {
            settings,
            textures: Default::default(),
        }
    }

    pub fn load_textures(&mut self) {
        let assets = find_folder::Search::ParentsThenKids(3, 3).for_folder("assets").unwrap();
        for entry in glob(assets.join("*.png").to_str().unwrap()).expect("Failed to find textures in asset directory!") {
            match entry {
                Ok(path) => {
                    let path_stem = path.file_stem().unwrap().to_str().unwrap();
                    let path = path.to_str().unwrap();
                    let texture = self.load_texture_from_path(path);
                    self.textures.insert(String::from(path_stem), texture);
                    println!("{}", path);

                }
                Err(e) => println!("{:?}", e),
            }
        }
    }

    fn load_texture_from_path(&self, path: &str) -> Texture {
        let texture_settings = TextureSettings::new().filter(Filter::Nearest);
        Texture::from_path(path, &texture_settings).unwrap()
    }

    fn get_scaling_factor(&self, texture: &Texture, desired_size: Scalar) -> Scalar {
        assert_eq!(texture.get_height(), texture.get_width());
        1.0 / (texture.get_width() as Scalar / desired_size)
    }

    pub fn draw_texture_at_position<G: Graphics<Texture = Texture>>(&self,
                                                c: &Context,
                                                g: &mut G,
                                                texture: &Texture,
                                                position: [f64;2],
                                                segment_size: Scalar) {
        let t = c.trans(position[0], position[1]).
            zoom(self.get_scaling_factor(texture, segment_size)).transform;
        graphics::image(texture, t, g);
    }



    /// Draw game view.
    pub fn draw<G: Graphics<Texture = Texture>>(&self, controller: &game_controller::GameController, c: &Context, g: &mut G, screen_size: [f64;2]) {
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
            let obstacle_left_upper_corner_position = [obstacle.0 as f64 * segment_size,
                obstacle.1 as f64 * segment_height];
            let square = graphics::rectangle::square(
                obstacle_left_upper_corner_position[0],
                obstacle_left_upper_corner_position[1],
                segment_size);
            let points = obstacle.2;
            let texture_name = self.settings.scores_lookup.get(&points).unwrap();
            let texture = self.textures.get(texture_name).unwrap();
            self.draw_texture_at_position( &c, g, texture, obstacle_left_upper_corner_position, segment_size);

        }

    }
}