# Test case: Complex module with mixed declarations and executable code
print("1. Module initialization starts")

# Global variable assignment (executable)
global_var = "initialized at module level"
print(f"2. Global variable set: {global_var}")

# Function definition (declaration)
def helper_function():
    print("4. Helper function called")
    return "helper_result"

# More executable code
print("3. Between function definition and class")
intermediate_result = 42 + 8
print(f"5. Intermediate calculation: {intermediate_result}")

# Class definition (declaration)
class TestClass:
    class_var = "class_variable"
    
    def __init__(self):
        print("6. TestClass instance created")
    
    def method(self):
        return f"method called with {self.class_var}"

# More executable code after class
print("7. After class definition")
instance = TestClass()
method_result = instance.method()
print(f"8. Method result: {method_result}")

# Import statement (should execute early)
import os
current_dir = os.getcwd() if hasattr(os, 'getcwd') else "unknown"
print(f"9. Current directory: {current_dir}")

# Another function that references earlier definitions
def main():
    print("10. User main function called")
    helper_result = helper_function()
    print(f"11. Helper returned: {helper_result}")
    return f"main completed with global_var={global_var}"

# More executable code
execution_count = 0
for i in range(3):
    execution_count += 1
    print(f"12.{i+1}. Loop iteration {i}, count={execution_count}")

# The main execution block
if __name__ == "__main__":
    print("13. Main execution block starts")
    main_result = main()
    print(f"14. Main function result: {main_result}")
    print(f"15. Final execution count: {execution_count}")
    print("16. Module execution completed")