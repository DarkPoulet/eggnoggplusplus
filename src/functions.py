from parameters import *


# From title and author in string, gets the title bytestring.
def compute_title(first_part, second_part) : 
	while len(first_part) > SIZE_FIRST_PART_TITLE : 
		first_part = first_part[:-1]
	while len(first_part) < SIZE_FIRST_PART_TITLE : 
		first_part += "-"
	while(len(second_part) > SIZE_SECOND_PART_TITLE) : 
		second_part = second_part[:-1]
	while(len(second_part) < SIZE_SECOND_PART_TITLE) : 
		second_part += "-"
	title = first_part + chr(0) + second_part + chr(0)
	return bytes(title, "utf-8")



# Write the binary
def write_my_eggnogg(my_eggnogg) : 
	output = open(MY_EGGNOGG_FILENAME, "wb")
	output.write(my_eggnogg)
	output.close()


# Takes a bytestring map and puts it in eggnogg instead of the bubble map.
def write_map(map) : 
	binary = open(EGGNOGG_FILENAME, 'rb').read()

	bubble_index = binary.find(b"BUBBLE")
	index_after_map_bubble = bubble_index + SIZE_TITLE + NB_PANELS * WIDTH_PANEL * HEIGHT_PANEL + NB_PANELS
	new_binary = binary[:bubble_index] + map + binary[index_after_map_bubble:]
	write_my_eggnogg(new_binary)


# Read a map from a .map file and converts it to bytestring.
def read_map(map_filename) : 
	data = open(map_filename, "r").read()
	line_title, _, map = data.split("\n", 2)

	first_part_title,second_part_title = line_title.split("#")
	title = compute_title(first_part_title, second_part_title)
	
	map = map.replace("\n", "")
	map = map.replace("-"*WIDTH_PANEL, chr(0))
	map = bytes(map, "utf-8")

	entire_map = title + map
	return entire_map


# Useless, just to print nice things
def demo_read_map() : 
	binary = open(EGGNOGG_FILENAME, 'rb').read()
	bubble_index = binary.find(b"BUBBLE")

	print("Title: ")
	print(binary[bubble_index : bubble_index + SIZE_TITLE])
	print()

	print("Map:")
	starting_index_map = bubble_index + SIZE_TITLE
	for panel in range(NB_PANELS) : 
		print("-"*WIDTH_PANEL)
		offset_due_to_EOF = 1*panel
		starting_index_panel = starting_index_map + (panel * WIDTH_PANEL * HEIGHT_PANEL) + offset_due_to_EOF
		for i in range(HEIGHT_PANEL) : 
			starting_index_line = starting_index_panel + i*WIDTH_PANEL
			ending_index_line = starting_index_panel + (i+1)*WIDTH_PANEL
			print(binary[starting_index_line : ending_index_line])


