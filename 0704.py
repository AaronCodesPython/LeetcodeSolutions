solved =  False
def checkValidString(s: str) -> bool:
    
    def dp(i,s,):
        global solved
        if not solved:
            open = 0
            for i in range(len(s)):
                if(open < 0):
                    return False
            
                if s[i] == "*":
                    s[i] = "("
                    dp(i, s)
                    if open > 0:
                        s[i] = ")"
                        dp(i, s)
                    s[i] = " "
                    dp(i, s)
                elif s[i] == "(":
                    open+=1
                elif s[i] == ")":
                    open-=1
            if(open == 0):
                
                solved = True
                
    dp(0,[x for x in s]) 
    global solved   
    return solved
print(checkValidString("*(*)(*))((*)*)))(*)())*())()(()*)*)****)())(()()*(*(*())()((())))*()****)(*(()))((*()*(**(*()*)*()************"))
#(**(*()**()**((**(*)
#()(()())(())((())())

#