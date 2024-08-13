"""
UWAGA

KOD MA RUN TIME ERROR NA CSES PZDR
"""

def dfs(grid, i ,j) -> list[list[str]]:
    directions = [(0, 1), (0, -1), (1, 0), (-1, 0)]
    visited = set()
    stack = [(i, j)]
    while stack:
        x, y = stack.pop()
        visited.add((x, y))
        for dx, dy in directions:
            nx, ny = x + dx, y + dy
            if 0 <= nx < len(grid) and 0 <= ny < len(grid[nx]) and grid[nx][ny] == '.' and (nx, ny) not in visited:
                stack.append((nx, ny))
                grid[nx][ny] = '#'
    return grid

if __name__ == "__main__":
    n, m = map(int, input().split())
    grid = [list(input().strip()) for _ in range(n)]

    count = 0
    for i in range(len(grid)):
        for j in range(len(grid[i])):
            if grid[i][j] == '.':            
                grid[i][j] = '#'
                grid = dfs(grid, i, j)
                count += 1

    print(count)