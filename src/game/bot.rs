use crate::engine::circuit_breaker::{CircuitBreaker, CircuitBreakerConfig};
use crate::engine::stamina::{StaminaConfig, StaminaController};
use crate::game::api::LolidleClient;
use crate::game::hunter::{
    calculate_patrol_pos, get_best_zone, ARENA_BASE_X, ARENA_BASE_Y,
};
use crate::game::types::{AccountConfig, PlayerSession, PlayerState};
use crate::network::socket::LolidleSocketClient;
use crate::subsystem::dailies::DailyManager;
use crate::subsystem::inventory::InventoryManager;
use crate::subsystem::potions::PotionManager;
use serde_json::Value;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};
use tracing::{debug, error, info, warn};

pub struct BotInstance {
    pub config: AccountConfig,
    pub circuit_breaker: Arc<CircuitBreaker>,
    pub stamina: Arc<StaminaController>,
    pub inventory: Arc<InventoryManager>,
    pub dailies: Arc<DailyManager>,
    pub potions: Arc<PotionManager>,
    pub api: Arc<LolidleClient>,
    pub socket: Arc<LolidleSocketClient>,

    pub session: Arc<Mutex<PlayerSession>>,
    pub kills: AtomicU64,
    pub gold: AtomicU64,
    pub exp: AtomicU64,
    pub current_level: AtomicU32,
    pub running: AtomicBool,
    start_time: Instant,
}

impl BotInstance {
    pub fn new(config: AccountConfig) -> Self {
        let cb_cfg = CircuitBreakerConfig::default();
        let cb = Arc::new(CircuitBreaker::new(&config.id, cb_cfg));
        let stam = Arc::new(StaminaController::new(StaminaConfig::default()));
        let api = Arc::new(LolidleClient::new(&config));

        let ws_url = if config.credentials.origin.is_empty() {
            "wss://lolidle.com/ws".to_string()
        } else {
            let orig = &config.credentials.origin;
            let scheme = if orig.starts_with("https") { "wss" } else { "ws" };
            let host = orig.trim_start_matches("https://").trim_start_matches("http://").trim_end_matches('/');
            format!("{}://{}/ws", scheme, host)
        };

        let socket = Arc::new(LolidleSocketClient::new(&ws_url));

        Self {
            config,
            circuit_breaker: cb,
            stamina: stam,
            inventory: Arc::new(InventoryManager::default()),
            dailies: Arc::new(DailyManager::default()),
            potions: Arc::new(PotionManager::default()),
            api,
            socket,
            session: Arc::new(Mutex::new(PlayerSession::default())),
            kills: AtomicU64::new(0),
            gold: AtomicU64::new(0),
            exp: AtomicU64::new(0),
            current_level: AtomicU32::new(1),
            running: AtomicBool::new(false),
            start_time: Instant::now(),
        }
    }

    pub async fn init(&self) -> Result<(), String> {
        info!("Validando sessão e carregando dados reais do personagem via /api/eu...");
        let sess = self.api.get_eu().await?;
        self.current_level.store(sess.estado.nivel, Ordering::Relaxed);
        self.gold.store(sess.estado.ouro as u64, Ordering::Relaxed);
        self.exp.store(sess.estado.exp as u64, Ordering::Relaxed);
        self.kills.store(sess.estado.totais.abates, Ordering::Relaxed);
        *self.session.lock().await = sess;
        Ok(())
    }

    pub async fn check_optimal_route(&self) {
        let lvl = self.current_level.load(Ordering::Relaxed);
        let mut sess = self.session.lock().await;
        let current_zone = sess.estado.local.clone();

        let target_zone = if self.config.settings.hunt_target.is_empty() || self.config.settings.hunt_target == "auto" {
            get_best_zone(lvl).to_string()
        } else {
            self.config.settings.hunt_target.clone()
        };

        if current_zone != target_zone {
            info!("🗺️ Nível {lvl}: Viajando para melhor rota de caça -> {target_zone}");
            sess.estado.local = target_zone.clone();
            let state_to_save = sess.estado.clone();
            drop(sess);
            let _ = self.api.save_state(&state_to_save).await;
        }
    }

    pub async fn heal_at_fountain(&self) {
        info!("🚑 Vida crítica! Retornando à Fonte para cura da Soraka e venda de espólios...");
        let mut sess = self.session.lock().await;
        sess.estado.local = "fonte".to_string();
        sess.estado.hp = 1000.0;
        sess.estado.mana = 1000.0;

        let total_espolios: u64 = sess.estado.espolios.values().map(|&q| q as u64 * 15).sum();
        if total_espolios > 0 {
            sess.estado.ouro += total_espolios as f64;
            self.gold.store(sess.estado.ouro as u64, Ordering::Relaxed);
            sess.estado.espolios.clear();
            info!("💰 Espólios vendidos na Fonte: +{total_espolios} ouro!");
        }

        let state_to_save = sess.estado.clone();
        drop(sess);

        let _ = self.api.save_state(&state_to_save).await;
        sleep(Duration::from_millis(1000)).await;
        self.check_optimal_route().await;
    }

    pub async fn run_loop(self: Arc<Self>) {
        self.running.store(true, Ordering::SeqCst);
        info!("🚀 Bot [{}] (LoL Idle Runeterra) iniciado com sucesso.", self.config.name);

        let token = &self.config.credentials.token;
        if !token.is_empty() && token != "seu-token-de-sessao-ou-jwt" {
            let _ = self.socket.connect(|_| {}).await;
            let _ = self.socket.send_json(&serde_json::json!({
                "t": "oi",
                "token": token
            })).await;
        }

        let mut pos_timer = tokio::time::interval(Duration::from_millis(500));
        let mut route_timer = tokio::time::interval(Duration::from_secs(30));
        let mut save_timer = tokio::time::interval(Duration::from_secs(15));
        let mut daily_timer = tokio::time::interval(Duration::from_secs(300));

        while self.running.load(Ordering::Relaxed) {
            tokio::select! {
                _ = pos_timer.tick() => {
                    if self.config.settings.arena_walk && self.socket.is_connected().await {
                        let elapsed = self.start_time.elapsed().as_secs_f64();
                        let base_x = if self.config.settings.arena_x != 0.0 { self.config.settings.arena_x } else { ARENA_BASE_X };
                        let base_y = if self.config.settings.arena_y != 0.0 { self.config.settings.arena_y } else { ARENA_BASE_Y };
                        let (x, y) = calculate_patrol_pos(base_x, base_y, elapsed, &self.config.name);

                        let _ = self.socket.send_json(&serde_json::json!({
                            "t": "pos",
                            "tj": elapsed,
                            "x": x,
                            "y": y,
                            "dx": 1,
                            "dy": 0,
                            "anda": true,
                            "atk": true,
                            "vida": 1000.0,
                            "skin": 0,
                            "pvp": false,
                            "parado": false,
                            "morto": false,
                            "mt": ""
                        })).await;
                    }
                }
                _ = route_timer.tick() => {
                    self.check_optimal_route().await;
                }
                _ = save_timer.tick() => {
                    let sess = self.session.lock().await;
                    let _ = self.api.save_state(&sess.estado).await;
                }
                _ = daily_timer.tick() => {
                    let _ = self.api.claim_daily().await;
                }
            }
        }
    }
}
