use effers::program;

#[program(MyCoolProgram =>
    Printer(print(&self) as p, available as printer_available),
    Logger(debug(&mut self), info(self))
)]
fn my_program(val: u8) -> u8 {
    if printer_available() {
        p("hey hi hello");
    }

    debug("this is a debug-level log");
    info("this is a info-level log");

    val + 3
}

#[program(Printer(print(&self) as p))]
fn other_program() {
    p("hey hi hello");
}

fn main() {
    // call the first program twice
    let result: u8 = MyCoolProgram.add(IoPrinter).add(FileLogger).run(3);
    assert_eq!(result, 6);
    let other_result: u8 = MyCoolProgram
        .add(IoPrinter)
        .add(NetworkLogger {
            credentials: "secret password".to_string(),
        })
        .run(8);
    assert_eq!(other_result, 11);

    // other program
    OtherProgram.add(IoPrinter).run();
}

trait Printer {
    fn print(&self, s: &str);
    fn available() -> bool;
}
trait Logger {
    fn debug(&mut self, s: &str);
    fn info(self, s: &str);
}

struct IoPrinter;
impl Printer for IoPrinter {
    fn print(&self, s: &str) {
        println!("{}", s)
    }
    fn available() -> bool {
        true
    }
}

struct FileLogger;
impl Logger for FileLogger {
    fn debug(&mut self, s: &str) {
        println!("debug: {}", s)
    }
    fn info(self, s: &str) {
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
    fn info(self, s: &str) {
        println!(
            "info through network: {}; with password {}",
            s, self.credentials
        )
    }
}
