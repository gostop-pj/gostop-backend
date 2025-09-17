use smartstring::alias::String;

use crate::cards::Card;
use crate::game_state::{GamePhase, SpecialCondition, TurnPhase};

#[derive(Debug, Clone)]
pub struct Game {
    pub id: String,
    pub deck: Vec<Card>,  // 더미 패
    pub field: Vec<Card>, // 바닥 패
    pub players: Vec<Player>,
    pub current_player_idx: usize,
    pub current_round: u8,
    pub game_phase: GamePhase,
    pub turn_phase: TurnPhase,
    pub go_history: Vec<GoHistory>,                // 고 선언 기록
    pub special_conditions: Vec<SpecialCondition>, // 특수 상황들
    pub nagari_count: u8,                          // 연속 나가리 횟수
}

#[derive(Debug, Clone)]
pub struct Player {
    pub id: String,
    pub name: String,
    pub hand: Vec<Card>,          // 손패
    pub captured: CapturedCards,  // 먹은 패
    pub score: Score,             // 점수 정보
    pub shaking_cards: Vec<Card>, // 흔든 패
    pub ppuk_count: u8,           // 뻑 횟수
    pub is_first_turn: bool,      // 첫 턴 여부 (원샷 체크용)
}

#[derive(Debug, Clone, Default)]
pub struct CapturedCards {
    pub gwang: Vec<Card>, // 광
    pub yeol: Vec<Card>,  // 열끗 (띠)
    pub meong: Vec<Card>, // 멍따 (동물)
    pub pi: Vec<Card>,    // 피
}

#[derive(Debug, Clone, Default)]
pub struct Score {
    pub base_score: u32,                   // 기본 점수
    pub go_count: u8,                      // 고 횟수
    pub multipliers: Vec<ScoreMultiplier>, // 적용된 배수
    pub total_multiplier: u32,             // 총 배수 (계산된 값)
    pub final_score: u32,                  // 최종 점수
}

#[derive(Debug, Clone, PartialEq)]
pub enum ScoreMultiplier {
    Go(u8),      // 고 배수
    PiBak,       // 피박
    GwangBak,    // 광박
    MeongBak,    // 멍박
    Shaking(u8), // 흔들기 (3장/4장)
    Ssaki,       // 싸기
    Nagari(u8),  // 나가리
    OneShot,     // 원샷
}

#[derive(Debug, Clone)]
pub struct GoHistory {
    pub player_id: String,
    pub round: u8,
    pub score_at_go: u32,
}

impl Game {
    pub fn new(id: String, player_count: u8) -> Self {
        let players = (0..player_count)
            .map(|i| {
                Player::new(
                    format!("player_{}", i).into(),
                    format!("Player {}", i + 1).into(),
                )
            })
            .collect();

        Self {
            id,
            deck: Vec::new(),
            field: Vec::new(),
            players,
            current_player_idx: 0,
            current_round: 0,
            game_phase: GamePhase::Waiting,
            turn_phase: TurnPhase::PlayingCard,
            go_history: Vec::new(),
            special_conditions: Vec::new(),
            nagari_count: 0,
        }
    }

    pub fn current_player(&self) -> Option<&Player> {
        self.players.get(self.current_player_idx)
    }

    pub fn current_player_mut(&mut self) -> Option<&mut Player> {
        self.players.get_mut(self.current_player_idx)
    }

    pub fn next_player_idx(&self) -> usize {
        (self.current_player_idx + 1) % self.players.len()
    }
}

impl Player {
    pub fn new(id: String, name: String) -> Self {
        Self {
            id,
            name,
            hand: Vec::new(),
            captured: CapturedCards::default(),
            score: Score::default(),
            shaking_cards: Vec::new(),
            ppuk_count: 0,
            is_first_turn: true,
        }
    }

    pub fn calculate_base_score(&self) -> u32 {
        let mut score = 0u32;

        // 피 점수 (쌍피 포함)
        let pi_count = self.calculate_pi_count();
        if pi_count >= 10 {
            score += (pi_count - 9) as u32;
        }

        // 열끗(띠) 점수
        let yeol_count = self.captured.yeol.len();
        if yeol_count >= 5 {
            score += (yeol_count - 4) as u32;
        }

        // 멍따(동물) 점수
        let meong_count = self.captured.meong.len();
        if meong_count >= 5 {
            score += (meong_count - 4) as u32;
        }

        // 광 점수
        let gwang_count = self.captured.gwang.len();
        match gwang_count {
            5 => score += 15, // 오광
            4 => {
                // 비광 포함 여부 확인
                if self.captured.gwang.contains(&Card::WillowRainman) {
                    score += 4; // 사광 (비광 포함)
                } else {
                    score += 4; // 사광 (비광 제외)
                }
            }
            3 => {
                // 비광 제외 3장인지 확인
                if !self.captured.gwang.contains(&Card::WillowRainman) {
                    score += 3; // 삼광
                } else {
                    score += 3; // 비삼광
                }
            }
            _ => {}
        }

        // 특수 조합 점수 계산
        score += self.calculate_special_combinations();

        score
    }

