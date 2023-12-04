import sys

def part1(fname: str, marble_count: dict) -> int:
    game_ids = []
    with open(fname) as inp:
        for i in inp.readlines():
            lst = i.split(":")
            invalid_bout = False
            game_id = int(lst[0].split(" ")[1])
            for bout in lst[1].split(";"):
                for ball in bout.split(","):
                    parsed_ball = ball.split()
                    if marble_count[parsed_ball[1]] < int(parsed_ball[0]):
                        invalid_bout = True

            if not invalid_bout:
                game_ids.append(game_id)
    return sum(game_ids)

def part2(fname: str) -> int:
    game_ids = []
    with open(fname) as inp:
        for i in inp.readlines():
            lst = i.split(":")
            invalid_bout = False
            game_id = int(lst[0].split(" ")[1])
            max_marbles = {"red": 1, "green": 1, "blue": 1}
            for bout in lst[1].split(";"):
                for ball in bout.split(","):
                    parsed_ball = ball.split()
                    max_marbles[parsed_ball[1]] = max(
                        int(parsed_ball[0]), max_marbles[parsed_ball[1]])
            game_ids.append(max_marbles["red"] * max_marbles["green"] *
                            max_marbles["blue"])
    return sum(game_ids)

def main():
    print(part1("test", {"red": 12, "green": 13, "blue": 14}))
    print(part1("input", {"red": 12, "green": 13, "blue": 14}))
    print(part2("test"))
    print(part2("input"))

main()
