from fibonacci import fibonacci

def main():
    cases = [3, 7, 13]

    for case in cases:
        print("fib({}) = {}".format(case, fibonacci(case)))

if __name__ == "__main__":
    main()
