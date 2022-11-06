import sys
import re
import zlib

if len(sys.argv) < 3:
    print("Usage: " + __file__ + " <PDF file path> <object number>")
    exit(1)

file_name = sys.argv[1]
object_number = sys.argv[2]

pdf = open(sys.argv[1], "rb").read()
string = sys.argv[2] + " 0 obj.*?FlateDecode.*?stream(.*?)endstream"
bytes = str.encode(string)
stream = re.compile(bytes, re.S)

s = re.search(stream, pdf).group(1).strip()
try:
    print(zlib.decompress(s))
    print("")
except:
    print("Error: Could not find desired object")
    exit(1)
