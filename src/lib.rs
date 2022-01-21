pub use effers_derive::program;

#[cfg(test)]
mod test {
    use super::*;

    #[program(Smth => Printer(print as p), Logger(mut debug, mut info), inc::Incrementer(mut increment))]
    fn smth(val: u8) -> u8 {
        let s = p("hey hi hello");

        debug("this is a debug-level log");
        info("this is a info-level log");

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
        fn debug(&mut self, s: &str);
        fn info(&mut self, s: &str);
    }
    mod inc {
        pub trait Incrementer {
            fn increment(&mut self, v: u8) -> u8;
        }
    }

    // TODO make nameless programs work
    #[program(Printer(print as p))]
    fn ohter() {
        let _s = p("hey hi hello");
    }
}
