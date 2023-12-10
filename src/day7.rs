use std::cmp::Ordering;

use crate::Day;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    TreeofKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

#[derive(Debug)]
struct Hand<'a> {
    ty: HandType,
    hand: &'a str,
    bid: u64,
}

impl HandType {
    fn from_same_count(count: usize) -> Self {
        match count {
            1 => Self::HighCard,
            2 => Self::OnePair,
            3 => Self::TreeofKind,
            4 => Self::FourOfKind,
            5 => Self::FiveOfKind,
            _ => unreachable!(),
        }
    }
}

pub struct Day7;

impl Day for Day7 {
    fn example_input(&self) -> (&'static str, &'static str) {
        let input = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;
        (input, input)
    }

    fn example_solution(&self) -> (&'static str, &'static str) {
        ("6440", "5905")
    }

    fn part_1(&mut self, input: String) -> String {
        let mut hands = input
            .split('\n')
            .map(|line| line.split_once(' ').expect("a hand has to be 2 items"))
            .map(|(hand, bid)| {
                (
                    get_hand_type(get_hand_types(hand)),
                    hand,
                    bid.parse::<u64>().expect("bid has to be an integer"),
                )
            })
            .map(|(ty, hand, bid)| Hand { ty, hand, bid })
            .collect::<Vec<Hand>>();

        hands.sort_by(|hand, other_hand| {
            hand.ty.cmp(&other_hand.ty).then_with(|| {
                hand.hand
                    .chars()
                    .zip(other_hand.hand.chars())
                    .map(|(c, other_char)| {
                        card_char_to_order(c).cmp(&card_char_to_order(other_char))
                    })
                    .find(|order| *order != Ordering::Equal)
                    .unwrap_or_else(|| Ordering::Equal)
            })
        });

        hands
            .iter()
            .enumerate()
            .map(|(i, hand)| (i as u64 + 1, hand))
            .map(|(i, hand)| hand.bid * i)
            .sum::<u64>()
            .to_string()
    }

    fn part_2(&mut self, input: String) -> String {
        let mut hands = input
            .split('\n')
            .map(|line| line.split_once(' ').expect("a hand has to be 2 items"))
            .map(|(hand, bid)| {
                (
                    get_hand_type(get_hand_types_wildcard(hand)),
                    hand,
                    bid.parse::<u64>().expect("bid has to be an integer"),
                )
            })
            .map(|(ty, hand, bid)| Hand { ty, hand, bid })
            .collect::<Vec<Hand>>();

        hands.sort_by(|hand, other_hand| {
            hand.ty.cmp(&other_hand.ty).then_with(|| {
                hand.hand
                    .chars()
                    .zip(other_hand.hand.chars())
                    .map(|(c, other_char)| {
                        card_char_to_order_wildcard(c).cmp(&card_char_to_order_wildcard(other_char))
                    })
                    .find(|order| *order != Ordering::Equal)
                    .unwrap_or_else(|| Ordering::Equal)
            })
        });

        dbg!(hands)
            .iter()
            .enumerate()
            .map(|(i, hand)| (i as u64 + 1, hand))
            .map(|(i, hand)| hand.bid * i)
            .sum::<u64>()
            .to_string()

        // "".into()
    }
}

fn get_hand_types(hand: &str) -> Vec<HandType> {
    let mut checked = Vec::with_capacity(hand.len());
    hand.chars()
        .filter_map(|c| {
            checked
                .iter()
                .find(|ic| **ic == c)
                .map(|_| None)
                .unwrap_or_else(|| Some(checked.push(c)))
                .map(|_| c)
        })
        .map(|c| hand.chars().filter(|other_c| *other_c == c).count())
        .map(HandType::from_same_count)
        .collect::<Vec<HandType>>()
}

fn get_hand_types_wildcard(hand: &str) -> Vec<HandType> {
    let mut checked = Vec::with_capacity(hand.len());
    hand.chars()
        .filter(|c| *c != 'J')
        .filter_map(|c| {
            checked
                .iter()
                .find(|ic| **ic == c)
                .map(|_| None)
                .unwrap_or_else(|| Some(checked.push(c)))
                .map(|_| c)
        })
        .map(|c| {
            hand.chars()
                .filter(|other_c| *other_c == c || *other_c == 'J')
                .count()
        })
        .map(HandType::from_same_count)
        .collect::<Vec<HandType>>()
}

fn get_hand_type(mut handtypes: Vec<HandType>) -> HandType {
    match (
        handtypes.iter().find(|ty| **ty == HandType::OnePair),
        handtypes.iter().find(|ty| **ty == HandType::TreeofKind),
    ) {
        (Some(_), Some(_)) => handtypes.push(HandType::FullHouse),
        (_, None)
            if handtypes
                .iter()
                .filter(|pair| **pair == HandType::OnePair)
                .count()
                >= 2 =>
        {
            handtypes.push(HandType::TwoPair)
        }
        _ => {}
    };

    handtypes.sort();
    *dbg!(handtypes)
        .last()
        .unwrap_or_else(|| &HandType::HighCard)
}

fn card_char_to_order(c: char) -> u8 {
    match c as u8 {
        b'A' => 19,
        b'K' => 18,
        b'Q' => 17,
        b'J' => 16,
        b'T' => 15,
        d if (('0' as u8)..=('9' as u8)).contains(&d) => d - b'0',
        _ => unreachable!(),
    }
}

fn card_char_to_order_wildcard(c: char) -> u8 {
    match c as u8 {
        b'A' => 19,
        b'K' => 18,
        b'Q' => 17,
        b'J' => 0,
        b'T' => 15,
        d if (('0' as u8)..=('9' as u8)).contains(&d) => d - b'0',
        _ => unreachable!(),
    }
}
