const readline = require('readline');

function dfs_rec(grid, i, j) {
    const directions = [[0, 1], [0, -1], [1, 0], [-1, 0]];
    const stack = [[i, j]];
    const visited = [];

    while (stack.length > 0) {
        const [x, y] = stack.pop();
        visited.push([x, y]);

        directions.forEach(([dx, dy]) => {
            const nx = x + dx;
            const ny = y + dy;

            if (nx >= 0 && nx < grid.length && ny >= 0 && ny < grid[0].length &&
                grid[nx][ny] === "." && !visited.some(([a, b]) => a === nx && b === ny)) {
                stack.push([nx, ny]);
                grid[nx][ny] = "#";
            }
        });
    }

    return grid;
}

function dfs_iter(grid) {
    let count = 0;

    grid.forEach((row, rowIndex) => {
        row.forEach((cell, cellIndex) => {
            if (cell === ".") {
                grid[rowIndex][cellIndex] = '#';
                grid = dfs_rec(grid, rowIndex, cellIndex);
                count++;
            }
        });
    });

    console.log(count);
}

function main() {
    const r1 = readline.createInterface({
        input: process.stdin,
        output: process.stdout
    });

    r1.question("Enter size of grid: ", (size) => {
        size = size.split(" ").map(Number);

        let grid = [];
        let rowCount = 0;

        function askRow() {
            if (rowCount < size[0]) {
                r1.question(`Enter row ${rowCount + 1} of grid: `, (row) => {
                    grid.push(row.split("")); // Split row into individual characters
                    rowCount++;
                    askRow(); 
                });
            } else {
                r1.close();
                dfs_iter(grid);
            }
        }

        askRow();
    });
}

main();
