def test_eval_arithmetic():
    r = eval("(3 + 5) * 2")
    assert r == 16

def test_eval_string_concat():
    r = eval("'hello ' + 'world'")
    assert r == "hello world"

def test_eval_bool():
    r = eval("1 < 2")
    assert r == True

def test_exec_simple():
    exec("x = 42")
    assert x == 42

def test_exec_multi():
    exec("""
a = 10
b = 20
c = a + b
""")
    assert c == 30

def test_eval_math():
    r = eval("3 * 7 + 1")
    assert r == 22
