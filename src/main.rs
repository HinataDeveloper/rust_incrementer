use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;

fn main() {
    println!("\n");

    let mut inc: Incrementer = Incrementer {
        counter: AtomicU32::new(0),
    };

    println!(" -> {}", inc.next());
    println!(" -> {}", inc.next());
    println!(" -> {}", inc.next());
    println!(" -> {}", inc.next());
    println!(" -> {}", inc.next());
    println!(" -> {}", inc.next());

    println!("\n ラミン、あなたはとてもすばらし (0.0.1)\n");
}

struct Incrementer {
    counter: AtomicU32,
}

impl Incrementer {
    fn next(&mut self) -> u32 {
        self.counter.fetch_add(1, Ordering::Acquire)
    }
}
