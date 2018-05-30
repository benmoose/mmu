// Memory parameters
// p = page size = 2**p
// m = physical address length = 2**m
// n = virtual address length = 2**n
pub struct Memory {
    pub p: u8,
    pub m: u8,
    pub n: u8,
}

impl Memory {
    pub fn new(p: u8, m: u8, n: u8) -> Memory {
        Memory {
            p: p,
            m: m,
            n: n,
        }
    }
}

// TLB params
// associativity = e
// set count = s
pub struct TLB {
    e: u8,
    s: u8,
}

impl TLB {
    pub fn new(e: u8, s: u8) -> TLB {
        TLB {
            e: e,
            s: s,
        }
    }
}

