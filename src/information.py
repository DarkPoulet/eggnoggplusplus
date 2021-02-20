EGGNOGG_FILENAME_LINUX = "ressources/eggnoggplus-linux"
MY_EGGNOGG_FILENAME_LINUX = "my_eggnogg_linux/eggnoggplus"

EGGNOGG_FILENAME_WIN = "ressources/eggnoggplus-windows.exe"
MY_EGGNOGG_FILENAME_WIN = "my_eggnogg_win/eggnoggplus.exe"

WIDTH_PANEL = 33
HEIGHT_PANEL = 12


class Map():
    def __init__(self, _name, _nb_panels, _gamemode, _width_panel, _size_author=0):
        self.name = _name
        self.nb_panels = _nb_panels
        self.nb_panels_to_design = (self.nb_panels//2) + 1
        self.gamemode = _gamemode
        self.size_author = _size_author
        self.size_title = len(self.name)
        self.size_header = self.size_author + (1 if self.size_author > 0 else 0) + self.size_title + 1
        self.width_panel = _width_panel


        
THE_BIG_STINKY = Map("THE BIG STINKY", 9, "SWORDS", 33, 0)
BUBBLE_BATTLEFIELD = Map("BUBBLE BATTLEFIELD", 11, "SWORDS", 33, 12)
SWORDSKETBRAWL = Map("SWORDSKETBRAWL", 1, "BASKET", 33, 0)
CLASSIC = Map("CLASSIC", 11, "SWORDS", 33, 0)
COMBAT_CAVERNS = Map("COMBAT CAVERNS", 7, "KARATE", 33, 0)




