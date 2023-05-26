def triangular(row, col):
    upto = row + col - 2
    return (upto * (upto+1)) // 2 + col


def modular_exp(base, mod, power):
    p = 1
    res = 1
    crnt_mod = base % mod
    while p < power:
        if (p & power) > 0:
            res = (res * crnt_mod) % mod
        crnt_mod = (crnt_mod * crnt_mod) % mod
        p = p << 1
    return res


def solve(row, col):
    base0 = 20151125
    base1 = 252533
    mod = 33554393
    power = triangular(row, col)
    mod_exp = modular_exp(base1, mod, power-1)
    return ((base0 % mod) * (mod_exp % mod)) % mod


if __name__ == "__main__":
    print("part1 ", solve(row=2981, col=3075))
