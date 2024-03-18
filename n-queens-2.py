
n = 0
def totalNQueens(n: int) -> int:
    def changeGrid(grid,row,col):
        for i in range(n):
            grid[row][i] = 1
            grid[i][col] = 1
            
            a = row+i < n
            b = col+i < n
            c = row-i >= 0
            d = col-i >= 0
            if a and b:  
                grid[row+i][col+i] = 1
            if c and d:
                grid[row-i][col-i] = 1
            if a and d:
                grid[row+i][col-i] = 1
            if b and c:
                grid[row-i][col+i] = 1  
            #grid[row][row]
        return grid
        # O(n)
    def dfs(np,cr,grid):
        if np == n:
            nonlocal n
            n+=1
            return
        for pos in range(n):
            if grid[cr][pos] != 1:
                newGrid = [row[:] for row in grid]  
                newGrid[cr][pos] = 1
                newGrid = changeGrid(newGrid, cr, pos).copy()
                dfs(np+1,cr+1, newGrid)
            
    grid = [[0]*n for _ in range(n)]
    

    dfs(0,0,grid)
    return n

totalNQueens(6)

'''
row-i col-i >= 0
row+i col+i
row-i col+1
row+i col-1  
[0, 1, 0, 0, 1, 0, 0, 1], 
[0, 0, 1, 0, 1, 0, 1, 0], 
[0, 0, 0, 1, 1, 1, 0, 0], 
[1, 1, 1, 1, 1, 1, 1, 1], 
[0, 0, 0, 1, 1, 1, 0, 0], 
[0, 0, 1, 0, 1, 0, 1, 0], 
[0, 1, 0, 0, 1, 0, 0, 1], 
[1, 0, 0, 0, 1, 0, 0, 0]]
4 4

'''