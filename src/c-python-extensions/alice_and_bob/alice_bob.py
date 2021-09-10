import time
import random
import exercise


def timeit(method):

    def timed(*args, **kw):
        ts = time.time()
        result = method(*args, **kw)
        te = time.time()

        print ('%r %2.2f sec' % (method.__name__, te-ts))
        return result

    return timed


@timeit
def alice_and_bob_python(alice, bob):
	alice_points = 0
	bob_points = 0
	if len(alice) == len(bob):
		for a in alice:
			index_a = alice.index(a)
			if a > bob[index_a]:
				alice_points = alice_points + 1
			else:
				bob_points = bob_points + 1

		print (alice_points, bob_points)
		return alice_points, bob_points


@timeit
def alice_and_bob_c(alice, bob):
	return exercise.alice_and_bob(alice, bob)


alice = []
for a in range(1000000):
	x = random.randint(1,101)
	alice.append(x)

bob = []
for b in range(1000000):
	x = random.randint(1,101)
	bob.append(x)


alice_and_bob_python(alice, bob)
print ("#################################")
alice_and_bob_c(alice, bob)

