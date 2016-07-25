module Grid exposing (..)
import Dict
import Dict exposing (Dict)
import String

type alias Location = (Int, Int)

type CellContents =
    Empty |
    Lit |
    CantLight |
    LitAndCantLight |
    Light |
    BadLight |
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

type alias Cell = {
    location: Location,
    contents: CellContents,
    litCount: Int
    up_nbr: Maybe Location,
    down_nbr: Maybe Location,
    left_nbr: Maybe Location,
    right_nbr: Maybe Location
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
        newCell = {
            location = loc, contents = getCellContents c, litCount = 0,
            up_nbr = Nothing,
            down_nbr = Nothing,
            left_nbr = Nothing,
            right_nbr = Nothing}
    in
        (loc, newCell)

putNeighborsInCell: Cell -> Grid -> Cell
putNeighborsInCell cell grid = 
    let
        getNeighbor: Direction -> Maybe Location 
        getNeighbor dir = moveFromLocation cell.location grid.height grid.width dir
    in
        {cell |
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
        populateWithNeighbors {cells = g, height = height, width = width}

getSightLine: Grid -> Cell -> List Cell
getSightLine grid cell =
    let
        directions: List (Cell -> Maybe Location)
        directions = [.up_nbr, .down_nbr, .left_nbr, .right_nbr]

        getNeighbor: Cell -> (Cell -> Maybe Location) -> Maybe Cell
        getNeighbor c dirFn = Maybe.andThen (dirFn c) (\l -> Dict.get l grid.cells)

        sightLineHelper: List Cell -> (Cell -> Maybe Location) -> Maybe Cell -> List Cell
        sightLineHelper cells dirFn currCell = case currCell of
            Just cc -> case cc.contents of
                Solid -> cells
                Constraint _ -> cells
                _ -> sightLineHelper (cc :: cells) dirFn (getNeighbor cc dirFn)
            Nothing -> cells

        getSightLineInDirection: (Cell -> Maybe Location) -> List Cell
        getSightLineInDirection dirFn = sightLineHelper [] dirFn (getNeighbor cell dirFn)
    in
        List.concat <| List.map getSightLineInDirection directions

markCellAsLit: Cell -> Cell
markCellAsLit cell =
    let newContents = case cell.contents of
        Empty -> Lit
        Lit -> Lit
        CantLight -> LitAndCantLight
        LitAndCantLight -> LitAndCantLight
        Light -> BadLight
        BadLight -> BadLight
        Solid -> Solid
        Constraint x -> Constraint x
    in
        {cell | litCount = cell.litCount + 1, contents = newContents}

markCellAsLight: Cell -> Maybe Cell
markCellAsLight cell =
    let newContents = case cell.contents of
        Empty -> Just Light
        Lit -> Just BadLight
        CantLight -> Just Light
        LitAndCantLight -> Just BadLight
        Light -> Nothing
        BadLight -> Nothing
        Solid -> Nothing
        Constraint _ -> Nothing
    in case newContents of
        Just x -> {cell | contents = x}
        Nothing -> Nothing

putLight: Grid -> Location -> Grid
putLight grid loc =
    let 
        newAtLoc = Maybe.andThen (Dict.get grid.cells loc) markCellAsLight
        sightLine = case newAtLoc of
            Just c -> getSightLine grid c
            Nothing -> Nothing
        newCells = Maybe.map (List.map markCellAsLit) sightLine


gridToString: Grid -> String
gridToString grid =
    let
        getSingleCellChar: Location -> Char
        getSingleCellChar loc = Maybe.withDefault '?' <|
            Maybe.map (printCellContents << .contents) <|
                Dict.get loc grid.cells 

        gridLineToCharList: Int -> List Char
        gridLineToCharList lineNum = List.map getSingleCellChar <|
            List.map ((,) lineNum) [0..grid.width - 1]
    in
        (String.fromList << List.concat << List.map gridLineToCharList) [0..grid.height - 1]

printCellContents: CellContents -> Char
printCellContents c = case c of
    Empty -> '_'
    Lit -> '#'
    Light -> '*'
    CantLight -> '^'
    LitAndCantLight -> '#'
    BadLight -> '!'
    Solid -> 'X'
    Constraint x -> case String.uncons <| toString x of
        Just (c, _) -> c
        Nothing -> '9'

getCellContents: Char -> CellContents
getCellContents c = case c of
    'X' -> Solid
    '0' -> Constraint 0
    '1' -> Constraint 1
    '2' -> Constraint 2
    '3' -> Constraint 3
    '4' -> Constraint 4
    _ -> Empty

