# script for solving relaxed/ideal problem variants

import random

# default len (1x)
INPUT_LEN = 1000

# Input size target (10 ==> 10x, 100 ==> 100x, etc.) 
INPUT_LEN *= 10000

# range of input vales:
# [0,9999] for default/relaxed variants
# [0,255] for ideal variant
data = [random.randint(0, 255) for x in range(INPUT_LEN)]


with open(r'./ideal_.txt', 'w') as fp:
    fp.write(",".join(str(x) for x in data))