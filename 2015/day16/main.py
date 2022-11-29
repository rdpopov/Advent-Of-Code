#!/usr/bin/python3
results = {
            "children": 3,
            "cats": 7,
            "samoyeds": 2,
            "pomeranians": 3,
            "akitas": 0,
            "vizslas": 0,
            "goldfish": 5,
            "trees": 3,
            "cars": 2,
            "perfumes": 1,
        }
ranges_less = ["pomeranians","goldfish"]
ranges_greater = ["trees","casts"]

class Sue:
    def __init__(self,line):
        l = (line[:-1] + ",").split()
        self.nr = int(l[1][:-1])
        self.attr = {}
        for n in range(3):
            key = l[(n+1)*2][:-1]
            value = int(l[((n+1)*2 +1)][:-1])
            self.attr[key] = value


    def likelyhood_exact(self):
        score = 0
        fail = False
        for i in results:
            if i in self.attr:
                if results[i] == self.attr[i]:
                    score += 1
                else:
                    fali = True

        return 0 if fail else score

    def likelyhood_drunk(self):
        score = 0
        fail = False
        for i in results:
            if i in self.attr:
                if i in ranges_less:
                    if results[i] > self.attr[i]:
                        score += 1
                elif i in ranges_greater:
                    if results[i] < self.attr[i]:
                        score += 1
                elif results[i] == self.attr[i]:
                    score += 1
                else:
                    fali = True

        return 0 if fail else score

    def __repr__(self):
        return "{} - {}".format(self.nr,str(self.attr))

        


def sceanrio1():
    max_likely = []
    with open("./input") as f:
        for i in f.readlines():
            tmp = Sue(i)
            if tmp.likelyhood_exact() == 3:
                max_likely.append(tmp)
        print(len(max_likely),max_likely,sep="\n")

def sceanrio2():
    max_likely = []
    with open("./input") as f:
        for i in f.readlines():
            tmp = Sue(i)
            if tmp.likelyhood_drunk() == 3:
                max_likely.append(tmp)
        print(len(max_likely),max_likely,sep="\n")


if __name__ == "__main__":
    sceanrio1()
    sceanrio2()
