"""
Puzzle generation routines for Akari.
"""

import random
import subprocess
import os

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

    def search_constraints(self, is_unique):
        self.set_constraints_full()
        cells_with_constraints = [x for x in self.squares.values()
                                  if x.num_surrounding_lights >= 0]
        random.shuffle(cells_with_constraints)
        # First try with all constraints on
        if not is_unique(self):
            return cells_with_constraints, None

        self.binary_search_to_constraints(cells_with_constraints, is_unique)
        self.incrementally_remove_constraints(cells_with_constraints, is_unique)

    def incrementally_remove_constraints(self, cell_list, is_unique):
        cell_set = set(x.location for x in cell_list)
        iters = 0
        while iters < 200 and len(cell_set) > 0:
            cell_to_unconstrain = random.sample(cell_set, 1)[0]
            self.squares[cell_to_unconstrain].has_number_constraint = False
            if is_unique(self):
                cell_set.remove(cell_to_unconstrain)
            else:
                self.squares[cell_to_unconstrain].has_number_constraint = True 
            iters += 1
        
    def binary_search_to_constraints(self, cell_list, is_unique):
        lower = 0
        upper = len(cell_list)
        mid = (upper - lower) // 2
        last_result = None

        while lower != mid: 
            last_result = self._is_unique_with_constraints(is_unique, cell_list, mid)
            if last_result:
                upper = mid
            else:
                lower = mid
            mid = lower + (upper - lower) // 2
        if last_result:
            return cell_list, mid
        else:
            self.set_num_constraints(cell_list, mid + 1)
            return mid + 1

    def set_num_constraints(self, cell_list, num_constrained):
        def set_cell_is_constrained_in_list(cells, is_constrained):
            for cell in cells:
                cell.has_number_constraint = is_constrained

        set_cell_is_constrained_in_list(cell_list[:num_constrained], True)
        set_cell_is_constrained_in_list(cell_list[num_constrained:], False)

    def _is_unique_with_constraints(self, is_unique, cell_list, num_constrained):
        self.set_num_constraints(cell_list, num_constrained)
        return is_unique(self)

    def __str__(self):
        return '\n'.join(
            ''.join(self.squares[(v, h)].to_canonical_string() for h in range(self.width))
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

    def to_canonical_string(self):
        if self.has_number_constraint:
            return str(self.num_surrounding_lights)
        if self.is_solid:
            return "X"
        return "_"

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

def call_solver_for_uniqueness(grid):
    env = dict(os.environ)
    env["SAT_SOLVER"] = "bin/glucose"
    sp = subprocess.Popen(["bin/akari_solver", "-u"], stdin=subprocess.PIPE,
                          stdout=subprocess.PIPE, universal_newlines=True, env=env)
    input_str = "{} {}\n{}".format(grid.height, grid.width, str(grid))
    res, _ = sp.communicate(input=input_str)
    return int(res) == 1
    
def generate_puzzle(height, width, density):
    best_grid = None
    best_ratio = 1
    for i in range(40):
        g1 = Grid(height, width, density)
        g1.populate_with_lights()
        g1.search_constraints(call_solver_for_uniqueness)

        num_solid = len([x for x in g1.squares.values() if x.is_solid])
        num_constrained = len([x for x in g1.squares.values() if x.has_number_constraint])
        curr_ratio = num_constrained / num_solid

        if curr_ratio < best_ratio:
            best_ratio = curr_ratio
            best_grid = g1

    return best_grid, best_ratio
