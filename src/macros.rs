#![allow(unused_macros)]
macro_rules! get4dir {
    ($x:expr, $y:expr, $h:expr, $w:expr) => {
        [(0, -1), (1, 0), (0, 1), (-1, 0)]
        .iter()
        .map(|(dx, dy)| ($x as i64 + dx, $y as i64 + dy))
        .filter(|&(x, y)| 0 <= x && x < $w as i64 && 0 <= y && y < $h as i64)
        .map(|(x, y)| (x as usize, y as usize))

    };
}