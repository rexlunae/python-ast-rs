# Test case 1: User-defined main() function with __name__ == "__main__" block
def main():
    print("User-defined main function")
    return "from_main"

def helper():
    print("Helper function")

if __name__ == "__main__":
    result = main()  # Call to user's main function
    helper()
    print(f"Result: {result}")
    print("Additional main block code")