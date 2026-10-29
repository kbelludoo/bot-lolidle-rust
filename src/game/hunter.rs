pub const ARENA_BASE_X: f64 = 125.0;
pub const ARENA_BASE_Y: f64 = 475.0;
pub const ARENA_RAIO: f64 = 3.0;
pub const ARENA_VEL: f64 = 0.6;

pub fn get_best_zone(level: u32) -> &'static str {
    match level {
        0..=5 => "rota-inf",
        6..=10 => "rota-sup",
        11..=15 => "lobos",
        16..=20 => "azul",
        21..=25 => "vermelho",
        26..=32 => "rio",
        33..=40 => "dragao",
        _ => "barao",
    }
}

pub fn round2(v: f64) -> f64 {
    (v * 100.0).round() / 100.0
}

pub fn fnv1a_hash(s: &str, seed: u32) -> u32 {
    let mut h = seed;
    for b in s.bytes() {
        h ^= b as u32;
        h = h.wrapping_mul(16777619);
        h ^= h >> 13;
    }
    h
}

pub fn arena_offset(name: &str) -> (f64, f64) {
    let dx = ((fnv1a_hash(name, 2166136261) % 7) as f64 - 3.0) * 0.8;
    let dy = ((fnv1a_hash(name, 2166136262) % 7) as f64 - 3.0) * 0.8;
    (dx, dy)
}

pub fn calculate_patrol_pos(base_x: f64, base_y: f64, elapsed_secs: f64, name: &str) -> (f64, f64) {
    let (ox, oy) = arena_offset(name);
    let ang = ARENA_VEL * elapsed_secs;
    let x = base_x + ARENA_RAIO * ang.cos() + ox;
    let y = base_y + ARENA_RAIO * ang.sin() + oy;
    (round2(x), round2(y))
}
