//! A very simple example of a pet dog that can eat food and change states.
//! It demonstrates some of the improvements each rust update brings.
//! [Rust-1.82.0](https://blog.rust-lang.org/2024/10/17/Rust-1.82.0.html)
//! [Rust-1.81.0](https://blog.rust-lang.org/2024/09/05/Rust-1.81.0.html)
mod pet;
mod dog;

use std::{collections::HashMap, sync::LazyLock};

use pet::*;
use dog::*;

const GRAVITY: f64 = 9.81;

static DOG_FOOD: LazyLock<HashMap<&'static str, DogFood>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert("Fido Bites", DogFood {
        name: "Fido Bites".to_string(),
        calories: 100.0,
    });
    m.insert("Pedigree Chum", DogFood {
        name: "Pedigree Chum".to_string(),
        calories: 200.0,
    });
    m
});

// Set the initial weight of Fida. Let's assume that the weight calculation of Fido is very
// complex and expensive, but we can use a const block to calculate the weight of Fido during
// compile time!
const fn get_fido_weight() -> f64 {
    1.5 * GRAVITY 
}

fn main() {
    
    // Specify the type of food that Fido likes to eat.
    let fidobites = DOG_FOOD.get("Fido Bites").expect("Fido Bites not found!");

    // Build Fido with the specified name, weight, and food it likes.
    let weight = get_fido_weight();
    let mut fido = Dog::new("Fido", weight, &fidobites.name );

    // Randomly decide if Fido sees a cat and if the cat attacks.
    let sees_cat = rand::random();
    let cat_attacks = rand::random();

    if sees_cat {
        fido.bark();
        if cat_attacks {
            fido.run();
        }
    } else {
        fido.sleep();
    }

    // Print out the state of Fido and show the noise it's making
    println!("{} is {}: {}", fido.name(), fido.state(), fido.make_sound());
    println!("{}'s starting weight is {} and loves to eat {}", fido.name(), fido.weight(), fido.eats());

    // Feed Fido it's favourite food
    feed(&mut fido, &fidobites);
    println!("{}'s weight is now: {}",fido.name(), fido.weight());

    // Feed Fido some Pedigree Chum and see if it likes it.
    let pedigree = DOG_FOOD.get("Pedigree Chum").expect("Pedigree Chum not found!");
    feed(&mut fido, &pedigree);
    println!("{}'s weight is now: {}",fido.name(), fido.weight());

    // Fido has found some trash and tries eating it.
    let trash = DogFood {
        name: "Trash".to_string(),
        calories: 0.0,
    };
    scavange(&mut fido, &trash);
    println!("{}'s weight is now: {}",fido.name(), fido.weight());
}
