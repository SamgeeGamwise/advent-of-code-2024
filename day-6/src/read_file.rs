use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub const EDGE_LENGTH: usize = 130;

pub fn file_to_2d_array(file_path: &str) -> ([[bool; EDGE_LENGTH]; EDGE_LENGTH], Point) {
    let mut mapped_area: [[bool; EDGE_LENGTH]; EDGE_LENGTH] = [[false; EDGE_LENGTH]; EDGE_LENGTH];
    let mut starting_position: Point = Point { x: 0, y: 0 };

    let file = File::open(file_path).expect("Unable to open file!");
    let reader = BufReader::new(file);

    for (row_index, line) in reader.lines().enumerate() {
        let line = line.expect("Error reading file");
        for (col_index, character) in line.chars().enumerate() {
            mapped_area[row_index][col_index] = character != '.';

            if character == '^' {
                starting_position = Point { y: row_index.try_into().unwrap(), x: col_index.try_into().unwrap() };
            }
        }
    }

    (mapped_area, starting_position)
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn set(&mut self, x: isize, y: isize) {
        self.x = x;
        self.y = y;
    }
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

impl std::ops::SubAssign for Point {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Location {
    pub position: Point,
    pub direction: Point,
}

impl Location {
    pub fn step(&mut self) {
        self.position += self.direction;
    }

    pub fn step_back(&mut self) {
        self.position -= self.direction;
    }

    
    pub fn turn(&mut self) {
        if self.direction.x == 1 {
            self.direction.set(0, 1);
        } else if self.direction.x == -1 {
            self.direction.set(0, -1);
        } else if self.direction.y == 1 {
            self.direction.set(-1, 0);
        } else if self.direction.y == -1 {
            self.direction.set(1, 0);
        }
    }
}