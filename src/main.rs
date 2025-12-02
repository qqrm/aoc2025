fn main() {}

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
