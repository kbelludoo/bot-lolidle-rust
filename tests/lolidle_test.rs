use bot_lolidle_rust::game::hunter::{calculate_patrol_pos, get_best_zone};
use bot_lolidle_rust::game::types::PlayerSession;
use serde_json::json;

#[test]
fn test_lolidle_zone_progression() {
    assert_eq!(get_best_zone(1), "rota-inf");
    assert_eq!(get_best_zone(5), "rota-inf");
    assert_eq!(get_best_zone(8), "rota-sup");
    assert_eq!(get_best_zone(12), "lobos");
    assert_eq!(get_best_zone(18), "azul");
    assert_eq!(get_best_zone(23), "vermelho");
    assert_eq!(get_best_zone(30), "rio");
    assert_eq!(get_best_zone(38), "dragao");
    assert_eq!(get_best_zone(50), "barao");
}

#[test]
fn test_lolidle_patrol_movement() {
    let (x1, y1) = calculate_patrol_pos(125.0, 475.0, 0.0, "Yasuo");
    let (x2, y2) = calculate_patrol_pos(125.0, 475.0, 5.0, "Yasuo");
    assert_ne!((x1, y1), (x2, y2));
    assert!(x1 > 100.0 && x1 < 150.0);
    assert!(y1 > 450.0 && y1 < 500.0);
}

#[test]
fn test_lolidle_session_deserialization() {
    let raw = json!({
        "nome": "Faker",
        "campeao": "Ahri",
        "diamantes": 500,
        "estado": {
            "nivel": 25,
            "xp": 48500.0,
            "pontos": 10,
            "ouro": 15400.0,
            "vida": 850.0,
            "mana": 420.0,
            "local": "vermelho",
            "espolios": {
                "dente_de_lobo": 12,
                "cristal_azul": 4
            },
            "totais": {
                "abates": 1420,
                "mortes": 0,
                "ouro": 250000.0
            }
        }
    });

    let sess: PlayerSession = serde_json::from_value(raw).unwrap();
    assert_eq!(sess.nome, "Faker");
    assert_eq!(sess.campeao, "Ahri");
    assert_eq!(sess.estado.nivel, 25);
    assert_eq!(sess.estado.totais.abates, 1420);
    assert_eq!(sess.estado.espolios.get("dente_de_lobo"), Some(&12));
}
