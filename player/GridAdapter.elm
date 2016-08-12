-- Methods which do the dirty work of converting a Grid into Html
module GridAdapter exposing (..)

import Grid exposing (..)
import Html exposing (Html, Attribute, div, text, span)
import Html.Attributes as Attr
import Html.Events as Events
import Dict
import Json.Decode as Json exposing (Decoder, (:=))
import Json.Encode

type GridAction = ToggleLight Location
    | ToggleCantLight Location
    | Reset
    | NoAction

encodeLocationList: List Location -> Json.Encode.Value
encodeLocationList locs =
    let
        locToJson: Location -> Json.Encode.Value
        locToJson (y, x) = Json.Encode.list [Json.Encode.int y, Json.Encode.int x]
    in
        Json.Encode.list <| List.map locToJson locs

getClickDecoder: Location -> Decoder GridAction
getClickDecoder loc = 
    let
        getMessage: Int -> Decoder GridAction
        getMessage mouseNumber =
            if mouseNumber == 0 then
                Json.succeed <| ToggleLight loc
            else
                Json.succeed <| NoAction
    in
        Json.andThen ("button" := Json.int) getMessage

getClassesForCellContents: CellContents -> List (Attribute GridAction)
getClassesForCellContents contents =
    let 
        getSpecialClasses: CellContents -> List String
        getSpecialClasses c = case c of
            Empty -> []
            Lit -> ["lit"]
            Light -> ["lit", "light"]
            CantLight -> ["cant-light"]
            LitAndCantLight -> ["lit", "cant-light"]
            BadLight -> [ "lit", "bad-light"]
            Solid -> ["solid"]
            Constraint _ -> ["constrained"]

        getAllClasses: CellContents -> List String
        getAllClasses c = (::) "grid-square" <| getSpecialClasses c

        toAttrs: List String -> List (Attribute GridAction)
        toAttrs classes = [Attr.classList <| List.map (\x -> (x, True)) classes]
    in
        toAttrs <| getAllClasses contents

getActionsForCell: Cell -> List (Attribute GridAction)
getActionsForCell cell = 
    let
        options = {stopPropagation = False, preventDefault = True}
        disableContextMenuHelper = Events.onWithOptions "contextmenu" options
        disableContextMenu = disableContextMenuHelper <| Json.succeed NoAction
        handleRightClick = disableContextMenuHelper (Json.succeed (ToggleCantLight cell.location))

    in case cell.contents of
        Solid -> [disableContextMenu]
        Constraint _ -> [disableContextMenu]
        _ -> [handleRightClick, Events.on "mousedown" <| getClickDecoder cell.location]

getTextForCellContents: CellContents -> List (Html GridAction)
getTextForCellContents contents = case contents of
    Constraint x -> [text <| toString x]
    _ -> []

cellToHtml: Cell -> Html GridAction
cellToHtml cell = div ((getClassesForCellContents cell.contents) ++ (getActionsForCell cell))
    (getTextForCellContents cell.contents)

gridRowToHtml: Grid -> Int -> Html GridAction
gridRowToHtml grid rowNum =
    let
        getCellInRow: Int -> Maybe Cell
        getCellInRow x = Dict.get (rowNum, x) grid.cells

        cells: List Cell
        cells = List.filterMap identity <| List.map getCellInRow [0..grid.width - 1]

        cellHtml: List (Html GridAction)
        cellHtml = List.map cellToHtml cells
    in
        div [Attr.class "grid-row"] cellHtml

gridToHtml: Grid -> Html GridAction
gridToHtml grid =
    div [Attr.class "grid-whole"] <| List.map (gridRowToHtml grid) [0..grid.height - 1]
