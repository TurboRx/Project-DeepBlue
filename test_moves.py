import requests
import chompjs
import json
import re

url = "https://raw.githubusercontent.com/smogon/pokemon-showdown/master/data/moves.ts"
response = requests.get(url)
text = response.text
# The definition is: export const Moves: import('../sim/dex-moves').MoveDataTable = {
start_idx = text.find('export const Moves')
start_idx = text.find('{', start_idx)

# We want to grab everything until the end of the file or the last closing brace
end_idx = text.rfind('};')
if end_idx == -1:
    end_idx = text.rfind('}')

js_obj = text[start_idx:end_idx+1]
try:
    data = chompjs.parse_js_object(js_obj)
    print("Success")
except Exception as e:
    print(f"Failed: {e}")
    error_idx = 484915
    print(js_obj[error_idx-50:error_idx+50])
