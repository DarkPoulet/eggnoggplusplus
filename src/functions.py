from information import *


# From title and author in string, gets the title bytestring.
def compute_title(first_part, second_part, size_first_part, size_second_part):
    while len(first_part) > size_first_part:
        first_part = first_part[:-1]
    while len(first_part) < size_first_part:
        first_part += "-"
    while(len(second_part) > size_second_part):
        second_part = second_part[:-1]
    while(len(second_part) < size_second_part):
        second_part += "-"
    title = first_part + chr(0)
    if size_second_part > 0:
        title += second_part + chr(0)
    return bytes(title, "utf-8")


# Write the binary in the correct place.
def write_my_eggnogg(my_eggnogg, output_filename):
    output = open(output_filename, "wb")
    my_eggnogg = betises(my_eggnogg)
    output.write(my_eggnogg)
    output.close()


def write_map_in_this_binary(original_map, new_map, binary):
    index_map = binary.find(bytes(original_map.name, "utf-8"))
    index_after_map = index_map + original_map.size_header + original_map.nb_panels_to_design * original_map.width_panel * HEIGHT_PANEL + original_map.nb_panels_to_design
    new_binary = binary[:index_map] + new_map + binary[index_after_map:]
    return new_binary


def get_binary(binary_filename):
    binary = open(binary_filename, 'rb').read()
    return binary


# Transforms a line to a line of the correct size (WIDTH_PANEL)
def adjust_line_size(line, wanted_line_len):
    if len(line) < wanted_line_len:
        line = line + (" "*(wanted_line_len - len(line)))
    else:
        line = line[:wanted_line_len]
    return line


# Read a map from a .map file and converts it to bytestring.
def read_map(map_filename, original_map):
    data = open(map_filename, "r").read()
    line_title, _, map = data.split("\n", 2)

    first_part_title,second_part_title = line_title.split("#")
    title = compute_title(first_part_title, second_part_title, original_map.size_title, original_map.size_author)

    lines_map = [line for line in map.split("\n")]
    lines_map_corrected = [adjust_line_size(line, original_map.width_panel) for line in lines_map]
    map = "\n".join(lines_map_corrected)
    map = map.replace(".", " ")
    map = map.replace("\n", "")
    map = map.replace("-"*original_map.width_panel, chr(0))
    map = bytes(map, "utf-8")

    entire_map = title + map
    return entire_map

def replace_all_maps(instead_of):
    linux_binary = get_binary(EGGNOGG_FILENAME_LINUX)
    windows_binary = get_binary(EGGNOGG_FILENAME_WIN)
    for (original, path_my_map) in instead_of.items():
        name = original.name
        nb_panels = original.nb_panels_to_design
        my_map = read_map(path_my_map, original)
        linux_binary = write_map_in_this_binary(original, my_map, linux_binary)
        windows_binary = write_map_in_this_binary(original, my_map, windows_binary)

    write_my_eggnogg(linux_binary, MY_EGGNOGG_FILENAME_LINUX)
    write_my_eggnogg(windows_binary, MY_EGGNOGG_FILENAME_WIN)



def betises(binary):
    # print(binary.find(bytes("KARATE INFERNO" + chr(0), "utf-8")))
    # new_binary = binary.replace(bytes("CLASSIC", "utf-8"), bytes("KARATE INFERNO" + chr(0) + "CLASSIC", "utf-8"))
    # new_binary = binary.replace(bytes("KARATE INFERNO" + chr(0), "utf-8"), bytes("MEPEUU ENFERRR" + chr(0), "utf-8"))
    new_binary = binary
    return new_binary



# just prints nice things
def print_map(map):
    binary = open(EGGNOGG_FILENAME_LINUX, 'rb').read()
    keyword_index = binary.find(bytes(map.name, "utf-8"))
    #size_title = binary[keyword_index:].find(bytes(chr(0), "utf-8"))

    #print(size_title)

    print("Title: ")
    print(binary[keyword_index: keyword_index + map.size_header])
    print()

    print("previous : ", binary[keyword_index + map.size_header -3 : keyword_index + map.size_header])

    print("Map:")
    starting_index_map = keyword_index + map.size_header
    for panel in range(map.nb_panels_to_design):
        print("-"*WIDTH_PANEL)
        offset_due_to_EOF = 1*panel
        starting_index_panel = starting_index_map + (panel * WIDTH_PANEL * HEIGHT_PANEL) + offset_due_to_EOF
        for i in range(HEIGHT_PANEL):
            starting_index_line = starting_index_panel + i*WIDTH_PANEL
            ending_index_line = starting_index_panel + (i+1)*WIDTH_PANEL
            print(binary[starting_index_line: ending_index_line])
