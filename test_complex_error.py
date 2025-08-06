import os.path
import subprocess
import sys

def main():
    (venvroot, python) = ensure_venv_ready(kind="tests")
    if ((python) != (sys.executable)):
        os.execv(python, [python] + sys.argv)
    
    proc = subprocess.run(
        [sys.executable, "-u", "-m", "pyperformance.tests"],
        cwd=os.path.dirname(__file__),
    )
    sys.exit(proc.returncode)

if __name__ == "__main__":
    main()