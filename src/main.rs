#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;

use std::env;
use std::fs;
use std::process;

mod card;
mod scry;
mod page;

fn main() {
    process_cmd();
}

fn process_cmd() {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Error opening file");
    let lines: Vec<String> = contents.lines().map(|s|s.to_string()).collect();

    let res = scry::query(lines);

    match res {
        Ok(cardresult) => {
            page::save_pdf(cardresult.cards, cardresult.not_found);
        }
        Err(err) => {
            println!("Runtime error: {}", err);
            process::exit(1);
        }
    }
}

fn fetch_and_print() {
    let res = scry::query(vec![String::from("fire") ,
             String::from("budoka gardener") ,
             String::from("archangel avacyn") ,
             String::from("sol ring") ,
             String::from("breaking") ,
             String::from("who//what//when//where//why") ,
             String::from("appeal") ,
             String::from("academy ruins") ,]);

    match res {
        Ok(cardresult) => {
            page::save_pdf(cardresult.cards, cardresult.not_found);
        }
        Err(err) => {
            println!("Runtime error: {}", err);
            process::exit(1);
        }
    }

}

fn fetch() {
    let res = scry::query(vec![String::from("fire") ,
             String::from("budoka gardener") ,
             String::from("archangel avacyn") ,
             String::from("sol ring") ,
             String::from("breaking") ,
             String::from("who//what//when//where//why") ,
             String::from("appeal") ,
             String::from("academy ruins") ,]);

    match res {
        Ok(cardresult) => {
            println!("Found");
            for card in cardresult.cards {
                card::print_card(&card);
                println!();
            }
            println!();
            println!("Not Found");
            for cardname in cardresult.not_found {
                println!("{}", cardname);
            }
        }
        Err(err) => {
            println!("Runtime error: {}", err);
            process::exit(1);
        }
    }
}

