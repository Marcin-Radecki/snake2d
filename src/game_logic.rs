//! Game logic.

use crate::snake;
use crate::board;
use crate::Direction;
use crate::Segment;
use crate::Obstacle;

use std::collections::HashSet;
use rand::Rng;

/// TODO comment
#[derive(PartialEq, Debug)]
enum Collision {
    None,
    Wall,
    Obstacle(Obstacle),
    Snake,
}

pub struct GameLogic {
    board: board::Board,
    snake: snake::Snake,
    main_loop_counter: usize,
    last_direction: Option<Direction>,
}

impl GameLogic {
    pub fn new(width: usize, height: usize, initial_snake_location: Segment) -> GameLogic {
        let board = board::Board::new(width, height);
        assert_eq!(board.segment_in(&initial_snake_location), true);
        let game_logic = GameLogic {
            board,
            snake: snake::Snake::new(&vec![initial_snake_location]),
            main_loop_counter: 0,
            last_direction: None,
        };

        game_logic
    }

    pub fn get_board_size(&self) -> (usize, usize) {
        (self.board.width(), self.board.height())
    }

    pub fn get_snake_segments(&self) -> snake::Body {
        self.snake.body.clone()
    }

    pub fn get_obstacles(&self) -> Vec<(usize, usize)> {
        let mut obstacles = Vec::new();
        for i in 0..self.board.width() {
            for j in 0..self.board.height() {
                if self.board.get_field(i, j) == Obstacle::Apple {
                    obstacles.push((i, j));
                }
            }
        }
        obstacles
    }

    pub fn get_points(&self) -> usize {
        self.snake.len()
    }

    fn generate_obstacles(&mut self, max_obstacles_count : usize) {
        self.set_obstacles(&self.generate_obstacles_positions(max_obstacles_count));
    }

    fn set_obstacles(&mut self, obstacles: &HashSet<(usize, usize)>) {
        for obstacle in obstacles {
            self.board.set_obstacle(obstacle.0, obstacle.1);
        }
    }

    fn generate_obstacles_positions(&self, max_obstacles_count: usize) -> HashSet<(usize, usize)> {
        assert!(max_obstacles_count > 0);
        let mut rng = rand::thread_rng();
        let board_capacity = self.board.height() * self.board.width();
        let obstacles_count = self.board.get_number_of_obstacles();
        let snake_segments_count = self.snake.body.len() as usize;
        let mut generated_obstacles = HashSet::new();
        for i in 0..max_obstacles_count {
            if i + 1 + obstacles_count + snake_segments_count > board_capacity {
                break;
            }
            loop {
                let random_index = rng.gen::<usize>() % board_capacity;
                let segment_guess = self.convert_index_to_coords(random_index);
                if !generated_obstacles.contains(&segment_guess) &&
                    self.board.get_field(segment_guess.0, segment_guess.1) == Obstacle::None &&
                    (!self.snake.body.contains(&Segment::new(segment_guess.0 as i32, segment_guess.1 as i32))) {
                    generated_obstacles.insert(segment_guess);
                    break;
                }
            }
        }
        generated_obstacles
    }

    fn check_collisions(&self) -> Collision {
        for segment in &self.snake.body {
            if segment.x < 0 || segment.x >= self.board.width() as i32 ||
                segment.y < 0 || segment.y >= self.board.height() as i32 {
                return Collision::Wall;
            }

            let obstacle = self.board.get_field(segment.x as usize, segment.y as usize);
            if obstacle != Obstacle::None {
                return Collision::Obstacle(obstacle);
            }
        }
        if !self.snake.has_unique_segments() {
            return Collision::Snake;
        }
        Collision::None
    }

    fn convert_index_to_coords(&self, index: usize) -> (usize, usize) {
        ((index / self.board.height()) % self.board.width(), index % self.board.height())
    }

    fn snake_eat(&mut self) {
        let snake_head = self.snake.body.front().unwrap();
        self.board.clear_obstacle(snake_head.x as usize, snake_head.y as usize);
        self.snake.grow(1);
    }

