use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountConfig {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub enabled: bool,
    #[serde(rename = "bindIP", default)]
    pub bind_ip: String,
    #[serde(default)]
    pub proxy: String,
    pub credentials: GameCredentials,
    #[serde(default)]
    pub settings: GameSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GameCredentials {
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub password: String,
    #[serde(rename = "sessionCookie", default)]
    pub session_cookie: String,
    #[serde(default)]
    pub token: String,
    #[serde(default)]
    pub origin: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GameSettings {
    #[serde(rename = "huntTarget", default)]
    pub hunt_target: String,
    #[serde(rename = "autoHunt", default = "default_true")]
    pub auto_hunt: bool,
    #[serde(rename = "autoRoute", default = "default_true")]
    pub auto_route: bool,
    #[serde(rename = "autoSell", default = "default_true")]
    pub auto_sell: bool,
    #[serde(rename = "autoHeal", default = "default_true")]
    pub auto_heal: bool,
    #[serde(rename = "autoUpgrade", default = "default_true")]
    pub auto_upgrade: bool,
    #[serde(rename = "autoDaily", default = "default_true")]
    pub auto_daily: bool,
    #[serde(rename = "autoChests", default = "default_true")]
    pub auto_chests: bool,
    #[serde(rename = "autoParty", default = "default_true")]
    pub auto_party: bool,
    #[serde(rename = "partyLeader", default)]
    pub party_leader: String,
    #[serde(rename = "arenaWalk", default = "default_true")]
    pub arena_walk: bool,
    #[serde(rename = "arenaX", default)]
    pub arena_x: f64,
    #[serde(rename = "arenaY", default)]
    pub arena_y: f64,
    #[serde(rename = "killRate", default)]
    pub kill_rate: f64,
    #[serde(rename = "maxLevel", default)]
    pub max_level: u32,
    #[serde(rename = "stealthLevel", default)]
    pub stealth_level: u32,
    #[serde(rename = "partyMembers", default)]
    pub party_members: Vec<String>,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlayerTotais {
    #[serde(default)]
    pub abates: u64,
    #[serde(default)]
    pub mortes: u64,
    #[serde(default)]
    pub dano: f64,
    #[serde(default)]
    pub ouro: f64,
    #[serde(rename = "porMonstro", default)]
    pub por_monstro: HashMap<String, u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlayerState {
    #[serde(default)]
    pub nivel: u32,
    #[serde(rename = "xp", default)]
    pub exp: f64,
    #[serde(default)]
    pub pontos: u32,
    #[serde(rename = "pontosAtrib", default)]
    pub pontos_atrib: u32,
    #[serde(default)]
    pub ouro: f64,
    #[serde(rename = "vida", default)]
    pub hp: f64,
    #[serde(default)]
    pub mana: f64,
    #[serde(default)]
    pub local: String,
    #[serde(default)]
    pub area: Option<String>,
    #[serde(default)]
    pub espolios: HashMap<String, u32>,
    #[serde(default)]
    pub totais: PlayerTotais,
    #[serde(rename = "tempoJogo", default)]
    pub tempo_jogo: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlayerSession {
    #[serde(default)]
    pub nome: String,
    #[serde(default)]
    pub campeao: String,
    #[serde(default)]
    pub diamantes: i64,
    #[serde(default)]
    pub estado: PlayerState,
}
