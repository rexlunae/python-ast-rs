# Test case: Simple main call with module-level code
print("Module level code")
x = 42

def main():
    print(f"Main function, x = {x}")
    return "done"

if __name__ == "__main__":
    main()  # Simple call - should rename due to module init needed