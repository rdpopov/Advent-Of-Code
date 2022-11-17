#!/usr/bin/python3
import numpy as np

def exec_line(cmd:str, lv:str , rv:str):
    if cmd == "AND":
        return np.uint16(lv) & np.uint16(rv)
    elif cmd == "OR":
        return np.uint16(lv) | np.uint16(rv)
    elif cmd =="LSHIFT":
        return np.uint16(lv) << np.uint16(rv)
    elif cmd =="RSHIFT":
        return np.uint16(lv) >> np.uint16(rv)

# def prep_value(ctx,tok):
#     if not tok:
#         return 0
#     if tok in ctx:
#         return ctx[tok]
#     else:
#         try:
#             return int(tok)
#         except ValueError:
#             return 0

# class Conn:
#     def __init__(self,descr):
#         self.name = descr[-1]
#         if len(descr) == 3: ## assignment case
#             self.value = lambda d: int(descr[0])
#         elif len(descr) == 4: ## NOT gate gase
#             self.value = lambda d: ~d[descr[1]].value(d)
#         elif len(descr) == 5: ## Any other gate gase
#             self.value = lambda d: exec_line(descr[1], d[descr[0]].value(d), d[descr[2]].value(d))


def create(descr):
    value = None
    if len(descr) == 3: ## assignment case
        if descr[0].isdigit():
            value = lambda d: np.uint16(descr[0])
        else:
            value = lambda d: d[descr[0]](d)
    elif len(descr) == 4: ## NOT gate gase
        value = lambda d: ~d[descr[1]](d)
    elif len(descr) == 5: ## Any other gate gase
        if descr[0].isdigit() and descr[2].isdigit():
            value = lambda d: exec_line(descr[1], np.uint16(descr[0]), np.uint16(descr[2]))
        elif descr[2].isdigit():
            value = lambda d: exec_line(descr[1], d[descr[0]](d), np.uint16(descr[2]))
        elif descr[0].isdigit():
            value = lambda d: exec_line(descr[1], np.uint16(descr[0]), d[descr[2]](d))
        else:
            value = lambda d: exec_line(descr[1], d[descr[0]](d), d[descr[2]](d))
    return (descr[-1],value)


def scenario_1():
    data = {}
    with open("./input") as inp:
        for i in inp.readlines():
            tokens = i.strip().split(" ")
            conn,val = create(tokens)
            data[conn] = val
    print(data['a'](data))

if __name__ == "__main__":
    scenario_1()
