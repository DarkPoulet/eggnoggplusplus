import sys
sys.path.insert(1, "src")

from information import *
from functions import * 
from maps import *

# Enter here all your choices. Add new choices in file src/maps.py
instead_of = dict()
instead_of[THE_BIG_STINKY] = MAP_THE_BIG_STINKY
instead_of[BUBBLE_BATTLEFIELD] = MAP_HEHE
instead_of[SWORDSKETBRAWL] = MAP_SWORDSKETBRAWL
instead_of[CLASSIC] = MAP_CLASSIC
instead_of[COMBAT_CAVERNS] = MAP_COMBAT_CAVERNS


# Do not modify this if you just want to use the script
print("Replacing maps...")
replace_all_maps(instead_of)
print("Done.")


# map = CLASSIC
# print_map(map)
