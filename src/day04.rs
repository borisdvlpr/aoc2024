use crate::helpers::read_lines;

pub fn run() {
    if let Ok(lines) = read_lines("./data.txt") {
        let lines: Vec<String> = lines.flatten().collect();
        let grid: Vec<Vec<char>> = lines
            .into_iter()
            .map(|line| line.chars().collect())
            .collect();

        let rows = grid.len();
        let cols = grid[0].len();
        let mut count = 0;

        for i in 1..rows - 1 {
            for j in 1..cols - 1 {
                if grid[i][j] != 'A' {
                    continue;
                }

                // m on the top
                if i + 1 < rows
                    && j > 0
                    && j + 1 < cols
                    && grid[i - 1][j - 1] == 'M'
                    && grid[i - 1][j + 1] == 'M'
                    && grid[i + 1][j - 1] == 'S'
                    && grid[i + 1][j + 1] == 'S'
                {
                    count += 1;
                }

                // m on the bottom
                if i + 1 < rows
                    && j > 0
                    && j + 1 < cols
                    && grid[i - 1][j - 1] == 'S'
                    && grid[i - 1][j + 1] == 'S'
                    && grid[i + 1][j - 1] == 'M'
                    && grid[i + 1][j + 1] == 'M'
                {
                    count += 1;
                }

                // m on the left
                if i > 0
                    && i + 1 < rows
                    && j + 1 < cols
                    && grid[i - 1][j - 1] == 'M'
                    && grid[i + 1][j - 1] == 'M'
                    && grid[i - 1][j + 1] == 'S'
                    && grid[i + 1][j + 1] == 'S'
                {
                    count += 1;
                }

                // m on the right
                if i > 0
                    && i + 1 < rows
                    && j > 0
                    && grid[i - 1][j + 1] == 'M'
                    && grid[i + 1][j + 1] == 'M'
                    && grid[i - 1][j - 1] == 'S'
                    && grid[i + 1][j - 1] == 'S'
                {
                    count += 1;
                }
            }
        }

        println!("'MAS' occurencies: {}", count);
    } else {
        println!("error: unable to read data file.");
    }
}
