use printpdf::*;
use std::fs::File;
use std::io::BufWriter;

use super::card;

const WHITE: Color = Color::Rgb(Rgb{r:1.0, g:1.0, b:1.0, icc_profile:None});
const BLACK: Color = Color::Rgb(Rgb{r:0.0, g:0.0, b:0.0, icc_profile:None});

const TEXT_MARGIN: In = In(0.125);
const LINE_WIDTH: usize = 50;

struct In(pub f64);

impl Into<Mm> for &In {
    fn into(self) -> Mm {
        Mm(self.0 * 25.4)
    }
}

impl Into<Mm> for In {
    fn into(self) -> Mm {
        Mm(self.0 * 25.4)
    }
}

impl From<Mm> for In {
    fn from(mm:Mm) -> In {
        In(mm.0 / 25.4)
    }
}

pub fn save_pdf(cards: Vec<card::Card>, not_found: Vec<String>) {
    let r1 = In(4.625);
    let r2 = In(0.325);
    let c1 = In(0.2);
    let c2 = In(2.9);
    let c3 = In(5.6);
    let c4 = In(8.3);

    let (doc, page1, layer1) = PdfDocument::new("PDF_Document_title", In(11.0).into(), In(8.5).into(), "Page 1");
    let font = doc.add_builtin_font(BuiltinFont::TimesRoman).unwrap();
    let mut page = page1;

    if not_found.len() != 0 {
        let x = In(1.0);
        let y = In(8.0);

        let current_layer = doc.get_page(page1).get_layer(layer1);
        current_layer.begin_text_section();
        current_layer.set_font(&font, 14);
        current_layer.set_line_height(14);
        current_layer.set_text_cursor(
            In(x.0+TEXT_MARGIN.0).into(),
            In(y.0).into());

        current_layer.write_text(
            String::from("Not Found"),
            &font);
        current_layer.add_line_break();

        current_layer.set_font(&font, 12);
        current_layer.set_line_height(12);

        for line in not_found {
            current_layer.write_text(
                line.clone(),
                &font);
            current_layer.add_line_break();
        }
        current_layer.end_text_section();
        let (page2, _layer) = doc.add_page(In(11.0).into(), In(8.5).into(),"Page 2");
        page = page2;
    }

    let a = [
        (&c1, &r1),
        (&c1, &r2),
        (&c2, &r1),
        (&c2, &r2),
        (&c3, &r1),
        (&c3, &r2),
        (&c4, &r1),
        (&c4, &r2),
    ];
    let mut cells = a.iter().cycle();

    let mut current_layer = doc.get_page(page).get_layer(layer1);

    let mut face_num = 0;
    for card in cards.iter() {
        match card.info.clone() {
            card::CardInfo::Normal(face) => {
                if face_num != 0 && face_num % 8 == 0 {
                    let (page, layer) = doc.add_page(
                        In(11.0).into(), In(8.5).into(),
                        format!("Page {}", (face_num/8)+1));
                    current_layer = doc.get_page(page).get_layer(layer);
                }
                face_num = face_num + 1;
                let (c, r) = cells.next().unwrap();
                draw_face(&c, &r, &face, &font, &current_layer);
            }

            card::CardInfo::Multifaced(faces) => {
                for face in faces {
                    if face_num != 0 && face_num % 8 == 0 {
                        let (page, layer) = doc.add_page(
                            In(11.0).into(), In(8.5).into(),
                            format!("Page {}", (face_num/8)+1));
                        current_layer = doc.get_page(page).get_layer(layer);
                    }
                    face_num = face_num + 1;
                    let (c, r) = cells.next().unwrap();
                    draw_face(&c, &r, &face, &font, &current_layer);
                }
            }

            card::CardInfo::Split(faces) => {
                if face_num != 0 && face_num % 8 == 0 {
                    let (page, layer) = doc.add_page(
                        In(11.0).into(), In(8.5).into(),
                        format!("Page {}", (face_num/8)+1));
                    current_layer = doc.get_page(page).get_layer(layer);
                }
                face_num = face_num + 1;
                let (c, r) = cells.next().unwrap();
                draw_split(&c, &r, &faces, &font, &current_layer);
            }

            card::CardInfo::Flip(faces) => {
                if face_num != 0 && face_num % 8 == 0 {
                    let (page, layer) = doc.add_page(
                        In(11.0).into(), In(8.5).into(),
                        format!("Page {}", (face_num/8)+1));
                    current_layer = doc.get_page(page).get_layer(layer);
                }
                face_num = face_num + 1;
                let (c, r) = cells.next().unwrap();
                draw_flip(&c, &r, &faces, &font, &current_layer);
            }
        }
    }

    doc.save(&mut BufWriter::new(File::create("cards.pdf").unwrap())).unwrap();
}

