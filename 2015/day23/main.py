import re as re
import itertools



class Instruction:
    def __init__(self, line: str):
        self.cmd = list(filter(lambda x: len(x) != 0, re.split(r' |,', line[:-1])))

    def do(self, ctxt):
        if self.cmd[0] == 'inc':
            ctxt[self.cmd[1]] += 1
            ctxt['ip'] += 1

        elif self.cmd[0] in ['tpl']:
            ctxt[self.cmd[1]] *= 3
            ctxt['ip'] += 1

        elif self.cmd[0] in ['hlf']:
            ctxt[self.cmd[1]] = ctxt[self.cmd[1]] // 2
            ctxt['ip'] += 1

        elif self.cmd[0] in ['jmp']:
            ctxt['ip'] = ctxt['ip'] + int(self.cmd[1])

        elif self.cmd[0] in ['jie']:
            if ctxt[self.cmd[1]] % 2 == 0:
                ctxt['ip'] = ctxt['ip'] + int(self.cmd[2])
            else:
                ctxt['ip'] += 1

        elif self.cmd[0] in ['jio']:
            if ctxt[self.cmd[1]] == 1:
                ctxt['ip'] = ctxt['ip'] + int(self.cmd[2])
            else:
                ctxt['ip'] += 1

        else:
            assert False, "unknown instruction " + str(self.cmd) + str(ctxt)

        return ctxt


def parse_input(file,ctxt):
    program = []
    with open(file) as f:
        for line in f.readlines():
            program.append(Instruction(line))
        while -1 < ctxt['ip'] and ctxt['ip'] < len(program):
            ctxt = program[ctxt['ip']].do(ctxt)
            # print(str(ctxt))
    return ctxt

if __name__ == "__main__":
    print("part1 ",str(parse_input("./input", {'a': 0, 'b': 0, 'ip': 0})))
    print("part2 ",str(parse_input("./input", {'a': 1, 'b': 0, 'ip': 0})))

