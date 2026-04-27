use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct VolatileStatus: u32 {
        const NONE = 0;
        const CONFUSION = 1 << 0;
        const FLINCH = 1 << 1;
        const TAUNT = 1 << 2;
        const ENCORE = 1 << 3;
        const LEECH_SEED = 1 << 4;
        const SUBSTITUTE = 1 << 5;
        const PROTECT = 1 << 6;
        const ROOST = 1 << 7;
        // Add more as needed
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct FieldHazards: u8 {
        const NONE = 0;
        const STEALTH_ROCK = 1 << 0;
        const SPIKES_1 = 1 << 1;
        const SPIKES_2 = 1 << 2;
        const SPIKES_3 = 1 << 3;
        const TOXIC_SPIKES_1 = 1 << 4;
        const TOXIC_SPIKES_2 = 1 << 5;
        const STICKY_WEB = 1 << 6;
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct WeatherTerrain: u8 {
        const NONE = 0;
        const SUN = 1 << 0;
        const RAIN = 1 << 1;
        const SAND = 1 << 2;
        const SNOW = 1 << 3;
        const ELECTRIC_TERRAIN = 1 << 4;
        const GRASSY_TERRAIN = 1 << 5;
        const MISTY_TERRAIN = 1 << 6;
        const PSYCHIC_TERRAIN = 1 << 7;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PokemonState {
    pub species_id: u16,
    pub item_id: u16,
    pub ability_id: u16,
    pub hp: u16,
    pub max_hp: u16,
    pub status: u8, // e.g., 0=None, 1=BRN, 2=PAR, 3=SLP, 4=FRZ, 5=PSN, 6=TOX
    pub volatile_status: VolatileStatus,
    pub moves: [u16; 4],
    pub pp: [u8; 4],
    pub stat_boosts: [i8; 7], // Atk, Def, Spa, Spd, Spe, Eva, Acc
    pub types: [u8; 2],
    pub tera_type: u8,
    pub is_terastallized: bool,
    pub active: bool,
    pub fainted: bool,
}

impl Default for PokemonState {
    fn default() -> Self {
        Self {
            species_id: 0,
            item_id: 0,
            ability_id: 0,
            hp: 0,
            max_hp: 0,
            status: 0,
            volatile_status: VolatileStatus::NONE,
            moves: [0; 4],
            pp: [0; 4],
            stat_boosts: [0; 7],
            types: [0; 2],
            tera_type: 0,
            is_terastallized: false,
            active: false,
            fainted: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlayerState {
    pub team: [PokemonState; 6],
    pub hazards: FieldHazards,
    pub active_pokemon_index: u8,
    pub light_screen_turns: u8,
    pub reflect_turns: u8,
    pub aurora_veil_turns: u8,
    pub tailwind_turns: u8,
    pub can_tera: bool,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            team: [PokemonState::default(); 6],
            hazards: FieldHazards::NONE,
            active_pokemon_index: 0,
            light_screen_turns: 0,
            reflect_turns: 0,
            aurora_veil_turns: 0,
            tailwind_turns: 0,
            can_tera: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BattleState {
    pub p1: PlayerState,
    pub p2: PlayerState,
    pub weather_terrain: WeatherTerrain,
    pub weather_turns: u8,
    pub terrain_turns: u8,
    pub trick_room_turns: u8,
    pub turn: u16,
}

impl Default for BattleState {
    fn default() -> Self {
        Self {
            p1: PlayerState::default(),
            p2: PlayerState::default(),
            weather_terrain: WeatherTerrain::NONE,
            weather_turns: 0,
            terrain_turns: 0,
            trick_room_turns: 0,
            turn: 0,
        }
    }
}
