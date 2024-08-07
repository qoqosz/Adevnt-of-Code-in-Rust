use aoc::aoc;
use itertools::Itertools;

fn hash(secret: &[u8], number: usize) -> md5::Digest {
    let mut data: Vec<u8> = secret.to_vec();
    data.extend_from_slice(number.to_string().as_bytes());
    md5::compute(&data)
}

fn check5(digest: &md5::Digest) -> bool {
    digest.0[0..2] == [0, 0] && (digest.0[2] & 0xF0 == 0)
}

fn hex_to_char(x: u8) -> char {
    let shift = if x < 10 { 48 } else { 87 };
    return (shift + x) as char;
}

#[aoc(2016, 5)]
pub fn main() {
    let input = "uqwqemis".as_bytes();
    let iter = (0..100_000_000).filter_map(|i| {
        let digest = hash(input, i);

        if check5(&digest) {
            return Some(digest);
        }
        None
    });

    // Part I
    let password = iter
        .clone()
        .take(8)
        .map(|digest| hex_to_char(digest.0[2] & 0x0F))
        .collect::<String>();
    println!("{password}");

    // Part II
    let password = iter
        .filter_map(|digest| match digest.0[2] & 0x0F {
            pos @ 0..8 => Some((pos, digest.0[3] >> 4)),
            _ => None,
        })
        .unique_by(|(pos, _)| *pos)
        .take(8)
        .sorted_unstable_by_key(|(pos, _)| *pos)
        .map(|(_, ch)| hex_to_char(ch))
        .collect::<String>();
    println!("{password}");
}
