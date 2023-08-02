a = list(map(int,input().split()))
f = True
for i in range(7):
    if a[i] > a[i+1]:
        f = False

for i in range(8):
    if a[i] < 100 or a[i] > 675:
        f = False

for i in range(8):
    if a[i] % 25 != 0:
        f = False

if f:
    print("Yes")
else:
    print("No")
