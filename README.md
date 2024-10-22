# Whats New in Rust?

This project is a simple toy project to find out and understand all the improvements added by each Rust release.


## Major 1.81.0 Updates

Stabilization of:
* `core::error::Error`
* `fs::exists`

Fixed [CVE-2024-43402](https://blog.rust-lang.org/2024/09/04/cve-2024-43402.html)

See [Announcing Rust 1.81.0](https://blog.rust-lang.org/2024/09/05/Rust-1.81.0.html).

## Major 1.80.0 and 1.80.1 Updates

Added LazyCell and LazyLock which can replace the lazy_static crate, e.g. before:
```rust
lazy_static! {
    static ref DOG_FOOD: HashMap<&'static str, DogFood> = {
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
    };
}
```

Now with `LazyLock` instead:
```rust
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
```

Exlusive ranges in patterns added, written as `a..b` or `..b`. 

See [Announcing Rust 1.80.1](https://blog.rust-lang.org/2024/08/08/Rust-1.80.1.html).