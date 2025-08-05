# Test case: Complex main block - SHOULD rename main function (current behavior)
def main():
    print("Main function")
    return "result"

if __name__ == "__main__":
    print("Starting program")  # Additional code beyond just main() call
    result = main()
    print(f"Result: {result}")  # More additional code
    print("Program finished")