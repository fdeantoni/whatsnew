use std::fmt::Display;
use crate::pet::*;

pub enum DogState {
    Barking(Sound),
    Sleeping(Sound),
    Running
}

/**
 * process_sound is a helper function that takes a Sound and performs some
 * complicated processing on it. This function is used to demonstrate how to
 * avoid cloning the Sound data when it is not necessary in the DogState enum.
 */
fn process_sound(sound: &Sound) -> String {
    format!("{}", sound.play())
}

/**
 * DogState is an enum that represents the different states a dog can be in.
 * Assume that the sound made during barking and sleeping can be very loud and
 * long, so we want to avoid cloning the sound data. Pre 1.79.0 we would have
 * to clone the sound data in the enum, but with the new match ergonomics we can
 * avoid cloning the sound data by using a reference to the Sound data.
 */
impl DogState {
    fn sound(&self) -> String {
        let sound = match self {
            DogState::Barking(sound) => sound,
            DogState::Sleeping(sound) => sound,
            DogState::Running => &Sound::new("Woooooshhhh...")
        };
        process_sound(sound)
    }
}

impl std::fmt::Display for DogState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let state = match self {
            DogState::Barking(_) => "Barking",
            DogState::Sleeping(_) => "Sleeping",
            DogState::Running => "Running"
        };
        write!(f, "{}", state)
    }
}

#[derive(Clone, Debug)]
pub struct DogFood {
    pub name: String,
    pub calories: f64,
}

impl Display for DogFood {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub struct Dog<'a> {
    name: String,
    weight: f64,
    food: &'a str,
    state: DogState
}

impl<'a> Pet for Dog<'a> {
    type Food = DogFood;
    type State = DogState;

    fn name(&self) -> &str {
        &self.name
    }

    fn eat(&mut self, food: &DogFood) {
        if food.name == self.food {
            println!("{}: Yummy! I love {}!", self.name(), food.name);
            self.weight += food.calories / 100.0;
        } else {
            println!("{}: Bleh! I don't like {}!", self.name(), food.name);
        }
    }

    fn eats(&self) -> &str {
        &self.food
    }

    fn make_sound(&self) -> String {
        self.state.sound()
    }

    fn state(&self) -> &DogState {
        &self.state
    }
}

impl<'a> Dog<'a> {
    pub fn new(name: &str, weight: f64, food: &'a str) -> Dog<'a> {
        Dog {
            name: name.to_string(),
            weight,
            food,
            state: DogState::Sleeping(Sound::new("Zzzzzzzz..."))
        }
    }

    pub fn bark(&mut self) {
        self.state = DogState::Barking(Sound::new("Woof! Woof!"));
    }

    pub fn run(&mut self) {
        self.state = DogState::Running;
    }

    pub fn sleep(&mut self) {
        self.state = DogState::Sleeping(Sound::new("Zzzzzzzz..."));
    }

    pub fn weight(&self) -> f64 {
        self.weight
    }
}