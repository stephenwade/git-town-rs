pub struct Counter(u32);

impl Counter {
    pub fn new() -> Self {
        Counter(0)
    }

    pub fn increment(&mut self) {
        self.0 += 1;
    }

    pub fn value(&self) -> u32 {
        self.0
    }
}
