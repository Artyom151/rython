def test_exec_simple():
    exec("x = 100")
    if x == 100:
        print("test_exec_simple: PASS")
    else:
        print("test_exec_simple: FAIL")

def test_exec_multi_statement():
    exec("""
a = 1
b = 2
c = a + b + 3
""")
    if c == 6:
        print("test_exec_multi_statement: PASS")
    else:
        print("test_exec_multi_statement: FAIL")

def test_exec_with_fn():
    exec("""
def add(a, b):
    return a + b
result = add(10, 20)
""")
    if result == 30:
        print("test_exec_with_fn: PASS")
    else:
        print("test_exec_with_fn: FAIL")

def test_exec_for_loop():
    exec("""
s = 0
for i in range(5):
    s = s + i
""")
    if s == 10:
        print("test_exec_for_loop: PASS")
    else:
        print("test_exec_for_loop: FAIL")

def test_eval_int():
    r = eval("42")
    if r == 42:
        print("test_eval_int: PASS")
    else:
        print("test_eval_int: FAIL")

def test_eval_arithmetic():
    r = eval("(3 + 5) * 2")
    if r == 16:
        print("test_eval_arithmetic: PASS")
    else:
        print("test_eval_arithmetic: FAIL")

def test_eval_string():
    r = eval("'Hello, ' + 'World!'")
    if r == "Hello, World!":
        print("test_eval_string: PASS")
    else:
        print("test_eval_string: FAIL")

def test_eval_list():
    r = eval("[1, 2, 3, 4]")
    if len(r) == 4:
        print("test_eval_list: PASS")
    else:
        print("test_eval_list: FAIL")

def test_eval_bool():
    r = eval("1 < 2")
    if r == True:
        print("test_eval_bool: PASS")
    else:
        print("test_eval_bool: FAIL")

def test_eval_nested():
    x = 5
    y = 3
    r = eval("x * y + 2")
    if r == 17:
        print("test_eval_nested: PASS")
    else:
        print("test_eval_nested: FAIL")

test_exec_simple()
test_exec_multi_statement()
test_exec_with_fn()
test_exec_for_loop()
test_eval_int()
test_eval_arithmetic()
test_eval_string()
test_eval_list()
test_eval_bool()
test_eval_nested()
print("ALL EVAL/EXEC TESTS DONE")
