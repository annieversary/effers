// Effects can be referenced from inside a module
#[effers::program(inc::Incrementer(increment(&self)))]
fn prog(val: u8) -> u8 {
    let x = increment(val);
    let y = increment(x);
    x + y
}

mod inc {
    pub trait Incrementer {
        fn increment(&self, v: u8) -> u8;
    }

    pub struct TestInc;
    impl Incrementer for TestInc {
        fn increment(&self, v: u8) -> u8 {
            v + 3
        }
    }
}

fn main() {
    Prog.add(inc::TestInc).run(1);
}
