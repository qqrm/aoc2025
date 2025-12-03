#![feature(stmt_expr_attributes)]

fn main() {}

// #[test]
// fn test_x_day_part_one() {
//     const INPUT: &str = include_str!("../input");

//     let res = unimplemented!();
//     dbg!(res);
// }

// #[test]
// fn test_x_day_part_two() {
//     const INPUT: &str = include_str!("../input");

//     let res = unimplemented!();
//     dbg!(res);
// }

#[test]
fn test_first_day_part_one() {
    // const INPUT = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";
    const INPUT: &str = include_str!("../input");

    let res = INPUT
        .split('\n')
        .filter(|s| !s.is_empty())
        .fold((50, 0), |(pos, zeros), s| {
            let (d, c) = s.split_at(1);
            let c = c.parse::<i32>().unwrap();
            let pos = (pos + if d == "L" { -c } else { c }).rem_euclid(100);
            (pos, zeros + i32::from(pos == 0))
        });
    dbg!(res.1);
}

#[test]
fn test_first_day_part_two() {
    const INPUT: &str = include_str!("../input");

    #[rustfmt::skip]
    let (_, ans) = INPUT
        .lines()
        .fold((50_i32, 0_i32), |(p, h), s| {
            let (d, x) = s.split_at(1);
            let n = x.parse::<i32>().unwrap();
            let m = p.rem_euclid(100);
            let f = if m == 0 { 100 } else if d == "R" { 100 - m } else { m };
            let h = h + if n < f { 0 } else { 1 + (n - f) / 100 };
            let p = p + if d == "R" { n } else { -n };
            (p, h)
        });

    dbg!(ans);
}

#[test]
fn test_second_day_part_one() {
    // const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862";
    const INPUT: &str = include_str!("../input");

    let res: u64 = INPUT
        .lines()
        .next()
        .unwrap()
        .split(|c| c == ',' || c == '\n')
        .map(|d| {
            let (l, r) = d.split_once('-').unwrap();
            let (l, r) = (l.parse::<u64>().unwrap(), r.parse::<u64>().unwrap());
            let max_len = r.to_string().len() as u32;
            (1..=max_len / 2)
                .map(|k| {
                    let m = 10u64.pow(k) + 1;
                    let h1 = 10u64.pow(k - 1).max((l + m - 1) / m);
                    let h2 = (10u64.pow(k) - 1).min(r / m);
                    #[rustfmt::skip]
                    if h1 > h2 { 0 } else { m * (h2 - h1 + 1) * (h1 + h2) / 2 }
                })
                .sum::<u64>()
        })
        .sum();

    // assert_eq!(1227775554, res);
    dbg!(res);
}

#[test]
fn test_second_day_part_two() {
    use std::collections::HashSet;
    const INPUT: &str = include_str!("../input");

    let res: u64 = INPUT
        .lines()
        .next()
        .unwrap()
        .split(|c| c == ',' || c == '\n')
        .filter(|d| !d.is_empty())
        .map(|d| {
            let (l, r) = d.split_once('-').unwrap();
            let (l, r) = (l.parse::<u64>().unwrap(), r.parse::<u64>().unwrap());
            let max_len = r.to_string().len() as u32;
            (1..=max_len / 2)
                .flat_map(|k| (2..=max_len / k).map(move |t| (k, t)))
                .flat_map(|(k, t)| {
                    let p = 10u64.pow(k);
                    let s = (10u64.pow(k * t) - 1) / (p - 1);
                    let a = ((l + s - 1) / s).max(10u64.pow(k - 1));
                    let b = (r / s).min(p - 1);
                    (a..=b).map(move |h| h * s)
                })
                .filter(|&n| n >= l && n <= r)
                .collect::<HashSet<_>>()
                .into_iter()
                .sum::<u64>()
        })
        .sum();

    dbg!(res);
}
