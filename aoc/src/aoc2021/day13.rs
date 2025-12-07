use aoc::{aoc, aoc_input};
use itertools::Itertools;
use num::Integer;
use std::{fmt::Display, num::ParseIntError};

enum Axis {
    X,
    Y,
}

impl TryFrom<&str> for Axis {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "x" => Ok(Axis::X),
            "y" => Ok(Axis::Y),
            _ => unimplemented!(),
        }
    }
}

#[derive(Clone)]
struct BoolArray2 {
    shape: (usize, usize),
    data: Vec<bool>,
}

impl BoolArray2 {
    fn zeros(h: usize, w: usize) -> Self {
        Self {
            shape: (h, w),
            data: vec![false; w * h],
        }
    }

    fn set(&mut self, h: usize, w: usize) {
        self.data[w + h * self.shape.1] = true;
    }

    fn tr(&self) -> Self {
        let (h, w) = self.shape;
        let data = (0..w)
            .cartesian_product(0..h)
            .map(|(i, j)| self.data[i + j * w])
            .collect();

        Self {
            shape: (w, h),
            data,
        }
    }

    fn fold(&self, line: usize, axis: Axis) -> Self {
        match axis {
            Axis::X => self.tr()._fold_y(line).tr(),
            Axis::Y => self._fold_y(line),
        }
    }

    fn _fold_y(&self, line: usize) -> Self {
        let w = self.shape.1;
        let mut data = self.data[..line * w].to_vec();
        let out = &self.data[(line + 1) * w..];

        let (n, m) = (data.len(), out.len());

        for i in 0..m {
            let (j, k) = i.div_rem(&w);
            data[n - (j + 1) * w + k] |= out[i];
        }

        Self {
            shape: (line, w),
            data,
        }
    }
}

impl Display for BoolArray2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (filled, empty) = ('🎁', '🎄');
        let (h, w) = self.shape;

        for i in 0..h {
            let line = self.data[i * w..(i + 1) * w]
                .iter()
                .map(|ch| match ch {
                    true => filled,
                    false => empty,
                })
                .collect::<String>();
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}

fn get_sheet(dots: &[(i32, i32)]) -> Option<BoolArray2> {
    let h = dots.iter().max_by_key(|x| x.0)?.0 + 1;
    let w = dots.iter().max_by_key(|x| x.1)?.1 + 1;

    let mut sheet = BoolArray2::zeros(h as usize, w as usize);

    for &(i, j) in dots {
        sheet.set(i as usize, j as usize);
    }

    Some(sheet.tr())
}

fn apply_folds<'a>(
    sheet: &'a BoolArray2,
    folds: impl IntoIterator<Item = (&'a str, i32)>,
) -> Result<BoolArray2, ()> {
    let mut sheet = sheet.clone();

    for (axis, line) in folds {
        sheet = sheet.fold(line as usize, Axis::try_from(axis)?)
    }

    Ok(sheet)
}

fn parse(data: &str) -> Result<(Vec<(i32, i32)>, Vec<(&str, i32)>), ParseIntError> {
    let (mut dots, mut folds) = (vec![], vec![]);
    let mut is_fold = false;

    for line in data.trim().lines() {
        let line = line.trim();

        if line.is_empty() {
            is_fold = true;
            continue;
        }
        if !is_fold {
            if let Some((x, y)) = line.split_once(',') {
                dots.push((x.parse::<i32>()?, y.parse::<i32>()?));
            }
        } else {
            if let Some((x, y)) = line
                .split_whitespace()
                .last()
                .map(|l| l.split_once('='))
                .flatten()
            {
                folds.push((x, y.parse::<i32>()?));
            }
        }
    }

    Ok((dots, folds))
}

#[aoc(2021, 13)]
pub fn main() {
    let data = aoc_input!(2021, 13).unwrap();
    let (dots, folds) = parse(&data).unwrap();
    let sheet = get_sheet(&dots).unwrap();

    // Part I
    let res = apply_folds(&sheet, [folds[0]]).unwrap();
    println!("{}", res.data.iter().filter(|x| **x).count());

    // Part II
    let res = apply_folds(&sheet, folds).unwrap();
    println!("{res}");
}
