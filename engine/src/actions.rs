use crate::cards::Card;
use crate::scoring::PlayerScore;

/// 플레이어가 수행할 수 있는 모든 게임 액션
#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    /// 게임 시작
    StartGame { player_count: u8 },

    /// 패 내기 - 손에서 카드를 하나 선택하여 바닥에 냄
    PlayCard { player_id: String, card: Card },

    /// 더미에서 카드 뒤집기
    DrawFromDeck { player_id: String },

    /// 매칭된 카드 가져가기
    TakeMatchedCards { player_id: String, cards: Vec<Card> },

    /// 폭탄(흔들기) 선언
    DeclareShaking {
        player_id: String,
        cards: Vec<Card>, // 같은 월 3장 또는 4장
    },

    /// 고 선언
    DeclareGo { player_id: String },

    /// 스톱 선언
    DeclareStop { player_id: String },

    /// 나가리 처리
    HandleNagari,

    /// 싸기(설사) 수행
    PerformSsaki {
        player_id: String,
        month: u8, // 1~12월
    },

    /// 뻑 처리
    HandlePpuk { player_id: String, card: Card },

    /// 따닥 처리
    HandleDdadak {
        player_id: String,
        matched_cards: Vec<Card>,
    },

    /// 쪽 처리
    HandleJjok { player_id: String, card: Card },

    /// 쓸 처리
    HandleSseul {
        player_id: String,
        swept_cards: Vec<Card>,
    },

    /// 피 주고받기 (뻑, 따닥, 쓸 등의 결과)
    TransferPi {
        from_player_id: String,
        to_player_id: String,
        count: u8,
    },

    /// 광 팔기 (4인 이상 게임)
    SellGwang {
        player_id: String,
        gwang_cards: Vec<Card>,
    },

    /// 턴 종료
    EndTurn { player_id: String },

    /// 게임 종료
    EndGame {
        winner_id: String,
        final_scores: Vec<PlayerScore>,
    },
}

impl Action {
    /// 액션이 유효한지 검증
    pub fn validate(&self) -> Result<(), String> {
        match self {
            Action::StartGame { player_count } => {
                if *player_count < 2 || *player_count > 6 {
                    return Err("플레이어 수는 2~6명이어야 합니다".to_string());
                }
                Ok(())
            }
            Action::DeclareShaking { cards, .. } => {
                if cards.len() != 3 && cards.len() != 4 {
                    return Err("흔들기는 3장 또는 4장이어야 합니다".to_string());
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    /// 액션의 우선순위 반환 (낮을수록 우선)
    pub fn priority(&self) -> u8 {
        match self {
            Action::HandlePpuk { .. } => 1,
            Action::HandleDdadak { .. } => 2,
            Action::HandleSseul { .. } => 3,
            Action::DeclareShaking { .. } => 4,
            _ => 10,
        }
    }
}
