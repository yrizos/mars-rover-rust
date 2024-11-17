use crate::direction::Direction;
use crate::instruction::Instruction;
use crate::plateau::Plateau;

#[derive(Debug, PartialEq)]
pub struct Rover<'a> {
    x: i32,
    y: i32,
    direction: Direction,
    plateau: &'a Plateau,
}

impl<'a> Rover<'a> {
    pub fn new(x: i32, y: i32, direction: Direction, plateau: &'a Plateau) -> Self {
        Rover {
            x,
            y,
            direction,
            plateau,
        }
    }

    pub fn turn_left(&mut self) {
        self.direction = match self.direction {
            Direction::N => Direction::W,
            Direction::W => Direction::S,
            Direction::S => Direction::E,
            Direction::E => Direction::N,
        };
    }

    pub fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::N => Direction::E,
            Direction::E => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::N,
        };
    }

    pub fn move_forward(&mut self) {
        let (new_x, new_y) = match self.direction {
            Direction::N => (self.x, self.y + 1),
            Direction::E => (self.x + 1, self.y),
            Direction::S => (self.x, self.y - 1),
            Direction::W => (self.x - 1, self.y),
        };

        if self.plateau.is_within_bounds(new_x, new_y) {
            self.x = new_x;
            self.y = new_y;
        }
    }

    pub fn execute_instructions(&mut self, instructions: &[Instruction]) {
        for &instruction in instructions {
            match instruction {
                Instruction::LEFT => self.turn_left(),
                Instruction::RIGHT => self.turn_right(),
                Instruction::MOVE => self.move_forward(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::direction::Direction;

    #[test]
    fn test_turn_left() {
        let plateau = Plateau::new(5, 5);
        let mut rover = Rover::new(0, 0, Direction::N, &plateau);

        rover.turn_left();
        assert_eq!(rover.direction, Direction::W);
        rover.turn_left();
        assert_eq!(rover.direction, Direction::S);
        rover.turn_left();
        assert_eq!(rover.direction, Direction::E);
        rover.turn_left();
        assert_eq!(rover.direction, Direction::N);
    }

    #[test]
    fn test_turn_right() {
        let plateau = Plateau::new(5, 5);
        let mut rover = Rover::new(0, 0, Direction::N, &plateau);
        rover.turn_right();
        assert_eq!(rover.direction, Direction::E);
        rover.turn_right();
        assert_eq!(rover.direction, Direction::S);
        rover.turn_right();
        assert_eq!(rover.direction, Direction::W);
        rover.turn_right();
        assert_eq!(rover.direction, Direction::N);
    }

    #[test]
    fn test_move_forward() {
        let plateau = Plateau::new(5, 5);
        let mut rover = Rover::new(0, 0, Direction::N, &plateau);

        rover.move_forward();
        assert_eq!(rover.x, 0);
        assert_eq!(rover.y, 1);

        rover.turn_right();
        rover.move_forward();
        assert_eq!(rover.x, 1);
        assert_eq!(rover.y, 1);

        rover.turn_right();
        rover.move_forward();
        assert_eq!(rover.x, 1);
        assert_eq!(rover.y, 0);

        rover.turn_right();
        rover.move_forward();
        assert_eq!(rover.x, 0);
        assert_eq!(rover.y, 0);
    }

    #[test]
    fn test_move_forward_within_bounds() {
        let plateau = Plateau::new(5, 5);
        let mut rover = Rover::new(5, 5, Direction::N, &plateau);

        rover.move_forward();
        assert_eq!(rover.x, 5);
        assert_eq!(rover.y, 5);

        rover.turn_right();
        rover.move_forward();
        assert_eq!(rover.x, 5);
        assert_eq!(rover.y, 5);
    }

    #[test]
    fn test_move_forward_out_of_bounds() {
        let plateau = Plateau::new(5, 5);
        let mut rover = Rover::new(5, 5, Direction::N, &plateau);

        rover.move_forward();
        assert_eq!(rover.x, 5);
        assert_eq!(rover.y, 5);

        rover.turn_right();
        rover.move_forward();
        assert_eq!(rover.x, 5);
        assert_eq!(rover.y, 5);

        rover.turn_right();
        rover.move_forward();
        assert_eq!(rover.x, 5);
        assert_eq!(rover.y, 4);

        rover.turn_right();
        rover.move_forward();
        assert_eq!(rover.x, 4);
        assert_eq!(rover.y, 4);

        rover.move_forward();
        rover.move_forward();
        rover.move_forward();
        rover.move_forward();
        rover.move_forward();
        assert_eq!(rover.x, 0);
        assert_eq!(rover.y, 4);

        rover.move_forward();
        assert_eq!(rover.x, 0);
        assert_eq!(rover.y, 4);
    }

    #[test]
    fn test_execute_instructions() {
        let plateau = Plateau::new(5, 5);
        let mut rover = Rover::new(1, 2, Direction::N, &plateau);
        let instructions = [
            Instruction::LEFT,
            Instruction::MOVE,
            Instruction::LEFT,
            Instruction::MOVE,
            Instruction::LEFT,
            Instruction::MOVE,
            Instruction::LEFT,
            Instruction::MOVE,
            Instruction::MOVE,
        ];
        rover.execute_instructions(&instructions);
        assert_eq!(rover.x, 1);
        assert_eq!(rover.y, 3);
        assert_eq!(rover.direction, Direction::N);

        let mut rover = Rover::new(3, 3, Direction::E, &plateau);
        let instructions = [
            Instruction::MOVE,
            Instruction::MOVE,
            Instruction::RIGHT,
            Instruction::MOVE,
            Instruction::MOVE,
            Instruction::RIGHT,
            Instruction::MOVE,
            Instruction::RIGHT,
            Instruction::RIGHT,
            Instruction::MOVE,
        ];
        rover.execute_instructions(&instructions);
        assert_eq!(rover.x, 5);
        assert_eq!(rover.y, 1);
        assert_eq!(rover.direction, Direction::E);
    }
}
