INSTANT = 0
DOT = 1
BUFF = 2
BOSS_DAMAGE = 10
def real_input():
    return [500,71,50], 10

def test1():
    return [250,13,10], 8

def test2():
    return [250,14,10], 8


ARM = 3
MANA = 4
MANA_USED = 3
BOSS_HP = 1
HP = 2
MP = 0

spells = [                                          # not sure if i will need type
        # cost, damage, healing, arm, mana, duration, type
        [113,   0,      0,       7,   0,    6,        BUFF],    #Shield
        [173,   3,      0,       0,   0,    6,        DOT],     #Poison
        [229,   0,      0,       0,   101,  5,        DOT],     #Recharge
        [53,    4,      0,       0,   0,    0,        INSTANT], #Magic Missile
        [73,    2,      -2,      0,   0,    0,        INSTANT], #Drain
]

#        mana, bosshp, hp, mana used
player = [500, 71,     50, 0]

class State:
    def __init__(self,state,buffs):
        self.state = state
        self.buffs = buffs

    def is_win(self) -> bool:
        return self.state[BOSS_HP] <= 0

    def is_fail(self) -> bool:
        return self.state[HP] <= 0 and self.state[MANA] < 53

def generate_future_states(state,buffs,spell_idx,depth):
    if depth == 0 or state

