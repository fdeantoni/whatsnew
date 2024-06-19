const GRAVITY: f64 = 9.81;

struct Sound(String);

impl Sound {
    fn new(sound: &str) -> Sound {
        Sound(sound.to_string())
    }
}

enum State {
    Barking(Sound),
    Sleeping(Sound),
    Running
}

impl State {
    fn sound(&self) -> String {
        let sound = match self {
            State::Barking(sound) => sound,
            State::Sleeping(sound) => sound,
            State::Running => &Sound::new("Woooooshhhh...")
        };
        format!("{}", sound.0)
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let state = match self {
            State::Barking(_) => "Barking",
            State::Sleeping(_) => "Sleeping",
            State::Running => "Running"
        };
        write!(f, "{}", state)
    }
}

struct Dog {
    weight: f64,
    state: State
}

impl Dog {
    fn new(weight: f64) -> Dog {
        Dog {
            weight,
            state: State::Sleeping(Sound::new("Zzzzzzzz..."))
        }
    }

    fn bark(&mut self) {
        self.state = State::Barking(Sound::new("Woof! Woof!"));
    }

    fn run(&mut self) {
        self.state = State::Running;
    }

    fn sleep(&mut self) {
        self.state = State::Sleeping(Sound::new("Zzzzzzzz..."));
    }

    fn make_sound(&self) -> String {
        self.state.sound()
    }
}

fn main() {
    
    let mut fido = Dog::new(const { 1.5 * GRAVITY });

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

    println!("Fido is {}: {}", fido.state, fido.make_sound());
    println!("Fido's weight is: {}", fido.weight);
}
