//! Math utilities for Minecraft Bedrock Edition geometry and physics.
//!
//! This module provides:
//! - [`Vector2`]: A generic 2D vector with standard arithmetic operators.
//! - [`Vector3`]: A generic 3D vector with standard arithmetic operators.
//! - [`Vector3f`] / [`Vector3i`]: Common type aliases for `f32` and `i32` vectors.
//! - [`BlockPos`]: A block position in world space (i32 x, y, z).
//! - [`BoundingBox`]: An axis-aligned bounding box defined by min/max corners.
//! - [`Direction`]: Cardinal and vertical facing directions used in block/entity orientation.
//! - Facing direction constants.

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

// ---------------------------------------------------------------------------
// Vector2
// ---------------------------------------------------------------------------

/// A generic 2D vector with `x` and `y` components.
///
/// Supports standard arithmetic operations when the element type does.
///
/// # Examples
///
/// ```
/// use perust_utils::math::Vector2;
///
/// let a = Vector2::new(3.0f32, 4.0);
/// let b = Vector2::new(1.0, 2.0);
/// let sum = a + b;
/// assert_eq!(sum.x, 4.0);
/// assert_eq!(sum.y, 6.0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vector2<T> {
    /// The X component.
    pub x: T,
    /// The Y component.
    pub y: T,
}

impl<T> Vector2<T> {
    /// Creates a new 2D vector from the given components.
    #[inline]
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Copy + num_traits::Zero> Vector2<T> {
    /// Returns the zero vector.
    #[inline]
    pub fn zero() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
        }
    }
}

// Vector2 arithmetic operators

