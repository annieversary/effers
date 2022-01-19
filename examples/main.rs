use effers::program;

#[program(Smth => Printer(print as p), Logger(debug, info))]
fn smth(val: u8) -> u8 {
    p("hey hi hello");

    debug("this is a debug-level log");
    info("this is a info-level log");

    val + 3
}

fn main() {
    // maybe smth like this?
    let result: u8 = Smth.add(IoPrinter).add(FileLogger).run(3);
    assert_eq!(result, 6);
    let other_result: u8 = Smth
        .add(IoPrinter)
        .add(NetworkLogger {
            credentials: "secret password".to_string(),
        })
        .run(8);
    assert_eq!(other_result, 11);
}

trait Printer {
    fn print(&mut self, s: &str);
}
trait Logger {
    fn debug(&mut self, s: &str);
    fn info(&mut self, s: &str);
}

struct IoPrinter;
impl Printer for IoPrinter {
    fn print(&mut self, s: &str) {
        println!("{}", s)
    }
}

struct FileLogger;
impl Logger for FileLogger {
    fn debug(&mut self, s: &str) {
        println!("debug: {}", s)
    }
    fn info(&mut self, s: &str) {
        println!("info: {}", s)
    }
}

struct NetworkLogger {
    credentials: String,
}
impl Logger for NetworkLogger {
    fn debug(&mut self, s: &str) {
        println!(
            "debug through network: {}; with password {}",
            s, self.credentials
        )
    }
    fn info(&mut self, s: &str) {
        println!(
            "info through network: {}; with password {}",
            s, self.credentials
        )
    }
}