fn draw_face(x: &In, y: &In, face: &card::CardFaceInfo, font: &IndirectFontRef, current_layer: &PdfLayerReference) {
    draw_border(&x, &y, current_layer);
    current_layer.set_fill_color(BLACK.clone());

    current_layer.use_text(
        face.name.clone(),
        12,
        In(x.0+TEXT_MARGIN.0).into(),
        In(y.0+3.25).into(),
        &font);

    let mc_len = face.mana_cost.len() as f64;
    current_layer.use_text(
        face.mana_cost.clone(),
        12,
        In(x.0+TEXT_MARGIN.0+2.3-(mc_len * 0.1)).into(),
        In(y.0+3.25).into(),
        &font);

    current_layer.use_text(
        face.type_line.clone(),
        10,
        In(x.0+TEXT_MARGIN.0).into(),
        In(y.0+3.0).into(),
        &font);

    current_layer.begin_text_section();
    current_layer.set_font(&font, 8);
    current_layer.set_line_height(8);
    current_layer.set_text_cursor(
        In(x.0+TEXT_MARGIN.0).into(),
        In(y.0+2.75).into());

    let oracle_text = &face.oracle_text;
    for text_line in oracle_text.lines() {
        let lines = word_wrap(&text_line.to_string());
        for line in lines {
            current_layer.write_text(
                line.clone(),
                &font);
            current_layer.add_line_break();
        }
        current_layer.add_line_break();
    }
    current_layer.end_text_section();

    match &face.card_type {
        card::CardType::NonCreature => {}
        card::CardType::Creature {power, toughness} => {
    current_layer.use_text(
        power.clone(),
        10,
        In(x.0+2.5-0.25-(TEXT_MARGIN.0*2.0)).into(),
        In(y.0+TEXT_MARGIN.0).into(),
        &font);

    current_layer.use_text(
        toughness.clone(),
        10,
        In(x.0+2.5-(TEXT_MARGIN.0*2.0)).into(),
        In(y.0+TEXT_MARGIN.0).into(),
        &font);

        }
        card::CardType::Planeswalker {loyalty} => {
    current_layer.use_text(
        loyalty.clone(),
        10,
        In(x.0+2.5-(TEXT_MARGIN.0*2.0)).into(),
        In(y.0+TEXT_MARGIN.0).into(),
        &font);

        }
    }

}

