def test_exec_basic():
    exec("x = 42")
    print(x)

def test_exec_multi():
    exec("""
a = 10
b = 20
c = a + b
""")
    print(c)

def test_eval_expr():
    result = eval("1 + 2 * 3")
    print(result)

def test_eval_str():
    result = eval("'hello' + ' ' + 'world'")
    print(result)

test_exec_basic()
test_exec_multi()
test_eval_expr()
test_eval_str()
