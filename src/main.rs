mod dfa;
use dfa::{Event, Machine, State};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum MyState {
    Initial,
    Accepting,
}

impl State for MyState {}

#[allow(unused_must_use)]
fn main() {
    let mut m: Machine<MyState, i32> = Machine::new(
        MyState::Initial,
        vec![
            ((MyState::Initial, 0), MyState::Initial),
            ((MyState::Initial, 1), MyState::Accepting),
            ((MyState::Accepting, 0), MyState::Initial),
            ((MyState::Accepting, 1), MyState::Accepting),
        ]
        .into_iter(),
    );

    m.feed(0);
    m.feed(1);
    m.feed(1);
    m.feed(0);
    m.feed(0);
    m.feed(1);

    println!("{:?}", m.state());
}
