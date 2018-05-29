mod hardware;

// returns vpo of a virtual address
pub fn get_vpo(mem: &hardware::Hardware, va: u64) -> u64 {
    va & 2u64.pow(mem.p as u32) - 1
}

pub fn get_vpn(mem: hardware::Hardware, va: u64) -> u64 { 
}

