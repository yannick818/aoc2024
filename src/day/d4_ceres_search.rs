
struct Words {
    lines: Vec<String>,
}

const XMAS: &str = "XMAS";
const SAMX: &str = "SAMX";

pub fn count_xmas(input: &str) -> usize {
    let mut counter = 0;
    let lines: Vec<_> = input.lines().collect();
    for line in &lines {
        counter += line.matches(XMAS).count();
        counter += line.matches(SAMX).count();
    }

    let mut rows = Vec::new();
    for i in 0..lines[0].chars().count() {
        let mut row = String::new();
        for line in &lines {
            row.push(line.chars().nth(i).unwrap());
        }
        rows.push(row);
    } 
    for row in &rows {
        counter += row.matches(XMAS).count();
        counter += row.matches(SAMX).count();
    }

    let mut diagonal = Vec::new();
    let row_cnt = rows.len();
    let col_cnt = lines.len();
    // direction / from left/west
    for start_row in 3..row_cnt {
        let mut diag = String::new();
        let start_col = 0;
        for step in 0..row_cnt.max(col_cnt) {
            let row = match start_row.checked_sub(step) {
                Some(row) => row,
                None => break,
            };
            let col = start_col + step;
            let ch = match rows[row].chars().nth(col) {
                Some(c) => c,
                None => break,
            };
            diag.push(ch);
        }
        diagonal.push(diag);
    }
    //direction / from bottom/south
    for start_col in 1..col_cnt-3 {
        let mut diag = String::new();
        let start_row = row_cnt - 1;
        for step in 0..row_cnt.max(col_cnt) {
            let row = match start_row.checked_sub(step) {
                Some(row) => row,
                None => break,
            }; 
            let col = start_col + step;
            let ch = match rows[row].chars().nth(col) {
                Some(c) => c,
                None => break,
            };
            diag.push(ch);
        }
        diagonal.push(diag);
    }
    // direction \ from right/east
    for start_row in 3..row_cnt {
        let mut diag = String::new();
        let start_col = col_cnt - 1;
        for step in 0..row_cnt.max(col_cnt) {
            let row = match start_row.checked_sub(step) {
                Some(row) => row,
                None => break,
            };
            let col = match start_col.checked_sub(step) {
                Some(col) => col,
                None => break,
            };
            let ch = rows[row].chars().nth(col).unwrap();
            diag.push(ch);
        }
        diagonal.push(diag);
    }
    // direction \ from bottom/south
    for start_col in 0..col_cnt-1 {
        let mut diag = String::new();
        let start_row = row_cnt - 1;
        for step in 0..row_cnt.max(col_cnt) {
            let row = match start_row.checked_sub(step) {
                Some(row) => row,
                None => break,
            }; 
            let col = match start_col.checked_sub(step) {
                Some(col) => col,
                None => break,
            };
            let ch = rows[row].chars().nth(col).unwrap();
            diag.push(ch);
        }
        diagonal.push(diag);
    }

    for diag in diagonal {
        counter += diag.matches(XMAS).count();
        counter += diag.matches(SAMX).count();
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
    }
}