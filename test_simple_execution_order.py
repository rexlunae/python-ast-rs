# Simple test case showing execution order issues
print("Step 1: Module starts")

x = 10
print(f"Step 2: x = {x}")

def func():
    return x * 2

y = func()  # This calls func() at module level
print(f"Step 3: y = func() = {y}")

class MyClass:
    z = y + 5  # Class variable depends on module-level y

obj = MyClass()
print(f"Step 4: obj.z = {obj.z}")

if __name__ == "__main__":
    print("Step 5: Main block")
    print(f"Final values: x={x}, y={y}, obj.z={obj.z}")