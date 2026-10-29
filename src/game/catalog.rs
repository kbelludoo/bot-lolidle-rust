#[derive(Debug, Clone)]
pub struct AreaCandidate {
    pub id: &'static str,
    pub min_level: u32,
    pub max_level: u32,
    pub base_exp: f64,
    pub base_gold: f64,
}

pub static HUNTS_CATALOG: &[AreaCandidate] = &[
        AreaCandidate { id: "rota-inf", min_level: 1, max_level: 18, base_exp: 110.0, base_gold: 60.0 },
        AreaCandidate { id: "rota-sup", min_level: 18, max_level: 35, base_exp: 290.0, base_gold: 160.0 },
        AreaCandidate { id: "rota-meio", min_level: 35, max_level: 55, base_exp: 750.0, base_gold: 420.0 },
        AreaCandidate { id: "selva-lobos", min_level: 55, max_level: 80, base_exp: 1900.0, base_gold: 1200.0 },
        AreaCandidate { id: "azul", min_level: 80, max_level: 115, base_exp: 4600.0, base_gold: 2900.0 },
        AreaCandidate { id: "vermelho", min_level: 115, max_level: 160, base_exp: 10800.0, base_gold: 7100.0 },
        AreaCandidate { id: "rio-aronguejo", min_level: 160, max_level: 220, base_exp: 26000.0, base_gold: 17500.0 },
        AreaCandidate { id: "covil-dragao", min_level: 220, max_level: 300, base_exp: 62000.0, base_gold: 42000.0 },
        AreaCandidate { id: "covil-barao", min_level: 300, max_level: 500, base_exp: 150000.0, base_gold: 105000.0 },
];

#[derive(Clone, Default)]
pub struct Catalog;

impl Catalog {
    pub fn select_best_hunt(&self, level: u32) -> &'static AreaCandidate {
        for area in HUNTS_CATALOG {
            if level >= area.min_level && level <= area.max_level {
                return area;
            }
        }
        &HUNTS_CATALOG[HUNTS_CATALOG.len() - 1]
    }
}