    pub fn main_loop(&mut self, snake_move: Option<Direction>) {
        let move_direction = match snake_move {
            Some(direction) => Some(direction),
            None => {
                match self.last_direction {
                    Some(direction) => Some(direction),
                    None => None,
                }
            }
        };

        if move_direction.is_some() {
            let mut direction = move_direction.unwrap();
            if self.last_direction.is_some() {
                let last_direction = self.last_direction.unwrap();
                let mut override_direction = false;
                match direction {
                    Direction::Up => {
                        if last_direction == Direction::Down {
                            override_direction = true;
                        }
                    },
                    Direction::Down => {
                        if last_direction == Direction::Up {
                            override_direction = true;
                        }
                    },
                    Direction::Left => {
                        if last_direction == Direction::Right {
                            override_direction = true;
                        }
                    },
                    Direction::Right => {
                        if last_direction == Direction::Left {
                            override_direction = true;
                        }
                    },
                }
                if override_direction {
                    direction = last_direction;
                }
            }

            self.snake.move_body(&direction);
            match self.check_collisions() {
                Collision::Wall => panic!("You've hit the wall!"),
                Collision::Obstacle(_obstacle) => {
                    self.snake_eat();
                },
                Collision::None => (),
                Collision::Snake => panic!("You've hit yourself!"),
            }
            self.last_direction = Some(direction);
        }

        self.main_loop_counter += 1;
        if self.main_loop_counter % 100 == 1 {
            self.generate_obstacles(10);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_initial_game_state_then_no_collisions_are_detected() {
        let game_logic = GameLogic::new(25, 23, Segment::new(10, 12));
        assert_eq!(game_logic.check_collisions(), Collision::None);
    }

    #[test]
    #[should_panic]
    fn given_snake_outside_board_x_then_wall_collision_is_detected() {
        let game_logic = GameLogic::new(25, 23, Segment::new(25, 12));

        assert_eq!(game_logic.check_collisions(), Collision::Wall);
    }

    #[test]
    #[should_panic]
    fn given_snake_outside_board_y_then_wall_collision_is_detected() {
        let game_logic = GameLogic::new(25, 23, Segment::new(10, 23));

        assert_eq!(game_logic.check_collisions(), Collision::Wall);
    }

    #[test]
    #[should_panic]
    fn given_snake_outside_board_x_negative_then_wall_collision_is_detected() {
        let game_logic = GameLogic::new(25, 23, Segment::new(-1, 12));

        assert_eq!(game_logic.check_collisions(), Collision::Wall);
    }

    #[test]
    #[should_panic]
    fn given_snake_outside_board_y_negative_then_wall_collision_is_detected() {
        let game_logic = GameLogic::new(25, 23, Segment::new(5, -12));

        assert_eq!(game_logic.check_collisions(), Collision::Wall);
    }

    #[test]
    fn given_coords_then_coords_have_correct_index_equivalents() {
        let (width, height) = (88usize,20usize);
        let (x, y) = (70usize, 2usize);
        let index = x * height + y;

        let game_logic = GameLogic::new(width, height, Segment::new(10, 12));
        assert_eq!(game_logic.convert_index_to_coords(index), (x, y));
        assert_eq!(game_logic.convert_index_to_coords(0), (0, 0));
        assert_eq!(game_logic.convert_index_to_coords(height-1), (0, height-1));
        assert_eq!(game_logic.convert_index_to_coords(height*(width-1)), (width-1, 0));
        assert_eq!(game_logic.convert_index_to_coords(height*(width-1) + height - 1), (width-1, height -1));
    }

    fn generate_n_obstacles(n:usize, width:usize, height:usize) {
        let game_logic = GameLogic::new(width, height, Segment::new(0, 0));
        let obstacles = game_logic.generate_obstacles_positions(n);
        if n == width * height {
            assert_eq!(obstacles.len(), n - game_logic.snake.body.len() as usize);
        } else {
            assert_eq!(obstacles.len(), n);
        }
        for obstacle in obstacles {
            match game_logic.snake.body.iter().find(| &&segment|
                (segment.x as usize, segment.y as usize) == obstacle) {
                Some(erroneus_segment) => panic!("Snake segment ({}, {}) has \
                    the same position as generated obstacle!", erroneus_segment.x, erroneus_segment.y),
                None => ()
            }            assert_eq!(game_logic.board.segment_in(&Segment::new(obstacle.0 as i32, obstacle.1 as i32)), true);
        }
    }

    #[test]
    fn given_empty_board_when_generating_obstacles_then_obstacles_are_generated() {
        let (width, height) = (7usize, 11usize);
        for i in 1..(width*height+1) {
            generate_n_obstacles(i, width, height);
        }
    }

    #[test]
    #[should_panic]
    fn given_mpty_board_when_0_obstacles_are_generated_then_panic() {
        let game_logic = GameLogic::new(1, 1, Segment::new(0, 0));
        let _obstacles = game_logic.generate_obstacles_positions(0);
    }

    #[test]
    fn given_empty_board_when_generating_obstacles_above_capacity_then_obstacles_are_generated_to_capacity_only() {
        let game_logic = GameLogic::new(5, 5, Segment::new(0, 0));
        let obstacles = game_logic.generate_obstacles_positions(5*5+1);
        // - 1 since we cannot generate an obstacle where snake segment is
        assert_eq!(obstacles.len(), 5*5 - 1);
    }

    #[test]
    fn given_empty_big_board_when_generating_many_obstcles_then_execution_should_be_fast() {
        // TODO set timeout for this unit test to 30s
        let game_logic = GameLogic::new(1000, 1000, Segment::new(0, 0));
        let obstacles = game_logic.generate_obstacles_positions(1000000);
        assert_eq!(obstacles.len(), 1000000 - game_logic.snake.body.len());
    }

    #[test]
    fn given_1x1_board_with_1_segment_snake_when_generating_1_obtacle_then_no_obstacles_are_generated() {
        let game_logic = GameLogic::new(1, 1, Segment::new(0, 0));
        let obstacles = game_logic.generate_obstacles_positions(1);
        assert_eq!(obstacles.len(), 0);
    }

    #[test]
    fn given_1x2_board_with_1_segment_snake_when_generating_1_obstacle_then_1_obstacle_is_generated_at_no_snake_position() {
        let game_logic = GameLogic::new(1, 2, Segment::new(0, 0));
        let obstacles = game_logic.generate_obstacles_positions(1);
        assert_eq!(obstacles.len(), 1);
        assert_ne!(obstacles.iter().next().unwrap(), &(0usize, 0usize));
    }

    //  A
    // A@A
    #[test]
    fn given_1_segment_snake_when_obstacles_are_at_snake_end_then_snake_eat_does_not_append_segment() {
        let mut game_logic = GameLogic::new(7, 13, Segment::new(5, 6));
        game_logic.board.set_obstacle(4,  6);
        game_logic.board.set_obstacle(5,  5);
        game_logic.board.set_obstacle(5,  6);
        game_logic.board.set_obstacle(6,  6);
        assert_eq!(game_logic.check_collisions(), Collision::Obstacle(Obstacle::Apple));
        game_logic.snake_eat();
        assert_eq!(game_logic.check_collisions(), Collision::None);
        assert_eq!(game_logic.snake.len(), 2);
        assert_eq!(game_logic.snake.body.front().unwrap(), &Segment::new(5, 6));
        assert_eq!(game_logic.snake.body.back().unwrap(), &Segment::new(5, 6));
    }

    fn test_snake_move_expected_collision(game_logic: &mut GameLogic, direction: &Direction) {
        let current_snake_len = game_logic.snake.len();
        let current_snake_front = game_logic.snake.body.front().unwrap();
        let current_snake_back = game_logic.snake.body.back().unwrap();
        let next_front_segment = direction.nearest_segment(current_snake_front);
        let next_back_segment: Segment;
        if game_logic.snake.will_grow() {
            next_back_segment = current_snake_back.clone();
        } else {
            next_back_segment = direction.nearest_segment(current_snake_back);
        }

        game_logic.snake.move_body(&direction);
        assert_eq!(game_logic.snake.len(), current_snake_len);
        assert_eq!(game_logic.check_collisions(), Collision::Obstacle(Obstacle::Apple));
        assert_eq!(game_logic.snake.body.front().unwrap(), &next_front_segment);
        assert_eq!(game_logic.snake.body.back().unwrap(), &next_back_segment);
        game_logic.snake_eat();
        assert_eq!(game_logic.snake.len(), current_snake_len + 1);
        assert_eq!(game_logic.check_collisions(), Collision::None);
    }

    //  AA      AA      AA
    // A@AA => A#@A => A##@
    //   A       A       A
    #[test]
    fn given_1_segment_snake_when_eat_twice_snake_has_3_segments() {
        let mut game_logic = GameLogic::new(8, 13, Segment::new(5, 6));
        game_logic.board.set_obstacle(4,  6);
        game_logic.board.set_obstacle(5,  5);
        game_logic.board.set_obstacle(6,  5);
        game_logic.board.set_obstacle(6,  6);
        game_logic.board.set_obstacle(7,  6);
        game_logic.board.set_obstacle(6,  7);

        test_snake_move_expected_collision(&mut game_logic, &Direction::Right);
        test_snake_move_expected_collision(&mut game_logic, &Direction::Right);
    }

    #[test]
    fn given_empty_board_when_set_obstacles_is_called_then_obstacles_have_correct_positions() {
        let mut game_logic = GameLogic::new(8, 13, Segment::new(5, 6));
        let mut obstacles=  vec![(0usize, 0usize), (1usize, 1usize), (7usize, 12usize),
            (7usize, 0usize), (0usize, 12usize), (4usize, 10usize)];
        let mut obstacles_set= HashSet::new();
        for &obstacle in obstacles.iter() {
            obstacles_set.insert(obstacle);
        }
        game_logic.set_obstacles(&obstacles_set);

        let mut obstacles_actual = game_logic.get_obstacles();
        obstacles.sort();
        obstacles_actual.sort();
        assert_eq!(&obstacles, &obstacles_actual);
    }

    #[test]
    fn given_more_than_1_segment_snake_when_snake_moves_and_crosses_itself_then_snake_collision_is_detected() {
        let mut game_logic = GameLogic::new(8, 13, Segment::new(0, 0));
        for i in 1..6 {
            game_logic.board.set_obstacle(i,  0);
        }

        for i in 1..6 {
            game_logic.main_loop(Some(Direction::Right));
            assert_eq!(game_logic.snake.len(), i + 1);
        }
        game_logic.main_loop(Some(Direction::Down));
        assert_eq!(game_logic.check_collisions(), Collision::None);
        game_logic.main_loop(Some(Direction::Left));
        assert_eq!(game_logic.check_collisions(), Collision::None);
        game_logic.snake.move_body(&Direction::Up);
        let segment = Segment::new(4, 0);
        assert_eq!(game_logic.snake.body.front().unwrap(), &segment);
        assert_eq!(game_logic.check_collisions(), Collision::Snake);
    }

    #[test]
    fn given_more_2_segment_snake_when_snake_moves_backward_then_nothing_happens() {
        let mut game_logic = GameLogic::new(8, 13, Segment::new(5, 6));
        game_logic.board.set_obstacle(6,  6);

        game_logic.main_loop(Some(Direction::Right));
        assert_eq!(game_logic.snake.len(), 2);
        assert_eq!(game_logic.snake.body.front().unwrap(), &Segment::new(6, 6));
        assert_eq!(game_logic.snake.body.back().unwrap(), &Segment::new(6,6 ));
        game_logic.main_loop(Some(Direction::Left));
        assert_eq!(game_logic.snake.body.front().unwrap(), &Segment::new(7, 6));
        assert_eq!(game_logic.snake.body.back().unwrap(), &Segment::new(6,6 ));
    }

    #[test]
    fn given_more_than_2_segment_snake_when_snake_moves_backward_then_nothing_happens() {
        let mut game_logic = GameLogic::new(100, 103, Segment::new(5, 6));
        game_logic.board.set_obstacle(6,  6);
        game_logic.board.set_obstacle(7,  6);

        game_logic.main_loop(Some(Direction::Right));
        assert_eq!(game_logic.snake.len(), 2);
        assert_eq!(game_logic.snake.body.front().unwrap(), &Segment::new(6, 6));
        assert_eq!(game_logic.snake.body.back().unwrap(), &Segment::new(6,6 ));

        game_logic.main_loop(Some(Direction::Right));
        assert_eq!(game_logic.snake.len(), 3);
        assert_eq!(game_logic.snake.body.front().unwrap(), &Segment::new(7, 6));
        assert_eq!(game_logic.snake.body.back().unwrap(), &Segment::new(6, 6));

        game_logic.main_loop(Some(Direction::Right));
        assert_eq!(game_logic.snake.len(), 3);
        assert_eq!(game_logic.snake.body.front().unwrap(), &Segment::new(8, 6));
        assert_eq!(game_logic.snake.body.back().unwrap(), &Segment::new(6, 6));

        game_logic.main_loop(Some(Direction::Left));
        assert_eq!(game_logic.snake.len(), 3);
        assert_eq!(game_logic.snake.body.front().unwrap(), &Segment::new(9, 6));
        assert_eq!(game_logic.snake.body.back().unwrap(), &Segment::new(7, 6));

    }
}