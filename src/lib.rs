use euclid::Point2D;

pub struct ScreenSpace;
pub type Segment = Point2D<i32, ScreenSpace>;

mod snake;
mod board;
pub mod game_logic;
pub mod game_view;
pub mod game_controller;

#[derive(Copy, Clone, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn nearest_segment(&self, segment: &Segment) -> Segment {
        match self {
            Direction::Down => Segment::new(segment.x, segment.y + 1),
            Direction::Up => Segment::new(segment.x, segment.y - 1),
            Direction::Left => Segment::new(segment.x -  1, segment.y),
            Direction::Right => Segment::new(segment.x + 1, segment.y),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Obstacle {
    None,
    Apple(u8),
}