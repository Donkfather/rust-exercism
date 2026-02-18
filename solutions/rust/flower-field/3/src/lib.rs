pub fn annotate(garden: &[&str]) -> Vec<String> {
    (0..garden.len())
        .map(|r| {
            (0..garden[r].len())
                .map(|c| get_garden_char(garden, r as i32, c as i32))
                .collect::<String>()
        })
        .collect::<Vec<String>>()
}

fn get_garden_char(garden: &[&str], row: i32, col: i32) -> char {
    match garden[row as usize].as_bytes()[col as usize] {
        b'*' => '*',
        _ => {
            let maxr = garden.len() as i32 - 1;
            let maxc = garden[row as usize].len() as i32 - 1;
            match ((row - 1).max(0)..=(row + 1).min(maxr))
                .flat_map(|newr| {
                    ((col - 1).max(0)..=(col + 1).min(maxc)).map(move |newc| (newr, newc))
                })
                .filter(|(r, c)| garden[*r as usize].as_bytes()[*c as usize] == b'*')
                .count()
            {
                0 => ' ',
                count => (count as u8 + b'0') as char,
            }
        }
    }
}
