"""
Puzzle generation routines for Akari.
"""

import random

class Grid():
    height = -1
    width = -1
    squares = None

    def __init__(self, height, width, density):
        self.height = height
        self.width = width
        self.squares = {}
        for v in range(height):
            for h in range(width):
                self.squares[(v, h)] = GridSquare((v, h))

        for square in self.squares.values():
            square.set_neighbors(self.squares)
            if density > random.random():
                square.is_solid = True

    def populate_with_lights(self):
        unlit_locations = set(x.location for x in self.squares.values()
                              if not (x.is_lit or x.is_light or x.is_solid))
        while len(unlit_locations) > 0:
            location_to_light = random.sample(unlit_locations, 1)[0]
            self.squares[location_to_light].place_light()
            unlit_locations.remove(location_to_light)
            unlit_locations -= set(self.squares[location_to_light].get_sight_line())

    def set_constraints_full(self):
        for cell in (c for c in self.squares.values() if c.is_solid):
            if cell.get_num_surrounding_nonsolid_squares() > 0:
                cell.set_num_surrounding_lights()
                cell.has_number_constraint = True

    def __str__(self):
        return '\n'.join(
            ''.join(str(self.squares[(v, h)]) for h in range(self.width))
            for v in range(self.height)
        )

class GridSquare():
    def __init__(self, location):
        self.location = location
        self.is_lit = False
        self.is_solid = False
        self.is_light = False
        self.has_number_constraint = False
        self.num_surrounding_lights = -1 
        # Ordered starting at the top going clockwise.
        self.neighbors = [None, None, None, None]


    def set_neighbors(self, loc_table):
        self.neighbors[0] = loc_table.get((self.location[0] - 1, self.location[1]))
        self.neighbors[1] = loc_table.get((self.location[0], self.location[1] + 1))
        self.neighbors[2] = loc_table.get((self.location[0] + 1, self.location[1]))
        self.neighbors[3] = loc_table.get((self.location[0], self.location[1] - 1))


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

    def set_num_surrounding_lights(self):
        self.num_surrounding_lights = self._count_neighbors(lambda n: n.is_light)

    def get_neighbor(self, direction):
        return self.neighbors[direction]

    def get_num_surrounding_nonsolid_squares(self):
        return self._count_neighbors(lambda n: not n.is_solid)

    def place_light(self):
        if self.is_lit or self.is_light or self.is_solid:
            return
        for cell in self.get_sight_line():
            cell.is_lit = True
        self.is_light = True

    def _count_neighbors(self, matches):
        result = 0
        for neighbor in self.neighbors:
            if neighbor is not None and matches(neighbor):
                result += 1
        return result

    def __str__(self):
        if self.has_number_constraint:
            return str(self.num_surrounding_lights)
        if self.is_solid:
            return "X"
        if self.is_light:
            return "*"
        if self.is_lit:
            return "#"
        return "_"

    def __repr__(self):
        return str(self) + " at {}".format(self.location)

def main():
    g1 = Grid(17, 17, 0.3)
    g1.populate_with_lights()
    print(str(g1))

if __name__ == "__main__":
    main()
