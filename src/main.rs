mod hardware;
mod address;

fn main() {
    // create memory model
    let mm = hardware::Memory::new(2, 12, 14);
    // dummy virtual addr
    let va: u64 = 13;
    // get the vpo
    let vpo = address::vpo
}
