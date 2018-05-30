mod hardware;
mod address;

fn main() {
    // create memory model
    let mm = hardware::Memory::new(2, 12, 14);
    // TLB with 4 sets, 4 lines/set
    let tlb = hardware::TLB::new(4, 4);
    // dummy virtual addr
    let va: u64 = 13;
    // get the vpo
    let vpo = address::po(&mm, &va);
    // print
    println!("va: {}, vpo: {}", va, vpo);
}
