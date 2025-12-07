pub struct IntcodeComputer;

impl IntcodeComputer {
    pub fn eval(input: &mut [usize]) -> usize {
        let mut i = 0;

        loop {
            let op = input[i];

            if op == 99 {
                break;
            }

            let (j, k, m) = (input[i + 1], input[i + 2], input[i + 3]);
            input[m] = match op {
                1 => input[j] + input[k],
                2 => input[j] * input[k],
                _ => unreachable!("invalid op code"),
            };

            i += 4;
        }

        input[0]
    }
}
