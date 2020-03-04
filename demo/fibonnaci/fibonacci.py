def fibonacci(n: int) -> int:
    first: int = 0
    second: int = 1
    c: int = 1
    total: int = 0
    next: int = 0

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
