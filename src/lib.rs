pub use effers_derive::program;

#[cfg(test)]
mod test {
    use super::*;

    #[program(Smth => Printer(print as p), Logger(debug, info))]
    fn smth(val: u8) -> u8 {
        p("hey hi hello");

        debug("this is a debug-level log");
        info("this is a info-level log");

        val + 3
    }

    trait Printer {
        fn print(&mut self, s: &str);
    }
    trait Logger {
        fn debug(&mut self, s: &str);
        fn info(&mut self, s: &str);
    }
}
