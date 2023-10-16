use md5;

fn hash(secret: &str, number: usize) -> md5::Digest {
    let data = format!("{}{}", secret, number);
    md5::compute(data)
}

fn check(digest: &md5::Digest, beginning: &str) -> bool {
    let out = format!("{:x}", digest);
    out.starts_with(beginning)
}

fn check5(digest: &md5::Digest) -> bool {
    check(digest, "00000")
}

fn check6(digest: &md5::Digest) -> bool {
    check(digest, "000000")
}

fn find(secret: &str, f: &dyn Fn(&md5::Digest) -> bool) -> usize {
    let mut n: usize = 0;

    loop {
        let h = hash(secret, n);
        let is_found = f(&h);

        if is_found {
            return n;
        }
        n += 1;
    }
}

fn main() {
    // Problem 1
    let secret_key = "yzbqklnj";
    println!("{}", find(&secret_key, &check5));

    // Problem 2
    println!("{}", find(&secret_key, &check6));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let h = hash("abcdef", 609043);
        assert!(check5(&h));
    }
}
