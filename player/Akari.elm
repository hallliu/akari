import Html exposing (Html, div, button, text)
import Html.Attributes as Attr
import Html.Events as Event
import Html.App as App
import Debug
import Json.Decode as Json exposing (Decoder, (:=), decodeString)

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

type SolutionState = Unknown
    | Incorrect 
    | Complete

type alias Model = {
    grid: Grid,
    lastCheckedSolutionState: SolutionState
    }

main = App.program
    { view = view
    , init = model
    , update = update
    , subscriptions = (\_ -> Sub.none)
    }

update: Msg -> Model -> (Model, Cmd Msg)
update msg model =
    case msg of
        ReqNewPuzzle -> (model, Cmd.none) --todo
        CheckPuzzle -> (model, Cmd.none) --todo
        GridAction a -> ({model | grid = updateForGrid a model.grid}, Cmd.none)
        NewPuzzleResult s -> case newPuzzle s of
            Ok newGrid -> ({grid = newGrid, lastCheckedSolutionState = Unknown}, Cmd.none)
            Err err -> (model, (\s -> Cmd.none) <| Debug.log "json parse" err)
        CheckPuzzleResult r -> (model, Cmd.none) -- todo
        Error -> (model, Cmd.none) -- todo

updateForGrid: GridAdapter.GridAction -> Grid -> Grid
updateForGrid action grid =
    case action of
        GridAdapter.ToggleLight loc -> toggleLight loc grid
        GridAdapter.ToggleCantLight loc -> toggleCantLight loc grid
        GridAdapter.NoAction -> grid
        GridAdapter.Reset -> reset grid

model: (Model, Cmd Msg)
model = ({grid = makeGridFromString 5 5 <|
    "XX__1
     X___X
     2____
     __X__
     _____"
    , lastCheckedSolutionState = Unknown}, Cmd.none)

view: Model -> Html Msg
view model = div [Attr.id "puzzle-container"] [
    div [Attr.class <| solutionStateToClass model.lastCheckedSolutionState] [],
    App.map GridAction <| GridAdapter.gridToHtml model.grid,
    div [Attr.id "button-container"] [
        button [Event.onClick ReqNewPuzzle, Attr.type' "button"] [text "New game"],
        button [Event.onClick (GridAction GridAdapter.Reset), Attr.type' "button"] [text "Reset"],
        button [Event.onClick CheckPuzzle, Attr.type' "button"] [text "Submit"]
        ]
    ]

solutionStateToClass: SolutionState -> String
solutionStateToClass ss = case ss of 
    Unknown -> "soln-unknown"
    Incorrect -> "soln-incorrect"
    Complete -> "soln-complete"

newPuzzle: String -> Result String Grid
newPuzzle gridJson =
    let
        gridDataDec = "data" := Json.string
        gridHeightDec = "height" := Json.int
        gridWidthDec = "width" := Json.int
        gridDecoder = Json.object3 makeGridFromString gridHeightDec gridWidthDec gridDataDec
    in
        decodeString gridDecoder gridJson

