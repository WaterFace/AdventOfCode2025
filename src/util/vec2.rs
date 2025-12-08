pub fn vec2(x: i32, y: i32) -> Vec2 {
    Vec2 { x, y }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
}

impl std::fmt::Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Vec2 {
    pub const ZERO: Vec2 = Vec2::new(0, 0);
    pub const X: Vec2 = Vec2::new(1, 0);
    pub const NEG_X: Vec2 = Vec2::new(-1, 0);
    pub const Y: Vec2 = Vec2::new(0, 1);
    pub const NEG_Y: Vec2 = Vec2::new(0, -1);

    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub const fn sqr_length(self) -> i32 {
        self.x * self.x + self.y * self.y
    }

    pub const fn sqr_length_u64(self) -> u64 {
        self.x.unsigned_abs() as u64 * self.x.unsigned_abs() as u64
            + self.y.unsigned_abs() as u64 * self.y.unsigned_abs() as u64
    }

    pub const fn manhattan_distance(self, other: Vec2) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    pub const fn dot(self, other: Vec2) -> i32 {
        self.x * other.x + self.y * other.y
    }
}

impl From<(i32, i32)> for Vec2 {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

impl From<Vec2> for (i32, i32) {
    fn from(value: Vec2) -> Self {
        (value.x, value.y)
    }
}

impl std::ops::Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl std::ops::Mul for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl std::ops::Mul<Vec2> for i32 {
    type Output = Vec2;
    fn mul(self, rhs: Vec2) -> Self::Output {
        Vec2 {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl std::ops::Mul<i32> for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: i32) -> Self::Output {
        Vec2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
