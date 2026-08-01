// `get_escrow` is defined twice inside the same impl block. This is the shape
// a badly resolved merge conflict produces.

impl Escrow {
    pub fn get_escrow(&self) -> u32 {
        1
    }

    pub fn other(&self) -> u32 {
        2
    }

    pub fn get_escrow(&self) -> u32 {
        3
    }
}
