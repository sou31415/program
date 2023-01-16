s = input()
i = 0
result = 0
while i < len(s):
    result += ord(s[len(s)-i-1]) * (26**s[len(s)-i-1])
    i+=1
print(result+1)