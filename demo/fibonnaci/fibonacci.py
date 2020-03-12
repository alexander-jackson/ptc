def fibonacci(n: int):
    first = 0
    second = 1
    c = 1
    total = 0
    next = 0

    while c < n:
        if c <= 1:
            next = c
        else:
            next = first + second
            first = second
            second = next

        c += 1
        total += next

    return total
