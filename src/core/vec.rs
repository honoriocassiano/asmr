pub trait Vec {
	type Scalar;

	fn length(&self) -> Self::Scalar;

	fn squared_length(&self) -> Self::Scalar;

	fn normalize(&mut self) -> &mut Self;

	fn normalized(&self) -> Self;

	fn dot(&self, v: Self) -> Self::Scalar;

	fn cross(&self, v: Self) -> Self;
}

pub trait Sqrt {
	fn sq_root(&self) -> Self;
}

macro_rules! impl_sqrt {
    ($type:ty) => {
		 impl Sqrt for $type {
			fn sq_root(&self) -> Self {
				self.sqrt()
			}
		}
    };
}

impl_sqrt!(f64);
impl_sqrt!(f32);
