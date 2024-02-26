use std::{
    any::{Any, TypeId},
    marker::PhantomData,
};

pub struct Fact<'a, T: 'static> {
    pub x: PhantomData<&'a T>,
}

impl<'a, T> Fact<'a, T> {
    pub fn new() -> Self {
        Fact { x: PhantomData }
    }

    pub fn fact(&self) -> bool {
        TypeId::of::<T>() == true.type_id()
    }
}

#[cfg(test)]

mod test {
    use super::*;

    #[test]
    fn test_fact() {
        let f: Fact<Vec<String>> = Fact::new();
        println!("Fact about Vec: {}", f.fact());
        println!("Fact about Vec: {}", f.fact());
    }
}
