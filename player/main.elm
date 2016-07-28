import Html exposing (Html)
import Html.App as App
import Grid exposing 
    (Grid, Location, toggleLight, toggleCantLight,
    makeGridFromString)

main = App.beginnerProgram
    { init = init
    , model = model
    , update = update
    }

update: Msg -> Grid -> Grid
update msg grid =
    case msg of
        ToggleLight loc -> toggleLight loc grid
        ToggleCantLight loc -> toggleCantLight loc grid
        NoAction -> grid

model: Grid
model = populateWithNeighbors


