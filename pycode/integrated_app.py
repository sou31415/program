# 2022.4.17 21:53(Sun)
"""
もうここに最小二乗法入れちゃっていいんじゃねの精神
あとcalculateモードとdrawモードの柔軟性があまりにも低いため
calculateは階乗モードの追加などを行いたい
また新たにscipyを使って方程式を解くものも余裕があれば作成する予定
素因数分解をする機能があってもよさそう
classを使っているメリットが現状ないので
新しい仕様の追加を検討したい
"""
import math
import csv
import math
import matplotlib.pyplot as plt
import numpy as np

class Almithy:

	def calculate(self):                                    ## calculate method
		print("calculate mode")
		a = str(input())
		if a == "sin":
			b = int(input())
			print(math.sin(math.radians(b)))
		elif a == "cos":
			b = int(input())
			print(math.cos(math.radians(b)))
		elif a == "log":
			c = int(input())
			b = int(input())
			print(math.log(b,c))
		elif a == "tan":
			b = int(input())
			print(math.tan(math.radians(b)))
		elif a == "involution":
			print("a ** x a = ?")
			b = int(input())
			print("x = ?")
			c = int(input())
			print(b ** c)
		else:
			print("Sorry,this string is invalid attribute.")
			
	def draw(self):                         ## drawing graph
		print("draw mode")
		a = str(input())
		x = np.arange(-5,5,0.01)
		if a == "sin":
			y = np.sin(x)
			plt.plot(x,y)
			plt.show()
			
		elif a == "cos":
			y = np.cos(x)
			plt.plot(x,y)
			plt.show()
			
		elif a == "tan":
			y = np.tan(x)
			plt.plot(x,y)
			plt.show()
						 										## 2021 12 23 12:27 
		else :
			print("Sorry,this string is invalid attribute.")
			
	def total(self):                                                ## total score method
		print('Enter the password...')
		password = input()
		if password == "nitac":
			print("What seasons?")
			seasons = input()
			print("Transition to GPA and total score calculate mode.")
			score = 0
			total_score = 0
			total_gp = 0
			print("How many subjects?")
			sj = int(input())
			print(sj , "subjects")
			for i in range(sj):
				print("Enter the score...")
				score = int(input())
				if  score >= 90:
					total_gp += 6
				elif score >= 80:
					total_gp += 5
				elif score >= 70:
					total_gp += 4
				elif score >= 60:
					total_gp += 3
				else:
					total_gp += 0
				total_score += score
			print("total score : ", total_score)
			print("Average score : " , total_score / sj)
			print('Do you also calculate GPA?')
			gpa = input()
			if gpa == "yes":
				print("GPA,"+str(total_gp / sj))
				print('Export this data?')
				data_q = input()
			if (data_q == "yes"):
				with open('result_test.csv','a') as f:
					writer = csv.writer(f)
					writer.writerow([seasons])
					writer.writerow([])
					writer.writerow(["total score,"+str((total_score))])
					writer.writerow(["average score,"+str((total_score / sj))])
					writer.writerow(["total GPA,"+str((total_gp / sj))])
					writer.writerow([])
					
			else:
				print("Program was stopped.")
		else:
			print("You entered password is wrong.")
			
am = Almithy()  ##instance
hollow = ""
print('Please select mode.')
hollow = input()
if hollow == "draw":
	am.draw()
elif hollow == "calculate":
	am.calculate()
elif hollow == "learning":
	am.total()
else:
	print("Sorry,this word is invalid string.\nProgram was stopped")