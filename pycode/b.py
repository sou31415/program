d = '.ABCDEFGHIJKLMNOPQRSTUVWXYZ'
a , b = map(int ,input().split())
for i in range(a):
    c = list(map(int ,input().split(' ')))
    for j in c:
        print(d[j] , end='')
    print()
