use playing_cards::*;

#[test]
fn test_card_description_face_cards() {
    assert_eq!(card_description(&Card::King), "King");
    assert_eq!(card_description(&Card::Queen), "Queen");
    assert_eq!(card_description(&Card::Jack), "Jack");
}

#[test]
fn test_card_description_numbered_cards() {
    let card = Card::Numbered(7, "Hearts".to_string());
    assert_eq!(card_description(&card), "7 of Hearts");

    let card = Card::Numbered(10, "Diamonds".to_string());
    assert_eq!(card_description(&card), "10 of Diamonds");
}
