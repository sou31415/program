import random as rd

def scrcramble():
    scr = []
    repeat_time = rd.randint(20,28)
    for i in range (repeat_time):
        place = rd.randint(0,5)
        reverscre = rd.randint(0,1)
        timescr = rd.randint(0,1)
        if place == 0:
            scr.append("L")
        elif place == 1:
            scr.append("R")
        elif place == 2:
            scr.append("U")
        elif place == 3:
            scr.append("D")
        elif place == 4:
            scr.append("F")
        elif place == 5:
            scr.append("B")
        if reverscre == 1:
            scr[i] += "'"
        if timescr == 1:
            scr[i] += "2"

    for i in range(repeat_time):
        print(scr[i])

scrcramble()
