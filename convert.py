import json

with open("./server/generated/reports/blocks.json", "r") as f:
    original = json.load(f)

palette = {}

for (k, v) in original.items():
    for state in v["states"]:
        palette[str(state["id"])] = {
            "name": k,
        }
        try:
            palette[str(state["id"])]["properties"] = state["properties"]
        except KeyError:
            pass

print(palette[9670])

with open("./palette.json", "w+") as f:
    json.dump(palette, f)
