import sys

if len(sys.argv) > 2:
    input = sys.argv[1:]
    input = [int(x.removesuffix(",")) for x in input]
else:
    input = sys.argv[1]
    input = input.removeprefix("[")
    input = input.removesuffix("]")
    input = input.split(",")
    input = [int(x) for x in input]

output = "".join(map(chr, input))
print(output)
