"""
readモードは近々csvから取得できるようにする(delimiter = ","とする)
listの要素の追加をスムーズにするためにcsvとの紐付けを検討
"""
import random as rd #標準ライブラリrandomをrdとして呼び出す

en = ['microfinance' , 'enable' , 'cycle' , 'poverty' , 'loan' , 'saving' , 'account' , 'seed' , 'basic' , 'financial' , 'isolated' , 'rent' , 'basis' , 'meanwhile' , 'safely' , 'insurance' , 'ruin' , 'innovative' , 'aspect' , 'responsible' , 'repayment' , 'assurance' , 'personalize' , 'succeed' , 'payment' , 'addition' , 'birthplace' , 'model' , 'halve' , 'enlarge' , 'encourage' , 'enrich' , 'rent' , 'weaken' , 'basis' , 'escape' , 'fare' , 'individual' , 'organize' , 'interest' , '~powered' , 'entirely' , 'mobility' , 'venture' , 'urban' , 'clown' , 'goods' , 'storage' , 'refill' , 'manufacturing' , 'attention' , 'pump' , 'amazedly' , 'currently' , 'phase' , 'ordinary' , 'application']
jp = ['小規模金融' , '可能にする' , '免れる' , '貧困' , '融資' , '預金' , '口座' , '種' , '最低限の' , '金融の' , '孤独な' , '借りる' , '根拠' , '一方では' , '安全に' , '保険' , '破産する' , '革新的な' , '側面' , '責務がある' , '返済金' , '保証' , '個人に適用する' , '成功する' , '支払い' , '追加分' , '出身地' , '模範' , '半分にする' , '大きくする' , '励ます' , '豊かにする' , '賃貸する' , '弱める' , '最低限の' , '逃げる' , '運賃' , '個人' , '組織する' , '利子' , '~を動力とする' , '完全に' , '移動手段' , 'リスクを伴う' , '都市部の' , 'ピエロ' , '物資' , '貯蔵庫' , '満タンにする' , '製造' , '関心' , 'ポンプ' , '驚いて' , '現在は' , '段階' , '通常の' , '応用']

#英単語を格納するリストenと各要素に対応させた日本語訳のリストjpの作成
mode = ''
print('モードを選択してください.(quiz or read)[q / r]') #qで英単語もしくは日本語訳をランダムで問う、rで範囲内の英単語と日本語訳を出力
mode = input() #q or r
l_e = len(en) #enの要素数をlenで取得

def read():
    for i in range(len(en)):
        print('En : '+en[i])
        print('Jp : '+jp[i]+'\n')
        
    print('Total word : '+str(l_e)) #合計単語数を出力

def reverse():
    for i in range(len(en)):
        print('Jp : '+jp[i])
        print('En : '+en[i]+'\n')

    print('Total Word : '+str(l_e))

def mixed_quiz():
    print('出題回数を入力してください')
    time = int(input())
    print('答えを表示する際はaキーを押した後returnキーを押してください')
    for i in range(time):
        p_number = rd.randint(0,l_e-1)    #問題番号の乱数化
        e_number = rd.randint(0,1)    #0のとき英語から日本語に翻訳' , '1のとき日本語から英語に翻訳させる
        print('第'+str(i+1)+'問')
        if e_number == 0:
            print(en[p_number])
            res = input()
            if res == 'a':
                print(jp[p_number]+'\n')

        elif e_number == 1:
            print(jp[p_number])
            res = input()
            if res == 'a':
                print(en[p_number]+'\n')

def e_quiz():
    print('出題回数を入力してください')
    time = int(input())
    print('答えを表示する場合はaキーを押した後returnキーを押してください')
    for i in range(time):
        p_number = rd.randint(0,l_e-1)
        print('第'+str(i+1)+'問')
        print(en[p_number])
        res = input()
        if res == 'a':
            print(jp[p_number]+'\n') 

def j_quiz():
    print('出題回数を入力してください')
    time = int(input())
    print('答えを表示する場合はaキーを押した後returnキーを押してください')
    for i in range(time):
        p_number = rd.randint(0,l_e-1)
        print('第'+str(i+1)+'問')
        print(jp[p_number])
        res = input()
        if res == 'a':
            print(en[p_number]+'\n')

if mode == 'r':
    print('Japanese advance or English advance [j / e]')
    advance = input()
    if advance == 'j':
        read()
    elif advance == 'e':
        reverse()

elif mode == 'q':
    print('Japanese advance , English advanvce or mix[j / e / m]')
    advance = input()
    if advance == 'm':
        mixed_quiz()
    elif advance == 'j':
        j_quiz()
    elif advance == 'e':
        e_quiz()