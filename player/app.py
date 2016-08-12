from flask import Flask, send_file, request
import flask
import json

from puzzle_generator import generate_puzzle

app = Flask(__name__)
puzzles = {}
next_puzzle_id = 0

@app.route("/")
def index():
    return app.send_static_file("index.html")

@app.route("/new_puzzle", methods=["GET"])
def new_puzzle():
    global next_puzzle_id
    height = int(request.args.get("height", 5))
    width = int(request.args.get("width", 5))
    density = 0.4 # default
    grid, ratio = generate_puzzle(height, width, density)
    puzzles[next_puzzle_id] = grid
    result = {
        "height": height,
        "width": width,
        "data": str(grid),
        "ratio": ratio,
        "puzzle_id": next_puzzle_id,
    }
    next_puzzle_id += 1

    return flask.jsonify(**result)

@app.route("/check_puzzle", methods=["POST"])
def check_puzzle():
    alt_data = json.loads(request.data.decode("utf-8"))
    puzzle_id = request.form.get("id", alt_data["id"])
    lit_lights = request.form.get("lights", alt_data["lights"])

    lit_light_set = set((l[0], l[1]) for l in lit_lights)
    grid_light_set = puzzles[puzzle_id].get_light_location_set()

    if lit_light_set <= grid_light_set:
        if grid_light_set <= lit_light_set:
            result = {
                "result": "soln-complete"
            }
        else:
            result = {
                "result": "soln-unknown"
            }
    else:
        result = {
            "result": "soln-incorrect"
        }
    
    return flask.jsonify(**result)

if __name__ == "__main__":
    app.run()
