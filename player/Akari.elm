import Html exposing (Html, div, button, text)
import Html.Attributes as Attr
import Html.Events as Event
import Html.App as App
import Debug
import Json.Decode as Json exposing (Decoder, (:=), decodeString)
import Json.Encode
import Http
import Task

import Grid exposing 
    (Grid, Location, toggleLight, toggleCantLight, reset, getLights,
    makeGridFromString)
import GridAdapter

type Msg = ReqNewPuzzle
    | CheckPuzzle
    | NewPuzzleResult String
    | CheckPuzzleResult String
    | Error String
    | GridAction GridAdapter.GridAction

type SolutionState = Unknown
    | Incorrect 
    | Complete

type alias Model = {
    grid: Grid,
    puzzleId: Int,
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
        ReqNewPuzzle -> (model, getNewPuzzle 25 25)
        CheckPuzzle -> (model, checkPuzzleSolution model.puzzleId model.grid)
        GridAction a -> ({model | grid = updateForGrid a model.grid}, Cmd.none)
        NewPuzzleResult s -> case newPuzzle s of
            Ok (puzzleId, newGrid) ->
                ({grid = newGrid, lastCheckedSolutionState = Unknown, puzzleId = puzzleId}
                , Cmd.none)
            Err err -> (model, (\s -> Cmd.none) <| Debug.log "json parse" err)
        CheckPuzzleResult r ->
            ({model | lastCheckedSolutionState = checkResultToState r}, Cmd.none)
        Error e -> (\_ -> (model, Cmd.none)) <| Debug.log "http error" e

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
    , lastCheckedSolutionState = Unknown
    , puzzleId = -1}, Cmd.none)

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

checkResultToState: String -> SolutionState
checkResultToState resultString =
    if resultString == "soln-unknown" then
        Unknown
    else if resultString == "soln-incorrect" then
        Incorrect
    else
        Complete

getNewPuzzle: Int -> Int -> Cmd Msg
getNewPuzzle height width =
    let
        url = Http.url "/new_puzzle" [("height", toString height), ("width", toString width)]
    in
        Task.perform (Error << toString) NewPuzzleResult <| Http.getString url

checkPuzzleSolution: Int -> Grid -> Cmd Msg
checkPuzzleSolution puzzleId grid =
    let
        lights = GridAdapter.encodeLocationList <| getLights grid
        gridLightJson = Json.Encode.object [("id", Json.Encode.int puzzleId), ("lights", lights)]
        gridLightBody = Http.string <| Json.Encode.encode 0 gridLightJson
        httpTask = Http.post ("result" := Json.string) "/check_puzzle" gridLightBody
    in
        Task.perform (Error << toString) CheckPuzzleResult httpTask

newPuzzle: String -> Result String (Int, Grid)
newPuzzle gridJson =
    let
        gridDataDec = "data" := Json.string
        gridHeightDec = "height" := Json.int
        gridWidthDec = "width" := Json.int
        puzzleIdDec = "puzzle_id" := Json.int
        gridDecoder = Json.object3 makeGridFromString gridHeightDec gridWidthDec gridDataDec
        puzzleDataDecoder = Json.tuple2 (,) puzzleIdDec gridDecoder
    in
        decodeString puzzleDataDecoder gridJson

