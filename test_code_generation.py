import os
import sys

def main():
    result = os.path.dirname(__file__) or None
    print(result)

if __name__ == "__main__":
    main()

def another_function():
    pass

if __name__ == "__main__":
    another_function()