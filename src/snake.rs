use std::collections::{LinkedList, HashSet};
use crate::Segment;
use crate::Direction;

pub type Body = LinkedList<Segment>;

#[derive(Debug, PartialEq)]
pub struct Snake {
    pub body: Body,
	
	segments_to_grow_by : u32,
}

impl Snake {
    pub fn new(segments: &Vec<Segment>) -> Snake {
        let mut snake_segments = LinkedList::new();
        snake_segments.extend(segments);
        Snake {
            body: snake_segments,
            segments_to_grow_by : 0,
        }
    }

    pub fn move_body(&mut self, direction: &Direction) {
        assert!(!self.body.is_empty());
        self.move_body_internal(direction);
		
	    if self.segments_to_grow_by > 0 {
            self.segments_to_grow_by -= 1;
	    } else {
            self.body.pop_back().unwrap();
	    }
    }
	
	pub fn grow(&mut self, grow_by : u32) {
	    self.segments_to_grow_by += grow_by;
	}

    pub fn push_back_segment(&mut self, new_segment: Segment) {
        if self.is_segment_in_body(&new_segment) {
            panic!("New segment ({}, {}) already in snake body!", new_segment.x, new_segment.y);
        }
        self.body.push_back(new_segment);
        if self.body.len() > 1 {
            let mut iter = self.body.iter();
            let mut previous_segment = iter.next().unwrap();
            loop {
                let next_iter = iter.next();
                if next_iter.is_none() {
                    return;
                }
                let segment = next_iter.unwrap();
                if (segment.x - previous_segment.x).abs() > 1 || (segment.y - previous_segment.y).abs() > 1 {
                    panic!("Two consecutive snake segments ({}, {}) and ({}, {}) are not connected to each other!",
                                    segment.x, segment.y, previous_segment.x, previous_segment.y);
                }
                previous_segment = segment;
            }
        }
    }

    pub fn is_segment_in_body(&self, segment: &Segment) -> bool {
        self.body.iter().any(|current_segment| segment == current_segment)
    }

    pub fn has_unique_segments(&self) -> bool {
        let mut uniq = HashSet::new();
        self.body.iter().all(|x| uniq.insert(x))
    }

