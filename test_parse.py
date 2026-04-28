import requests
import chompjs
import json
import re

def convert_ts_to_json(url):
    print(f"Downloading {url}...")
    response = requests.get(url)
    response.raise_for_status()
    text = response.text

    # Showdown's data files usually look like: `export const Pokedex: {[k: string]: SpeciesData} = { ... };`
    # We find the first '{' and the last '}'
    start_idx = text.find('{')
    end_idx = text.rfind('}')
    if start_idx != -1 and end_idx != -1:
        js_obj = text[start_idx:end_idx+1]
        try:
            data = chompjs.parse_js_object(js_obj)
            return data
        except Exception as e:
            print(f"Failed to parse JS object: {e}")
            return None
    return None

pokedex = convert_ts_to_json("https://raw.githubusercontent.com/smogon/pokemon-showdown/master/data/pokedex.ts")
if pokedex:
    print(f"Pokedex parsed. Number of keys: {len(pokedex.keys())}")
    print(f"Bulbasaur: {pokedex.get('bulbasaur')}")
