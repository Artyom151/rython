def test_int_operations():
    a = 10
    b = 3
    if a + b == 13: print("test_int_add: PASS")
    if a - b == 7: print("test_int_sub: PASS")
    if a * b == 30: print("test_int_mul: PASS")
    if a // b == 3: print("test_int_floordiv: PASS")
    if a % b == 1: print("test_int_mod: PASS")
    if a ** 2 == 100: print("test_int_pow: PASS")

def test_float_operations():
    a = 10.0
    b = 3.0
    if abs(a / b - 3.333) < 0.01: print("test_float_div: PASS")
    if a + b == 13.0: print("test_float_add: PASS")
    if a - b == 7.0: print("test_float_sub: PASS")
    if a * b == 30.0: print("test_float_mul: PASS")

def test_bool_operations():
    t = True
    f = False
    if t and t: print("test_bool_and_true: PASS")
    if not (t and f): print("test_bool_and_false: PASS")
    if t or f: print("test_bool_or: PASS")
    if not f: print("test_bool_not: PASS")
    if (1 < 2) == True: print("test_bool_compare: PASS")

def test_string_operations():
    s = "hello"
    if s + " world" == "hello world": print("test_str_concat: PASS")
    if len(s) == 5: print("test_str_len: PASS")
    if s.upper() == "HELLO": print("test_str_upper: PASS")
    if s.lower() == "hello": print("test_str_lower: PASS")
    if "he" in s: print("test_str_contains: PASS")
    if s.startswith("he"): print("test_str_startswith: PASS")
    if s.endswith("lo"): print("test_str_endswith: PASS")
    if s.replace("l", "x") == "hexxo": print("test_str_replace: PASS")

def test_list_operations():
    lst = [1, 2, 3, 4, 5]
    if len(lst) == 5: print("test_list_len: PASS")
    lst.append(6)
    if len(lst) == 6: print("test_list_append: PASS")
    if lst[0] == 1: print("test_list_index: PASS")
    if lst[-1] == 6: print("test_list_neg_index: PASS")
    lst.sort()
    if lst[0] == 1: print("test_list_sort: PASS")
    lst.reverse()
    if lst[0] == 6: print("test_list_reverse: PASS")

def test_dict_operations():
    d = {"a": 1, "b": 2}
    if len(d) == 2: print("test_dict_len: PASS")
    d["c"] = 3
    if len(d) == 3: print("test_dict_set: PASS")
    if d["a"] == 1: print("test_dict_get: PASS")

def test_comparisons():
    if 1 < 2: print("test_lt: PASS")
    if 2 <= 2: print("test_le: PASS")
    if 2 > 1: print("test_gt: PASS")
    if 2 >= 2: print("test_ge: PASS")
    if 1 == 1: print("test_eq: PASS")
    if 1 != 2: print("test_ne: PASS")
    if 1 is 1: print("test_is: PASS")
    if 1 is not 2: print("test_is_not: PASS")

def test_builtins():
    if abs(-5) == 5: print("test_abs: PASS")
    if min(1, 2, 3) == 1: print("test_min: PASS")
    if max(1, 2, 3) == 3: print("test_max: PASS")
    if sum([1, 2, 3]) == 6: print("test_sum: PASS")
    if len([1, 2, 3]) == 3: print("test_len: PASS")
    if isinstance(42, int): print("test_isinstance_int: PASS")

test_int_operations()
test_float_operations()
test_bool_operations()
test_string_operations()
test_list_operations()
test_dict_operations()
test_comparisons()
test_builtins()
print("ALL TYPE TESTS DONE")
