# Demonstrate with statement transpilation

x = {"name": "rython"}
with x as obj:
    print("name:", obj.name)

# Multiple context managers
a = {"val": 10}
b = {"val": 20}
with a as first, b as second:
    print("first:", first.val, "second:", second.val)

# Bare with (no 'as')
y = "hello"
with y:
    print("y:", y)
