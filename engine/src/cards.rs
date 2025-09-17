/// 화투 카드의 각 종류를 나타내는 열거형입니다.
///
/// 각 월(月)별로 4장의 카드가 있으며, 카드마다 고유한 이름과 역할이 있습니다.
/// 주석에는 한글 카드명과 역할이 함께 표기되어 있습니다.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Card {
    /// 1월: 소나무
    SonghakPine,         // 소나무
    SonghakPine2,        // 소나무 2장
    SonghakCrane,        // 두루미(일광)
    SonghakHongdan,      // 홍단

    /// 2월: 매화
    MaejouPlum,          // 매화
    MaejouPlum2,         // 매화 2장
    MaejouWhistlingBird, // 휘파람새★
    MaejouHongdan,       // 홍단

    /// 3월: 벚꽃
    SakuraCherry,        // 벚꽃
    SakuraCherry2,       // 벚꽃 2장
    SakuraCurtain,       // 장막(삼광)
    SakuraHongdan,       // 홍단

    /// 4월: 등나무
    DeungnamuWisteria,   // 등나무
    DeungnamuWisteria2,  // 등나무 2장
    DeungnamuCuckoo,     // 두견새★
    DeungnamuChodan,     // 초단

    /// 5월: 제비붓꽃
    Iris,                // 제비붓꽃
    Iris2,               // 제비붓꽃 2장
    IrisYatsuhashi,      // 제비붓꽃과 야츠하시
    IrisChodan,          // 초단

    /// 6월: 모란
    Peony,               // 모란
    Peony2,              // 모란 2장
    PeonyButterfly,      // 모란과 나비
    PeonyCheongdan,      // 청단

    /// 7월: 싸리
    SariBushClover,      // 싸리
    SariBushClover2,     // 싸리 2장
    SariBoar,            // 멧돼지
    SariChodan,          // 초단

    /// 8월: 억새
    EoksaePampas,        // 억새
    EoksaePampas2,       // 억새 2장
    EoksaeGoose,         // 기러기★
    EoksaeMoon,          // 달(팔광)

    /// 9월: 국화
    Chrysanthemum,       // 국화
    Chrysanthemum2,      // 국화 2장
    ChrysanthemumSakazuki, // 사카즈키
    ChrysanthemumCheongdan, // 청단

    /// 10월: 단풍
    Maple,               // 단풍
    Maple2,              // 단풍 2장
    MapleDeer,           // 사슴
    MapleCheongdan,      // 청단

    /// 11월: 오동
    Paulownia,           // 오동
    Paulownia2,          // 오동 2장
    PaulowniaDoublePi,   // 오동 쌍피
    PaulowniaPhoenix,    // 봉황(똥광)

    /// 12월: 버들
    Willow,              // 버들
    WillowDoublePi,      // 지옥문 쌍피
    WillowChodan,        // 초단
    WillowSwallow,       // 제비★
    WillowRainman,       // 오노노 도후(비광)

    /// 보너스 패
    Bonus1,
    Bonus2,
}

impl Card {
    /// 해당 카드가 보너스 패인지 나타내는 함수
    pub fn is_bonus(&self) -> bool {
        match self {
            Card::Bonus1 | Card::Bonus2 => true,
            _ => false,
        }
    }

    /// 해당 카드가 쌍피인지 나타내는 함수
    pub fn is_double_pi(&self) -> bool {
        match self {
            Card::PaulowniaDoublePi | Card::WillowDoublePi | Card::Bonus1 | Card::Bonus2 => true,
            _ => false,
        }
    }

    /// 해당 카드가 광인지 나타내는 함수
    pub fn is_bright(&self) -> bool {
        match self {
            Card::SonghakCrane |
            Card::SakuraCurtain |
            Card::EoksaeMoon |
            Card::PaulowniaPhoenix |
            Card::WillowRainman => true,
            
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_bright() {
        assert!(Card::SonghakCrane.is_bright());
    }
    #[test]
    fn test_is_not_bright() {
        assert!(!Card::SonghakPine.is_bright());
    }
}