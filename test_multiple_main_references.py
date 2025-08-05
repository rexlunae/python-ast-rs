# Test case 3: Multiple references to user's main function
def main(arg1, arg2="default"):
    print(f"Main called with {arg1}, {arg2}")
    return arg1 + len(arg2)

def wrapper():
    return main("hello", "world")

class TestClass:
    def call_main(self):
        return main("class", "method")

if __name__ == "__main__":
    # Direct call
    main("direct")
    
    # Call through wrapper
    wrapper_result = wrapper()
    
    # Call through class method
    test_obj = TestClass()
    class_result = test_obj.call_main()
    
    print(f"Results: {wrapper_result}, {class_result}")