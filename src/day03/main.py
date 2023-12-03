import re

with open(r"src/day03/testData.txt") as rawData:
    data = []
    for line in rawData:
        line = line.rstrip()
        data.append(line)


def getNumberList(dataLine):
    return re.findall(r"\d+", dataLine)


def getNumberIndex(dataLine, number: str):
    return [dataLine.index(number), dataLine.index(number) + len(number)]

def isValid(data, numberIndex):
    top = 
    bot =
    left =
    right = 

for line in data:
    numberList = getNumberList(line)
    if len(numberList) > 0:
        for number in numberList:
            numberIndex = getNumberIndex(line, number)

