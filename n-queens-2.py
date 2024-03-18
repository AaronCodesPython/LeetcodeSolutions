

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
    res = 0
    def dfs(np,cr,grid):
        if np == n:
            nonlocal res
            res+=1
            return
        for pos in range(n):
            if grid[cr][pos] != 1:
                newGrid = [row[:] for row in grid]  
                newGrid[cr][pos] = 1
                newGrid = changeGrid(newGrid, cr, pos).copy()
                dfs(np+1,cr+1, newGrid)
            
    grid = [[0]*n for _ in range(n)]
    

    dfs(0,0,grid)
    return res

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

# better solution:
def totalNQueens_optimized(n):
    col = set()
    posDiag = set()
    negDiag = set()

    res = 0
    def backtrack(r): # r = row
        if r == n:
            nonlocal res
            res+=1
            return
        for c in range(n):
            if c in col or (r+c) in posDiag or (r-c) in negDiag:
                continue
            col.add(c)
            posDiag.add(r+c)
            negDiag.add(r-c)
            backtrack(r+1)

            #backtrack
            col.remove(c)
            posDiag.remove(r+c)
            negDiag.remove(r-c)
    backtrack(0)
    return res

#print(totalNQueens(12)) -> 4.19 sec
#print(totalNQueens_optimized(12)) -> 1.254 sec
#print(totalNQueens(13)) -> 23.549 seconds
#print(totalNQueens_optimized(13)) -> 6.645 seconds
print(totalNQueens_optimized(14))