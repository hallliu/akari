"""
Puzzle generation routines for Akari.
"""

import random

class GridSquare():
    location = None
    is_lit = False
    is_solid = False
    is_light = False
    number_constraint = None

    # Ordered starting at the top going clockwise.
    neighbors = [None, None, None, None]

    def __init__(self, location):
        self.location = location

    def set_neighbors(self, loc_table):
        self.neighbors[0] = loc_table[(self.location[0] - 1, self.location[1])]
        self.neighbors[1] = loc_table[(self.location[0], self.location[1] + 1)]
        self.neighbors[2] = loc_table[(self.location[0] + 1, self.location[1])]
        self.neighbors[3] = loc_table[(self.location[0], self.location[1] - 1)]

    """
    Assumes that the neighbors have been initialized.
    Yields all squares within sight.
    """
    def get_sight_line(self):
        sight_line = []
        for nbr_dir, nbr in enumerate(self.neighbors):
            curr_square = nbr
            while curr_square is not None:
                if curr_square.is_solid:
                    break
                yield curr_square
                curr_square = curr_square.get_neighbor(nbr_dir)

    def get_num_surrounding_lights(self):
        result = 0
        for neighbor in self.neighbors:
            if neighbor is not None and neighbor.is_light:
                result += 1

        return result

    def get_neighbor(self, direction):
        return self.neighbors[direction]

    def __str__(self):
        if self.number_constraint is not None:
            return str(self.number_constraint)
        if self.is_solid:
            return "X"
        return "_"


def make_empty_grid(height, width):
    table = {}
    for v, h in zip(range(height), range(width)):
        table[(v, h)] = GridSquare((v, h))

    for square in table.values():
        square.set_neighbors(table)

    return table

def make_unlit_grid(height, width, density):
    table = make_empty_grid(height, width)
    for square in table.values():
        if density > random.random():
            square.is_solid = True

    return table
