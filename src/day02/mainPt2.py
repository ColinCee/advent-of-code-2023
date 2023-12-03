def getGameNumber(dataString):
    gameNumber = dataString.split(":")[0].split(" ")[1]
    return gameNumber


def splitGameData(dataString) -> list:
    splitData = dataString.split(":")[1].split(";")
    return splitData


def getMaxNumber(gamelist: list) -> dict:
    checkColours = {"red": 0, "green": 0, "blue": 0}
    for game in gamelist:
        game = game.strip()
        gameDetails = game.split(",")
        for row in gameDetails:
            rowDetails = row.strip().split(" ")
            if int(rowDetails[0]) > checkColours[rowDetails[1]]:
                checkColours[rowDetails[1]] = int(rowDetails[0])

    return checkColours


def checkValidGame(checkColours: dict, elfInfo: dict):
    if (
        checkColours["red"] <= elfInfo["red"]
        and checkColours["green"] <= elfInfo["green"]
        and checkColours["blue"] <= elfInfo["blue"]
    ):
        return True
    else:
        return False


elfInfo = {"red": 12, "green": 13, "blue": 14}


def checkGame(dataString):
    # gameNumber = getGameNumber(dataString)
    gameData = splitGameData(dataString)
    checkColours = getMaxNumber(gameData)
    power = 1
    for value in checkColours.values():
        power *= value
    # print all the variables in one line
    # print("gameNumber: ", gameNumber, 
    #       "gameData: ", gameData, 
    #       "checkColours: ", checkColours, 
    #       "elfInfo: ", elfInfo, sep="\n")
    # if checkValidGame(checkColours, elfInfo):
    #     return gameNumber
    # else:
    #     return None
    return power 


with open(r"src/day02/inputData.txt") as data:
    total = 0
    for line in data:
        result = checkGame(line)
        print("Result: ", result)
        
        if result is not None:
            total += int(result)

    print("Total: ", total)
