import subprocess
import sys

number_threads = int(sys.argv[1])
binary = sys.argv[2]

i = 0
while i != number_threads:
    subprocess.run([binary, sys.argv[3], sys.argv[4],sys.argv[5]])
    i += 1