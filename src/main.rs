mod pet;
mod dog;

use pet::*;
use dog::*;

const GRAVITY: f64 = 9.81;


fn main() {
    
    let fidobites = DogFood {
        name: "Fido Bites".to_string(),
        calories: 100.0,
        tasty: true
    };
    let mut fido = Dog::new("Fido", 1.5 * GRAVITY, fidobites.clone() );

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

    println!("{} is {}: {}", fido.name(), fido.state(), fido.make_sound());
    println!("{}'s weight is: {}", fido.name(), fido.weight());
    println!("{} likes to eat {}", fido.name(), fido.eats().name);
    
    feed(&mut fido, &fidobites);
    println!("{}'s weight is now: {}",fido.name(), fido.weight());

    let pedigree = DogFood {
        name: "Pedigree".to_string(),
        calories: 200.0,
        tasty: false
    };
    feed(&mut fido, &pedigree);
    println!("{}'s weight is now: {}",fido.name(), fido.weight());


}
