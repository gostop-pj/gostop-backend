/// 플레이어 점수 정보
#[derive(Debug, Clone, PartialEq)]
pub struct PlayerScore {
    pub player_id: String,
    pub base_score: u32,              // 기본 점수
    pub go_count: u8,                 // 고 횟수
    pub multipliers: Vec<Multiplier>, // 적용된 배수들
    pub final_score: u32,             // 최종 점수
}

/// 점수 배수 종류
#[derive(Debug, Clone, PartialEq)]
pub enum Multiplier {
    Go(u8),      // 고 배수 (3고부터 2배씩)
    PiBak,       // 피박 (2배)
    GwangBak,    // 광박 (2배)
    MeongBak,    // 멍박 (2배)
    GoBak,       // 고박 (2배)
    DokBak,      // 독박 (2배)
    Shaking(u8), // 흔들기 (3장: 2배, 4장: 4배)
    Ssaki,       // 싸기 (2배)
    Nagari(u8),  // 나가리 횟수
    OneShot,     // 원샷 (2배)
}
