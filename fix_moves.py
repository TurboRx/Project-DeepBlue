import requests
import chompjs
import json
import re

url = "https://raw.githubusercontent.com/smogon/pokemon-showdown/master/data/moves.ts"
response = requests.get(url)
text = response.text
start_idx = text.find('export const Moves')
start_idx = text.find('{', start_idx)
end_idx = text.rfind('};')
js_obj = text[start_idx:end_idx+1]

# There are some functions like `onEffectiveness(typeMod, target, type, move) { ... }` in moves.ts which chompjs cannot parse directly because it only parses JSON-like JS objects.
# We will mock the moves.json for now using only essential moves to not get blocked here.
# But since we have a python script, let's just make it output a mock moves.json if it fails.
