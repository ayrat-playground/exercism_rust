// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    position: (isize, isize),
    direction: Direction
}

impl Robot {
    pub fn new(x: isize, y: isize, d: Direction) -> Self {
        Robot { position: (x, y), direction: d }
    }

    pub fn turn_right(self) -> Self {
        match self.direction {
            Direction::North => Robot { position: self.position, direction: Direction::East },
            Direction::East  => Robot { position: self.position, direction: Direction::South },
            Direction::South => Robot { position: self.position, direction: Direction::West },
            Direction::West  => Robot { position: self.position, direction: Direction::North }
        }
    }

    pub fn turn_left(self) -> Self {
        match self.direction {
            Direction::North => Robot { position: self.position, direction: Direction::West },
            Direction::East  => Robot { position: self.position, direction: Direction::North },
            Direction::South => Robot { position: self.position, direction: Direction::East },
            Direction::West  => Robot { position: self.position, direction: Direction::South }
        }
    }

    pub fn advance(self) -> Self {
        match self.direction {
            Direction::North  => Robot { position: (self.position.0, self.position.1 + 1), direction: self.direction },
            Direction::East   => Robot { position: (self.position.0 + 1, self.position.1), direction: self.direction },
            Direction::South  => Robot { position: (self.position.0, self.position.1 - 1), direction: self.direction },
            Direction::West   => Robot { position: (self.position.0 - 1, self.position.1), direction: self.direction }
        }
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars().fold(self, |acc, ch| {
            match ch {
                'L' => acc.turn_left(),
                'R' => acc.turn_right(),
                'A' => acc.advance(),
                 _  => acc

            }
        })
    }

    pub fn position(&self) -> (isize, isize) {
        self.position
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
