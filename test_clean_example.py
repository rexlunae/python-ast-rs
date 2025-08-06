import sys
import os

def example():
    print("sys.executable:", sys.executable)
    print("sys.argv:", sys.argv)
    return 42

obj = example()
sys.exit(obj)