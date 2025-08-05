# Test case: Simple main() call with return value - should NOT rename main function
def main():
    print("Main function with return")
    return 42

if __name__ == "__main__":
    result = main()  # Only a call to main() with assignment - should still optimize