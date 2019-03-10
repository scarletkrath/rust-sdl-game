/* 
    utils/vec2.rs -> sdl2's "Point" uses i32. It's better to use floating
    numbers for vectors.
    
    Copyright (C) 2019 - scarletkrath

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use std::ops::{Add, Sub, Mul, Div};
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign};
// The *Assign traits are the +=, -=, *= and /= operators.

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec2f {
    pub x: f32,
    pub y: f32
}

impl Vec2f {
    pub fn newf(x: f32, y: f32) -> Self { Vec2f { x: x, y: y } }
    pub fn newi(x: i32, y: i32) -> Self { Vec2f { x: x as f32, y: y as f32 } }
}

// We want to operate with other vectors AND other numbers, like this:
// Vec2f(5, 5) + Vec2f(5, 10) -> Vec2f(10, 15)
// Vec2f(5, 5) + 5 -> Vec2f(10, 10)

impl Add<Vec2f> for Vec2f {
    type Output = Vec2f;
 
    fn add(self, other: Vec2f) -> Vec2f {
        Vec2f { x: self.x + other.x, y: self.y + other.y }
    }
}

impl Add<f32> for Vec2f {
    type Output = Vec2f;
 
    fn add(self, other: f32) -> Vec2f {
        Vec2f { x: self.x + other, y: self.y + other }
    }
}

impl AddAssign<Vec2f> for Vec2f {
    fn add_assign(&mut self, other: Vec2f) {
        *self = Vec2f {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl AddAssign<f32> for Vec2f {
    fn add_assign(&mut self, other: f32) {
        *self = Vec2f {
            x: self.x + other,
            y: self.y + other,
        };
    }
}

impl Sub<Vec2f> for Vec2f {
    type Output = Vec2f;
 
    fn sub(self, other: Vec2f) -> Vec2f {
        Vec2f { x: self.x - other.x, y: self.y - other.y }
    }
}

impl Sub<f32> for Vec2f {
    type Output = Vec2f;
 
    fn sub(self, other: f32) -> Vec2f {
        Vec2f { x: self.x - other, y: self.y - other }
    }
}

impl SubAssign<Vec2f> for Vec2f {
    fn sub_assign(&mut self, other: Vec2f) {
        *self = Vec2f {
            x: self.x - other.x,
            y: self.y - other.y,
        };
    }
}

impl SubAssign<f32> for Vec2f {
    fn sub_assign(&mut self, other: f32) {
        *self = Vec2f {
            x: self.x - other,
            y: self.y - other,
        };
    }
}

impl Mul<Vec2f> for Vec2f {
    type Output = Vec2f;
 
    fn mul(self, other: Vec2f) -> Vec2f {
        Vec2f { x: self.x * other.x, y: self.y * other.y }
    }
}

impl Mul<f32> for Vec2f {
    type Output = Vec2f;
 
    fn mul(self, other: f32) -> Vec2f {
        Vec2f { x: self.x * other, y: self.y * other }
    }
}

impl MulAssign<Vec2f> for Vec2f {
    fn mul_assign(&mut self, other: Vec2f) {
        *self = Vec2f {
            x: self.x * other.x,
            y: self.y * other.y,
        };
    }
}

impl MulAssign<f32> for Vec2f {
    fn mul_assign(&mut self, other: f32) {
        *self = Vec2f {
            x: self.x * other,
            y: self.y * other,
        };
    }
}

impl Div<Vec2f> for Vec2f {
    type Output = Vec2f;
 
    fn div(self, other: Vec2f) -> Vec2f {
        Vec2f { x: self.x / other.x, y: self.y / other.y }
    }
}

impl Div<f32> for Vec2f {
    type Output = Vec2f;
 
    fn div(self, other: f32) -> Vec2f {
        Vec2f { x: self.x / other, y: self.y / other }
    }
}

impl DivAssign<Vec2f> for Vec2f {
    fn div_assign(&mut self, other: Vec2f) {
        *self = Vec2f {
            x: self.x / other.x,
            y: self.y / other.y,
        };
    }
}

impl DivAssign<f32> for Vec2f {
    fn div_assign(&mut self, other: f32) {
        *self = Vec2f {
            x: self.x / other,
            y: self.y / other,
        };
    }
}