import sys
import pyperclip

def generate_test(test_name, filename):
    """Generates a test from a source file

    Args:
        test_name (str): The name to give the test, eg. what it is testing
        filename (str): The filename to process

    Returns: A generated test that can be pasted into tests/codegen.rs

    """
    python_source = filename + ".py"
    c_source = filename + ".c"

    # Read the two files and get their contents
    with open(python_source, "r") as f:
        python_code = repr("".join(f.readlines()))

    with open(c_source, "r") as f:
        c_code = repr("".join(f.readlines()))

    return "{}: {}, {},".format(
        test_name,
        python_code,
        c_code
    ).replace("\"", "\\\"").replace("'", "\"")

def main():
    """Entry point for the program.
    """
    if len(sys.argv) < 2:
        print("Usage: python3 generate_test.py [TEST_NAME] [FILENAME]")
        return

    test_name = sys.argv[1]
    filename = sys.argv[2]
    generated = generate_test(test_name, filename)

    print(generated)
    pyperclip.copy(generated)

if __name__ == "__main__":
    main()
