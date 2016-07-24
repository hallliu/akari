module Grid exposing (..)
import Dict
import Dict exposing (Dict)

type alias Location = (Int, Int)

type CellContents =
    Empty |
    Lit |
    CantLight |
    LitAndCantLight |
    Light |
    Solid |
    Constraint Int

type Direction =
    Up |
    Down |
    Left |
    Right

moveFromLocation: Location -> Int -> Int -> Direction -> Maybe Location
moveFromLocation (y, x) h w d = case d of
   Up ->
       if y == 0 then
           Nothing
       else
           Just (y - 1, x)
   Down ->
       if y == (h - 1) then
           Nothing
       else
           Just (y + 1, x)
   Left ->
       if x == 0 then
           Nothing
       else
           Just (y, x - 1)
   Right ->
       if x == (w - 1) then
           Nothing
       else
           Just (y, x + 1)

type Cell = Cell {
    location: Location,
    contents: CellContents,
    up_nbr: Maybe Cell,
    down_nbr: Maybe Cell,
    left_nbr: Maybe Cell,
    right_nbr: Maybe Cell
}

type alias Grid = {
    cells: Dict Location Cell,
    height: Int,
    width: Int
} 

listProduct: List a -> List b -> List (a, b)
listProduct x y = List.concatMap (\a -> List.map (\b -> (a, b)) y) x

makeCell: Location -> Char -> (Location, Cell)
makeCell loc c =
    let
        newCell = Cell {
            location = loc, contents = getCellContents c,
            up_nbr = Nothing,
            down_nbr = Nothing,
            left_nbr = Nothing,
            right_nbr = Nothing}
    in
        (loc, newCell)

putNeighborsInCell: Cell -> Grid -> Cell
putNeighborsInCell (Cell cell) grid = 
    let
        locationToCell: Maybe Location -> Maybe Cell
        locationToCell lm = Maybe.andThen lm (\l -> Dict.get l grid.cells)

        getNeighbor: Direction -> Maybe Cell
        getNeighbor dir = locationToCell <| moveFromLocation cell.location grid.height grid.width dir
    in
        Cell {cell |
            up_nbr = getNeighbor Up,
            down_nbr = getNeighbor Down,
            right_nbr = getNeighbor Right,
            left_nbr = getNeighbor Left}

populateWithNeighbors: Grid -> Grid
populateWithNeighbors grid =
    let
        newCells = Dict.map (\_ c -> putNeighborsInCell c grid) grid.cells
    in
        {grid | cells = newCells}

makeGrid: Int -> Int -> List Char -> Grid
makeGrid height width data = 
    let 
        g = Dict.fromList <| List.map2 makeCell (listProduct [0..height - 1] [0..width - 1]) data
    in
        {cells = g, height = height, width = width}

getCellContents: Char -> CellContents
getCellContents c = case c of
    'X' -> Solid
    '0' -> Constraint 0
    '1' -> Constraint 1
    '2' -> Constraint 2
    '3' -> Constraint 3
    '4' -> Constraint 4
    _ -> Empty

