use bevy::prelude::*;
use indexmap::IndexMap;
use rand::Rng;

static KHMER_CONSONS_NUMBER: usize = 33;

#[derive(Clone)]
struct TextBoxArea {
    x: f32,
    y: f32,
    z: f32,
    font_size: f32,
}
static LEFT_BOX: TextBoxArea = TextBoxArea {
    x: -220.0,
    y: 0.0,
    z: 2.0,
    font_size: 160.0,
};

static RIGHT_BOX: TextBoxArea = TextBoxArea {
    x: 120.0,
    y: 150.0,
    z: 2.0,
    font_size: 30.0,
};

static BOTTOM_BOX: TextBoxArea = TextBoxArea {
    x: -120.0,
    y: -300.0,
    z: 2.0,
    font_size: 20.0,
};

pub struct AlphabetPlugin;

impl Plugin for AlphabetPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_consons);
    }
}

fn setup_consons(mut commands: Commands, asset_server: Res<AssetServer>) {
    let consons = get_consons();
    let n: usize = random_number(13);

    let letter = consons.get_index(n);

    // let letter = consons
    //     .iter()
    //     .find(|item| item.0 == &"T");

    display_text(
        &mut commands,
        &asset_server,
        &&("Hey, provide the khmer letter that corresponds to that sound on the LEFT BOX ?"),
        &BOTTOM_BOX,
    );

    let _ = match letter {
        Some(name) => {
            display_text(&mut commands, &asset_server, *name.0, &LEFT_BOX);

            let matrix_lines: Vec<Vec<ConsonMatrixItem>> = random_consons_matrix();

            matrix_lines.iter().for_each(|consons_line| {
                consons_line.iter().for_each(|x| {
                    // let my_text = format!("{}. {} ", x.conson.0, x.conson.1);
                    let my_text = format!("{}", x.conson.1);

                    let b = TextBoxArea {
                        x: RIGHT_BOX.x + (x.column_number as f32 * 60.0),
                        y: RIGHT_BOX.y + (x.line_number as f32 * -60.0),
                        ..RIGHT_BOX
                    };

                    display_text(&mut commands, &asset_server, &my_text, &b);
                });
            });
        }
        None => println!("No name provided"),
    };
}

#[derive(Clone)]
pub struct Conson {
    pub letter: &'static str,
    pub phonetic: &'static str,
}

fn get_consons() -> IndexMap<&'static str, Vec<Conson>> {
    return IndexMap::from([
        (
            "K",
            vec![
                Conson {
                    letter: "ក",
                    phonetic: "Kö",
                },
                Conson {
                    letter: "ខ",
                    phonetic: "Khö",
                },
                Conson {
                    letter: "គ",
                    phonetic: "Ko",
                },
                Conson {
                    letter: "ឃ",
                    phonetic: "Kho",
                },
            ],
        ),
        (
            "Tch",
            vec![
                Conson {
                    letter: "ច",
                    phonetic: "Tchö",
                },
                Conson {
                    letter: "ឆ",
                    phonetic: "Tchhö",
                },
                Conson {
                    letter: "ជ",
                    phonetic: "Tcho",
                },
                Conson {
                    letter: "ឈ",
                    phonetic: "Tchho",
                },
            ],
        ),
        (
            "D",
            vec![
                Conson {
                    letter: "ដ",
                    phonetic: "Dö",
                },
                Conson {
                    letter: "ឌ",
                    phonetic: "Do",
                },
            ],
        ),
        (
            "D",
            vec![
                Conson {
                    letter: "ដ",
                    phonetic: "Dö",
                },
                Conson {
                    letter: "ឌ",
                    phonetic: "Do",
                },
            ],
        ),
        (
            "T",
            vec![
                Conson {
                    letter: "ឋ",
                    phonetic: "Thö",
                },
                Conson {
                    letter: "ឍ",
                    phonetic: "Tho",
                },
                Conson {
                    letter: "ត",
                    phonetic: "Tö",
                },
                Conson {
                    letter: "ថ",
                    phonetic: "Thö",
                },
                Conson {
                    letter: "ទ",
                    phonetic: "To",
                },
                Conson {
                    letter: "ផ",
                    phonetic: "Tho",
                },
            ],
        ),
        (
            "R",
            vec![Conson {
                letter: "រ",
                phonetic: "Ro",
            }],
        ),
        (
            "S",
            vec![Conson {
                letter: "ស",
                phonetic: "Sö",
            }],
        ),
        (
            "L",
            vec![
                Conson {
                    letter: "ឥ",
                    phonetic: "Lo",
                },
                Conson {
                    letter: "ឦ",
                    phonetic: "Lö",
                },
            ],
        ),
        (
            "V",
            vec![Conson {
                letter: "វ",
                phonetic: "Vo",
            }],
        ),
        (
            "P",
            vec![
                Conson {
                    letter: "ព",
                    phonetic: "Po",
                },
                Conson {
                    letter: "ភ",
                    phonetic: "Pho",
                },
            ],
        ),
        (
            "M",
            vec![Conson {
                letter: "ម",
                phonetic: "Mo",
            }],
        ),
        (
            "B",
            vec![Conson {
                letter: "ប",
                phonetic: "Bo",
            }],
        ),
        (
            "N",
            vec![
                Conson {
                    letter: "ន",
                    phonetic: "No",
                },
                Conson {
                    letter: "ណ",
                    phonetic: "Nö",
                },
            ],
        ),
        (
            "NG",
            vec![
                Conson {
                    letter: "ង",
                    phonetic: "Ngo",
                },
                Conson {
                    letter: "ញ",
                    phonetic: "Nio",
                },
            ],
        ),
        (
            "H",
            vec![Conson {
                letter: "ហ",
                phonetic: "Hö",
            }],
        ),
    ]);
}

#[derive(Clone)]
struct ConsonMatrixItem {
    line_number: usize,
    column_number: usize,
    conson: (usize, &'static str),
}

fn random_consons_matrix() -> Vec<Vec<ConsonMatrixItem>> {
    let consons_local = get_consons();

    let mut consons: Vec<(usize, &Conson)> = consons_local
        .iter()
        .flat_map(|f| f.1.iter())
        .map(|x| (random_number(KHMER_CONSONS_NUMBER), x))
        .collect();

    consons.sort_by(|a, b| a.0.cmp(&b.0));

    consons
        .iter()
        .map(|x| x.1)
        .cloned()
        .collect::<Vec<Conson>>()
        .into_iter()
        .enumerate()
        .map(|(index, c)| (index, c.letter))
        .collect::<Vec<(usize, &'static str)>>()
        .chunks(5)
        .enumerate()
        .map(|(line_number, c)| {
            c.into_iter()
                .enumerate()
                .map(|(column_number, x)| ConsonMatrixItem {
                    line_number: line_number,
                    column_number: column_number,
                    conson: x.clone(),
                })
                .into_iter()
                .collect::<Vec<ConsonMatrixItem>>()
                .to_vec()
        })
        .collect()
}

fn random_number(max: usize) -> usize {
    let mut rng = rand::rng();
    rng.random_range(0..=max)
}

fn display_text(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    text: &str,
    area: &TextBoxArea,
) {
    commands.spawn((
        Text2d::new(text.to_string()),
        TextFont {
            font: asset_server.load("fonts/khmer.ttf"),
            font_size: area.font_size,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform::from_xyz(area.x, area.y, area.z),
    ));
}
