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

impl std::cmp::PartialEq<(u16, u16)> for Solution {
    fn eq(&self, other: &(u16, u16)) -> bool {
        (self.year, self.day) == *other
    }
}

inventory::collect!(Solution);
