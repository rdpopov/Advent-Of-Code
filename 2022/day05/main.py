#!/usr/bin/python3

def execute_commands(stack, cmd,rev = False):
    fr = cmd["from"]
    to = cmd["to"]
    n  = cmd["n"]
    a = stack[fr][:n]
    if rev:
        a.reverse()
    stack[to] = a + stack[to]
    stack[fr] = stack[fr][n:]


def get_head(stack):
    ans = []
    # print(str(stack))
    for i in range(1,len(stack)+1):
        ans.append(stack[i][0])
    return ''.join(ans)



def scenario1(fname):
    stacks = {}
    commands = []
    read_cmd = False

    with open(fname) as f:
        for i in f.readlines():
            line = i[:-1].split()
            if len(line) == 0:
                read_cmd = True
                continue
            if not read_cmd: 
                stacks[int(line[1])] = [crate for crate in line[0]]
            else:
                cmd = { "from": int(line[3]), "to": int(line[5]), "n": int(line[1]), }
                commands.append(cmd)

    for comm in commands:
        execute_commands(stacks,comm,rev=True)
    print("scenario1",fname,get_head(stacks))



def scenario2(fname):
    stacks = {}
    commands = []
    read_cmd = False

    with open(fname) as f:
        for i in f.readlines():
            line = i[:-1].split()
            if len(line) == 0:
                read_cmd = True
                continue
            if not read_cmd: 
                stacks[int(line[1])] = [crate for crate in line[0]]
            else:
                cmd = { "from": int(line[3]), "to": int(line[5]), "n": int(line[1]), }
                commands.append(cmd)

    for comm in commands:
        execute_commands(stacks,comm,rev=False)
    print("scenario2",fname,get_head(stacks))



if __name__ == "__main__":
    scenario1("./input1")
    scenario1("./input")
    scenario2("./input1")
    scenario2("./input")
