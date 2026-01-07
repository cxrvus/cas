use std::{collections::HashSet, hash::Hash};

use anyhow::Result;

use crate::logic::Function;

pub struct Group<'f, G>
where
	G: Eq + Hash,
{
	set: HashSet<G>,
	func: &'f Function<(G, G), G>,
}

impl<'f, G> Group<'f, G>
where
	G: Eq + Hash,
{
	pub fn new(set: HashSet<G>, func: &'f Function<(G, G), G>) -> Result<Self> {
		Ok(Self { set, func })
	}
}
