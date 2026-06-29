import time

# Test list methods
lst = [3, 1, 2]
lst.append(4)
print(lst)

lst.sort()
print(lst)

lst.reverse()
print(lst)

x = lst.pop()
print(x)
print(lst)

lst.insert(0, 0)
print(lst)

print(lst.count(4))

print(len(lst))

# Test dict methods
d = {"a": 1, "b": 2}
print(d.keys())
print(d.values())
print(d.items())
print(d.get("a"))
print(d.get("x", 42))

# Test str methods
s = "hello world"
print(s.split())
print(s.upper())
print(s.lower())
print(s.replace("world", "there"))
print(s.startswith("hello"))
print(s.endswith("world"))
print(s.find("world"))
print(s.strip())
print(s.isdigit())
print(s.isalpha())

# Test builtins
print(abs(-5))
print(abs(-3.5))
print(min(3, 1, 2))
print(max(3, 1, 2))
print(sum([1, 2, 3, 4]))
print(type(42))
print(type("hello"))

# Test isinstance
print(isinstance(42, int))
print(isinstance("hello", str))

# Test iteration over list
total = 0
for x in [1, 2, 3, 4, 5]:
    total += x
print(total)

# Test try/except
try:
    print("in try")
    x = 1 / 0
except:
    print("caught exception")

# Test lambda
f = lambda x, y: x + y
print(f(10, 20))

# Test enumerate
for i, v in enumerate(["a", "b", "c"]):
    print(i, v)

# Test any/all
print(any([False, True, False]))
print(all([True, True, True]))

# Test sorted/reversed
print(sorted([3, 1, 4, 1, 5]))
print(reversed([1, 2, 3]))

# Test range with step
total2 = 0
for i in range(0, 10, 2):
    total2 = total2 + i
print(total2)
