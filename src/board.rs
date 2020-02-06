use crate::Obstacle;
use crate::Segment;

pub struct Board {
    width: usize,
    height: usize,
    board: Vec<Vec<Obstacle>>,
    number_of_obstacles: usize,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Board {
        let mut column: Vec<Obstacle> = Vec::with_capacity(height as usize);
        column.resize(height as usize, Obstacle::None);

        let mut board: Vec<Vec<Obstacle>> = Vec::with_capacity(width as usize);
        board.resize(width as usize, column.clone());

        Board {
            width,
            height,
            board,
            number_of_obstacles: 0,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get_field(&self, x: usize, y: usize) -> Obstacle {
        self.board[x][y]
    }

    pub fn set_obstacle(&mut self, x: usize, y: usize) {
        self.board[x][y] = Obstacle::Apple;
        self.number_of_obstacles += 1;
    }

    pub fn clear_obstacle(&mut self, x: usize, y: usize) {
        self.board[x][y] = Obstacle::None;
        self.number_of_obstacles -= 1;
    }

    pub fn segment_in(&self, segment: &Segment) -> bool {
        segment.x >= 0 && segment.x < self.width as i32 &&
            segment.y >= 0 && segment.y < self.height as i32
    }

    pub fn get_number_of_obstacles(&self) -> usize {
        self.number_of_obstacles
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn given_board_when_segments_are_in_then_segment_in_returns_true() {
        let board = Board::new(3, 8);
        assert_eq!(board.segment_in(&Segment::new(0, 0)), true);
        assert_eq!(board.segment_in(&Segment::new(2, 7)), true);
    }

    #[test]
    fn given_board_when_segments_are_not_in_then_segment_in_returns_false() {
        let board = Board::new(8, 3);
        assert_eq!(board.segment_in(&Segment::new(-1, 0)), false);
        assert_eq!(board.segment_in(&Segment::new(0, -1)), false);
        assert_eq!(board.segment_in(&Segment::new(-1, -1)), false);
        assert_eq!(board.segment_in(&Segment::new(8, 2)), false);
        assert_eq!(board.segment_in(&Segment::new(7, 3)), false);
        assert_eq!(board.segment_in(&Segment::new(8, 3)), false);
    }

    fn assert_empty_board(board: &Board) {
        for i in 0..board.width {
            for j in 0..board.height {
                assert_eq!(board.get_field(i, j), Obstacle::None);
            }
        }
    }

    #[test]
    fn given_empty_board_then_there_are_no_obstacles() {
        let board = Board::new(3, 8);
        assert_empty_board(&board);
    }

    #[test]
    fn given_obstacles_on_board_then_obstacles_have_correct_positions() {
        let mut board = Board::new(20, 45);
        assert_empty_board(&board);

        board.set_obstacle(10, 30);
        board.set_obstacle(0, 0);
        board.set_obstacle(19, 44);

        assert_eq!(board.get_field(10, 30), Obstacle::Apple);
        assert_eq!(board.get_field(0, 0), Obstacle::Apple);
        assert_eq!(board.get_field(19, 44), Obstacle::Apple);

        assert_eq!(board.get_field(18, 44), Obstacle::None);
    }

    #[test]
    #[should_panic]
    fn given_board_when_access_position_not_in_board_then_panic() {
        let board = Board::new(40, 5);
        board.get_field(40, 0);
    }
}