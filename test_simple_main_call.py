# Test case: Simple main() call pattern - should NOT rename main function
def main():
    print("This is the main function")
    return "main_result"

if __name__ == "__main__":
    main()  # Only a call to main() - should use main() directly as Rust entry point