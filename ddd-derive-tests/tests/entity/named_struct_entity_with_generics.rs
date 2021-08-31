use ddd_derive::Entity;

#[derive(Entity)]
struct Person<T> {
    raw: T,
    name: String,
}

fn main() {}