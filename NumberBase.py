import math

def createNumberSystem(translator):  # 62 characters
    for i in range(0, 10):
        translator.append(str(i))    # 1- 10
    for i in range(0, 26):
        translator.append(chr(97+i)) # a -z
    for i in range(0, 26):
        translator.append(chr(65+i)) # A - z

def addSpecialCharacters(translator):# 15 charaters.. 77 overall
    for i in range(33, 48):
        translator.append(chr(i))
    for i in range(58, 41):
        translator.append(chr(i))

def numberBase(translator, number, base):
    power = getOutputLength(number, base)
    print('power', power)
    result = ''
    while(power > -1):
        digit = math.pow(base, power)
        hiConstant = highestConstant(number, digit, base)
        print(base,"^", power, "*", hiConstant,"\t=", digit*hiConstant)
        if(hiConstant <= number):
            number -= int(digit*hiConstant)
            result += translator[hiConstant]
        else:
            digit = 0
            result += '0'
        power -= 1
    return result

def highestConstant(number, digit, base):
    for i in range(1, base):
        if(digit*i > number):
            return i-1
        if(digit*i == number):
            return i
    return base -1


def getOutputLength(number, base):
    power = 0
    while(math.pow(base, power) <= number):
        power+=1
    if(power == 0):
        return 0
    return power-1

if __name__ == '__main__':
    translator = []
    createNumberSystem(translator)
    addSpecialCharacters(translator)
    print('max-base size:', len(translator))
    N = input("Enter a number: ")
    B = input("Enter a base: ")
    print(numberBase(translator, int(N), int(B)))
