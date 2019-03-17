extern crate error_chain;
extern crate serde_derive;
extern crate serde_json;
extern crate reqwest;

use super::card;

error_chain! {
    foreign_links {
        Reqwest(reqwest::Error);
    }
}

pub struct QueryResults {
    pub cards: Vec<card::Card>,
    pub not_found: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
struct Card {
    id: String,
    name: String,
    uri: String,
    scryfall_uri: String,
    rulings_uri: String,
    layout: String,
    mana_cost: Option<String>,
    cmc: f64,
    color_identity: Vec<String>,
    colors: Option<Vec<String>>,
    set_name: String,
    type_line: String,
    oracle_text: Option<String>,
    card_faces: Option<Vec<CardFace>>,
    legalities: CardLegalities,
    rarity: String,
    image_uris: Option<CardImagery>,
    power: Option<String>,
    toughness: Option<String>,
    loyalty: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
struct CardFace {
    name: String,
    mana_cost: String,
    type_line: String,
    colors: Option<Vec<String>>,
    oracle_text: String,
    image_uris: Option<CardImagery>,
    power: Option<String>,
    toughness: Option<String>,
    loyalty: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
struct CardLegalities {
    standard: String,
    future: String,
    frontier: String,
    modern: String,
    legacy: String,
    pauper: String,
    vintage: String,
    penny: String,
    commander: String,
    duel: String,
}

#[derive(Deserialize, Debug, Clone)]
struct CardImagery {
    png: String,
    large: String,
    normal: String,
    small: String,
    border_crop: String,
    art_crop: String,
}

#[derive(Deserialize, Debug, Clone)]
struct CardList {
    data: Vec<Card>,
    not_found: Vec<CardIdentifier>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct CardIdentifier {
    name: String,
}

#[derive(Serialize, Debug, Clone)]
struct CardIdentifiers {
    identifiers: Vec<CardIdentifier>,
}

pub fn query(card_names: Vec<String>) -> Result<QueryResults> {
    let request_url = format!("https://api.scryfall.com/cards/collection");
    let card_query = CardIdentifiers {
        identifiers: card_names.iter().map(|card_name| {
            CardIdentifier { name: card_name.clone() }
        }).collect()
    };
    let serialized = serde_json::to_string(&card_query).unwrap();

    let client = reqwest::Client::new();


    let mut response = client.post(&request_url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .body(serialized)
        .send()?;

    let text = response.text()?;
    let cardlist: CardList = serde_json::from_str(&text).unwrap();
    let result = dto_to_cards(cardlist);
    Ok(result)
}

fn dto_to_cards(cardlist: CardList) -> QueryResults {
    let not_found = cardlist.not_found.into_iter().map(|ident|{
        ident.name
    }).collect();


    let cards = cardlist.data.into_iter().map(|card|{
        let colors = card.colors.clone();
        let image_uris = card.image_uris.clone();

        let info: card::CardInfo = match card.card_faces {
            Some(faces) => {
                match card.layout.as_ref() {
                    "split" => {

                card::CardInfo::Split (faces.into_iter().map(|face|{
                    println!("{:?}", &face);
                    let card_type = dto_to_type(&face.type_line, face.power, face.toughness, face.loyalty);
                    card::CardFaceInfo {
                        card_type,
                        colors: face.colors.unwrap_or_else(||colors.clone().unwrap_or(Vec::new())),
                        image_uris: dto_to_imagery(face.image_uris.unwrap_or_else(||image_uris.clone().unwrap())),
                        mana_cost: face.mana_cost,
                        name: face.name,
                        oracle_text: face.oracle_text,
                        type_line: face.type_line,
                    }
                }).collect())
                    }
                    "flip" => {

                card::CardInfo::Flip (faces.into_iter().map(|face|{
                    println!("{:?}", &face);
                    let card_type = dto_to_type(&face.type_line, face.power, face.toughness, face.loyalty);
                    card::CardFaceInfo {
                        card_type,
                        colors: face.colors.unwrap_or_else(||colors.clone().unwrap_or(Vec::new())),
                        image_uris: dto_to_imagery(face.image_uris.unwrap_or_else(||image_uris.clone().unwrap())),
                        mana_cost: face.mana_cost,
                        name: face.name,
                        oracle_text: face.oracle_text,
                        type_line: face.type_line,
                    }
                }).collect())
                    }
                    _ => {

                card::CardInfo::Multifaced (faces.into_iter().map(|face|{
                    println!("{:?}", &face);
                    let card_type = dto_to_type(&face.type_line, face.power, face.toughness, face.loyalty);
                    card::CardFaceInfo {
                        card_type,
                        colors: face.colors.unwrap_or_else(||colors.clone().unwrap_or(Vec::new())),
                        image_uris: dto_to_imagery(face.image_uris.unwrap_or_else(||image_uris.clone().unwrap())),
                        mana_cost: face.mana_cost,
                        name: face.name,
                        oracle_text: face.oracle_text,
                        type_line: face.type_line,
                    }
                }).collect())
                    }
                }
            }

            None => {
                let card_type = dto_to_type(&card.type_line, card.power, card.toughness, card.loyalty);
                card::CardInfo::Normal (card::CardFaceInfo {
                    card_type,
                    colors: card.colors.unwrap(),
                    image_uris: dto_to_imagery(card.image_uris.unwrap()),
                    mana_cost: card.mana_cost.unwrap(),
                    name: card.name.clone(),
                    oracle_text: card.oracle_text.unwrap(),
                    type_line: card.type_line.clone(),
                })

            }
        };

        let legalities = dto_to_legalities(card.legalities);

        card::Card{
            id: card.id,
            uri: card.uri,
            scryfall_uri: card.scryfall_uri,
            rulings_uri: card.rulings_uri,
            name: card.name,
            cmc: card.cmc,
            color_identity: card.color_identity,
            type_line: card.type_line,
            layout: card.layout,
            set_name: card.set_name,
            rarity: card.rarity,
            info,
            legalities,
        }
    }).collect();

    QueryResults {
        cards,
        not_found,
    }
}

fn dto_to_type(type_line: &String, power: Option<String>, toughness: Option<String>, loyalty: Option<String>) -> card::CardType {
    //FIXME Doesn't properly detect Legendary or multitype
    if type_line.starts_with("Planeswalker") {
        card::CardType::Planeswalker {loyalty: loyalty.unwrap()}
    }
    else if type_line.contains("Creature") || type_line.contains("Vehicle") {
        card::CardType::Creature {power: power.unwrap(), toughness: toughness.unwrap()}
    }
    else {
        card::CardType::NonCreature
    }
}

fn dto_to_legalities(leg: CardLegalities) -> card::CardLegalities {
    card::CardLegalities {
        standard: leg.standard,
        future: leg.future,
        frontier: leg.frontier,
        modern: leg.modern,
        legacy: leg.legacy,
        pauper: leg.pauper,
        vintage: leg.vintage,
        penny: leg.penny,
        commander: leg.commander,
        duel: leg.duel,
    }
}

fn dto_to_imagery(im: CardImagery) -> card::CardImagery {
    card::CardImagery {
        png: im.png,
        large: im.large,
        normal: im.normal,
        small: im.small,
        border_crop: im.border_crop,
        art_crop: im.art_crop,
    }
}

