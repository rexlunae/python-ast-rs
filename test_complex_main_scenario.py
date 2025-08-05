# Complex test case: Main function with various reference patterns
def main(arg1="default", *args, **kwargs):
    """User-defined main function with complex signature"""
    print(f"Main function called with arg1={arg1}, args={args}, kwargs={kwargs}")
    return {"result": "success", "args": len(args)}

def call_main_wrapper():
    """Function that calls main in different ways"""
    # Direct call
    result1 = main()
    
    # Call with positional args
    result2 = main("test", "extra")
    
    # Call with keyword args  
    result3 = main(arg1="keyword", extra="value")
    
    return [result1, result2, result3]

class MainCaller:
    """Class that has methods calling main"""
    
    def __init__(self):
        self.main_result = None
    
    def call_main_method(self):
        self.main_result = main("from_class")
        return self.main_result
    
    @staticmethod
    def static_main_call():
        return main("static_call")

# Module-level main calls
GLOBAL_MAIN_RESULT = main("global")

if __name__ == "__main__":
    print("Starting complex main test")
    
    # Direct main call
    direct_result = main("direct_from_main_block")
    
    # Call through wrapper
    wrapper_results = call_main_wrapper()
    
    # Call through class
    caller = MainCaller()
    class_result = caller.call_main_method()
    static_result = MainCaller.static_main_call()
    
    print(f"Direct: {direct_result}")
    print(f"Wrapper: {wrapper_results}")
    print(f"Class: {class_result}")
    print(f"Static: {static_result}")
    print(f"Global: {GLOBAL_MAIN_RESULT}")
    
    print("Complex main test completed")