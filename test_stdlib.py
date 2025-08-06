#!/usr/bin/env python3

# Test script to verify our Python standard library implementations work correctly
import sys
import os
import subprocess

print("Testing sys module:")
print("sys.executable:", sys.executable)
print("sys.argv:", sys.argv)
print("sys.platform:", sys.platform)

print("\nTesting os module:")
print("os.getcwd():", os.getcwd())
print("os.getenv('HOME'):", os.getenv('HOME'))

print("\nTesting os.path module:")
print("os.path.exists('.'):", os.path.exists('.'))
print("os.path.isdir('.'):", os.path.isdir('.'))
print("os.path.dirname('/usr/bin/python'):", os.path.dirname('/usr/bin/python'))
print("os.path.basename('/usr/bin/python'):", os.path.basename('/usr/bin/python'))

print("\nTesting subprocess module:")
result = subprocess.run(['echo', 'hello world'], capture_output=True, text=True)
print("subprocess.run result:", result.returncode, result.stdout.strip())