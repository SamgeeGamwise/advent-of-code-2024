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

impl Default for Location {
    fn default() -> Self {
        Self { position: Point::default(), direction: Point::default() }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Hash)]
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

impl Eq for Point {}

impl Default for Point {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }    
}    