    /// 피 개수 계산 (쌍피 고려)
    pub fn calculate_pi_count(&self) -> usize {
        self.captured
            .pi
            .iter()
            .map(|card| {
                if card.is_double_pi() {
                    2 // 쌍피는 2장으로 계산
                } else {
                    1
                }
            })
            .sum()
    }

    /// 특수 조합 점수 계산
    pub fn calculate_special_combinations(&self) -> u32 {
        let mut score = 0u32;

        // 홍단 (1월, 2월, 3월)
        if self.has_hongdan() {
            score += 3;
        }

        // 청단 (6월, 9월, 10월)
        if self.has_cheongdan() {
            score += 3;
        }

        // 초단 (4월, 5월, 7월)
        if self.has_chodan() {
            score += 3;
        }

        // 고도리 (2월 매조, 4월 흑싸리, 8월 공산)
        if self.has_godori() {
            score += 5;
        }

        score
    }

    /// 홍단 체크
    pub fn has_hongdan(&self) -> bool {
        let hongdan_cards = [
            Card::SonghakHongdan, // 1월
            Card::MaejouHongdan,  // 2월
            Card::SakuraHongdan,  // 3월
        ];
        hongdan_cards
            .iter()
            .all(|card| self.captured.yeol.contains(card))
    }

    /// 청단 체크
    pub fn has_cheongdan(&self) -> bool {
        let cheongdan_cards = [
            Card::PeonyCheongdan,         // 6월
            Card::ChrysanthemumCheongdan, // 9월
            Card::MapleCheongdan,         // 10월
        ];
        cheongdan_cards
            .iter()
            .all(|card| self.captured.yeol.contains(card))
    }

    /// 초단 체크
    pub fn has_chodan(&self) -> bool {
        let chodan_cards = [
            Card::DeungnamuChodan, // 4월
            Card::IrisChodan,      // 5월
            Card::SariChodan,      // 7월
        ];
        chodan_cards
            .iter()
            .all(|card| self.captured.yeol.contains(card))
    }

    /// 고도리 체크
    pub fn has_godori(&self) -> bool {
        let godori_cards = [
            Card::MaejouWhistlingBird, // 2월 휘파람새
            Card::DeungnamuCuckoo,     // 4월 두견새
            Card::EoksaeGoose,         // 8월 기러기
        ];
        godori_cards
            .iter()
            .all(|card| self.captured.meong.contains(card))
    }

    /// 총 배수 계산
    pub fn calculate_total_multiplier(&self) -> u32 {
        let mut multiplier = 1u32;

        for m in &self.score.multipliers {
            match m {
                ScoreMultiplier::Go(go_count) => {
                    // 3고부터 2배씩 증가
                    if *go_count >= 3 {
                        multiplier *= 2u32.pow((*go_count - 2) as u32);
                    }
                }
                ScoreMultiplier::PiBak
                | ScoreMultiplier::GwangBak
                | ScoreMultiplier::MeongBak
                | ScoreMultiplier::Ssaki
                | ScoreMultiplier::OneShot => {
                    multiplier *= 2;
                }
                ScoreMultiplier::Shaking(count) => {
                    match count {
                        3 => multiplier *= 2,
                        4 => multiplier *= 4, // 총통
                        _ => {}
                    }
                }
                ScoreMultiplier::Nagari(count) => {
                    multiplier *= 2u32.pow(*count as u32);
                }
            }
        }

        multiplier
    }

    pub fn has_go_stop_decision(&self) -> bool {
        self.calculate_base_score() >= 3 // 고스톱 기준 3점 (맞고는 7점)
    }

    /// 최종 점수 업데이트
    pub fn update_final_score(&mut self) {
        self.score.base_score = self.calculate_base_score();
        self.score.total_multiplier = self.calculate_total_multiplier();
        self.score.final_score = self.score.base_score * self.score.total_multiplier;
    }

    /// 배수 정보 문자열로 반환
    pub fn get_multiplier_info(&self) -> String {
        if self.score.multipliers.is_empty() {
            return "배수 없음".into();
        }

        let mut info = Vec::new();
        for m in &self.score.multipliers {
            match m {
                ScoreMultiplier::Go(count) => info.push(format!("{}고", count)),
                ScoreMultiplier::PiBak => info.push("피박".to_string()),
                ScoreMultiplier::GwangBak => info.push("광박".to_string()),
                ScoreMultiplier::MeongBak => info.push("멍박".to_string()),
                ScoreMultiplier::Shaking(count) => info.push(format!("흔들기({}장)", count)),
                ScoreMultiplier::Ssaki => info.push("싸기".to_string()),
                ScoreMultiplier::Nagari(count) => info.push(format!("나가리×{}", count)),
                ScoreMultiplier::OneShot => info.push("원샷".to_string()),
            }
        }

        format!(
            "{} (총 ×{}배)",
            info.join(", "),
            self.score.total_multiplier
        )
        .into()
    }
}

impl CapturedCards {
    pub fn add_card(&mut self, card: Card) {
        // TODO: 카드 타입에 따라 적절한 벡터에 추가
        // 임시로 피에 추가
        self.pi.push(card);
    }

    pub fn total_count(&self) -> usize {
        self.gwang.len() + self.yeol.len() + self.meong.len() + self.pi.len()
    }
}
