# Test case: Only module-level executable code, no main function
print("Module starts")

x = 42
y = x * 2

def helper():
    return "helper"

result = helper()
print(f"Result: {result}")

for i in range(2):
    print(f"Loop {i}")

print("Module ends")