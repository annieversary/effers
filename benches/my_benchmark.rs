use criterion::{black_box, criterion_group, criterion_main, Criterion};
use effers::program;

#[program(State(get(&self), put(&mut self)))]
fn prog() -> u32 {
    loop {
        let n = get();
        if n <= 0 {
            return n;
        } else {
            put(n - 1);
        }
    }
}

trait State {
    fn get(&self) -> u32;
    fn put(&mut self, val: u32);
}
struct MyState {
    v: u32,
}
impl State for MyState {
    fn get(&self) -> u32 {
        self.v
    }

    fn put(&mut self, val: u32) {
        self.v = val;
    }
}

fn run_with_effect(v: u32) {
    Prog.add(MyState { v }).run();
}

fn run_without_effect(mut n: u32) {
    let r = loop {
        if n <= 0 {
            break n;
        } else {
            n = n - 1;
        }
    };
    assert_eq!(0, r);
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("state: effers: 20", |b| {
        b.iter(|| run_with_effect(black_box(20)))
    });
    c.bench_function("state: effers: 20000", |b| {
        b.iter(|| run_with_effect(black_box(20000)))
    });
    c.bench_function("state: no effect system: 20", |b| {
        b.iter(|| run_without_effect(black_box(20)))
    });
    c.bench_function("state: no effect system: 20000", |b| {
        b.iter(|| run_without_effect(black_box(20000)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
