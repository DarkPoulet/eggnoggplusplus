import sys
sys.path.insert(1, "src")

from information import *
from functions import * 
from maps import *

print("Random generation..")
generate_new_random_maps()
replace_all_maps_for_random()
print()
print("Done.")

