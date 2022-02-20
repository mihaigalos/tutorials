import random
import time

list1 = [random.randint(0,999) for _ in range(2_000_000)]

print("Started")
start_time = time.time()
list1.sort()
print("--- Done in %s seconds ---" % (time.time() - start_time))
