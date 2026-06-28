import sys
import json

v = sys.version()
print(v)

data = json.dumps([1, 2, 3])
print(data)

parsed = json.loads(data)
print(parsed)
