// The same method name in two different impl blocks is legitimate Rust and
// must not be reported.

impl A {
    pub fn value(&self) -> u32 {
        1
    }
}

impl B {
    pub fn value(&self) -> u32 {
        2
    }
}
