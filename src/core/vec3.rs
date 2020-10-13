use std::ops;
use std::fmt;
use std::fmt::Display;
use super::vec::{Vec, Sqrt};


#[derive(Copy, Clone)]
pub struct Vec3<Scalar>(Scalar, Scalar, Scalar);
// pub struct Vec3(Scalar, Scalar, Scalar);


impl<Scalar: Copy + Sqrt> Vec3<Scalar> {
	pub fn new(x: Scalar, y: Scalar, z: Scalar) -> Self {
		Self(x, y, z)
	}
	//
	// pub fn zero() -> Self {
	// 	Self(0 as Scalar, 0 as Scalar, 0 as Scalar)
	// }

	pub fn x(&self) -> Scalar {
		self.0
	}

	pub fn y(&self) -> Scalar {
		self.1
	}

	pub fn z(&self) -> Scalar {
		self.2
	}
}


impl<T: Sqrt> Vec for Vec3<T>
	where T: Copy + ops::Add<Output=T> + ops::Sub<Output=T> + ops::Mul<Output=T>
	+ ops::Div<Output=T> {
	type Scalar = T;

	fn length(&self) -> Self::Scalar {
		self.squared_length().sq_root()
	}

	fn squared_length(&self) -> Self::Scalar {
		(self.0 * self.0) + (self.1 * self.1) + (self.2 * self.2)
	}

	fn normalize(&mut self) -> &mut Self {
		let norm = self.length();

		*self = Self(
			self.0 / norm,
			self.1 / norm,
			self.2 / norm,
		);

		self
	}

	#[must_use = "method returns a new Vec3 and does not mutate the original value"]
	fn normalized(&self) -> Self {
		let norm = self.length();

		Self(self.0 / norm, self.1 / norm, self.2 / norm)
	}

	fn dot(&self, v: Self) -> Self::Scalar {
		(self.0 * v.0) + (self.1 * v.1) + (self.2 * v.2)
	}

	#[must_use = "method returns a new Vec3 and does not mutate the original value"]
	fn cross(&self, v: Self) -> Self {
		Self(
			(self.1 * v.2) - (self.2 * v.1),
			(self.2 * v.0) - (self.0 * v.2),
			(self.0 * v.1) - (self.1 * v.0),
		)
	}
}

// Addition operators
impl<Scalar> ops::Add for Vec3<Scalar> where Scalar: ops::Add<Output=Scalar> {
	type Output = Self;

	fn add(self, other: Self) -> Self::Output {
		Self(
			self.0 + other.0,
			self.1 + other.1,
			self.2 + other.2,
		)
	}
}


impl<Scalar> ops::AddAssign for Vec3<Scalar> where Scalar: Copy + ops::Add<Output=Scalar> {
	fn add_assign(&mut self, other: Self) {
		*self = Self(
			self.0 + other.0,
			self.1 + other.1,
			self.2 + other.2,
		);
	}
}


// Subtraction operators
impl<Scalar> ops::Sub for Vec3<Scalar> where Scalar: ops::Sub<Output=Scalar> {
	type Output = Self;

	fn sub(self, other: Self) -> Self::Output {
		Self(
			self.0 - other.0,
			self.1 - other.1,
			self.2 - other.2,
		)
	}
}


impl<Scalar> ops::SubAssign for Vec3<Scalar> where Scalar: Copy + ops::Sub<Output=Scalar> {
	fn sub_assign(&mut self, other: Self) {
		*self = Self(
			self.0 - other.0,
			self.1 - other.1,
			self.2 - other.2,
		);
	}
}


// Unary negation
impl<Scalar> ops::Neg for Vec3<Scalar> where Scalar: Copy + ops::Neg<Output=Scalar> {
	type Output = Self;

	fn neg(self) -> Self::Output {
		Self(
			-self.0,
			-self.1,
			-self.2,
		)
	}
}


// Multiplication operators
impl<Scalar> ops::Mul<Scalar> for Vec3<Scalar> where Scalar: Copy + ops::Mul<Output=Scalar> {
	type Output = Self;

	fn mul(self, scalar: Scalar) -> Self::Output {
		Self(
			self.0 * scalar,
			self.1 * scalar,
			self.2 * scalar,
		)
	}
}


impl<Scalar> ops::Mul<Vec3<Scalar>> for Vec3<Scalar> where Scalar: Copy + ops::Mul<Output=Scalar> {
	type Output = Self;

	fn mul(self, other: Self) -> Self::Output {
		Self(
			self.0 * other.0,
			self.1 * other.1,
			self.2 * other.2,
		)
	}
}


impl<Scalar> ops::MulAssign<Scalar> for Vec3<Scalar> where Scalar: Copy + ops::Mul<Output=Scalar> {
	fn mul_assign(&mut self, scalar: Scalar) {
		*self = Self(
			self.0 * scalar,
			self.1 * scalar,
			self.2 * scalar,
		);
	}
}


// Right hand scalar multiplication operator
// impl<Scalar: Copy> ops::Mul<Vec3<Scalar>> for Scalar {
// 	type Output = Vec3<Scalar>;
//
// 	fn mul(self, vec: Self::Output) -> Self::Output {
// 		Vec3(
// 			vec.0 * self,
// 			vec.1 * self,
// 			vec.2 * self,
// 		)
// 	}
// }


// Division operators
impl<Scalar> ops::Div<Scalar> for Vec3<Scalar> where Scalar: Copy + ops::Div<Output=Scalar> {
	type Output = Self;

	fn div(self, scalar: Scalar) -> Self::Output {
		Self(
			self.0 / scalar,
			self.1 / scalar,
			self.2 / scalar,
		)
	}
}


impl<Scalar> ops::DivAssign<Scalar> for Vec3<Scalar> where Scalar: Copy + ops::Div<Output=Scalar> {
	fn div_assign(&mut self, scalar: Scalar) {
		*self = Self(
			self.0 / scalar,
			self.1 / scalar,
			self.2 / scalar,
		);
	}
}


// Indexing operators
impl<Scalar> ops::Index<usize> for Vec3<Scalar> where Scalar: Copy {
	type Output = Scalar;

	fn index(&self, other: usize) -> &Scalar {
		match other {
			0 => &self.0,
			1 => &self.1,
			2 => &self.2,
			_ => panic!("Index {} is not in Vec3", other)
		}
	}
}


impl<Scalar> ops::IndexMut<usize> for Vec3<Scalar> where Scalar: Copy {
	fn index_mut(&mut self, other: usize) -> &mut Scalar {
		match other {
			0 => &mut self.0,
			1 => &mut self.1,
			2 => &mut self.2,
			_ => panic!("Index {} is not in Vec3", other)
		}
	}
}


// Format string
impl<Scalar> fmt::Display for Vec3<Scalar> where Scalar: Copy + Display {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "({}, {}, {})", self.0, self.1, self.2)
	}
}


macro_rules! right_hand_mul {
    ($type:ty) => {
    	impl ops::Mul<Vec3<$type>> for $type {
			type Output = Vec3<$type>;

			fn mul(self, vec: Self::Output) -> Self::Output {
				Vec3(
					vec.0 * self,
					vec.1 * self,
					vec.2 * self,
				)
			}
		}
    };
}

// right_hand_mul!(f64);
// right_hand_mul!(i32);