import asyncio
import websockets
import json
import re
import sys
import deepblue_engine

class Relay:
    def __init__(self, data_dir="data"):
        print(f"[Python] Initializing Rust Engine with data_dir={data_dir}")
        self.engine = deepblue_engine.DeepBlueEngine(data_dir=data_dir)
        self.state = {
            "turn": 0,
            "p1": {"team": []},
            "p2": {"team": []},
            "weather": None,
            "terrain": None
        }
        self.ws_url = "ws://sim.smogon.com:8000/showdown/websocket"
        self.battle_room = None
        self.player_id = "p1"

    async def connect(self):
        print(f"[Python] Connecting to {self.ws_url}...")
        async with websockets.connect(self.ws_url) as ws:
            self.ws = ws
            await self.listen_loop()

    async def listen_loop(self):
        async for message in self.ws:
            self.handle_message(message)

    def handle_message(self, message):
        lines = message.splitlines()
        for line in lines:
            if not line.startswith('|'):
                if line.startswith('>battle-'):
                    self.battle_room = line[1:]
                    print(f"[Python] Joined battle room: {self.battle_room}")
                continue

            parts = line.split('|')
            if len(parts) < 2:
                continue

            msg_type = parts[1]
            if msg_type == 'request':
                if len(parts) > 2 and parts[2]:
                    try:
                        req = json.loads(parts[2])
                        self.handle_request(req)
                    except json.JSONDecodeError:
                        pass
            elif msg_type == 'player':
                # |player|p1|username|avatar|rating
                if len(parts) > 3 and "deepblue" in parts[3].lower():
                    self.player_id = parts[2]
            elif msg_type == 'switch' or msg_type == 'drag':
                self.handle_switch(parts)
            elif msg_type == 'move':
                self.handle_move(parts)
            elif msg_type == '-damage' or msg_type == '-heal':
                self.handle_hp_change(parts)
            elif msg_type == 'faint':
                self.handle_faint(parts)
            elif msg_type == 'turn':
                if len(parts) > 2:
                    self.state["turn"] = int(parts[2])
                    self.sync_state_to_rust()
                    self.request_action()
            elif msg_type in ['win', 'tie']:
                print(f"[Python] Battle ended: {line}")

    def get_or_create_pokemon(self, player, ident):
        # ident is like 'p1a: Bulbasaur'
        name = ident.split(': ')[1] if ': ' in ident else ident
        team = self.state[player]["team"]
        for p in team:
            if p["name"] == name:
                return p

        new_p = {
            "name": name,
            "hp": 100,
            "max_hp": 100,
            "status": None,
            "active": False,
            "fainted": False,
            "moves": [],
            "item": None,
            "ability": None,
            "stat_boosts": {"atk": 0, "def": 0, "spa": 0, "spd": 0, "spe": 0, "accuracy": 0, "evasion": 0}
        }
        team.append(new_p)
        return new_p

    def handle_request(self, req):
        print(f"[Python] Parsed |request| message")
        if "side" in req and "pokemon" in req["side"]:
            self.state["p1"]["team"] = []
            for p_data in req["side"]["pokemon"]:
                # ident looks like "p1: Bulbasaur"
                name = p_data["ident"].split(': ')[1] if ': ' in p_data["ident"] else p_data["ident"]
                hp_str = p_data["condition"].split(' ')[0] if p_data["condition"] != "0 fnt" else "0/100"
                hp, max_hp = (int(x) for x in hp_str.split('/')) if '/' in hp_str else (0, 100)

                self.state["p1"]["team"].append({
                    "name": name,
                    "hp": hp,
                    "max_hp": max_hp,
                    "status": p_data["condition"].split(' ')[1] if ' ' in p_data["condition"] and p_data["condition"] != "0 fnt" else None,
                    "active": p_data.get("active", False),
                    "fainted": hp == 0,
                    "moves": p_data.get("moves", []),
                    "item": p_data.get("item", None),
                    "ability": p_data.get("baseAbility", None),
                    "stat_boosts": {"atk": 0, "def": 0, "spa": 0, "spd": 0, "spe": 0, "accuracy": 0, "evasion": 0}
                })

    def handle_switch(self, parts):
        # |switch|p1a: Bulbasaur|Bulbasaur, L100|100/100
        ident = parts[2]
        player = ident[:2] # p1 or p2
        name = ident.split(': ')[1] if ': ' in ident else ident

        hp_str = parts[4].split(' ')[0] if ' ' in parts[4] else parts[4]
        hp, max_hp = (int(x) for x in hp_str.split('/')) if '/' in hp_str else (0, 100)

        # Mark all inactive
        for p in self.state[player]["team"]:
            p["active"] = False

        pokemon = self.get_or_create_pokemon(player, ident)
        pokemon["active"] = True
        pokemon["hp"] = hp
        pokemon["max_hp"] = max_hp
        print(f"[Python] Parsed |switch| message: {player} switched to {name} ({hp}/{max_hp})")

    def handle_move(self, parts):
        # |move|p1a: Bulbasaur|Tackle|p2a: Charmander
        ident = parts[2]
        player = ident[:2]
        move = parts[3]

        pokemon = self.get_or_create_pokemon(player, ident)
        # Learn move if not known
        if move not in pokemon["moves"]:
            pokemon["moves"].append(move)

        print(f"[Python] Parsed |move| message: {ident} used {move}")

    def handle_hp_change(self, parts):
        # |-damage|p2a: Charmander|50/100
        ident = parts[2]
        player = ident[:2]
        hp_str = parts[3].split(' ')[0] if ' ' in parts[3] else parts[3]
        hp, max_hp = (int(x) for x in hp_str.split('/')) if '/' in hp_str else (0, 100)

        pokemon = self.get_or_create_pokemon(player, ident)
        pokemon["hp"] = hp
        if max_hp: pokemon["max_hp"] = max_hp

    def handle_faint(self, parts):
        # |faint|p2a: Charmander
        ident = parts[2]
        player = ident[:2]
        pokemon = self.get_or_create_pokemon(player, ident)
        pokemon["hp"] = 0
        pokemon["fainted"] = True
        print(f"[Python] Parsed |faint| message: {ident} fainted")

    def sync_state_to_rust(self):
        state_json = json.dumps(self.state)
        self.engine.set_state(state_json)

    def request_action(self):
        time_limit_ms = 1000
        best_action = self.engine.search(time_limit_ms)
        print(f"[Python] Engine selected action: {best_action}")
        return best_action

if __name__ == "__main__":
    relay = Relay()

    print("Testing protocol parser with dummy messages...")
    relay.handle_message(">battle-gen9ou-1234")
    relay.handle_message("|request|{\"side\":{\"pokemon\":[{\"ident\":\"p1: Bulbasaur\",\"condition\":\"100/100\",\"active\":true,\"moves\":[\"tackle\"]}]}}")
    relay.handle_message("|switch|p2a: Charmander|Charmander, L100|100/100")
    relay.handle_message("|move|p1a: Bulbasaur|Tackle|p2a: Charmander")
    relay.handle_message("|-damage|p2a: Charmander|50/100")
    relay.handle_message("|turn|1")
