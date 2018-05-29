mod hardware;

fn main() {
    let mem = hardware::Memory::new(2, 12, 14);
    println!("Hello, world!");
}
