mod mymod;
use mymod::Machine;

#[allow(unused_must_use)]
fn main() {
    let mut m: Machine<i32, i32> = Machine::new(
        0,
        vec![((0, 0), 0), ((0, 1), 1), ((1, 0), 0), ((1, 1), 1)].into_iter(),
    );

    m.feed(0);
    m.feed(1);
    m.feed(1);
    m.feed(0);
    m.feed(0);
    m.feed(1);

    println!("{}", m.state());
}
