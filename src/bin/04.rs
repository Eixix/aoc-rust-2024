advent_of_code::solution!(4);


fn search_word_in_matrix(
    word: &str,
    matrix: &Vec<Vec<char>>,
    dir: (i32, i32),
    start: (i32, i32),
) -> bool {
    let (mut x, mut y) = start;
    for c in word.chars() {
        if x < 0
            || y < 0
            || x >= matrix.len() as i32
            || y >= matrix[0].len() as i32
            || matrix[x as usize][y as usize] != c
        {
            return false;
        }
        // Move in the specified direction
        x += dir.0;
        y += dir.1;
    }
    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut matrix: Vec<Vec<char>> = vec![];
    for line in input.lines() {
        let row: Vec<char> = line.chars().collect();
        matrix.push(row.clone());
    }

    // All eight directions
    let directions: [(i32, i32); 8] = [(0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1)];
    let mut sum = 0;

    for (x, row) in matrix.iter().enumerate() {
        for (y, c) in row.iter().enumerate() {
            if *c == 'X' {
                for direction in directions.iter() {
                    sum += if search_word_in_matrix("XMAS", &matrix, *direction, (x as i32, y as i32)) { 1 } else { 0 };
                }
            }
        }
    }

    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut matrix: Vec<Vec<char>> = vec![];
    for line in input.lines() {
        let row: Vec<char> = line.chars().collect();
        matrix.push(row.clone());
    }

    // Diagonal directions
    let directions: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
    let sum = 0;

    for (x, row) in matrix.iter().enumerate() {
        for (y, c) in row.iter().enumerate() {

        }
    }

    Some(sum as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