impl<T: Copy + Add<Output = T>> Add for Vector2<T> {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Copy + AddAssign> AddAssign for Vector2<T> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Copy + Sub<Output = T>> Sub for Vector2<T> {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Copy + SubAssign> SubAssign for Vector2<T> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Copy + Mul<Output = T>> Mul<T> for Vector2<T> {
    type Output = Self;
    #[inline]
    fn mul(self, scalar: T) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl<T: Copy + MulAssign> MulAssign<T> for Vector2<T> {
    #[inline]
    fn mul_assign(&mut self, scalar: T) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl<T: Copy + Div<Output = T>> Div<T> for Vector2<T> {
    type Output = Self;
    #[inline]
    fn div(self, scalar: T) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

impl<T: Copy + DivAssign> DivAssign<T> for Vector2<T> {
    #[inline]
    fn div_assign(&mut self, scalar: T) {
        self.x /= scalar;
        self.y /= scalar;
    }
}

impl<T: Copy + Neg<Output = T>> Neg for Vector2<T> {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Vector2<f32> {
    /// Computes the length (magnitude) of the vector.
    #[inline]
    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Computes the squared length of the vector (avoids a sqrt).
    #[inline]
    pub fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    /// Returns the normalized (unit) vector. Returns zero if length is zero.
    #[inline]
    pub fn normalize(self) -> Self {
        let len = self.length();
        if len == 0.0 {
            Self::zero()
        } else {
            self / len
        }
    }

    /// Computes the dot product of two vectors.
    #[inline]
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    /// Computes the distance between two vectors.
    #[inline]
    pub fn distance(self, other: Self) -> f32 {
        (self - other).length()
    }
}

impl Vector2<f64> {
    /// Computes the length (magnitude) of the vector.
    #[inline]
    pub fn length(self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Returns the normalized (unit) vector. Returns zero if length is zero.
    #[inline]
    pub fn normalize(self) -> Self {
        let len = self.length();
        if len == 0.0 {
            Self::zero()
        } else {
            self / len
        }
    }
}

// ---------------------------------------------------------------------------
// Vector3
// ---------------------------------------------------------------------------

/// A generic 3D vector with `x`, `y`, and `z` components.
///
/// Supports standard arithmetic operations when the element type does.
///
/// # Examples
///
/// ```
/// use perust_utils::math::Vector3;
///
/// let pos = Vector3::new(1.0f32, 64.0, 3.0);
/// let offset = Vector3::new(0.0, 1.0, 0.0);
/// let new_pos = pos + offset;
/// assert_eq!(new_pos.y, 65.0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vector3<T> {
    /// The X component.
    pub x: T,
    /// The Y component.
    pub y: T,
    /// The Z component.
    pub z: T,
}

impl<T> Vector3<T> {
    /// Creates a new 3D vector from the given components.
    #[inline]
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T: Copy + num_traits::Zero> Vector3<T> {
    /// Returns the zero vector.
    #[inline]
    pub fn zero() -> Self {
        Self {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }
}

// Vector3 arithmetic operators

impl<T: Copy + Add<Output = T>> Add for Vector3<T> {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: Copy + AddAssign> AddAssign for Vector3<T> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T: Copy + Sub<Output = T>> Sub for Vector3<T> {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: Copy + SubAssign> SubAssign for Vector3<T> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T: Copy + Mul<Output = T>> Mul<T> for Vector3<T> {
    type Output = Self;
    #[inline]
    fn mul(self, scalar: T) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl<T: Copy + Mul<Output = T>> Mul<Vector3<T>> for Vector3<T> {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl<T: Copy + MulAssign> MulAssign<T> for Vector3<T> {
    #[inline]
    fn mul_assign(&mut self, scalar: T) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

impl<T: Copy + Div<Output = T>> Div<T> for Vector3<T> {
    type Output = Self;
    #[inline]
    fn div(self, scalar: T) -> Self {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl<T: Copy + DivAssign> DivAssign<T> for Vector3<T> {
    #[inline]
    fn div_assign(&mut self, scalar: T) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
    }
}

impl<T: Copy + Neg<Output = T>> Neg for Vector3<T> {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Vector3<f32> {
    /// Computes the length (magnitude) of the vector.
    #[inline]
    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Computes the squared length of the vector (avoids a sqrt).
    #[inline]
    pub fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Returns the normalized (unit) vector. Returns zero if length is zero.
    #[inline]
    pub fn normalize(self) -> Self {
        let len = self.length();
        if len == 0.0 {
            Self::zero()
        } else {
            self / len
        }
    }

    /// Computes the dot product of two vectors.
    #[inline]
    pub fn dot(self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Computes the cross product of two vectors.
    #[inline]
    pub fn cross(self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// Computes the Euclidean distance between two vectors.
    #[inline]
    pub fn distance(self, other: Self) -> f32 {
        (self - other).length()
    }

    /// Computes the squared distance between two vectors (avoids a sqrt).
    #[inline]
    pub fn distance_squared(self, other: Self) -> f32 {
        (self - other).length_squared()
    }

    /// Linearly interpolates between `self` and `other` by `t` (0.0..=1.0).
    #[inline]
    pub fn lerp(self, other: Self, t: f32) -> Self {
        self * (1.0 - t) + other * t
    }

    /// Returns the component-wise floor.
    #[inline]
    pub fn floor(self) -> Self {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
            z: self.z.floor(),
        }
    }

    /// Returns the component-wise ceil.
    #[inline]
    pub fn ceil(self) -> Self {
        Self {
            x: self.x.ceil(),
            y: self.y.ceil(),
            z: self.z.ceil(),
        }
    }

    /// Returns the component-wise absolute value.
    #[inline]
    pub fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
            z: self.z.abs(),
        }
    }

    /// Returns the horizontal (XZ-plane) length.
    #[inline]
    pub fn horizontal_length(self) -> f32 {
        (self.x * self.x + self.z * self.z).sqrt()
    }
}

impl Vector3<f64> {
    /// Computes the length (magnitude) of the vector.
    #[inline]
    pub fn length(self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Returns the normalized (unit) vector. Returns zero if length is zero.
    #[inline]
    pub fn normalize(self) -> Self {
        let len = self.length();
        if len == 0.0 {
            Self::zero()
        } else {
            self / len
        }
    }

    /// Computes the dot product of two vectors.
    #[inline]
    pub fn dot(self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Computes the cross product of two vectors.
    #[inline]
    pub fn cross(self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl Vector3<i32> {
    /// Converts this integer vector to a [`Vector3f`].
    #[inline]
    pub fn to_f32(self) -> Vector3f {
        Vector3f::new(self.x as f32, self.y as f32, self.z as f32)
    }
}

/// A 3D vector of `f32` components. Commonly used for positions, velocities, and directions.
pub type Vector3f = Vector3<f32>;

/// A 3D vector of `i32` components. Commonly used for chunk/block coordinates.
pub type Vector3i = Vector3<i32>;

// ---------------------------------------------------------------------------
// BlockPos
// ---------------------------------------------------------------------------

/// A block position in world space, stored as three `i32` values.
///
/// Unlike [`Vector3i`], `BlockPos` is a distinct type that signals "this is a
/// block coordinate" in the type system, avoiding confusion with other integer
/// vectors (e.g., chunk positions).
///
/// # Examples
///
/// ```
/// use perust_utils::math::BlockPos;
///
/// let pos = BlockPos::new(10, 64, -5);
/// assert_eq!(pos.x, 10);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BlockPos {
    /// The X coordinate.
    pub x: i32,
    /// The Y coordinate.
    pub y: i32,
    /// The Z coordinate.
    pub z: i32,
}

impl BlockPos {
    /// Creates a new `BlockPos` from the given coordinates.
    #[inline]
    pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    /// The zero block position (0, 0, 0).
    #[inline]
    pub const fn zero() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }

    /// Converts this block position to a [`Vector3f`] for use in floating-point math.
    #[inline]
    pub fn to_vector3f(self) -> Vector3f {
        Vector3f::new(self.x as f32, self.y as f32, self.z as f32)
    }

    /// Converts this block position to a [`Vector3i`].
    #[inline]
    pub fn to_vector3i(self) -> Vector3i {
        Vector3i::new(self.x, self.y, self.z)
    }

    /// Returns the block position offset by the given direction.
    #[inline]
    pub fn offset(self, direction: Direction) -> Self {
        let d = direction.vector();
        Self {
            x: self.x + d.x,
            y: self.y + d.y,
            z: self.z + d.z,
        }
    }
}

impl Add for BlockPos {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for BlockPos {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl From<Vector3i> for BlockPos {
    fn from(v: Vector3i) -> Self {
        Self { x: v.x, y: v.y, z: v.z }
    }
}

impl From<BlockPos> for Vector3i {
    fn from(pos: BlockPos) -> Self {
        Self { x: pos.x, y: pos.y, z: pos.z }
    }
}

// ---------------------------------------------------------------------------
// BoundingBox
// ---------------------------------------------------------------------------

/// An axis-aligned bounding box (AABB) defined by minimum and maximum corners.
///
/// The `min` corner should have all components ≤ the corresponding components
/// of `max`. Constructors and methods do not enforce this invariant for
/// performance reasons; it is the caller's responsibility to provide valid
/// inputs.
///
/// # Examples
///
/// ```
/// use perust_utils::math::{BoundingBox, Vector3f};
///
/// let aabb = BoundingBox::new(
///     Vector3f::new(-0.3, 0.0, -0.3),
///     Vector3f::new(0.3, 1.8, 0.3),
/// );
/// assert_eq!(aabb.width(), 0.6);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoundingBox {
    /// The minimum corner (inclusive).
    pub min: Vector3f,
    /// The maximum corner (exclusive).
    pub max: Vector3f,
}

impl BoundingBox {
    /// Creates a new bounding box from min and max corners.
    #[inline]
    pub const fn new(min: Vector3f, max: Vector3f) -> Self {
        Self { min, max }
    }

    /// Creates a bounding box centered at `center` with the given half-size.
    ///
    /// `half_size` is treated as the half-extent in each axis.
    #[inline]
    pub fn from_center_half_size(center: Vector3f, half_size: Vector3f) -> Self {
        Self {
            min: center - half_size,
            max: center + half_size,
        }
    }

    /// Returns the width of the bounding box along the X axis.
    #[inline]
    pub fn width(self) -> f32 {
        self.max.x - self.min.x
    }

    /// Returns the height of the bounding box along the Y axis.
    #[inline]
    pub fn height(self) -> f32 {
        self.max.y - self.min.y
    }

    /// Returns the depth of the bounding box along the Z axis.
    #[inline]
    pub fn depth(self) -> f32 {
        self.max.z - self.min.z
    }

    /// Returns the center point of the bounding box.
    #[inline]
    pub fn center(self) -> Vector3f {
        (self.min + self.max) * 0.5
    }

    /// Returns `true` if the given point lies inside this bounding box.
    #[inline]
    pub fn contains_point(self, point: Vector3f) -> bool {
        point.x >= self.min.x
            && point.x <= self.max.x
            && point.y >= self.min.y
            && point.y <= self.max.y
            && point.z >= self.min.z
            && point.z <= self.max.z
    }

    /// Returns `true` if this bounding box intersects with another.
    #[inline]
    pub fn intersects(self, other: Self) -> bool {
        self.min.x <= other.max.x
            && self.max.x >= other.min.x
            && self.min.y <= other.max.y
            && self.max.y >= other.min.y
            && self.min.z <= other.max.z
            && self.max.z >= other.min.z
    }

    /// Expands the bounding box by `amount` in all directions and returns a new one.
    #[inline]
    pub fn expand(self, amount: f32) -> Self {
        let expansion = Vector3f::new(amount, amount, amount);
        Self {
            min: self.min - expansion,
            max: self.max + expansion,
        }
    }

    /// Offsets the bounding box by the given vector and returns a new one.
    #[inline]
    pub fn offset(self, offset: Vector3f) -> Self {
        Self {
            min: self.min + offset,
            max: self.max + offset,
        }
    }
}

// ---------------------------------------------------------------------------
// Direction
// ---------------------------------------------------------------------------

/// Facing directions used for block faces, entity rotation, and directional logic.
///
/// Each variant corresponds to a cardinal or vertical direction in the Minecraft
/// world coordinate system:
/// - **X axis**: East (+) / West (−)
/// - **Y axis**: Up (+) / Down (−)
/// - **Z axis**: South (+) / North (−)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    /// North (−Z).
    North,
    /// South (+Z).
    South,
    /// East (+X).
    East,
    /// West (−X).
    West,
    /// Up (+Y).
    Up,
    /// Down (−Y).
    Down,
}

impl Direction {
    /// Returns the unit integer vector corresponding to this direction.
    ///
    /// | Direction | Vector |
    /// |-----------|--------|
    /// | North     | (0, 0, −1) |
    /// | South     | (0, 0, +1) |
    /// | East      | (+1, 0, 0) |
    /// | West      | (−1, 0, 0) |
    /// | Up        | (0, +1, 0) |
    /// | Down      | (0, −1, 0) |
    pub fn vector(self) -> Vector3i {
        match self {
            Direction::North => Vector3i::new(0, 0, -1),
            Direction::South => Vector3i::new(0, 0, 1),
            Direction::East => Vector3i::new(1, 0, 0),
            Direction::West => Vector3i::new(-1, 0, 0),
            Direction::Up => Vector3i::new(0, 1, 0),
            Direction::Down => Vector3i::new(0, -1, 0),
        }
    }

    /// Returns the opposite direction.
    pub fn opposite(self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }

    /// Returns the axis this direction lies on.
    pub fn axis(self) -> Axis {
        match self {
            Direction::North | Direction::South => Axis::Z,
            Direction::East | Direction::West => Axis::X,
            Direction::Up | Direction::Down => Axis::Y,
        }
    }

    /// Returns `true` if this is a horizontal direction.
    #[inline]
    pub fn is_horizontal(self) -> bool {
        matches!(self, Direction::North | Direction::South | Direction::East | Direction::West)
    }

    /// Returns `true` if this is a vertical direction.
    #[inline]
    pub fn is_vertical(self) -> bool {
        matches!(self, Direction::Up | Direction::Down)
    }

    /// Converts a Minecraft Bedrock "block face" integer to a [`Direction`].
    ///
    /// The block face values follow the Bedrock protocol:
    /// - `0` → Down
    /// - `1` → Up
    /// - `2` → North
    /// - `3` → South
    /// - `4` → West
    /// - `5` → East
    ///
    /// Returns `None` for invalid face values.
    pub fn from_block_face(face: u32) -> Option<Self> {
        match face {
            0 => Some(Direction::Down),
            1 => Some(Direction::Up),
            2 => Some(Direction::North),
            3 => Some(Direction::South),
            4 => Some(Direction::West),
            5 => Some(Direction::East),
            _ => None,
        }
    }

    /// Converts this direction to the corresponding Minecraft block face integer.
    ///
    /// See [`from_block_face`](Self::from_block_face) for the mapping.
    pub fn to_block_face(self) -> u32 {
        match self {
            Direction::Down => 0,
            Direction::Up => 1,
            Direction::North => 2,
            Direction::South => 3,
            Direction::West => 4,
            Direction::East => 5,
        }
    }

    /// Returns an iterator over all six directions.
    pub fn all() -> impl Iterator<Item = Direction> {
        [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
            Direction::Up,
            Direction::Down,
        ]
        .into_iter()
    }

    /// Returns an iterator over the four horizontal directions.
    pub fn horizontal() -> impl Iterator<Item = Direction> {
        [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ]
        .into_iter()
    }
}

// ---------------------------------------------------------------------------
// Axis
// ---------------------------------------------------------------------------

/// The three coordinate axes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Axis {
    /// The X axis (East/West).
    X,
    /// The Y axis (Up/Down).
    Y,
    /// The Z axis (North/South).
    Z,
}

// ---------------------------------------------------------------------------
// Facing constants
// ---------------------------------------------------------------------------

/// Unit vector pointing North (−Z).
pub const FACING_NORTH: Vector3i = Vector3i::new(0, 0, -1);
/// Unit vector pointing South (+Z).
pub const FACING_SOUTH: Vector3i = Vector3i::new(0, 0, 1);
/// Unit vector pointing East (+X).
pub const FACING_EAST: Vector3i = Vector3i::new(1, 0, 0);
/// Unit vector pointing West (−X).
pub const FACING_WEST: Vector3i = Vector3i::new(-1, 0, 0);
/// Unit vector pointing Up (+Y).
pub const FACING_UP: Vector3i = Vector3i::new(0, 1, 0);
/// Unit vector pointing Down (−Y).
pub const FACING_DOWN: Vector3i = Vector3i::new(0, -1, 0);

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector2_arithmetic() {
        let a = Vector2::new(3.0f32, 4.0);
        let b = Vector2::new(1.0, 2.0);
        let sum = a + b;
        assert_eq!(sum, Vector2::new(4.0, 6.0));

        let diff = a - b;
        assert_eq!(diff, Vector2::new(2.0, 2.0));

        let scaled = a * 2.0;
        assert_eq!(scaled, Vector2::new(6.0, 8.0));
    }

    #[test]
    fn test_vector3_arithmetic() {
        let a = Vector3f::new(1.0, 2.0, 3.0);
        let b = Vector3f::new(4.0, 5.0, 6.0);

        let sum = a + b;
        assert_eq!(sum, Vector3f::new(5.0, 7.0, 9.0));

        let diff = b - a;
        assert_eq!(diff, Vector3f::new(3.0, 3.0, 3.0));

        let scaled = a * 2.0;
        assert_eq!(scaled, Vector3f::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_vector3f_length() {
        let v = Vector3f::new(3.0, 4.0, 0.0);
        assert!((v.length() - 5.0).abs() < 0.001);
    }

    #[test]
    fn test_vector3f_normalize() {
        let v = Vector3f::new(3.0, 0.0, 4.0);
        let n = v.normalize();
        assert!((n.length() - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_vector3f_cross() {
        let a = Vector3f::new(1.0, 0.0, 0.0);
        let b = Vector3f::new(0.0, 1.0, 0.0);
        let c = a.cross(b);
        assert_eq!(c, Vector3f::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_vector3f_dot() {
        let a = Vector3f::new(1.0, 2.0, 3.0);
        let b = Vector3f::new(4.0, 5.0, 6.0);
        assert_eq!(a.dot(b), 32.0);
    }

    #[test]
    fn test_block_pos() {
        let pos = BlockPos::new(10, 64, -5);
        assert_eq!(pos.x, 10);
        assert_eq!(pos.y, 64);
        assert_eq!(pos.z, -5);

        let north = pos.offset(Direction::North);
        assert_eq!(north.z, -6);
    }

    #[test]
    fn test_bounding_box() {
        let aabb = BoundingBox::new(
            Vector3f::new(-0.3, 0.0, -0.3),
            Vector3f::new(0.3, 1.8, 0.3),
        );
        assert!((aabb.width() - 0.6).abs() < 0.001);
        assert!((aabb.height() - 1.8).abs() < 0.001);

        assert!(aabb.contains_point(Vector3f::new(0.0, 0.9, 0.0)));
        assert!(!aabb.contains_point(Vector3f::new(5.0, 0.9, 0.0)));
    }

    #[test]
    fn test_bounding_box_intersection() {
        let a = BoundingBox::new(
            Vector3f::new(0.0, 0.0, 0.0),
            Vector3f::new(1.0, 1.0, 1.0),
        );
        let b = BoundingBox::new(
            Vector3f::new(0.5, 0.5, 0.5),
            Vector3f::new(1.5, 1.5, 1.5),
        );
        assert!(a.intersects(b));

        let c = BoundingBox::new(
            Vector3f::new(2.0, 2.0, 2.0),
            Vector3f::new(3.0, 3.0, 3.0),
        );
        assert!(!a.intersects(c));
    }

    #[test]
    fn test_direction_from_block_face() {
        assert_eq!(Direction::from_block_face(0), Some(Direction::Down));
        assert_eq!(Direction::from_block_face(1), Some(Direction::Up));
        assert_eq!(Direction::from_block_face(2), Some(Direction::North));
        assert_eq!(Direction::from_block_face(3), Some(Direction::South));
        assert_eq!(Direction::from_block_face(4), Some(Direction::West));
        assert_eq!(Direction::from_block_face(5), Some(Direction::East));
        assert_eq!(Direction::from_block_face(6), None);
    }

    #[test]
    fn test_direction_to_block_face() {
        assert_eq!(Direction::Down.to_block_face(), 0);
        assert_eq!(Direction::Up.to_block_face(), 1);
        assert_eq!(Direction::North.to_block_face(), 2);
        assert_eq!(Direction::South.to_block_face(), 3);
        assert_eq!(Direction::West.to_block_face(), 4);
        assert_eq!(Direction::East.to_block_face(), 5);
    }

    #[test]
    fn test_direction_opposite() {
        assert_eq!(Direction::North.opposite(), Direction::South);
        assert_eq!(Direction::East.opposite(), Direction::West);
        assert_eq!(Direction::Up.opposite(), Direction::Down);
    }

    #[test]
    fn test_direction_vector() {
        assert_eq!(Direction::North.vector(), Vector3i::new(0, 0, -1));
        assert_eq!(Direction::South.vector(), Vector3i::new(0, 0, 1));
        assert_eq!(Direction::East.vector(), Vector3i::new(1, 0, 0));
        assert_eq!(Direction::West.vector(), Vector3i::new(-1, 0, 0));
        assert_eq!(Direction::Up.vector(), Vector3i::new(0, 1, 0));
        assert_eq!(Direction::Down.vector(), Vector3i::new(0, -1, 0));
    }

    #[test]
    fn test_facing_constants() {
        assert_eq!(FACING_NORTH, Direction::North.vector());
        assert_eq!(FACING_SOUTH, Direction::South.vector());
        assert_eq!(FACING_EAST, Direction::East.vector());
        assert_eq!(FACING_WEST, Direction::West.vector());
        assert_eq!(FACING_UP, Direction::Up.vector());
        assert_eq!(FACING_DOWN, Direction::Down.vector());
    }

    #[test]
    fn test_vector3i_to_f32() {
        let vi = Vector3i::new(1, 2, 3);
        let vf = vi.to_f32();
        assert_eq!(vf, Vector3f::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_vector3f_lerp() {
        let a = Vector3f::new(0.0, 0.0, 0.0);
        let b = Vector3f::new(10.0, 20.0, 30.0);
        let mid = a.lerp(b, 0.5);
        assert!((mid.x - 5.0).abs() < 0.001);
        assert!((mid.y - 10.0).abs() < 0.001);
        assert!((mid.z - 15.0).abs() < 0.001);
    }

    #[test]
    fn test_block_pos_from_vector3i() {
        let v = Vector3i::new(5, 10, 15);
        let pos = BlockPos::from(v);
        assert_eq!(pos.x, 5);
        assert_eq!(pos.y, 10);
        assert_eq!(pos.z, 15);
    }
}
