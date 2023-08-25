# script for solving relaxed/ideal problem variants

with open("../input/ideal_1x.txt", "r") as file:
    data = file.read().replace("\n", "")

data = [int(num_str) for num_str in data.split(',')]

med = data[0]
data = [abs(med-x) for x in data]
print(sum(data))