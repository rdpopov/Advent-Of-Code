#!/bin/python3

INSTANT = 0
DOT = 1
BUFF = 2
BOSS_DAMAGE = 10
def real_input():
    return [500,71,50,0], 10

def test1():
    return [250,13,10,0], 8

def test2():
    return [250,14,10,0], 8

def list_dif(l1,l2):
    return [i1-i2  for i1,i2 in zip(l1,l2)]


ARM = 3
MANA = 4
DAMAGE =2
MANA_USED = 3
DURATION = 5
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


spell_names = [
        "Shield ",
        "Poison ",
        "Recharge ",
        "Magic Missile ",
        "Drain "
        ]

def spells_chain_repr(trail):
    res = ""
    for i in trail:
        res += spell_names[i]
    return res

#        mana, bosshp, hp, mana used
# player = [500, 71,     50, 0]

def tick_buffs(state,buffs):
    state[MP] += int(buffs[2] > 0) * spells[2][MANA]
    state[BOSS_HP] -= int(buffs[1] > 0) * spells[1][BOSS_HP] # boss hp is the same as damage
    buffs = list(map(lambda x: max(0,x-1),buffs))
    return state, buffs

def can_cast(state,buffs,spell_idx):
    res = True
    if spell_idx < len(buffs):
        res = res and buffs[spell_idx] == 0
    return res and state[MP] > spells[spell_idx][MP]

def cast(state,buffs,spell_idx):
    if spell_idx < len(buffs):
        buffs[spell_idx] = spells[spell_idx][DURATION]
        state[MP]      -= spells[spell_idx][MP]
        state[MANA_USED] += spells[spell_idx][MP]
    else:
        state[MP]      -= spells[spell_idx][MP]
        state[BOSS_HP] -= spells[spell_idx][BOSS_HP]
        state[HP]      -= spells[spell_idx][HP]
        state[MANA_USED] += spells[spell_idx][MP]
    return state, buffs

def recive_attack(state,buffs,dam):
    state[HP] -= max(1, dam - int(buffs[0] > 0) * spells[0][ARM]) # max function is not strictly needed since least boss damage is 3
    return state

class State:
    def __init__(self,state,buffs,trail = None):
        self.state = state
        self.buffs = buffs
        self.trail = trail or []

    def is_win(self) -> bool:
        return (self.state[BOSS_HP] <= 0)

    def is_fail(self) -> bool:
        return self.state[HP] <= 0 and self.state[MANA] < 53

    def simple_key(self):
        return self.state[MANA_USED]

    def to_key(self):
        return str([self.state[MP], self.state[MANA_USED]])

    def __str__(self):
        return 'State: {} is_win: {} casted: {}'.format(self.state,self.is_win(),spells_chain_repr(self.trail))

    def __lt__(self, other):
        return self.state[3] < other.state[3]

    def __gt__(self, other):
        return self.state[3] > other.state[3]





def generate_future_states_part1(_state, _buffs, spell_idx,trail,dam, depth):
    # player turn
    state = _state.copy()
    buffs = _buffs.copy()

    state, buffs = tick_buffs(state, buffs)
    if can_cast(state, buffs, spell_idx):
        state, buffs = cast(state, buffs, spell_idx)
        #boss turn
        # print(state , buffs)
        state, buffs = tick_buffs(state, buffs)
        # print(state , buffs)
        if state[BOSS_HP] <= 0:
            yield State(state, buffs,trail)
        else:
            state = recive_attack(state, buffs,dam)
            if state[HP] > 0:
                if depth == 0:
                    yield State(state, buffs,trail)
                else:
                    for sp_i, _ in enumerate(spells):
                        yield from generate_future_states_part1(state, buffs, sp_i, trail + [sp_i] ,dam, depth - 1)

def generate_future_states_part2(_state, _buffs, spell_idx,trail,dam, depth):
    # player turn
    state = _state.copy()
    buffs = _buffs.copy()

    state[HP] -= 1

    if state[HP] > 0:
        state, buffs = tick_buffs(state, buffs)
        if can_cast(state, buffs, spell_idx):
            state, buffs = cast(state, buffs, spell_idx)
            #boss turn
            # print(state , buffs)
            state, buffs = tick_buffs(state, buffs)
            # print(state , buffs)
            if state[BOSS_HP] <= 0:
                yield State(state, buffs,trail)
            else:
                state = recive_attack(state, buffs,dam)
                if state[HP] > 0:
                    if depth == 0:
                        yield State(state, buffs,trail)
                    else:
                        for sp_i, _ in enumerate(spells):
                            yield from generate_future_states_part2(state, buffs, sp_i, trail + [sp_i] ,dam, depth - 1)

def find_min_idx(lst:list,idx):
    res = 0
    for i,l in enumerate(lst):
        if l.state[idx] < lst[res].state[idx]:
            res = i
    return lst[res]

def find_max_idx(lst:list,idx):
    res = 0
    for i,l in enumerate(lst):
        if l.state[idx] > lst[res].state[idx]:
            res = i
    return lst[res]

# some states might be identical but only differ in an inconsequential inversion in spell order,
# say cast drain or magic missile and the inverse.
# the order doesnt really matter we cant overheal but hte state is the same.
# that would introduce needless branching.
# also, a state where it deffers on health or boss health,
# only the one with the smallest boss health and biggest own health should be taken
# as they are most sucessful

def prune_states(states):
    dict_res = {}
    res = []
    for st in states:
        key = st.simple_key()
        if key not in dict_res:
            dict_res[key] = []
        dict_res[key].append(st)

    for k,v in dict_res.items():
        if len(v) > 1:
            # could still produce duplicates
            res.append(find_min_idx(v,BOSS_HP))
            res.append(find_max_idx(v,HP))
        else:
            res.append(v[0])
    return res



def solve(state,depth,gen_function):
    won = []
    dam = state[1]
    # the player can survive at the absolute best 50 turns
    # then the boss deals net damage of 1 to player
    # which is also unacheavable, full uptime of Shield and Recharge and
    # also casting drain every turn, and that can possibly happen. 
    # also boss will be dead at about 
    states = [State(state[0], [0, 0, 0])]
    while 1:
        if len(states) == 0:
            break
        current_states = []
        for start_state in states:
            for sp_idx, _ in enumerate(spells):
                for st in gen_function(start_state.state,
                                                 start_state.buffs,
                                                 sp_idx,
                                                 start_state.trail + [sp_idx],
                                                 dam,
                                                 depth):
                    if st.is_win():
                        won.append(st)
                    else:
                        current_states.append(st)
        print("Unpruned {} Pruned {}".format(len(current_states),len(prune_states(current_states)))) #wow is there a difference, a good speedup
        # print("Unpruned {} Pruned {}".format(len(won),len(prune_states(won)))) #wow is there a difference, a good speedup
        states = prune_states(current_states)
        # states = current_states
        won = prune_states(won)

    ret = won[0].state[3]
    for i in won:
        ret = min(ret, i.state[3])
    return ret

if __name__ == "__main__":
    # part 1
    print("Part 1",solve(real_input(),7,generate_future_states_part1))
    print("Part 2",solve(real_input(),7,generate_future_states_part2))
