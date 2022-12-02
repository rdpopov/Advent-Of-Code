#!/usr/bin/python3
tr_table = {
        "AX" : 4 ,
        "AY" : 8 ,
        "AZ" : 3 ,
        "BX" : 1 ,
        "BY" : 5 ,
        "BZ" : 9 ,
        "CX" : 7 ,
        "CY" : 2 ,
        "CZ" : 6 ,
        }

tr_table_adj = {
        "AX" : tr_table['AZ'] ,
        "AY" : tr_table['AX'] ,
        "AZ" : tr_table['AY'] ,
        "BX" : tr_table['BX'] ,
        "BY" : tr_table['BY'] ,
        "BZ" : tr_table['BZ'] ,
        "CX" : tr_table['CY'] ,
        "CY" : tr_table['CZ'] ,
        "CZ" : tr_table['CX'],
        }

def score(f,s,table):
    return table[f+s]


def scenario1(fname):
    res = 0
    with open(fname) as f:
        for i in f.readlines():
            line = i[:-1].split()
            result = tr_table[line[0]+line[1]] 
            res += result

    print("scenario1",fname+ "\t",res)


def scenario2(fname):
    res = 0
    with open(fname) as f:
        for i in f.readlines():
            line = i[:-1].split()
            result = tr_table_adj[line[0]+line[1]] 
            res += result
    print("scenario2",fname+ "\t",res)

if __name__ == "__main__":
    # test_score()
    scenario1("./input")
    scenario1("./input1")
    scenario2("./input")
    scenario2("./input1")
