# Final comprehensive test: Module init + main function conflict
print("1. Module starts")

# Module-level variables
MODULE_VAR = "initialized"
print(f"2. Module var: {MODULE_VAR}")

# Function definition (declaration)
def helper():
    print("3. Helper function")
    return "helper_result"

# Module-level function call (executable)
helper_result = helper()
print(f"4. Helper result: {helper_result}")

# User's main function (will be renamed to python_main)
def main():
    print("5. User main function")
    local_helper = helper()
    return f"main_result: {local_helper}"

# More module-level executable code
print("6. Before main block")

if __name__ == "__main__":
    print("7. Main block starts")
    user_main_result = main()
    print(f"8. User main returned: {user_main_result}")
    print(f"9. Module var still: {MODULE_VAR}")
    print("10. Execution completed")