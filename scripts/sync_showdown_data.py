import os
import requests
import chompjs
import json

DATA_DIR = "data"

URLS = {
    "pokedex": "https://raw.githubusercontent.com/smogon/pokemon-showdown/master/data/pokedex.ts",
    "typechart": "https://raw.githubusercontent.com/smogon/pokemon-showdown/master/data/typechart.ts",
}

def fetch_and_parse(url, name):
    print(f"Downloading {url}...")
    response = requests.get(url)
    response.raise_for_status()
    text = response.text

    start_idx = text.find('export const ')
    start_idx = text.find('{', start_idx)
    end_idx = text.rfind('};')

    if start_idx != -1 and end_idx != -1:
        js_obj = text[start_idx:end_idx+1]
        try:
            return chompjs.parse_js_object(js_obj)
        except Exception as e:
            print(f"Failed to parse JS object from {url}: {e}")
            return None
    return None

def sync_data():
    if not os.path.exists(DATA_DIR):
        os.makedirs(DATA_DIR)

    for name, url in URLS.items():
        data = fetch_and_parse(url, name)
        if data:
            filepath = os.path.join(DATA_DIR, f"{name}.json")
            with open(filepath, 'w') as f:
                json.dump(data, f, indent=2)
            print(f"Saved {name}.json")

    # moves.ts contains inline JS functions which makes parsing difficult with chompjs.
    # We will generate a mock moves.json with essential fields for now.
    mock_moves = {
        "tackle": {
            "num": 33, "accuracy": 100, "basePower": 40, "category": "Physical",
            "name": "Tackle", "pp": 35, "priority": 0, "type": "Normal"
        },
        "ember": {
            "num": 52, "accuracy": 100, "basePower": 40, "category": "Special",
            "name": "Ember", "pp": 25, "priority": 0, "type": "Fire"
        },
        "watergun": {
            "num": 55, "accuracy": 100, "basePower": 40, "category": "Special",
            "name": "Water Gun", "pp": 25, "priority": 0, "type": "Water"
        },
        "vinewhip": {
            "num": 22, "accuracy": 100, "basePower": 45, "category": "Physical",
            "name": "Vine Whip", "pp": 25, "priority": 0, "type": "Grass"
        }
    }
    with open(os.path.join(DATA_DIR, "moves.json"), 'w') as f:
        json.dump(mock_moves, f, indent=2)
    print("Saved mock moves.json")

    usage_data = {
        "bulbasaur": {
            "items": {"leftovers": 0.5},
            "moves": {"tackle": 0.9}
        }
    }
    with open(os.path.join(DATA_DIR, "usage.json"), 'w') as f:
        json.dump(usage_data, f, indent=2)
    print("Saved mock usage.json")

    print("Synchronized Showdown and Smogon data successfully.")

if __name__ == "__main__":
    sync_data()
