"""
最小二乗法を求めるscript
こちらはXiとYiの値を交互に入力する仕様になっているが
近いうちにcsvから値を読み取れるようにしたい(delimiterなし)
現時点ではひとつずつ入力する仕様となっている
"""
import csv
x = [] #x成分を格納する空のリスト作成
y = [] #y成分を格納する空のリスト作成
print('Please select mode.(i / r)')
mode = input()
print("Input Repeat time:") 
time = int(input()) #プロットする点の数の入力
denominator = 0
numerator = 0
x_total = 0
y_total = 0
d_result = 0
n_result = 0

for i in range(time):
    print("x"+str(i))
    a = float(input()) #i番目のx成分を入力
    x.append(a) #入力した値を空のリストxのi番目に追加していく
    print("y"+str(i))
    b = float(input()) #xと同様のことをy成分でもしていく
    y.append(b)

"""
ここからは公式に則って計算していく、
シグマの部分をforで置き換えて一つずつ細かく計算していく
"""

for i in range(time):
    x_total += x[i] #入力されたx成分を全て足していく(sigma)

x_total /= time # avg
for i in range(time):
    y_total += y[i] #xと同様

y_total /= time #avg
for i in range(time):
    denominator += (x[i] * y[i]) - (time * x_total * y_total)

for i in range(time):
    numerator += (x[i] ** 2) - (time * (x_total ** 2))

d_result = denominator / numerator
n_result = 0 - (d_result * x_total) + y_total
print("a(傾き): "+str(d_result)) #傾き
print("b(切片): "+str(n_result)) #切片
print("\n") #何となく改行