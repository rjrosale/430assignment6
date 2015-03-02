struct Implementer1;

struct Implementer2;

struct Container<T> { s: T }

trait TheTrait {}

impl TheTrait for Implementer1 {}
impl TheTrait for Implementer2 {}
impl<X: TheTrait> Container<X> {}

fn main() {
    let c1 = Container { s: Implementer1 };
    let c2 = Container { s: Implementer2 };
    println!("c1 = {}", c1);
    println!("c2 = {}", c2);
}