fn print() {
    let cards = vec![
        card::Card {
            id: String::from(""),
            uri: String::from(""),
            scryfall_uri: String::from(""),
            rulings_uri: String::from(""),
            name: String::from("Sol Ring"),
            cmc: 2.0,
            color_identity: Vec::new(),
            type_line: String::from("Artifact"),
            layout: String::from("normal"),
            set_name: String::from(""),
            rarity: String::from(""),
            info: card::CardInfo::Normal (card::CardFaceInfo {
                card_type: card::CardType::NonCreature,
                colors: Vec::new(),
                image_uris: card::CardImagery {
                    png: String::new(),
                    large: String::new(),
                    normal: String::new(),
                    small: String::new(),
                    border_crop: String::new(),
                    art_crop: String::new(),
                },
                mana_cost: String::from("{2}{U}{B}"),
                name: String::from("Sol Ring"),
                oracle_text: String::from("{T}: add {C}{C}"),
                type_line: String::from("Artifact"),
            }),
            legalities: card::CardLegalities {
                standard: String::new(),
                future: String::new(),
                frontier: String::new(),
                modern: String::new(),
                legacy: String::new(),
                pauper: String::new(),
                vintage: String::new(),
                penny: String::new(),
                commander: String::new(),
                duel: String::new(),
            },
        },
        card::Card {
            id: String::from(""),
            uri: String::from(""),
            scryfall_uri: String::from(""),
            rulings_uri: String::from(""),
            name: String::from("Also Sol Ring"),
            cmc: 2.0,
            color_identity: Vec::new(),
            type_line: String::from("Artifact"),
            layout: String::from("normal"),
            set_name: String::from(""),
            rarity: String::from(""),
            info: card::CardInfo::Split (
                vec![
                card::CardFaceInfo {
                    card_type: card::CardType::NonCreature,
                    colors: Vec::new(),
                    image_uris: card::CardImagery {
                        png: String::new(),
                        large: String::new(),
                        normal: String::new(),
                        small: String::new(),
                        border_crop: String::new(),
                        art_crop: String::new(),
                    },
                    mana_cost: String::from("{1}"),
                    name: String::from("Authority"),
                    oracle_text: String::from("{T}: add {C}{C}"),
                    type_line: String::from("Artifact"),
                },
                card::CardFaceInfo {
                    card_type: card::CardType::NonCreature,
                    colors: Vec::new(),
                    image_uris: card::CardImagery {
                        png: String::new(),
                        large: String::new(),
                        normal: String::new(),
                        small: String::new(),
                        border_crop: String::new(),
                        art_crop: String::new(),
                    },
                    mana_cost: String::from("{2}"),
                    name: String::from("Appeal"),
                    oracle_text: String::from("{T}: add {C}{C}"),
                    type_line: String::from("Artifact"),
                },
                ]),
            legalities: card::CardLegalities {
                standard: String::new(),
                future: String::new(),
                frontier: String::new(),
                modern: String::new(),
                legacy: String::new(),
                pauper: String::new(),
                vintage: String::new(),
                penny: String::new(),
                commander: String::new(),
                duel: String::new(),
            },
        },
        card::Card {
            id: String::from(""),
            uri: String::from(""),
            scryfall_uri: String::from(""),
            rulings_uri: String::from(""),
            name: String::from("Another Sol Ring"),
            cmc: 2.0,
            color_identity: Vec::new(),
            type_line: String::from("Creature"),
            layout: String::from("normal"),
            set_name: String::from(""),
            rarity: String::from(""),
            info: card::CardInfo::Multifaced (
                vec![
                card::CardFaceInfo {
                    card_type: card::CardType::NonCreature,
                    colors: Vec::new(),
                    image_uris: card::CardImagery {
                        png: String::new(),
                        large: String::new(),
                        normal: String::new(),
                        small: String::new(),
                        border_crop: String::new(),
                        art_crop: String::new(),
                    },
                    mana_cost: String::from("{1}"),
                    name: String::from("Archangel Avacyn"),
                    oracle_text: String::from("Flash\nFlying, vigilance\nWhen Archangel Avacyn enters the battlefield, creatures you control gain indestructible until end of turn.\nWhen a non-Angel creature you control dies, transform Archangel Avacyn at the beginning of the next upkeep."),
                    type_line: String::from("Artifact"),
                },
                card::CardFaceInfo {
                    card_type: card::CardType::NonCreature,
                    colors: Vec::new(),
                    image_uris: card::CardImagery {
                        png: String::new(),
                        large: String::new(),
                        normal: String::new(),
                        small: String::new(),
                        border_crop: String::new(),
                        art_crop: String::new(),
                    },
                    mana_cost: String::from("{2}"),
                    name: String::from("Avacyn the Purifier"),
                    oracle_text: String::from("{T}: add {C}{C}"),
                    type_line: String::from("Artifact"),
                },
                ]),
            legalities: card::CardLegalities {
                standard: String::new(),
                future: String::new(),
                frontier: String::new(),
                modern: String::new(),
                legacy: String::new(),
                pauper: String::new(),
                vintage: String::new(),
                penny: String::new(),
                commander: String::new(),
                duel: String::new(),
            },
        },
        card::Card {
            id: String::from(""),
            uri: String::from(""),
            scryfall_uri: String::from(""),
            rulings_uri: String::from(""),
            name: String::from("Yet Another Sol Ring"),
            cmc: 2.0,
            color_identity: Vec::new(),
            type_line: String::from("Planeswalker"),
            layout: String::from("normal"),
            set_name: String::from(""),
            rarity: String::from(""),
            info: card::CardInfo::Flip (
                vec![
                card::CardFaceInfo {
                    card_type: card::CardType::NonCreature,
                    colors: Vec::new(),
                    image_uris: card::CardImagery {
                        png: String::new(),
                        large: String::new(),
                        normal: String::new(),
                        small: String::new(),
                        border_crop: String::new(),
                        art_crop: String::new(),
                    },
                    mana_cost: String::from("{1}"),
                    name: String::from("Budoka Gardener"),
                    oracle_text: String::from("{T}: add {C}{C}"),
                    type_line: String::from("Artifact"),
                },
                card::CardFaceInfo {
                    card_type: card::CardType::NonCreature,
                    colors: Vec::new(),
                    image_uris: card::CardImagery {
                        png: String::new(),
                        large: String::new(),
                        normal: String::new(),
                        small: String::new(),
                        border_crop: String::new(),
                        art_crop: String::new(),
                    },
                    mana_cost: String::from("{2}"),
                    name: String::from("Dokai Weaver of Life"),
                    oracle_text: String::from("{T}: add {C}{C}"),
                    type_line: String::from("Artifact"),
                },
                ]),
            legalities: card::CardLegalities {
                standard: String::new(),
                future: String::new(),
                frontier: String::new(),
                modern: String::new(),
                legacy: String::new(),
                pauper: String::new(),
                vintage: String::new(),
                penny: String::new(),
                commander: String::new(),
                duel: String::new(),
            },
        },
        card::Card {
            id: String::from(""),
            uri: String::from(""),
            scryfall_uri: String::from(""),
            rulings_uri: String::from(""),
            name: String::from("Sol Ring"),
            cmc: 2.0,
            color_identity: Vec::new(),
            type_line: String::from("Artifact"),
            layout: String::from("normal"),
            set_name: String::from(""),
            rarity: String::from(""),
            info: card::CardInfo::Normal (card::CardFaceInfo {
                card_type: card::CardType::NonCreature,
                colors: Vec::new(),
                image_uris: card::CardImagery {
                    png: String::new(),
                    large: String::new(),
                    normal: String::new(),
                    small: String::new(),
                    border_crop: String::new(),
                    art_crop: String::new(),
                },
                mana_cost: String::from("{2}{U}{B}"),
                name: String::from("Sol Ring"),
                oracle_text: String::from("{T}: add {C}{C}"),
                type_line: String::from("Artifact"),
            }),
            legalities: card::CardLegalities {
                standard: String::new(),
                future: String::new(),
                frontier: String::new(),
                modern: String::new(),
                legacy: String::new(),
                pauper: String::new(),
                vintage: String::new(),
                penny: String::new(),
                commander: String::new(),
                duel: String::new(),
            },
        },
        card::Card {
            id: String::from(""),
            uri: String::from(""),
            scryfall_uri: String::from(""),
            rulings_uri: String::from(""),
            name: String::from("Also Sol Ring"),
            cmc: 2.0,
            color_identity: Vec::new(),
            type_line: String::from("Artifact"),
            layout: String::from("normal"),
            set_name: String::from(""),
            rarity: String::from(""),
            info: card::CardInfo::Normal (card::CardFaceInfo {
                card_type: card::CardType::NonCreature,
                colors: Vec::new(),
                image_uris: card::CardImagery {
                    png: String::new(),
                    large: String::new(),
                    normal: String::new(),
                    small: String::new(),
                    border_crop: String::new(),
                    art_crop: String::new(),
                },
                mana_cost: String::from("{2}"),
                name: String::from("Also Sol Ring"),
                oracle_text: String::from("{T}: add {C}{C}"),
                type_line: String::from("Artifact"),
            }),
            legalities: card::CardLegalities {
                standard: String::new(),
                future: String::new(),
                frontier: String::new(),
                modern: String::new(),
                legacy: String::new(),
                pauper: String::new(),
                vintage: String::new(),
                penny: String::new(),
                commander: String::new(),
                duel: String::new(),
            },
        },
        card::Card {
            id: String::from(""),
            uri: String::from(""),
            scryfall_uri: String::from(""),
            rulings_uri: String::from(""),
            name: String::from("Another Sol Ring"),
            cmc: 2.0,
            color_identity: Vec::new(),
            type_line: String::from("Artifact"),
            layout: String::from("normal"),
            set_name: String::from(""),
            rarity: String::from(""),
            info: card::CardInfo::Normal (card::CardFaceInfo {
                card_type: card::CardType::NonCreature,
                colors: Vec::new(),
                image_uris: card::CardImagery {
                    png: String::new(),
                    large: String::new(),
                    normal: String::new(),
                    small: String::new(),
                    border_crop: String::new(),
                    art_crop: String::new(),
                },
                mana_cost: String::from("{2}"),
                name: String::from("Another Sol Ring"),
                oracle_text: String::from("{T}: add {C}{C}"),
                type_line: String::from("Artifact"),
            }),
            legalities: card::CardLegalities {
                standard: String::new(),
                future: String::new(),
                frontier: String::new(),
                modern: String::new(),
                legacy: String::new(),
                pauper: String::new(),
                vintage: String::new(),
                penny: String::new(),
                commander: String::new(),
                duel: String::new(),
            },
        },
        card::Card {
            id: String::from(""),
            uri: String::from(""),
            scryfall_uri: String::from(""),
            rulings_uri: String::from(""),
            name: String::from("Yet Another Sol Ring"),
            cmc: 2.0,
            color_identity: Vec::new(),
            type_line: String::from("Artifact"),
            layout: String::from("normal"),
            set_name: String::from(""),
            rarity: String::from(""),
            info: card::CardInfo::Normal (card::CardFaceInfo {
                card_type: card::CardType::NonCreature,
                colors: Vec::new(),
                image_uris: card::CardImagery {
                    png: String::new(),
                    large: String::new(),
                    normal: String::new(),
                    small: String::new(),
                    border_crop: String::new(),
                    art_crop: String::new(),
                },
                mana_cost: String::from("{2}"),
                name: String::from("Yet Another Sol Ring"),
                oracle_text: String::from("Flash\nFlying, vigilance\nWhen Archangel Avacyn enters the battlefield, creatures you control gain indestructible until end of turn.\nWhen a non-Angel creature you control dies, transform Archangel Avacyn at the beginning of the next upkeep."),
                type_line: String::from("Artifact"),
            }),
            legalities: card::CardLegalities {
                standard: String::new(),
                future: String::new(),
                frontier: String::new(),
                modern: String::new(),
                legacy: String::new(),
                pauper: String::new(),
                vintage: String::new(),
                penny: String::new(),
                commander: String::new(),
                duel: String::new(),
            },
        },
        card::Card {
            id: String::from(""),
            uri: String::from(""),
            scryfall_uri: String::from(""),
            rulings_uri: String::from(""),
            name: String::from("Sol Ring"),
            cmc: 2.0,
            color_identity: Vec::new(),
            type_line: String::from("Artifact"),
            layout: String::from("normal"),
            set_name: String::from(""),
            rarity: String::from(""),
            info: card::CardInfo::Normal (card::CardFaceInfo {
                card_type: card::CardType::NonCreature,
                colors: Vec::new(),
                image_uris: card::CardImagery {
                    png: String::new(),
                    large: String::new(),
                    normal: String::new(),
                    small: String::new(),
                    border_crop: String::new(),
                    art_crop: String::new(),
                },
                mana_cost: String::from("{2}{U}{B}"),
                name: String::from("Sol Ring"),
                oracle_text: String::from("{T}: add {C}{C}"),
                type_line: String::from("Artifact"),
            }),
            legalities: card::CardLegalities {
                standard: String::new(),
                future: String::new(),
                frontier: String::new(),
                modern: String::new(),
                legacy: String::new(),
                pauper: String::new(),
                vintage: String::new(),
                penny: String::new(),
                commander: String::new(),
                duel: String::new(),
            },
        },
        card::Card {
            id: String::from(""),
            uri: String::from(""),
            scryfall_uri: String::from(""),
            rulings_uri: String::from(""),
            name: String::from("Also Sol Ring"),
            cmc: 2.0,
            color_identity: Vec::new(),
            type_line: String::from("Artifact"),
            layout: String::from("normal"),
            set_name: String::from(""),
            rarity: String::from(""),
            info: card::CardInfo::Normal (card::CardFaceInfo {
                card_type: card::CardType::NonCreature,
                colors: Vec::new(),
                image_uris: card::CardImagery {
                    png: String::new(),
                    large: String::new(),
                    normal: String::new(),
                    small: String::new(),
                    border_crop: String::new(),
                    art_crop: String::new(),
                },
                mana_cost: String::from("{2}"),
                name: String::from("Also Sol Ring"),
                oracle_text: String::from("{T}: add {C}{C}"),
                type_line: String::from("Artifact"),
            }),
            legalities: card::CardLegalities {
                standard: String::new(),
                future: String::new(),
                frontier: String::new(),
                modern: String::new(),
                legacy: String::new(),
                pauper: String::new(),
                vintage: String::new(),
                penny: String::new(),
                commander: String::new(),
                duel: String::new(),
            },
        },
        card::Card {
            id: String::from(""),
            uri: String::from(""),
            scryfall_uri: String::from(""),
            rulings_uri: String::from(""),
            name: String::from("Another Sol Ring"),
            cmc: 2.0,
            color_identity: Vec::new(),
            type_line: String::from("Artifact"),
            layout: String::from("normal"),
            set_name: String::from(""),
            rarity: String::from(""),
            info: card::CardInfo::Normal (card::CardFaceInfo {
                card_type: card::CardType::NonCreature,
                colors: Vec::new(),
                image_uris: card::CardImagery {
                    png: String::new(),
                    large: String::new(),
                    normal: String::new(),
                    small: String::new(),
                    border_crop: String::new(),
                    art_crop: String::new(),
                },
                mana_cost: String::from("{2}"),
                name: String::from("Another Sol Ring"),
                oracle_text: String::from("{T}: add {C}{C}"),
                type_line: String::from("Artifact"),
            }),
            legalities: card::CardLegalities {
                standard: String::new(),
                future: String::new(),
                frontier: String::new(),
                modern: String::new(),
                legacy: String::new(),
                pauper: String::new(),
                vintage: String::new(),
                penny: String::new(),
                commander: String::new(),
                duel: String::new(),
            },
        },
        card::Card {
            id: String::from(""),
            uri: String::from(""),
            scryfall_uri: String::from(""),
            rulings_uri: String::from(""),
            name: String::from("Yet Another Sol Ring"),
            cmc: 2.0,
            color_identity: Vec::new(),
            type_line: String::from("Artifact"),
            layout: String::from("normal"),
            set_name: String::from(""),
            rarity: String::from(""),
            info: card::CardInfo::Normal (card::CardFaceInfo {
                card_type: card::CardType::NonCreature,
                colors: Vec::new(),
                image_uris: card::CardImagery {
                    png: String::new(),
                    large: String::new(),
                    normal: String::new(),
                    small: String::new(),
                    border_crop: String::new(),
                    art_crop: String::new(),
                },
                mana_cost: String::from("{2}"),
                name: String::from("Yet Another Sol Ring"),
                oracle_text: String::from("Flash\nFlying, vigilance\nWhen Archangel Avacyn enters the battlefield, creatures you control gain indestructible until end of turn.\nWhen a non-Angel creature you control dies, transform Archangel Avacyn at the beginning of the next upkeep."),
                type_line: String::from("Artifact"),
            }),
            legalities: card::CardLegalities {
                standard: String::new(),
                future: String::new(),
                frontier: String::new(),
                modern: String::new(),
                legacy: String::new(),
                pauper: String::new(),
                vintage: String::new(),
                penny: String::new(),
                commander: String::new(),
                duel: String::new(),
            },
        },
    ];

    let not_found = vec!["foobar".to_string(), "booasf".to_string()];
    page::save_pdf(cards, not_found);
}

