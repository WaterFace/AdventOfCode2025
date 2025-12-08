pub fn vec3(x: i32, y: i32, z: i32) -> Vec3 {
    Vec3 { x, y, z }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Vec3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Vec3 {
    pub const ZERO: Vec3 = Vec3::new(0, 0, 0);
    pub const X: Vec3 = Vec3::new(1, 0, 0);
    pub const NEG_X: Vec3 = Vec3::new(-1, 0, 0);
    pub const Y: Vec3 = Vec3::new(0, 1, 0);
    pub const NEG_Y: Vec3 = Vec3::new(0, -1, 0);
    pub const Z: Vec3 = Vec3::new(0, 0, 1);
    pub const NEG_Z: Vec3 = Vec3::new(0, 0, -1);

    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub const fn sqr_length(self) -> i32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub const fn sqr_length_u64(self) -> u64 {
        self.x.unsigned_abs() as u64 * self.x.unsigned_abs() as u64
            + self.y.unsigned_abs() as u64 * self.y.unsigned_abs() as u64
            + self.z.unsigned_abs() as u64 * self.z.unsigned_abs() as u64
    }

    pub const fn manhattan_distance(self, other: Vec3) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y) + self.z.abs_diff(other.z)
    }

    pub const fn dot(self, other: Vec3) -> i32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl From<(i32, i32, i32)> for Vec3 {
    fn from((x, y, z): (i32, i32, i32)) -> Self {
        Self { x, y, z }
    }
}

impl From<Vec3> for (i32, i32) {
    fn from(value: Vec3) -> Self {
        (value.x, value.y)
    }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl std::ops::Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl std::ops::Mul<Vec3> for i32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl std::ops::Mul<i32> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: i32) -> Self::Output {
        Vec3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
