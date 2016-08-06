import Html exposing (Html, div, button, text)
import Html.Attributes as Attr
import Html.Events as Event

import Html.App as App
import Grid exposing 
    (Grid, Location, toggleLight, toggleCantLight, reset,
    makeGridFromString)
import GridAdapter

type Msg = ReqNewPuzzle
    | CheckPuzzle
    | NewPuzzleResult String
    | CheckPuzzleResult String
    | Error
    | GridAction GridAdapter.GridAction

main = App.beginnerProgram
    { view = view
    , model = model
    , update = update
    }

update: Msg -> Grid -> (Grid, Cmd Msg)
update msg grid =
    case msg of
        ReqNewPuzzle -> (grid, Cmd.none) --todo
        CheckPuzzle -> (grid, Cmd.none) --todo
        GridAction a -> (updateForGrid a grid, Cmd.none)
        NewPuzzleResult s -> (newPuzzle s, Cmd.none)
        CheckPuzzleResult 

updateForGrid: GridAdapter.GridAction -> Grid -> Grid
updateForGrid action grid =
    case action of
        GridAdapter.ToggleLight loc -> toggleLight loc grid
        GridAdapter.ToggleCantLight loc -> toggleCantLight loc grid
        GridAdapter.NoAction -> grid
        GridAdapter.Reset -> reset grid

model: Grid
model = makeGridFromString 5 5 <|
    "XX__1
     X___X
     2____
     __X__
     _____"

view: Grid -> Html Msg
view grid = div [Attr.id "puzzle-container"] [
    App.map GridAction <| GridAdapter.gridToHtml grid,
    div [Attr.id "button-container"] [
        button [Event.onClick NewPuzzle, Attr.type' "button"] [text "New game"],
        button [Event.onClick (GridAction GridAdapter.Reset), Attr.type' "button"] [text "Reset"],
        button [Event.onClick CheckPuzzle, Attr.type' "button"] [text "Submit"]
        ]
    ]
