def build(attack , crit_rate , crit_damage ,buff):
	crit_rate /= 100
	crit_damage /= 100
	rate = 1 + (crit_rate * crit_damage)
	buff /= 100
	buff += 1
	print('expect : {}'.format(attack * rate * buff * 0.45))

build(2275 , 78.5 , 172.9 , 130.4)
build(2191 , 71.9 , 172.1 , 160.4)
build(2228 , 79.3 , 185.3 , 130.4)
