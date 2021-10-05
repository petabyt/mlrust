import re

f = open("main.S", "r")
c = f.read()

c = re.sub(r"bl\t[_A-Za-z0-9]+panicking[_A-Za-z0-9]+", "bl panic", c)

f = open("main.S", "w")
f.write(c)
f.close()
