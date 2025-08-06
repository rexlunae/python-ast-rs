import subprocess
import sys
import os

result = subprocess.run([sys.executable, "-u", "-m", "test"], cwd=os.path.dirname(__file__))
print("Return code:", result.returncode)