# Whats New in Rust?

This project is a simple toy project to find out and understand all the improvements added by each Rust release.


## Major 1.82.0 Updates

Now const floating point operations are possible! Where before the following was necessary:
```rust
const GRAVITY: f64 = 9.81;
fn get_fido_weight() -> f64 {
    const { 1.5 * GRAVITY }
}
```
Now you can simply do:
```rust
const GRAVITY: f64 = 9.81;
const fn get_fido_weight() -> f64 {
    1.5 * GRAVITY
}
```

Furthermore a lot safety updates:
* Native syntax for creating raw pointers. Before:
    ```rust
    let ptr = std::ptr::addr_of!(p.not_aligned_field);
    ```
    After 
    ```rust
    let ptr = &raw const p.not_aligned_field;
    ```
* Safe items with `unsafe extern`
* Unsafe attributes to properly mark functions decorated with an unsafe attribute
* Safely addressing static place expressions. Before:
    ```rust
    static mut STATIC_MUT: Type = Type::new();
    fn main() {
        let static_mut_ptr = unsafe { 
            std::ptr::addr_of_mut!(STATIC_MUT) 
        };
    }    
    ```
    After:
    ```rust
    static mut STATIC_MUT: Type = Type::new();
    fn main() {
        let static_mut_ptr = &raw mut STATIC_MUT;
    }
    ```

There is also a new `use<..>` syntax to define lifetime bounds for opaque types. Before:
```rust
trait Captures<T: ?Sized> {}
impl<T: ?Sized, U: ?Sized> Captures<T> for U {}

struct Ctx<'cx>(&'cx u8);

fn f<'cx, 'a>(
    cx: Ctx<'cx>,
    x: &'a u8,
) -> impl Iterator<Item = &'a u8> + Captures<&'cx ()> {
    core::iter::once_with(move || {
        eprintln!("LOG: {}", cx.0);
        x
    })
}
```
After
```rust
struct Ctx<'cx>(&'cx u8);

fn f<'cx, 'a>(
    cx: Ctx<'cx>,
    x: &'a u8,
) -> impl Iterator<Item = &'a u8> + use<'a,'cx> {
    core::iter::once_with(move || {
        eprintln!("LOG: {}", cx.0);
        x
    })
}
```

The new cargo command `cargo info` was also introduced.

See [Announcing Rust 1.82.0](https://blog.rust-lang.org/2024/10/17/Rust-1.82.0.html)

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