use std::fmt::Display;

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
    type Food: Display;
    type State;

    fn name(&self) -> &str;

    fn eat(&mut self, food: &Self::Food);
    fn eats(&self) -> &str;

    fn state(&self) -> &Self::State;

    fn make_sound(&self) -> String;
}

fn buy_food<T: Clone + Display>(food: &T) -> T {
    let food = food.clone();
    println!("Human has bought some {} from the store...", &food);
    food
}

/**
 * Feed a pet some food, but we only want to feed the pet food that we can buy
 * from the store (i.e. clone).
 *
 * With the improved bounds in associated type feature in rust 1.79 we can now
 * specify the bounds for the associated type Food in the Pet trait.
 */
pub fn feed<T>(animal: &mut T, food: &T::Food)
where
    T: Pet<Food: Clone>
{
    let food = buy_food(food);
    println!("Human is feeding {} some {}...", animal.name(), food);
    animal.eat(&food);
}

/**
 * The pet has found some food and wants to try eating it.
 */
pub fn scavange<T: Pet>(animal: &mut T, food: &T::Food) {
    println!("{} went scavaging, found some {}, and will now eat it...", animal.name(), food);
    animal.eat(food);
}