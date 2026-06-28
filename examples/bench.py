import time

start = time.time()
s = 0
for i in range(1000000):
    s = s + i
end = time.time()
print(end - start)
