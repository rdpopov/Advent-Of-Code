#!/usr/bin/python3
from math import lcm

def gen_op(op):
    """ Generates a lambda function for the current operation
        op ():
    """
    expr = op.split("=")[1].strip() # creates the function
    lmd = lambda old: eval(expr,{'old': old})
    return lmd

class Monkey:
    def __init__(self,args):
        self.id = args["id"]
        self.items = args["items"]
        self.throwTo = lambda x: args["throwT"] if x % args["testDiv"] == 0 else args["throwF"]
        self.examined_items = 0
        self.div = args["testDiv"]
        self.operation = args["op"]
        self.lcm = 1

    def examine_items(self,monke_dict):
        for i in range(len(self.items)):
            tmp = int(self.operation(self.items[i])/3)
            next_monkey = self.throwTo(tmp)
            self.examined_items += 1
            # print("from {} to {} throw {}".format(self.id,next_monkey,tmp))
            monke_dict[next_monkey].get_items(tmp)
        # we cant handle throwin to self
        self.items = []

    def examine_items2(self,monke_dict):
        for i in range(len(self.items)):
            #the snake will have to handle it, it is what it is
            tmp = int(self.operation(self.items[i])) % self.lcm
            next_monkey = self.throwTo(tmp)
            self.examined_items += 1
            # print("from {} to {} throw {}".format(self.id,next_monkey,tmp))
            monke_dict[next_monkey].get_items2(tmp)
        # we cant handle throwin to self
        self.items = []

    def get_items(self,item):
        self.items.append(item)

    def get_items2(self,item):
        self.items.append(item)
def tick(monkeys):
    for i in monkeys:
        monkeys[i].examine_items(monkeys)

def tick2(monkeys):
    for i in monkeys:
        monkeys[i].examine_items2(monkeys)

def get_monkeys(fname):
    monkey_map = {}
    monkey_args = {}
    lcm_of = 1
    with open(fname) as f:
        for line in f.readlines():
            if "Monkey" in line:
                id = int((line.split()[1])[:-1])
                monkey_args["id"] = id
            elif "Starting" in line:
                itm = list(map(int,line.split(':')[1].strip().split(',')))
                monkey_args["items"] = itm
            elif "Operation" in line:
                lmd = gen_op(line)
                monkey_args["op"] = lmd
            elif "Test" in line:
                monkey_args["testDiv"] = int(line.split()[-1])
                lcm_of = lcm(lcm_of,int(line.split()[-1]))
            elif "true" in line:
                monkey_args["throwT"] = int(line.split()[-1])
            elif "false" in line:
                monkey_args["throwF"] = int(line.split()[-1])
                # print(str(monkey_args))
                monkey_map[monkey_args["id"]] = Monkey(monkey_args)
                monkey_args = {}
            else:
                pass
    for i in monkey_map:
        monkey_map[i].lcm = lcm_of
    return monkey_map


def scenario1(fname):
    monke = get_monkeys(fname)
    for i in range(20):
        tick(monke)
    res = [monke[i].examined_items for i in monke]
    res.sort(reverse=True)
    print(res[0] * res[1])

def scenario2(fname):
    monke = get_monkeys(fname)
    for i in range(10000):
        tick2(monke)
    res = [monke[i].examined_items for i in monke]
    res.sort(reverse=True)
    print(res)
    print(res[0] * res[1])

if __name__ == "__main__":
    scenario1("./input1")
    scenario1("./input")
    scenario2("./input1")
    scenario2("./input")
