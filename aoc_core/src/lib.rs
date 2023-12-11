pub struct Solution {
    pub year: u16,
    pub day: u16,
    f: fn(),
}

impl Solution {
    pub const fn new(year: u16, day: u16, f: fn()) -> Self {
        Self { year, day, f }
    }

    pub fn run(&self) {
        (self.f)()
    }
}

inventory::collect!(Solution);
