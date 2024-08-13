use std::io;

fn dfs_rec(grid: &mut Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, i: usize, j: usize) {
    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut stack = vec![(i, j)];

    while let Some((x, y)) = stack.pop() {
        if visited[x][y] {
            continue;
        }

        visited[x][y] = true;
        grid[x][y] = '#'; // Marking the cell as visited by changing its value

        for &(dx, dy) in directions.iter() {
            let nx = (x as isize + dx) as usize;
            let ny = (y as isize + dy) as usize;

            if nx < grid.len() && ny < grid[0].len() && grid[nx][ny] == '.' && !visited[nx][ny] {
                stack.push((nx, ny));
            }
        }
    }
}

fn dfs(grid: &mut Vec<Vec<char>>, max_x: usize, max_y: usize) -> usize {
    let mut visited = vec![vec![false; max_y]; max_x];
    let mut count = 0;

    for i in 0..max_x {
        for j in 0..max_y {
            if grid[i][j] == '.' && !visited[i][j] {
                dfs_rec(grid, &mut visited, i, j);
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let mut size_in = String::new();
    println!("Podaj rozmiar: ");

    io::stdin().read_line(&mut size_in)
        .expect("Błąd przy wczytywaniu danych");

    let size: Vec<usize> = size_in
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Błędny rozmiar"))
        .collect();

    let mut grid = Vec::new();

    println!("Podaj wiersze:");

    for _ in 0..size[0] {
        let mut row_in = String::new();
        io::stdin().read_line(&mut row_in)
            .expect("Błąd przy wczytywaniu danych");

        let row: Vec<char> = row_in
            .trim()
            .chars()
            .collect();

        grid.push(row);
    }

    let rooms_count = dfs(&mut grid, size[0], size[1]);
    println!("Liczba pokoi: {}", rooms_count);
}
