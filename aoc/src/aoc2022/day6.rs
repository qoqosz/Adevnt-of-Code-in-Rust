use aoc::{aoc, aoc_input};

#[inline]
fn is_unique(vec: &[u8]) -> bool {
    vec.iter()
        .all(|elem| vec.iter().filter(|x| *x == elem).count() == 1)
}

#[inline]
fn find<const N: usize>(txt: &[u8]) -> Option<usize> {
    let mut deque: [u8; N] = txt[..N].try_into().unwrap();
    (N..)
        .find(|&i| {
            deque[i % N] = txt[i];
            is_unique(&deque)
        })
        .map(|i| i + 1)
}

#[aoc(2022, 6)]
pub fn main() {
    let data = aoc_input!(2022, 6).unwrap();
    let bytes = data.as_bytes();

    // Part I
    println!("{}", find::<4>(bytes).unwrap());

    // Part II
    println!("{}", find::<14>(bytes).unwrap());
}
