import random
import uuid
import time
import numpy as np

feats = dict()
for _ in range(80_000_000):
    feats[str(uuid.uuid4())] = np.float32(random.random())

print("Done")

time.sleep(600)



# Python: 13306932K = 13gb
# Rust:   8078820K  = 8gb
# Diff:   