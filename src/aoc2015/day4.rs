use rayon::prelude::*;

fn hash(secret: &[u8], number: usize) -> md5::Digest {
    let mut data: Vec<u8> = secret.to_vec();
    data.extend_from_slice(number.to_string().as_bytes());
    md5::compute(&data)
}

fn check5(digest: &md5::Digest) -> bool {
    digest.0[0..2] == [0, 0] && (digest.0[2] & 0xF0 == 0)
}

fn check6(digest: &md5::Digest) -> bool {
    digest.0[0..3] == [0, 0, 0]
}

fn find(secret: &[u8], f: &(dyn Fn(&md5::Digest) -> bool + Sync)) -> Option<usize> {
    (0..10000000usize)
        .into_par_iter()
        .find_map_first(|i| match f(&hash(secret, i)) {
            true => Some(i),
            false => None,
        })
}

fn main() {
    // Problem 1
    let secret_key = "yzbqklnj".as_bytes();
    println!("{}", find(secret_key, &check5).unwrap());

    // Problem 2
    println!("{}", find(secret_key, &check6).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let h = hash(b"abcdef", 609043);
        assert!(check5(&h));
    }
}
