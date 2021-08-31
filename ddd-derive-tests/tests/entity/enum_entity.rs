use ddd_derive::Entity;

#[derive(Entity)]
enum Status {
    OPEN,
    CLOSED,
    PENDING,
}

fn main() {}