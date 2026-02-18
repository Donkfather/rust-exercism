pub fn annotate(garden: &[&str]) -> Vec<String> {
    if garden.is_empty() {
        return vec![];
    }

    let rows = garden.len();
    let cols = garden.first().map_or(0, |row| row.len());

    if cols == 0 {
        return vec![String::new()];
    }

    let garden_bytes: Vec<&[u8]> = garden.iter().map(|row| row.as_bytes()).collect();

    let mut result: Vec<Vec<u8>> = garden_bytes.iter().map(|row| row.to_vec()).collect();

    for row in 0..rows {
        for col in 0..cols {
            if garden_bytes[row][col] == b' ' {
                let count = count_adjacent_flowers(&garden_bytes, row, col, rows, cols);
                if count > 0 {
                    result[row][col] = b'0' + count;
                }
            }
        }
    }

    result
        .into_iter()
        .map(|row| String::from_utf8(row).unwrap())
        .collect()
}

fn count_adjacent_flowers(
    garden: &[&[u8]],
    row: usize,
    col: usize,
    rows: usize,
    cols: usize,
) -> u8 {
    let mut count = 0;

    for dr in [-1, 0, 1] {
        for dc in [-1, 0, 1] {
            if dr == 0 && dc == 0 {
                continue;
            }

            let new_row = row as i32 + dr;
            let new_col = col as i32 + dc;

            if new_row >= 0
                && new_row < rows as i32
                && new_col >= 0
                && new_col < cols as i32
                && garden[new_row as usize][new_col as usize] == b'*'
            {
                count += 1;
            }
        }
    }

    count
}
