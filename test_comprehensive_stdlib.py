import sys
import os
import subprocess

# Test various standard library functions
print("System info:")
print("Platform:", sys.platform)
print("Executable:", sys.executable)

print("File operations:")
cwd = os.getcwd()
print("Current directory:", cwd)

# Test os.path functions
print("Path operations:")
print("Dirname of /usr/bin/python:", os.path.dirname("/usr/bin/python"))
print("Basename of /usr/bin/python:", os.path.basename("/usr/bin/python"))
print("Does current dir exist:", os.path.exists("."))

# Test subprocess
print("Running subprocess:")
result = subprocess.run(["echo", "Hello from subprocess"], cwd=None)
print("Subprocess return code:", result.returncode)