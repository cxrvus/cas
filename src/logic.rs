use std::{
	collections::HashSet,
	hash::Hash,
	ops::{Deref, DerefMut},
};

// pub type Set<T> = HashSet<T>;

pub struct Set<T>(HashSet<T>)
where
	T: Eq + Hash;

impl<T> Deref for Set<T>
where
	T: Eq + Hash,
{
	type Target = HashSet<T>;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<T> DerefMut for Set<T>
where
	T: Eq + Hash,
{
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

pub struct Function<I, O>
where
	I: Eq + Hash,
	O: Eq + Hash,
{
	func: dyn Fn(Set<I>) -> Set<O>,
}

impl<T> Function<(T, T), T>
where
	T: Eq + Hash,
{
	pub fn is_commutative(&self) -> bool {
		true
	}
}
