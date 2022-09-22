pub fn solve_first(s: &str) -> usize {
    let nums: Vec<u32> = s.split("\n")
        .map(|s| s.parse().unwrap())
        .collect();
    1 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_first() {
        let input = "1721
979
366
299
675
1456";
        assert_eq!(solve_first(input), 2)
    }
}
