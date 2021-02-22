# eggnoggplusplus #

This little script replaces the maps you want by the maps of your design for the game eggnogg+: "https://madgarden.itch.io/eggnogg".
This script directly modify the binary code, it is not of great beauty.

Works on Linux and Windows. Requires python3.

## How to use ##
0. Continue to read this README, there is some useful information
1. Download this github repository on your computer and go inside the folder `eggnoggplusplus` 
2. Create a map in the folder "maps", either using the `mapeditor` or manually by copying the syntax from the other examples
3. Change the file `main.py` to use your new map
4. Run `main.py` with python3. (`python3 main.py` on linux)
5. Done. The eggnogg binary in folder "my_eggnogg_linux" or "my_eggnogg_windows" has the new maps according to what you changed in `main.py`. (Go to the directory "my_eggnogg-\*", then run `eggnoggplus`). If you run into error "segmentation fault", it is probably because your map didn't respect the format decribed below, or that you didn't go in the folder "my_eggnogg_*" before running eggnogg.


## Limitations ##
- You have to make a 6-panel map (of size 12\*33, as in examples). You only design the left part of the map, and the right part will be symmetric. 
- You must strictly respect the format in the .map files. Do not remove the maps separators "-------" or add extra blank lines, etc.

Most of these limitations are because I did not find the meta-data part in the binary, but it should be somewhere.

## The map format ##
The .map format describes maps in a textual manner. You can edit them with any text editor.
A map is a list of (6) panels. Each panel is constituted of 12\*33 blocks, each block is represented by the following characters.
### Block codes ###
#### Gameplay blocks ####
-  (space): empty/air
- @: ground
- _: ceil (functions like ground but display "top wall" even as a floor)
- !: floor (functions like ground ?)
- v: spikes downwards
- X: spikes upwards
- m: Mine
- *: sword
- K: spike ball
- ^: wave (the kind that make you win)
- E: eggnogg (makes you win)
- w: wave (the kind that kill you)
- W: black wave (that kill you) with a tide (3 blocks)

#### Decoration blocks ####
Small (not bigger than the block):
- #: wall bricks
- I: large column
- H: vertical zigzags
- F: Vertical line
- P: eggnogg ideogram (animated)
- S: big eye
- Q: skull on a pike
- q: wall-handcuffed skeleton
- =: unregular grid (transparent)
- x: regular grid (transparent)
- `: thinner regular grid (transparent)
- |: vertical large cylinder
- f: foggy matrix
- F: small vertical column
- :: vertical small cylinders
- -: horizontal small cylinders
- +: mushrooms
- ~: waterfall (with source in the upper block when free)
- (: left gate
- ): right gate
- u: little gate
- Z: plated metal
- l: simple lamp
Bigger and static:
- L: big provoking ideogram (2left*4up)
- N: big shield ideogram (2left*4up)
- Y: big drinking t-rex ideogram (2left*4up)
- G: big circular ideogram (3*3up (shifted 1up))
Bigger and moving:
- c: lamp (slow movement) (1*3down)
- C: lamp (fast movement) (3*3down)
- A: spinning wheel (3*3 centered)
- s: tentacle (1*6up)
- t: tentacle (1*6up)
- T: big tentacles (1*7up)
Background (different travelling)
- O: sun
- i: bleachers
- Some more..?

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
├── mapeditor
│   ├── editor.html                 -> an html/javascript map editor
│   └── ...
├── maps                            -> new maps 
│   ├── dejavu.map
│   ├── hehe.map                    -> excellent map - professional level required - 
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
