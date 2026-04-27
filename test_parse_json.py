import json

with open("data/usage.json") as f:
    usage = json.load(f)
    print(list(usage.keys()))
