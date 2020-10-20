import sys
sys.path.insert(1, "src")

from parameters import *
from functions import * 


## Available MAPS:
MAP_BUBBLE_CLASSIC = "maps/map_bubble_classic.map"
MAP_DEJAVU = "maps/dejavu.map"
MAP_HEHE = "maps/hehe.map"
MAP_BOOM = "maps/boom.map"
MAP_EMPTY = "maps/vide.map"
MAP_HEHE2 = "maps/hehe2.map"
MAP_TMP = "maps/tmp.map"
### Add your map name here
OTHER_MAP="maps/???.map"

## Change here to use to variable you defined just above
the_map_you_want_to_use = MAP_HEHE


# Do not modify this if you just want to use the script
print("Replacing bubble map by " + the_map_you_want_to_use + ".")
map = read_map(the_map_you_want_to_use)
write_map(map)
print("Done.")

demo_read_map("COMBAT", 5)





