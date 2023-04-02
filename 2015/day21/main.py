#!/usr/bin/python3
from math import ceil,floor

Weapons = {
        "Dagger":     ( 8,4,0),
        "Shortsword": (10,5,0),
        "Warhammer":  (25,6,0),
        "Longsword":  (40,7,0),
        "Greataxe":   (74,8,0),
           }
Armour = {
        "Naked":      (0,0,0),
        "Leather":    (13,0,1),
        "Chainmail":  (31,0,2),
        "Splintmail": (53,0,3),
        "Bandedmail": (75,0,4),
        "Platemail":  (102,0,5),
}

Rings ={ 
        "The one ring": (0,0,0),
        "The two ring": (0,0,0),
        "Damage +1  " :  (25 ,1, 0),
        "Damage +2  " :  (50 ,2, 0),
        "Damage +3  " :  (100 ,3, 0),
        "Defense +1 " :  (20 ,0, 1),
        "Defense +2 " :  (40 ,0, 2),
        "Defense +3 " :  (80 ,0, 3),
        }

def sim_battle(me,boss):
    me_true_dam = (me["dam"] - boss["arm"])
    if me_true_dam <= 0:
        me_true_dam = 1
    boss_true_dam = (boss["dam"] - me["arm"])
    if boss_true_dam <= 0:
        boss_true_dam = 1
    boss_soak = ceil(boss["hp"] / me_true_dam)
    me_soak = ceil(me["hp"] / boss_true_dam)
    return me_soak >= boss_soak

def sim_battle2(me,boss):
    me_true_dam = (me["dam"] - boss["arm"])
    if me_true_dam <= 0:
        me_true_dam = 1
    boss_true_dam = (boss["dam"] - me["arm"])
    if boss_true_dam <= 0:
        boss_true_dam = 1
    boss_soak = ceil(boss["hp"] / me_true_dam)
    me_soak = ceil(me["hp"] / boss_true_dam)
    return me_soak < boss_soak

def get_cost(loadout:dict):
    dam = loadout["w"][1] + loadout["r1"][1] + loadout["r2"][1]
    arm = loadout["a"][2] + loadout["r1"][2] + loadout["r2"][2]
    cost =loadout["w"][0] +  loadout["a"][0] + loadout["r1"][0] + loadout["r2"][0]
    return {"dam":dam,"arm":arm,"cost":cost,"hp":100}


def find_perm_min(boss_stats):
    mincost = 1000
    for w in Weapons:
        for a in Armour:
            for r1 in Rings:
                for r2 in Rings:
                    if r1 != r2:
                        loadout = get_cost({"w":Weapons[w], "a":Armour[a],"r1":Rings[r1], "r2":Rings[r2]})
                        if sim_battle(loadout,boss_stats):
                            mincost = min(mincost,loadout["cost"])
    return mincost


def sceanrio1(fname):
    boss_stats = {}
    with open(fname) as f:
        for line in f.readlines():
            l = line[:-1].split(":")
            boss_stats[l[0]] = int(l[1])
    print(find_perm_min(boss_stats))

def find_perm_max(boss_stats):
    maxcost = 0
    for w in Weapons:
        for a in Armour:
            for r1 in Rings:
                for r2 in Rings:
                    if r1 != r2:
                        loadout = get_cost({"w":Weapons[w], "a":Armour[a],"r1":Rings[r1], "r2":Rings[r2]})
                        if sim_battle2(loadout,boss_stats):
                            maxcost = max(maxcost,loadout["cost"])
    return maxcost

def sceanrio2(fname):
    boss_stats = {}
    with open(fname) as f:
        for line in f.readlines():
            l = line[:-1].split(":")
            boss_stats[l[0]] = int(l[1])
    print(find_perm_max(boss_stats))


if __name__ == "__main__":
    sceanrio1("./input")
    # sceanrio1("./input1")
    sceanrio2("./input")
    # sceanrio2("./input1")