fn draw_split(x: &In, y: &In, faces: &Vec<card::CardFaceInfo>, font: &IndirectFontRef, current_layer: &PdfLayerReference) {
    draw_border(&x, &y, current_layer);
    current_layer.set_fill_color(BLACK.clone());
    let num_faces = faces.len() as f64;

    for (face_num, face) in faces.iter().enumerate() {
        let face_num_f = face_num as f64;
        let face_pos = 3.25 - ((3.0/num_faces)*face_num_f);
        current_layer.use_text(
            face.name.clone(),
            12,
            In(x.0+TEXT_MARGIN.0).into(),
            In(y.0+face_pos).into(),
            &font);

        let mc_len = face.mana_cost.len() as f64;
        current_layer.use_text(
            face.mana_cost.clone(),
            12,
            In(x.0+TEXT_MARGIN.0+2.3-(mc_len * 0.1)).into(),
            In(y.0+face_pos).into(),
            &font);

        current_layer.use_text(
            face.type_line.clone(),
            10,
            In(x.0+TEXT_MARGIN.0).into(),
            In(y.0+face_pos-0.25).into(),
            &font);

        current_layer.begin_text_section();
        current_layer.set_font(&font, 8);
        current_layer.set_line_height(8);
        current_layer.set_text_cursor(
            In(x.0+TEXT_MARGIN.0).into(),
            In(y.0+face_pos-0.4).into());

        let oracle_text = &face.oracle_text;
        for text_line in oracle_text.lines() {
            let lines = word_wrap(&text_line.to_string());
            for line in lines {
                current_layer.write_text(
                    line.clone(),
                    &font);
                current_layer.add_line_break();
            }
            current_layer.add_line_break();
        }
        current_layer.end_text_section();

        let ptl_pos = (3.25 - ((3.0/num_faces)*(face_num_f+1.0)))
            + (TEXT_MARGIN.0*2.0);
        match &face.card_type {
            card::CardType::NonCreature => {}
            card::CardType::Creature {power, toughness} => {
                current_layer.use_text(
                    power.clone(),
                    10,
                    In(x.0+2.5-0.25-(TEXT_MARGIN.0*2.0)).into(),
                    In(y.0+ptl_pos).into(),
                    &font);

                current_layer.use_text(
                    toughness.clone(),
                    10,
                    In(x.0+2.5-(TEXT_MARGIN.0*2.0)).into(),
                    In(y.0+ptl_pos).into(),
                    &font);

            }
            card::CardType::Planeswalker {loyalty} => {
                current_layer.use_text(
                    loyalty.clone(),
                    10,
                    In(x.0+2.5-(TEXT_MARGIN.0*2.0)).into(),
                    In(y.0+ptl_pos).into(),
                    &font);

            }
        }
    }

}

fn draw_flip(x: &In, y: &In, faces: &Vec<card::CardFaceInfo>, font: &IndirectFontRef, current_layer: &PdfLayerReference) {
    draw_split(&x, &y, &faces, &font, &current_layer);
}

fn word_wrap(text: &String) -> Vec<String> {
    let words = text.split_whitespace();
    let mut lines = Vec::new();
    let mut line = String::new();

    for word in words {
        if line.len() + word.len() + 1 < LINE_WIDTH {
            line.push(' ');
            line.push_str(word);
        }
        else {
            lines.push(line);
            line = String::new();
            line.push_str(word);
        }
    }
    lines.push(line);
    lines
}

fn draw_border(x: &In, y:&In, current_layer: &PdfLayerReference) {
    // Quadratic shape. The "false" determines if the next (following)
    // point is a bezier handle (for curves)
    // If you want holes, simply reorder the winding of the points to be
    // counterclockwise instead of clockwise.
    let border_points = vec![
        (Point::new(x.into(), y.into()), false),
        (Point::new(x.into(), In(y.0+3.5).into()), false),
        (Point::new(In(x.0+2.5).into(), In(y.0+3.5).into()), false),
        (Point::new(In(x.0+2.5).into(), y.into()), false)];

    // Is the shape stroked? Is the shape closed? Is the shape filled?
    let border = Line {
        points: border_points,
        is_closed: true,
        has_fill: true,
        has_stroke: true,
        is_clipping_path: false,
    };

    current_layer.set_fill_color(WHITE.clone());
    current_layer.set_outline_color(BLACK.clone());
    current_layer.set_outline_thickness(5.0);
    current_layer.set_line_join_style(LineJoinStyle::Round);

    current_layer.add_shape(border);

}

