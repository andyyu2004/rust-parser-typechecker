
pub struct Counter {
   i: u64,
}

impl Counter {
    pub fn new() -> Self {
        Self { i: 0 }
    }
    
    pub fn next(&mut self) -> u64 {
        self.i += 1;
        self.i
    }
}
