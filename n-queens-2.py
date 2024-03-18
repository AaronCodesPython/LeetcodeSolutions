

def totalNQueens(n: int) -> int:
    def changeGrid(grid,row,column):
        for i in range(n):
            grid[row][i] = 1
            grid[i][row] = 1
            #grid[row][row]
        return grid
        # O(n)
    def dfs(np,cr,grid):
        if np == n:
            print("Placed all")
            return
        for pos in range(n):
            if grid[cr][n] != 1:
                grid[cr][n] = 1
                dfs(np+1,cr+1, grid)
            
    grid = [[0]*n for _ in range(n)]
    changeGrid(grid, 3,4)
    print(grid)
    return 0

totalNQueens(8)