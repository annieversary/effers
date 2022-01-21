use effers::program;

#[program(Incrementer(increment(&self)), Printer(print(&self)))]
fn prog(val: u8) -> u8 {
    let x = increment(val);
    let y = increment(x);

    print(x);

    x + y
}

pub trait Incrementer {
    fn increment(&self, v: u8) -> u8;
}

#[derive(Clone, Copy)]
pub struct TestInc;
impl Incrementer for TestInc {
    fn increment(&self, v: u8) -> u8 {
        v + 3
    }
}

trait Printer {
    fn print(&self, s: u8);
}

#[derive(Clone, Copy)]
struct Printer1;
impl Printer for Printer1 {
    fn print(&self, s: u8) {
        println!("1: {}", s)
    }
}
#[derive(Clone)]
struct Printer2 {
    prefix: String,
}
impl Printer for Printer2 {
    fn print(&self, s: u8) {
        println!("2: {} {}", self.prefix, s)
    }
}

fn main() {
    // if a program only has Clone effects, the program also becomes clone
    // same applies for Copy

    // a is Copy since TestInc is Copy
    let a = Prog.add(TestInc);

    let b = a.add(Printer1);
    let c = a.add(Printer2 {
        prefix: "this is a number".to_string(),
    });

    // both TestInc and Printer1 are Copy,
    // therefore b is copy, and we can call it as much as we want
    let first_result = b.run(0);
    assert_eq!(first_result, 9);
    let second_result = b.run(2);
    assert_eq!(second_result, 13);

    // since Printer2 is not Copy, but it is Clone,
    // c is Clone but not Copy
    let first_result = c.clone().run(0);
    assert_eq!(first_result, 9);
    let second_result = c.run(2);
    assert_eq!(second_result, 13);
}
