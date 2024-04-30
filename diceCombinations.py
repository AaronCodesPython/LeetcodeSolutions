

from collections import defaultdict


n = int(input())
MOD = 10**9+7
memo = defaultdict(int)
memo[0] = 1

for i in range(1,n+1):
    for j in range(1,7):
        if i-j >= 0:
            memo[i] = (memo[i]+memo[i-j]) % MOD 
print(memo[n])