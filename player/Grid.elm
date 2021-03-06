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

type alias Cell = {
    location: Location,
    contents: CellContents,
    litCount: Int,
    up_nbr: Maybe Location,
    down_nbr: Maybe Location,
    left_nbr: Maybe Location,
    right_nbr: Maybe Location
}

type alias Grid = {
    initialState: Dict Location Cell,
    cells: Dict Location Cell,
    height: Int,
    width: Int
} 

-- Utility functions for creating and working with Cells

listProduct: List a -> List b -> List (a, b)
listProduct x y = List.concatMap (\a -> List.map (\b -> (a, b)) y) x

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

getCellContentsFromChar: Char -> CellContents
getCellContentsFromChar c = case c of
    'X' -> Solid
    '0' -> Constraint 0
    '1' -> Constraint 1
    '2' -> Constraint 2
    '3' -> Constraint 3
    '4' -> Constraint 4
    _ -> Empty

makeCell: Location -> Char -> (Location, Cell)
makeCell loc c =
    let
        newCell = {
            location = loc, contents = getCellContentsFromChar c, litCount = 0,
            up_nbr = Nothing,
            down_nbr = Nothing,
            left_nbr = Nothing,
            right_nbr = Nothing}
    in
        (loc, newCell)

-- Functions for changing how a cell is lit

castLightOnCell: Cell -> Cell
castLightOnCell cell =
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

uncastLightOnCell: Cell -> Cell
uncastLightOnCell cell =
    let 
        onlyLitOnce = cell.litCount == 1
        newLitCount = max 0 (cell.litCount - 1)
        newContents = case cell.contents of
            Empty -> Empty
            Lit ->
                if onlyLitOnce then
                    Empty
                else
                    Lit
            CantLight -> CantLight
            LitAndCantLight -> 
                if onlyLitOnce then
                    CantLight
                else
                    LitAndCantLight
            Light -> Light
            BadLight ->
                if onlyLitOnce then
                    Light
                else
                    BadLight
            Solid -> Solid
            Constraint x -> Constraint x
    in
        {cell | litCount = newLitCount, contents = newContents}

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
        Just x -> Just {cell | contents = x}
        Nothing -> Nothing

unmarkCellAsLight: Cell -> Maybe Cell
unmarkCellAsLight cell =
    let newContents = case cell.contents of
        Empty -> Nothing
        Lit -> Nothing
        CantLight -> Nothing
        LitAndCantLight -> Nothing
        Light ->
            if cell.litCount > 0 then
                Just Lit
            else
                Just Empty
        BadLight -> Just Lit
        Solid -> Nothing
        Constraint _ -> Nothing
    in case newContents of
        Just x -> Just {cell | contents = x}
        Nothing -> Nothing

markCellAsCantLight: Cell -> (Cell, Bool)
markCellAsCantLight cell = 
    let newContents = case cell.contents of
            Empty -> CantLight
            Lit -> LitAndCantLight
            CantLight -> Empty
            LitAndCantLight -> Lit
            Light -> Light
            BadLight -> BadLight
            Solid -> Solid
            Constraint x -> Constraint x
    in
        ({cell | contents = newContents}, isCellLight cell)

isCellLight: Cell -> Bool
isCellLight cell = case cell.contents of
    Light -> True
    BadLight -> True
    _ -> False

-- Functions for Grid creation

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
        {grid | initialState = newCells, cells = newCells}

makeGrid: Int -> Int -> List Char -> Grid
makeGrid height width data = 
    let 
        g = Dict.fromList <| List.map2 makeCell (listProduct [0..height - 1] [0..width - 1]) data
    in
        populateWithNeighbors {initialState = g, cells = g, height = height, width = width}

makeGridFromString: Int -> Int -> String -> Grid
makeGridFromString height width data =
    let 
        isValidChar: Char -> Bool
        isValidChar c =
            (c == '_')
            || (c == 'X')
            || (c == '0')
            || (c == '1')
            || (c == '2')
            || (c == '3')
            || (c == '4')

        trimmedData = String.filter isValidChar data
    in
        populateWithNeighbors <| makeGrid height width <| String.toList trimmedData

-- Functions for manipulating and getting information on a Grid 

replaceCellsInGrid: Grid -> List Cell -> Grid
replaceCellsInGrid grid newCells =
    let
        putCellInDict: Cell -> Dict Location Cell -> Dict Location Cell
        putCellInDict cell dict = Dict.insert cell.location cell dict

        newGridDict = List.foldr putCellInDict grid.cells newCells
    in
        {grid | cells = newGridDict}

lightToggleHelper: (Cell -> Maybe Cell) -> (Cell -> Cell) -> Location -> Grid -> Grid
lightToggleHelper locMarker sightMarker loc grid =
    let 
        newAtLoc = Maybe.andThen (Dict.get loc grid.cells) locMarker
        sightLine = case newAtLoc of
            Just c -> Just <| getSightLine grid c
            Nothing -> Nothing
        newSightLineCells = Maybe.map (List.map sightMarker) sightLine
        allNewCells = Maybe.map2 (::) newAtLoc newSightLineCells
    in
        case allNewCells of
            Just nc -> replaceCellsInGrid grid nc
            Nothing -> grid

putLight: Location -> Grid -> Grid
putLight = lightToggleHelper markCellAsLight castLightOnCell

removeLight: Location -> Grid -> Grid
removeLight = lightToggleHelper unmarkCellAsLight uncastLightOnCell

toggleLight: Location -> Grid -> Grid
toggleLight loc grid = case Dict.get loc grid.cells of
    Nothing -> grid
    Just cell -> 
        if isCellLight cell then
            removeLight loc grid
        else
            putLight loc grid

toggleCantLight: Location -> Grid -> Grid
toggleCantLight loc grid =
    let
        markedCell: Maybe (Cell, Bool)
        markedCell = Maybe.map markCellAsCantLight <| Dict.get loc grid.cells
    in
        case markedCell of
            Nothing -> grid
            Just (newCell, doRemoveLightFirst) ->
                if doRemoveLightFirst then
                    toggleCantLight loc <| removeLight loc grid
                else
                    replaceCellsInGrid grid [newCell]

reset: Grid -> Grid
reset grid = {grid | cells = grid.initialState}

getLights: Grid -> List Location
getLights grid =
    let
        foldFn: Location -> Cell -> List Location -> List Location
        foldFn loc cell locs = case cell.contents of
            Light -> loc :: locs
            BadLight -> loc :: locs
            _ -> locs
    in
        Dict.foldr foldFn [] grid.cells

-- Utilities for debugging

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


