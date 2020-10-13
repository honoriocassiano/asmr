pub trait Vec {
	type Scalar;

	fn length(&self) -> Self::Scalar;

	fn squared_length(&self) -> Self::Scalar;

	fn normalize(&mut self) -> &mut Self;

	fn normalized(&self) -> Self;

	fn dot(&self, v: Self) -> Self::Scalar;

	fn cross(&self, v: Self) -> Self;
}