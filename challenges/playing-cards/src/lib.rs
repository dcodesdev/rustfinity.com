pub enum Card {
    King,
    Queen,
    Jack,
    Numbered(u8, String), // Tuple variant with value and suit
}

pub fn card_description(card: Card) -> String {
    match card {
        Card::King => "King".to_string(),
        Card::Queen => "Queen".to_string(),
        Card::Jack => "Jack".to_string(),
        Card::Numbered(value, suit) => format!("{} of {}", value, suit),
    }
}
