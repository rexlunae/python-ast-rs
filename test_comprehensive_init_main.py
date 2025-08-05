# Comprehensive test: Module init + main function conflict + execution order
print("1. Module initialization starts")

# Module-level variables
MODULE_VAR = "module_level"
COUNTER = 0

def increment_counter():
    global COUNTER
    COUNTER += 1
    return COUNTER

# Module-level function call
initial_count = increment_counter()
print(f"2. Initial counter: {initial_count}")

# User-defined main function (will conflict with Rust main)
def main():
    print("3. User main function called")
    global COUNTER
    user_count = increment_counter()
    print(f"4. User main counter: {user_count}")
    return f"user_main_result_{user_count}"

# More module-level code
print("5. Between main definition and main block")
pre_main_count = increment_counter()

# Helper function
def helper():
    count = increment_counter()
    print(f"6. Helper called, counter: {count}")
    return count

if __name__ == "__main__":
    print("7. Main execution block starts")
    
    # Call user's main function
    user_result = main()
    print(f"8. User main returned: {user_result}")
    
    # Call helper
    helper_count = helper()
    
    # Show final state
    print(f"9. Final counter: {COUNTER}")
    print(f"10. Module var: {MODULE_VAR}")
    print("11. Main execution completed")