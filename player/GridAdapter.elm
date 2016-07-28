-- Methods which do the dirty work of converting a Grid into Html
module GridAdapter exposing (..)

import Grid exposing (..)
import Html exposing (Html, Attribute, div, text)
import Html.Attributes as Attr
import Html.Events as Events
import Dict
import Json.Decode as Json exposing (Decoder, (:=))

type Msg = ToggleLight Location
    | ToggleCantLight Location
    | NoAction

getDecoder: Location -> Decoder Msg
getDecoder loc = 
    let
        getMessage: Int -> Decoder Msg
        getMessage mouseNumber =
            if mouseNumber == 0 then
                Json.succeed <| ToggleLight loc
            else if mouseNumber == 2 then
                Json.succeed <| ToggleCantLight loc
            else
                Json.succeed <| NoAction
    in
        Json.andThen ("button" := Json.int) getMessage

getClassesForCellContents: CellContents -> List (Attribute Msg)
getClassesForCellContents contents = case contents of
    Empty -> []
    Lit -> [Attr.class "lit"]
    Light -> [Attr.class "lit", Attr.class "light"]
    CantLight -> [Attr.class "cant-light"]
    LitAndCantLight -> [Attr.class "lit", Attr.class "cant-light"]
    BadLight -> [Attr.class "lit", Attr.class "bad-light"]
    Solid -> [Attr.class "solid"]
    Constraint _ -> [Attr.class "constrained"]

getActionsForCell: Cell -> List (Attribute Msg)
getActionsForCell cell = case cell.contents of
    Solid -> []
    Constraint _ -> []
    _ -> [Events.on "click" <| getDecoder cell.location]

getTextForCellContents: CellContents -> List (Html Msg)
getTextForCellContents contents = case contents of
    Constraint x -> [text <| toString x]
    _ -> []

cellToHtml: Cell -> Html Msg
cellToHtml cell = div ((getClassesForCellContents cell.contents) ++ (getActionsForCell cell))
    (getTextForCellContents cell.contents)

gridRowToHtml: Grid -> Int -> Html Msg
gridRowToHtml grid rowNum =
    let
        getCellInRow: Int -> Maybe Cell
        getCellInRow x = Dict.get (rowNum, x) grid.cells

        cells: List Cell
        cells = List.filterMap identity <| List.map getCellInRow [0..grid.width - 1]

        cellHtml: List (Html Msg)
        cellHtml = List.map cellToHtml cells
    in
        div [Attr.class "grid-row"] cellHtml

gridToHtml: Grid -> Html Msg
gridToHtml grid =
    div [Attr.class "grid-whole"] <| List.map (gridRowToHtml grid) [0..grid.height - 1]
