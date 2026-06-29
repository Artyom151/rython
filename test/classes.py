class Point:
    def __init__(self, x, y):
        self.x = x
        self.y = y

    def distance(self, other):
        dx = self.x - other.x
        dy = self.y - other.y
        return (dx * dx + dy * dy) ** 0.5

p1 = Point(1, 2)
p2 = Point(4, 6)
d = p1.distance(p2)
print(d)
