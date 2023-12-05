pub fn solve() -> String {
    let input = include_str!("input.txt");
    let mut res = 0;

    for line in input.lines() {
        let left = line
            .find(char::is_numeric)
            .and_then(|i| line.chars().nth(i));
        let right = line
            .rfind(char::is_numeric)
            .and_then(|i| line.chars().nth(i));

        match left.zip(right) {
            Some((l, r)) => {
                let digits: u32 = format!("{l}{r}").parse().unwrap();
                res += digits;
            }
            _ => panic!("Invalid line: {line}"),
        };
    }

    return res.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solve();
        assert_eq!(result, "53921");
    }
}
