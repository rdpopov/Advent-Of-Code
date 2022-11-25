#!/usr/bin/python3
import json
# def sumMachineGun(s):
#     sum = 0
#     c = []
#     number_chars = "-0123456789"
#     for i in s:
#         if i in number_chars:
#             c.append(i)
#         elif len(c) > 0:
#             if c[0] == '-' and len(c) > 2:
#                     sum += int(''.join(c))
#             else:
#                 sum += int(''.join(c))
#             c = []
#     return sum

def sumMachineGun(s):
    sum = 0
    c = []
    number_chars = "-0123456789"
    with open(s) as f:
        while True:
            i = f.read(1)
            if i in number_chars:
                c.append(i)
            elif len(c) > 0:
                if c[0] == '-' and len(c) > 2:
                        sum += int(''.join(c))
                else:
                    sum += int(''.join(c))
                c = []
            if not i:
                break
    return sum

def sceanrio1():
    print(sumMachineGun("./input"))

def remove_red(json_obj) -> int: 
    sum = 0
    if isinstance(json_obj,dict):
        if "red" in json_obj:
            pass
        else:
            for key in json_obj: 
                if isinstance(json_obj[key], int):
                    sum += json_obj[key]
                else:
                    if json_obj[key] == "red":
                        sum = 0
                        break
                    sum += remove_red(json_obj[key])
    elif isinstance(json_obj,list):
        for item in json_obj:
            if isinstance(item, int):
                sum += item
            else:
                sum += remove_red(item)
    return sum

def sceanrio2():
    with open("./input") as f:
        doc = json.load(f)
        # print(json.dumps(doc,indent =2))
        print(remove_red(doc))
    

if __name__ == "__main__":
    sceanrio1()
    sceanrio2()
