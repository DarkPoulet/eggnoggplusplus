import sys
sys.path.insert(1, "src")

from information import *
from functions import * 
from maps import *

# Enter here all your choices. Add new choices in file src/maps.py
instead_of = dict()
instead_of[THE_BIG_STINKY] = MAP_THE_BIG_STINKY
instead_of[BUBBLE_BATTLEFIELD] = MAP_OMNISKILL
instead_of[SWORDSKETBRAWL] = MAP_RICOCHET
instead_of[CLASSIC] = MAP_WAOU
instead_of[COMBAT_CAVERNS] = MAP_HANDMASTER


# Do not modify this if you just want to use the script
print("Replacing:")
for (old, new) in instead_of.items():
    print(old.name.ljust(20) + "-->  " + new)
replace_all_maps(instead_of)
print()
print("Done.")
