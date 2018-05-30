/// Memory defines the parameters of system memory.
pub struct Memory {
    /// Page size in bits (P = 2^p)
    pub p: u8,
    /// Physical address size in bits (M = 2^m)
    pub m: u8,
    /// Virtual address size in bits (N = 2^n)
    pub n: u8,
}

impl Memory {
    /// Creates a new Memory struct.
    ///
    /// # Arguments
    ///
    /// * `p` - The page size in bits.
    /// * `m` - The physical address size in bits.
    /// * `n` - The virtual address size in bits.
    ///
    /// # Example
    ///
    /// ```
    /// use mmu::hardware;
    /// // create a memory model with a (p, m, n) = (4, 12, 14).
    /// let mm = hardware::Memory::new(4, 12, 14);
    /// ```
    pub fn new(p: u8, m: u8, n: u8) -> Memory {
        Memory {
            p: p,
            m: m,
            n: n,
        }
    }
}

/// TLB cache specification
pub struct TLB {
    /// Associativity (number of lines / set)
    e: u8,
    /// Set count
    s: u8,
}

impl TLB {
    /// Creates a new TLB struct.
    ///
    /// # Arguments
    ///
    /// * `e` - The associativity of the cache.
    /// * `s` - The number of sets in the cache.
    ///
    /// # Example
    ///
    /// ```
    /// use mmu::hardware;
    /// // create a new TLB with 4 sets, each with 4 lines.
    /// let tlb = hardware::TLB::new(4, 4);
    /// ```
    pub fn new(e: u8, s: u8) -> TLB {
        TLB {
            e: e,
            s: s,
        }
    }
}

