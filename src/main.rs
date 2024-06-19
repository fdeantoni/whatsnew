mod pet;
mod dog;

use pet::*;
use dog::*;

const GRAVITY: f64 = 9.81;
const FIDO_MASS: f64 = 1.5;

fn main() {
    
    // Specify the type of food that Fido likes to eat.
    let fidobites = DogFood {
        name: "Fido Bites".to_string(),
        calories: 100.0,
        tasty: true
    };
    // Set the initial weight of Fida. Let's assume that the weight calculation of Fido is very
    // complex and expensive.
    let weight = FIDO_MASS * GRAVITY;
    // Build Fido with the specified name, weight, and food it likes.
    let mut fido = Dog::new("Fido", weight, fidobites.clone() );

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
    println!("{}'s starting weight is {} and loves to eat {}", fido.name(), fido.weight(), fido.eats().name);

    // Feed Fido it's favourite food
    feed(&mut fido, &fidobites);
    println!("{}'s weight is now: {}",fido.name(), fido.weight());

    // Feed Fido some Pedigree Chum and see if it likes it.
    let pedigree = DogFood {
        name: "Pedigree Chum".to_string(),
        calories: 200.0,
        tasty: false
    };
    feed(&mut fido, &pedigree);
    println!("{}'s weight is now: {}",fido.name(), fido.weight());

    // Fido has found some trash and tries eating it.
    let trash = DogFood {
        name: "Trash".to_string(),
        calories: 0.0,
        tasty: false
    };
    scavange(&mut fido, &trash);
    println!("{}'s weight is now: {}",fido.name(), fido.weight());
}
