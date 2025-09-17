use crate::cards::Card;

/// 게임 페이즈
#[derive(Debug, Clone, PartialEq)]
pub enum GamePhase {
    Waiting,  // 게임 대기
    Starting, // 게임 시작 중
    Dealing,  // 패 나누기
    Playing,  // 게임 진행 중
    Scoring,  // 점수 계산
    Ended,    // 게임 종료
}

/// 턴 페이즈
#[derive(Debug, Clone, PartialEq)]
pub enum TurnPhase {
    PlayingCard,    // 패 내기
    DrawingCard,    // 더미에서 뒤집기
    TakingCards,    // 카드 가져가기
    DecidingGoStop, // 고/스톱 결정
}

/// 카드 위치
#[derive(Debug, Clone, PartialEq)]
pub enum Location {
    PlayerHand(String),    // 플레이어 손패
    Field,                 // 바닥
    Deck,                  // 더미
    PlayerCapture(String), // 플레이어가 먹은 패
}

/// 특수 상황 감지
#[derive(Debug, Clone)]
pub struct SpecialCondition {
    pub condition_type: SpecialConditionType,
    pub player_id: String,
    pub cards: Vec<Card>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SpecialConditionType {
    Ppuk,      // 뻑
    JaPpuk,    // 자뻑
    ThreePpuk, // 3뻑
    Ddadak,    // 따닥
    Jjok,      // 쪽
    Sseul,     // 쓸
    Ssaki,     // 싸기
    Chongtong, // 총통 (4장)
    Shaking,   // 흔들기 (3장)
}

/// 게임 상태 변경 정보
#[derive(Debug, Clone)]
pub enum StateChange {
    CardsMoved {
        from: Location,
        to: Location,
        cards: Vec<Card>,
    },
    ScoreUpdated {
        player_id: String,
        new_score: u32,
    },
    TurnChanged {
        new_player_id: String,
    },
    GameEnded {
        winner_id: String,
    },
}
