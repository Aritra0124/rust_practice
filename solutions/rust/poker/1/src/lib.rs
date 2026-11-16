use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandRank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}

fn parse_card(card: &str) -> (u8, char) {
    let (rank_str, suit_str) = card.split_at(card.len() - 1);
    let rank = match rank_str {
        "A" => 14,
        "K" => 13,
        "Q" => 12,
        "J" => 11,
        n => n.parse::<u8>().unwrap(),
    };
    let suit = suit_str.chars().next().unwrap();
    (rank, suit)
}

fn evaluate_hand(hand: &str) -> (HandRank, Vec<u8>) {
    let cards: Vec<(u8, char)> = hand.split_whitespace().map(parse_card).collect();
    
    // 1. Check flush
    let is_flush = cards.iter().all(|&(_, suit)| suit == cards[0].1);

    // 2. Count rank frequencies
    let mut counts: HashMap<u8, u8> = HashMap::new();
    for &(rank, _) in &cards {
        *counts.entry(rank).or_insert(0) += 1;
    }

    // Sort by (frequency desc, rank desc)
    let mut freq_rank: Vec<(u8, u8)> = counts.into_iter().map(|(r, c)| (c, r)).collect();
    freq_rank.sort_unstable_by(|a, b| b.cmp(a));

    let frequencies: Vec<u8> = freq_rank.iter().map(|&(c, _)| c).collect();
    let mut ranks: Vec<u8> = freq_rank.iter().map(|&(_, r)| r).collect();

    // 3. Check straight (only possible when all 5 cards have distinct ranks)
    let is_ace_low = ranks == vec![14, 5, 4, 3, 2];
    let is_straight = frequencies.len() == 5 && (ranks[0] - ranks[4] == 4 || is_ace_low);

    if is_ace_low {
        ranks = vec![5, 4, 3, 2, 1]; // 5-high straight loses to 6-high straight
    }

    // 4. Classify hand
    let hand_rank = match (is_straight, is_flush, frequencies.as_slice()) {
        (true, true, _) => HandRank::StraightFlush,
        (_, _, [4, 1]) => HandRank::FourOfAKind,
        (_, _, [3, 2]) => HandRank::FullHouse,
        (_, true, _) => HandRank::Flush,
        (true, _, _) => HandRank::Straight,
        (_, _, [3, 1, 1]) => HandRank::ThreeOfAKind,
        (_, _, [2, 2, 1]) => HandRank::TwoPair,
        (_, _, [2, 1, 1, 1]) => HandRank::OnePair,
        _ => HandRank::HighCard,
    };

    (hand_rank, ranks)
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let evaluated: Vec<(&'a str, (HandRank, Vec<u8>))> = hands
        .iter()
        .map(|&h| (h, evaluate_hand(h)))
        .collect();

    let Some(best_score) = evaluated.iter().map(|(_, score)| score).max() else {
        return vec![];
    };

    evaluated
        .iter()
        .filter(|(_, score)| score == best_score)
        .map(|&(hand, _)| hand)
        .collect()
}