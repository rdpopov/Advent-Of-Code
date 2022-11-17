#!/usr/bin/python3
import numpy as np

def exec_line(cmd:str, lv:str , rv:str):
    if cmd == "AND":
        return np.uint16(lv & rv)
    elif cmd == "OR":
        return np.uint16(lv | rv)
    elif cmd =="LSHIFT":
        return np.uint16(rv << lv)
    elif cmd =="RSHIFT":
        return np.uint16(rv >> lv)
    elif cmd == "NOT":
        return ~np.uint16(rv)
    else:  # cmd = own
        return np.uint16(rv)

def get_value_or_token(s):
    if s.isdigit():
        return int(s)
    else:
        return s

class Conn:
    def __init__(self,descr):
        self.name = descr[-1]
        if len(descr) == 3: ## assignment case
            self.op = "OWN" # assigment operation
            self.rvalue = get_value_or_token(descr[0])
            self.value = None
            if not isinstance(self.rvalue,str):
                self.value = self.rvalue
            self.lvalue = None
        elif len(descr) == 4: ## NOT gate gase
            self.op = "NOT"
            self.rvalue = get_value_or_token( descr[1])
            self.lvalue = None
            self.value = None
        elif len(descr) == 5: ## Any other gate gase
            self.op = descr[1]
            self.rvalue = get_value_or_token(descr[0])
            self.lvalue = get_value_or_token(descr[2])
            self.value = None

    def exec(self, dct):
        if self.value is not None:
            return
        if isinstance(self.lvalue,str):
            if dct[self.lvalue].value is None:
                return
            else:
                self.lvalue = dct[self.lvalue].value
        if isinstance(self.rvalue,str):
            if dct[self.rvalue].value is None:
                return
            else:
                self.rvalue = dct[self.rvalue].value
        self.value = exec_line(self.op,self.lvalue,self.rvalue)

def create(descr):
    return (descr[-1], Conn(descr))

def scenario_1():
    data = {}
    goal = "a"
    with open("./input") as inp:
        for i in inp.readlines():
            tokens = i.strip().split(" ")
            conn,val = create(tokens)
            data[conn] = val
        while (not data[goal].value):
            for i in data:
                data[i].exec(data)
    print(data[goal].value)
    return data[goal].value

if __name__ == "__main__":
    scenario_1()
