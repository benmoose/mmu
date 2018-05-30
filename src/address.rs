// Functions for manipulating virtual and physical addresses.

use hardware::Memory;

/// Gets the page offset of an address.
/// Takes a Memory struct specifying the memory model
/// and an address (can be physical or virtual).
///
/// # Examples
///
/// ```
/// use address;
///
/// let va: u64 = 27;
/// let mm = Memory::new(4, 12, 14);
/// 
/// let po = address::po(&mm, &va);
/// // po = 11
/// ```
pub fn po(mem: &Memory, va: &u64) -> u64 {
    *va & 2u64.pow(mem.p as u32) - 1
}

/// Gets the page number of an address.
/// Takes a Memory struct specifying the memory model
/// and an address (can be physical or virtual).
///
/// # Examples
///
/// ```
/// use address;
///
/// let va: u64 = 27;
/// let mm = Memory::new(4, 12, 14);
/// 
/// let pn = address::pn(&mm, &va);
/// // pn = 1
/// ```
pub fn pn(mem: &Memory, va: &u64) -> u64 { 
    *va >> mem.p
}

