mod dfa;
use dfa::Machine;
use std::convert::TryFrom;
use std::io::BufRead;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum State {
    Initial,
    Accepting,
}

#[derive(PartialEq, Eq, Debug, Hash)]
enum Symbol {
    Zero,
    One,
}

impl dfa::State for State {}
impl dfa::Event for Symbol {}

impl TryFrom<&str> for Symbol {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, &'static str> {
        match s {
            "0" => Ok(Symbol::Zero),
            "1" => Ok(Symbol::One),
            _ => Err("Symbol must be either 0 or 1"),
        }
    }
}

#[allow(unused_must_use)]
fn main() -> dfa::Result<()> {
    let mut m: Machine<State, Symbol> = Machine::new(
        State::Initial,
        vec![
            ((State::Initial, Symbol::Zero), State::Initial),
            ((State::Initial, Symbol::One), State::Accepting),
            ((State::Accepting, Symbol::Zero), State::Initial),
            ((State::Accepting, Symbol::One), State::Accepting),
        ]
        .into_iter(),
    );

    let mut line = String::new();
    std::io::stdin().lock().read_line(&mut line);
    line.split_whitespace()
        .map(Symbol::try_from)
        .map(Result::unwrap)
        .map(|symbol| m.feed(symbol))
        .for_each(Result::unwrap);

    println!("{:?}", m.state() == State::Accepting);
    Ok(())
}