    fn move_body_internal(&mut self, direction: &Direction) {
        let &snake_head = self.body.front().unwrap();
        let new_segment = direction.nearest_segment(&snake_head);
        if self.body.len() > 1  {
            assert!(self.body.iter().nth(1).unwrap() != &new_segment);
        }
        self.body.push_front(new_segment);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // @
    // ###
    fn get_4_segment_snake() -> Vec<Segment> {
        vec![
            Segment::new(2, 2),
            Segment::new(2, 3),
            Segment::new(3, 3),
            Segment::new(4, 3),
        ]
    }

    // @
    fn get_1_segment_snake() -> Vec<Segment> {
        vec![
            Segment::new(5, 12),
        ]
    }

    #[test]
    fn given_4_segment_snake_when_snake_moves_up_then_segments_have_correct_positions() {
        test_move_generic(Segment::new(2, 1),
                          Direction::Up,
                          &get_4_segment_snake());
    }

    #[test]
    #[should_panic]
    fn given_4_segment_snake_when_snake_moves_against_itself_then_panic() {
        test_move_generic(Segment::new(2, 3),
                          Direction::Down,
                          &get_4_segment_snake());
    }

     #[test]
    fn given_4_segment_snake_when_snake_moves_left_then_segments_have_correct_positions() {
        test_move_generic(Segment::new(1, 2),
                          Direction::Left,
                          &get_4_segment_snake());
    }

    #[test]
    fn given_4_segment_snake_when_snake_moves_right_then_segments_have_correct_positions() {
        test_move_generic(Segment::new(3, 2),
                          Direction::Right,
                          &get_4_segment_snake());
    }

    #[test]
    fn given_1_segment_snake_when_snake_moves_up_then_segments_have_correct_positions() {
        test_move_generic(Segment::new(5, 11),
                          Direction::Up,
                          &get_1_segment_snake());
    }

    #[test]
    fn given_1_segment_snake_when_snake_moves_down_then_segments_have_correct_positions() {
        test_move_generic(Segment::new(5, 13),
                          Direction::Down,
                          &get_1_segment_snake());
    }

    #[test]
    fn given_1_segment_snake_when_snake_moves_left_then_segments_have_correct_positions() {
        test_move_generic(Segment::new(4, 12),
                          Direction::Left,
                          &get_1_segment_snake());
    }

    #[test]
    fn given_1_segment_snake_when_snake_moves_right_then_segments_have_correct_positions() {
        test_move_generic(Segment::new(6, 12),
                          Direction::Right,
                          &get_1_segment_snake());
    }

    // @
    // ###
    #[test]
    fn given_4_segments_snake_when_push_back_consecutive_segment_up_then_code_does_not_panic() {
        let mut snake = Snake::new(&get_4_segment_snake());
        snake.push_back_segment(Segment::new(4, 2));
    }

    // @
    // ###
    #[test]
    fn given_4_segments_snake_when_push_back_consecutive_segment_down_then_code_does_not_panic() {
        let mut snake = Snake::new(&get_4_segment_snake());
        snake.push_back_segment(Segment::new(4, 4));
    }

    // @
    // ###
    #[test]
    #[should_panic]
    fn given_4_segments_snake_when_push_back_already_existing_segment_then_panic() {
        let mut snake = Snake::new(&get_4_segment_snake());
        snake.push_back_segment(Segment::new(4, 3));
    }

    // @
    // ###
    #[test]
    fn given_4_segments_snake_when_push_back_consecutive_segment_right_then_code_does_not_panic() {
        let mut snake = Snake::new(&get_4_segment_snake());
        snake.push_back_segment(Segment::new(5, 3));
    }

    #[test]
    #[should_panic]
    fn given_1_segment_snake_when_push_back_the_same_segment_then_panic() {
        let mut snake = Snake::new(&get_1_segment_snake());
        snake.push_back_segment(Segment::new(5, 12));
    }

    #[test]
    fn given_1_segment_snake_when_push_back_consecutive_segment_left_then_code_does_not_panic() {
        let mut snake = Snake::new(&get_1_segment_snake());
        snake.push_back_segment(Segment::new(4, 12));
    }

    #[test]
    fn given_1_segment_snake_when_push_back_consecutive_segment_right_then_code_does_not_panic() {
        let mut snake = Snake::new(&get_1_segment_snake());
        snake.push_back_segment(Segment::new(6, 12));
    }

    #[test]
    fn given_1_segment_snake_when_push_back_consecutive_segment_up_then_code_does_not_panic() {
        let mut snake = Snake::new(&get_1_segment_snake());
        snake.push_back_segment(Segment::new(5, 11));
    }

    #[test]
    fn given_1_segment_snake_when_push_back_consecutive_segment_down_then_code_does_not_panic() {
        let mut snake = Snake::new(&get_1_segment_snake());
        snake.push_back_segment(Segment::new(5, 13));
    }

    #[test]
    #[should_panic]
    fn given_1_segment_snake_when_push_back_not_consecutive_segment_down_then_panic() {
        let mut snake = Snake::new(&get_1_segment_snake());
        snake.push_back_segment(Segment::new(5, 14));
    }

    #[test]
    #[should_panic]
    fn given_4_segment_snake_when_push_back_not_consecutive_segment_down_then_panic() {
        let mut snake = Snake::new(&get_4_segment_snake());
        snake.push_back_segment(Segment::new(5, 14));
    }

    #[test]
    fn given_snake_with_no_duplicates_then_has_unique_segments_returns_false() {
        let snake = Snake::new(&get_4_segment_snake());
        assert_eq!(snake.has_unique_segments(), true);
    }

    #[test]
    fn given_snake_with_duplicates_then_has_unique_segments_returns_true() {
        let snake = Snake::new(&vec![
            Segment::new(2, 2),
            Segment::new(2, 3),
            Segment::new(3, 3),
            Segment::new(4, 3),
            Segment::new(2, 2),
        ]);
        assert_eq!(snake.has_unique_segments(), false);
    }

    fn test_move_generic(segment: Segment, direction: Direction, body: &Vec<Segment>) {
        let mut snake = Snake::new(body);
        snake.move_body(&direction);

        let mut vec: Vec<Segment> = vec![segment];
        let mut moved_body = body.clone();
        moved_body.pop();
        vec.append(&mut moved_body);

        assert_eq!(snake, Snake::new(&vec));
    }
}
