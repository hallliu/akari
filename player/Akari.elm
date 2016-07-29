import Html exposing (Html)
import Html.App as App
import Grid exposing 
    (Grid, Location, toggleLight, toggleCantLight,
    makeGridFromString)
import GridAdapter exposing (Msg)

main = App.beginnerProgram
    { view = view
    , model = model
    , update = update
    }

update: Msg -> Grid -> Grid
update msg grid =
    case msg of
        GridAdapter.ToggleLight loc -> toggleLight loc grid
        GridAdapter.ToggleCantLight loc -> toggleCantLight loc grid
        GridAdapter.NoAction -> grid

model: Grid
model = makeGridFromString 5 5 <|
    "XX__X
     X___X
     2____
     __X__
     _____"

view: Grid -> Html Msg
view = GridAdapter.gridToHtml
