#!/usr/bin/env python3

import itertools, math

ALICES_SUPER_SECRET_MESSAGE = [65426, 79042, 53889, 42039, 49636, 66493, 41225, 58964,
126715, 67136, 146654, 30668, 159166, 75253, 123703, 138090,
118085, 120912, 117757, 145306, 10450, 135932, 152073, 141695,
42039, 137851, 44057, 16497, 100682, 12397, 92727, 127363,
146760, 5303, 98195, 26070, 110936, 115638, 105827, 152109,
79912, 74036, 26139, 64501, 71977, 128923, 106333, 126715,
111017, 165562, 157545, 149327, 60143, 117253, 21997, 135322,
19408, 36348, 103851, 139973, 35671, 93761, 11423, 41336,
36348, 41336, 156366, 140818, 156366, 93166, 128570, 19681,
26139, 39292, 114290, 19681, 149668, 70117, 163780, 73933,
154421, 156366, 126548, 87726, 41418, 87726, 3486, 151413,
26421, 99611, 157545, 101582, 100345, 60758, 92790, 13012,
100704, 107995]

# What do you call it when you're writing a shared-secret algorithm and really mess up the Git tree?
# A Diffie-Hellman diffing hell, man!

# I got the algorithm from: https://en.wikipedia.org/wiki/Diffie%E2%80%93Hellman_key_exchange
# (It looks like it has details on how one might crack it lower down the page, but I only looked
# at how you implement it.)

def crack_diffie(g, p, public):
    # This is the impossibly difficult part.
    # Real implementations of this use P in the neighborhood of 10^600.
    for guess in range(p):
        if public == (g ** guess) % p:
            return guess
    raise Exception("oh dear did not find the secret")

def decode_diffie_hellman(g, p, alice2bob, bob2alice):
    # be my mirror, my sword and shield
    # my missionaries in a finite field
    alice_key = crack_diffie(g, p, alice2bob)
    bob_key = crack_diffie(g, p, bob2alice)

    alices_shared = (bob2alice ** alice_key) % p
    bobs_shared = (alice2bob ** bob_key) % p
    if alices_shared != bobs_shared:
        raise Exception(f"something went wrong, I think Alice's shared secret is {alices_shared} but Bob's is {bobs_shared}")

    return alices_shared


def factor(n):
    for p in range(2, int(math.sqrt(n) + 1)):
        if n % p == 0:
            return (p, n // p)
    raise Exception(f"could not factor {n}")

# If no one falls for my link in the Slack I'm going to be very sad
# Also, I don't think this cracking works if the modulus has multiple prime factorizations
def crack_rsa(pubkey, mod):
    p, q = factor(mod)
    for d_guess in range(1, mod):
        if (pubkey * d_guess) % ((p - 1) * (q - 1)) == 1:
            break
    else:
        raise Exception("Did not find d")

    return d_guess


def decode_rsa(privkey, mod, msg):
    decoded = [(x ** privkey) % mod for x in msg]
    # From reading the raw output of the message, I note that each number output is under 32768.
    # This is 2^15.
    # Given that ASCII always has the high bit unset, I'm guessing that each number here is 2 packed
    # bytes...
    text = []
    for short in decoded:
        high = short >> 8
        low = short & 0xff
        text += [high, low]
    return "".join(chr(x) for x in text)
    


if __name__ == "__main__":
    secret = decode_diffie_hellman(7, 61, 30, 17)
    print(f"Shared 'secret': {secret}")
    # I suppose one weakness of RSA is, if you *do* somehow find out the answer to any particular
    # message, you can impersonate Alice for the rest of time because decoding it gets you her privkey.

    privkey = crack_rsa(17, 170171)
    print(f"Alice's private key: {privkey}. Message:")
    # Amazingly, it takes LONGER to decode the message than it does to crack Alice's key.
    # My guess is this because arbitrary-precision arithmetic is slow
    print(decode_rsa(privkey, 170171, ALICES_SUPER_SECRET_MESSAGE))

