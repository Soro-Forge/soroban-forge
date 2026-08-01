// The impl block opened on line 3 is never closed. This is the shape produced
// by a badly resolved merge conflict.

impl Thing {
    pub fn first(&self) -> u32 {
        1
    }

    pub fn second(&self) -> u32 {
        2
    }
