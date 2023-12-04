use aoc::aoc_input;

#[derive(Debug)]
struct Box {
    l: usize,
    w: usize,
    h: usize,
}

impl From<&str> for Box {
    fn from(dims: &str) -> Self {
        // Convert input line into a Box object.
        let vec: Vec<usize> = dims
            .split('x')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        Box::from(&vec)
    }
}

impl From<&Vec<usize>> for Box {
    fn from(value: &Vec<usize>) -> Self {
        assert_eq!(value.len(), 3);
        Box {
            l: value[0],
            w: value[1],
            h: value[2],
        }
    }
}

impl Box {
    fn surface_area(&self) -> usize {
        // Total surface area.
        2 * self.sides_area().iter().sum::<usize>()
    }

    fn sides_area(&self) -> Vec<usize> {
        // Area of 3 different sides.
        vec![self.l * self.w, self.w * self.h, self.h * self.l]
    }

    fn wrapping_area(&self) -> usize {
        // How much wrapping paper is used to wrap a box.
        let extra: usize = *self.sides_area().iter().min().unwrap();
        self.surface_area() + extra
    }

    fn faces_perimeters(&self) -> Vec<usize> {
        // Perimeters of 3 different faces.
        vec![
            2 * (self.l + self.w),
            2 * (self.l + self.h),
            2 * (self.w + self.h),
        ]
    }

    fn volume(&self) -> usize {
        // Box volume.
        self.l * self.w * self.h
    }

    fn ribbon_len(&self) -> usize {
        // Ribbon length to pack a gift.
        self.faces_perimeters().iter().min().unwrap() + self.volume()
    }
}

fn total_wrapping_area(boxes: &[Box]) -> usize {
    boxes.iter().map(|b| b.wrapping_area()).sum()
}

fn total_ribbon_len(boxes: &[Box]) -> usize {
    boxes.iter().map(|b| b.ribbon_len()).sum()
}

pub fn main() {
    let data = aoc_input!(2015, 2).unwrap();
    let boxes: Vec<Box> = data
        .lines()
        .filter(|&x| !x.is_empty())
        .map(Box::from)
        .collect();

    // Part I
    println!("{:?}", total_wrapping_area(&boxes));

    // Part II
    println!("{:?}", total_ribbon_len(&boxes));
}
