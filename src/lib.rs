pub use effers_derive::program;

#[cfg(test)]
mod test {
    use super::*;

    #[program(Smth => Printer(print(&self) as p), Logger(debug(self), info(&mut self)), inc::Incrementer(increment))]
    fn smth(val: u8) -> u8 {
        let s = p("hey hi hello");

        info("this is a info-level log");
        debug("this is a debug-level log");

        let _s = p("hey hi hello");

        dbg!(s);

        let x = increment(val);
        let y = increment(x);
        x + y
    }

    trait Printer {
        fn print(&self, s: &str) -> &str;
    }
    trait Logger {
        fn debug(self, s: &str);
        fn info(&mut self, s: &str);
    }
    mod inc {
        pub trait Incrementer {
            fn increment(v: u8) -> u8;
        }
    }

    #[program(Printer(print(&self) as p))]
    fn ohter() {
        let _s = p("hey hi hello");
    }
}
