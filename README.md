# eggnoggplusplus #

This little script replaces the map "BUBBLE BATTLEFIELD" by the map of your design for the game eggnogg+: "https://madgarden.itch.io/eggnogg".
This script directly modify the binary code, it is not of great beauty.

Works on Linux and Windows. Require python3.

## How to use ##
0. Continue to read this README, there is some useful information
1. Download this github repository on your computer and go inside the folder `eggnoggplusplus` 
2. Create a map in the folder "maps", copying the syntax from the other examples
3. Change the file `main.py` to use your new map
4. Run `main.py` with python3. (`python3 main.py` on linux)
5. Done. The eggnogg binary in folder "my_eggnogg_linux" or "my_eggnogg_windows" has the new map instead of "BUBBLE BATTLEFIELD". (Go to the directory "my_eggnogg-\*", then run `eggnoggplus`). If you run into error "segmentation fault", it it probably because your map didn't respect the format decribed below.


## Limitations ##
- You can only replace one map at a time, and only "BUBBLE BATTLEFIELD".
- You have to make a 6-panel map (of size 12\*33, as in examples). You only design the left part of the map, and the right part will be symmetric. 
- You must strictly respect the format in the .map files. No extra whitespace, no extra character, nothing.

Most of these limitations are because I did not find the meta-data part in the binary, but it should be somewhere.

## The map format ##
The .map format describes maps in a textual manner. You can edit them with any text editor.
A map is a list of (6) panels. Each panel is constituted of 12\*33 blocks, each block is represented by the following characters.
### Block codes ###
- Empty:   (space)
- Ground: @
- Spikes downwards: v
- Spikes upwards: X
- Bomb: m
- wave (the kind that make you win): ^
- wave (the kind that kill you): w
- Eggnogg: E
- Vertical line (decoration): I, F
- Sun (decoration): O
- Flowers (decoration): Q
- Matrix (decoration): f
- Some more..

## Share your nice maps or extend the program ##
If you improved the program, you can share it with us and ask for a merge request.

If you designed new nice maps, you can also ask for a merge request or simply paste your map here: "https://annuel2.framapad.org/p/eggnoggplusplusmaps" and I'll merge myself


## Repository overview ##
```
.
├── eggnoggplus-linux               -> standard linux game
│   └── ...
├── eggnoggplus-win                 -> standard windows game
│   └── ...
├── main.py                         -> The file you want to run
├── maps                            -> new maps 
│   ├── dejavu.map
│   └── map_bubble_classic.map
├── my_eggnogg_linux                -> generated game for linux 
│   └── ...
├── my_eggnogg_win                  -> generated game for windows 
│   └── ...
├── README.md                       -> you should read it
├── ressources                      -> a copy of the binaries
│   ├── eggnoggplus-linux
│   └── eggnoggplus-windows.exe
└── src                             -> source code of script
    ├── functions.py
    └── parameters.py
```
