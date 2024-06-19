use std::fmt::Debug;

#[derive(Clone)]
pub struct Sound(String);

impl Sound {
    pub fn new(sound: &str) -> Sound {
        Sound(sound.to_string())
    }
    pub fn play(&self) -> &str {
        self.0.as_str()
    }
}

// A pet can eat any food and can have any state.
pub trait Pet {
    type Food;
    type State;

    fn name(&self) -> &str;

    fn eat(&mut self, food: Self::Food);
    fn eats(&self) -> &Self::Food;

    fn state(&self) -> &Self::State;

    fn make_sound(&self) -> String;
}

/**
 * Feed a pet some food.
 *
 * With the improved bounds in associated type feature in rust 1.79 we can now
 * specify the bounds for the associated type Food in the Pet trait.
 */
pub fn feed<T>(animal: &mut T, food: &T::Food)
where
    T: Pet<Food: Clone + Debug>
{
    println!("Feeding {} some {:?}!", animal.name(), food);
    animal.eat(food.clone());
}

