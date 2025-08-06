#!/usr/bin/env python3

import sys
import os
import subprocess

# This simulates the original error code
if sys.executable != "test":
    os.execv(sys.executable, [sys.executable, *sys.argv])

proc = subprocess.run(
    [sys.executable, "-u", "-m", "pyperformance.tests"],
    cwd=os.path.dirname(__file__),
    env=dict(os.environ, venvroot="test"),
)
sys.exit(proc.returncode)