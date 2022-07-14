# Prints the bytes of a float
# Used for converting sensor values to values that can be
# used in the config files
# USAGE:
# from the structs dir run python, run `from float_to_bytes import float_to_bytes` in the python terminal
# use the float_to_bytes function to get the bytes for your float value
import struct
def float_to_bytes(f):
    ba = bytearray(struct.pack("f", f))
    print(["%d" % b for b in ba])

