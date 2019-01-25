
#[derive(Clone)]
pub struct Card {
    pub id: String,
    pub uri: String,
    pub scryfall_uri: String,
    pub rulings_uri: String,
    pub name: String,
    pub cmc: f64,
    pub color_identity: Vec<String>,
    pub type_line: String,
    pub layout: String,
    pub set_name: String,
    pub rarity: String,
    pub info: CardInfo,
    pub legalities: CardLegalities,
}

#[derive(Clone)]
pub enum CardInfo {
    Normal (CardFaceInfo),
    Split (Vec<CardFaceInfo>),
    Flip (Vec<CardFaceInfo>),
    Multifaced (Vec<CardFaceInfo>),
}

#[derive(Clone)]
pub struct CardFaceInfo {
    pub card_type: CardType,
    pub colors: Vec<String>,
    pub image_uris: CardImagery,
    pub mana_cost: String,
    pub name: String,
    pub oracle_text: String,
    pub type_line: String,
}

#[derive(Clone, Debug)]
pub enum CardType {
    Planeswalker {loyalty: String},
    Creature {power: String, toughness: String},
    NonCreature,
}

#[derive(Clone, Debug)]
pub struct CardLegalities {
    pub standard: String,
    pub future: String,
    pub frontier: String,
    pub modern: String,
    pub legacy: String,
    pub pauper: String,
    pub vintage: String,
    pub penny: String,
    pub commander: String,
    pub c1v1: String,
    pub duel: String,
    pub brawl: String,
}

#[derive(Clone)]
pub struct CardImagery {
    pub png: String,
    pub large: String,
    pub normal: String,
    pub small: String,
    pub border_crop: String,
    pub art_crop: String,
}

pub fn print_card(card: &Card) {
    println!("id\t{}", card.id);
    println!("name\t{}", card.name);
    println!("layout\t{}", card.layout);
    println!("rarity\t{}", card.rarity);
    println!("cmc\t{}", card.cmc);
    println!("typestr\t{}", card.type_line);

    match &card.info {
        CardInfo::Normal (info) => {
            println!("type\t{:?}", info.card_type);
            println!("text\t{:?}", info.oracle_text);
        }

        CardInfo::Split (faces) => {
            for face in faces {
                println!("------- name\t{}", face.name);
                println!("\tmc\t{}", face.mana_cost);
                println!("\ttypestr\t{}", face.type_line);
                println!("\ttype\t{:?}", face.card_type);
                println!("\ttext\t{:?}", face.oracle_text);
            }
        }

        CardInfo::Flip (faces) => {
            for face in faces {
                println!("------- name\t{}", face.name);
                println!("\tmc\t{}", face.mana_cost);
                println!("\ttypestr\t{}", face.type_line);
                println!("\ttype\t{:?}", face.card_type);
                println!("\ttext\t{:?}", face.oracle_text);
            }
        }

        CardInfo::Multifaced (faces) => {
            for face in faces {
                println!("------- name\t{}", face.name);
                println!("\tmc\t{}", face.mana_cost);
                println!("\ttypestr\t{}", face.type_line);
                println!("\ttype\t{:?}", face.card_type);
                println!("\ttext\t{:?}", face.oracle_text);
            }

        }
    }

    println!("{:?}", card.legalities);
}

