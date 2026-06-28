squares = [x * x for x in range(10)]
print("squares:", squares)

evens = [x for x in range(20) if x % 2 == 0]
print("evens:", evens)

unique = {x % 3 for x in range(10)}
print("unique:", unique)

gen = (x * 2 for x in range(5))
print("gen:", gen)

odd_squares = [x * x for x in range(10) if x % 2 == 1]
print("odd_squares:", odd_squares)

print("DONE")
