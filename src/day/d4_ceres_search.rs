pub fn count_xmas(input: &str) -> usize {
    let mut counter = 0;
    let rows: Vec<_> = input.lines().collect();
    for (r, row) in rows.iter().enumerate() {
        for (c, ch) in row.chars().enumerate() {
            if ch == 'X' {
                let r = r as isize;
                let c = c as isize;
                #[allow(clippy::identity_op)] //for optics
                [
                    [(r + 1, c + 0), (r + 2, c + 0), (r + 3, c + 0)],
                    [(r - 1, c + 0), (r - 2, c + 0), (r - 3, c + 0)],
                    [(r + 0, c + 1), (r + 0, c + 2), (r + 0, c + 3)],
                    [(r + 0, c - 1), (r + 0, c - 2), (r + 0, c - 3)],
                    [(r + 1, c + 1), (r + 2, c + 2), (r + 3, c + 3)],
                    [(r - 1, c - 1), (r - 2, c - 2), (r - 3, c - 3)],
                    [(r + 1, c - 1), (r + 2, c - 2), (r + 3, c - 3)],
                    [(r - 1, c + 1), (r - 2, c + 2), (r - 3, c + 3)],
                ]
                .into_iter()
                .for_each(|idx| {
                    let mut iter = idx.iter().map(|(r, c)| {
                        if *r < 0 || *c < 0 {
                            return None;
                        }
                        rows.get(*r as usize)
                            .and_then(|row| row.chars().nth(*c as usize))
                    });
                    let m = iter.next().unwrap();
                    let a = iter.next().unwrap();
                    let s = iter.next().unwrap();
                    if let (Some('M'), Some('A'), Some('S')) = (m, a, s) {
                        counter += 1
                    }
                });
            }
        }
    }
    counter
}

pub fn count_x_mas(input: &str) -> usize {
    let rows: Vec<_> = input.lines().collect();
    let mut counter = 0;
    let row_cnt = rows.len();
    let col_cnt = rows[0].chars().count();
    for (r, row) in rows.iter().enumerate().skip(1).take(row_cnt - 2) {
        for (c, ch) in row.chars().enumerate().skip(1).take(col_cnt - 2) {
            if ch == 'A' {
                let neighbours = [
                    (r - 1, c - 1),
                    (r + 1, c + 1),
                    (r - 1, c + 1),
                    (r + 1, c - 1),
                ]
                .map(|(r, c)| rows[r].chars().nth(c).unwrap());
                match neighbours {
                    ['M', 'S', 'M', 'S']
                    | ['M', 'S', 'S', 'M']
                    | ['S', 'M', 'M', 'S']
                    | ['S', 'M', 'S', 'M'] => counter += 1,
                    _ => {}
                }
            }
        }
    }
    counter
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_count_xmas() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(18, count_xmas(input));
        assert_eq!(9, count_x_mas(input));
    }
}
