def square_root(n):
    root = n / 2
    temp = 0.0

    while root != temp:
        temp = root
        root = (n / temp + temp) / 2

    return root

def solve_quadratic_positive(a, b, c):
    numerator = -b + square_root(b * b - 4 * a * c)
    denominator: float = 2 * a
    return numerator / denominator

def solve_quadratic_negative(a, b, c):
    numerator = -b - square_root(b * b - 4 * a * c)
    denominator: float = 2 * a
    return numerator / denominator

def calculate():
    a = 5.5
    b = 3.4
    c = -6.4
    pos = solve_quadratic_positive(a, b, c)
    neg = solve_quadratic_negative(a, b, c)
    return pos + neg
