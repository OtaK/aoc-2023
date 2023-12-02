const PUZZLE_INPUT: &'static str = include_str!("input.txt");

fn gather_matches(s: &str) -> Vec<(usize, u64)> {
    s.match_indices("one")
        .map(|(idx, _s)| (idx, 1))
        .chain(s.match_indices("two").map(|(idx, _s)| (idx, 2)))
        .chain(s.match_indices("three").map(|(idx, _s)| (idx, 3)))
        .chain(s.match_indices("four").map(|(idx, _s)| (idx, 4)))
        .chain(s.match_indices("five").map(|(idx, _s)| (idx, 5)))
        .chain(s.match_indices("six").map(|(idx, _s)| (idx, 6)))
        .chain(s.match_indices("seven").map(|(idx, _s)| (idx, 7)))
        .chain(s.match_indices("eight").map(|(idx, _s)| (idx, 8)))
        .chain(s.match_indices("nine").map(|(idx, _s)| (idx, 9)))
        .chain(
            s.match_indices(|c: char| c.is_digit(10))
                .map(|(idx, s)| (idx, s.parse::<u64>().unwrap())),
        )
        .collect()
}

fn main() {
    let sum = PUZZLE_INPUT.lines().fold(0u64, |acc, line| {
        let mut matches = gather_matches(line);
        matches.sort_by_key(|(idx, _)| *idx);
        let first = matches.first().map(|v| v.1).unwrap();
        let last = matches.last().map(|v| v.1).unwrap_or(first);
        acc + format!("{first}{last}").parse::<u64>().unwrap()
    });

    println!("Calibration sum: {sum}");
}
