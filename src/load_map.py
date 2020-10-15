from parameters import *
from functions import * 


the_map_you_want_to_use = MAP_DEJAVU

print("Replacing bubble map by " + the_map_you_want_to_use + ".")
map = read_map(the_map_you_want_to_use)
write_map(map)
print("Done.")

# demo_read_map()





