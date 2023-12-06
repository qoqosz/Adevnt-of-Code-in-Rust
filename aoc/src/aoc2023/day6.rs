use aoc::aoc;

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn new(time: u64, distance: u64) -> Self {
        Race { time, distance }
    }

    /// Number of wins in a race is a solution to the quadratic inequality:
    ///
    ///   v * (self.time - v) > self.distance
    ///
    /// where `v` is the speed/the number of ms the button was held.
    fn nwins(&self) -> u64 {
        let delta_sqrt = ((self.time * self.time - 4 * self.distance) as f64).sqrt();
        let time = self.time as f64;
        let t1 = (time + delta_sqrt) * 0.5;
        let t2 = (time - delta_sqrt) * 0.5;
        (t1.ceil() - t2.ceil()).floor() as u64
    }
}

#[aoc(2023, 6)]
pub fn main() {
    let races = [
        Race::new(53, 250),
        Race::new(91, 1330),
        Race::new(67, 1081),
        Race::new(68, 1025),
    ];
    // Part I
    let n: u64 = races.iter().map(|race| race.nwins()).product();
    println!("{n}");

    // Part II
    println!("{}", Race::new(53916768, 250133010811025).nwins());
}
