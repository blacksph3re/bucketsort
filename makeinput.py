import random

for i in range(0, 100000):
	for j in range(0, 7):
		print(chr(random.randint(33, 126)), end="")
	print()
