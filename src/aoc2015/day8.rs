use aoc::aoc_input;

pub trait InMemorySize {
    fn in_mem_size(&self) -> usize;
}

impl InMemorySize for &str {
    fn in_mem_size(&self) -> usize {
        let mut input = self.chars().peekable();
        let mut size: usize = 0;

        loop {
            match input.next() {
                Some('"') => {}
                Some('\\') => match input.peek() {
                    Some('x') => {
                        input.next();
                        input.next();
                        input.next();
                        size += 1;
                    }
                    _ => {
                        input.next();
                        size += 1
                    }
                },
                Some(_) => size += 1,
                _ => break,
            }
        }
        size
    }
}

fn encoded_len<T: Into<String>>(text: T) -> usize {
    text.into().chars().fold(2, |sum, ch| match ch {
        '"' | '\\' => sum + 2,
        _ => sum + 1,
    })
}

pub fn main() {
    let data = aoc_input!(2015, 8).unwrap();
    let lines: Vec<_> = data.lines().filter(|&x| !x.is_empty()).collect();

    // Part I
    let mut n_code = 0;
    let mut n_mem = 0;
    let mut n_enc = 0;

    for line in lines.into_iter() {
        n_code += line.len();
        n_mem += line.in_mem_size();
        n_enc += encoded_len(line);
    }
    println!("{}", n_code - n_mem);

    // Part II
    println!("{}", n_enc - n_code);
}
