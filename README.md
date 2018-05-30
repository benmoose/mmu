# Memory Management Unit

#### What is an MMU

A **M**emory **M**anagement **U**nit is a component on modern CPU chips (on systems that implement virtual memory) that maps virtual addresses (emitted from the CPU) to their physical addresses, so they can be located in memory.

#### Project Structure

Source code can be found under the `src/`.
Functionality is grouped as follows:

| File | Description |
| --- | --- |
| `hardware.rs` | Contains structs and associated functions that define the system parameters (e.g. virtual/physical memory size, page size, cache specifications). |
| `address.rs` | Contains functions for manipulating virtual and physical addresses. |

