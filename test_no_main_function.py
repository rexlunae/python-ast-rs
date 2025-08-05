# Test case 4: Only __name__ == "__main__" block, no user main function
def helper1():
    print("Helper 1")

def helper2():
    print("Helper 2")

if __name__ == "__main__":
    print("Starting execution")
    helper1()
    helper2()
    print("Finished execution")