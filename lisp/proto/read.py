#!/usr/bin/env python3

from lib.inventory import Order  # generated module, package `inventory` -> module `inventory`

data = bytes.fromhex("""
0a 07 4f 52 44 2d 30 30 31 12 21 0a 0b 31 32 33 20 4d 61 69 6e 20 53 74 12 0b 53 70 72 69 6e 67 66 69 65 6c 64 1a 05 30 30 30 30 30 1a 18 0a 06 4c 61 70 74 6f 70 10 01 19 52 b8 1e 85 eb 3f 8f 40 22 03 01 04 07 1a 17 0a 07 42 61 6e 61 6e 61 73 10 02 19 e1 7a 14 ae 47 e1 e2 3f 22 01 02 22 04 e9 07 ea 07
""")
order = Order().parse(data)
print(order)
