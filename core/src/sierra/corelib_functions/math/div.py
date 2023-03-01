# def invmod(a, b):
#     if a == 0:
#         return 0
#     elif b % a == 0:
#         return 1
#     else:
#         return b - invmod(b % a, a) * b // a


# print(
#     (6 * invmod(3, 3618502788666131213697322783095070105623107215331596699973092056135872020481))
#     % 3618502788666131213697322783095070105623107215331596699973092056135872020481
# )
# print(
#     6
#     * pow(6, -1, 3618502788666131213697322783095070105623107215331596699973092056135872020481)
#     % 3618502788666131213697322783095070105623107215331596699973092056135872020481
# )
prime = 3618502788666131213697322783095070105623107215331596699973092056135872020481


def modular_inverse(a, m):
    """
    Calcule l'inverse modulaire de a modulo m en utilisant des soustractions répétées.
    Retourne l'inverse modulaire si il existe, sinon retourne None.
    """

    # Initialisation
    x = 0
    y = 1
    r = m
    s = a
    # Boucle de soustractions répétées
    while s != 0:
        q = r // s
        print(q, s)
        r, s = s, r - q * s
        x, y = y, x - q * y
        # print(q % prime)

    # Si r = pgcd(a, m) = 1, alors l'inverse modulaire est x
    return x % m


6 * modular_inverse(2, prime) % prime
