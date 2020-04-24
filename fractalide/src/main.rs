use {
    crate::state::{State, FractalideState, CopernicaState, LuceoState, WhistleState, IdentitiesState, SettingsState},
    structopt::StructOpt,
    std::{
        sync::{Arc, Mutex},
        io::BufReader,
        thread,
        path::{
            Path,
        },
        fs,
    },
    serde_derive::Deserialize,
    async_std::{ task, },
};

mod gui;
mod state;

#[derive(StructOpt, Debug)]
#[structopt(
    name    = "fractalide",
    about   = "A front end for Copernica, Luceo and Whistles",
    author  = "Stewart Mackenzie <sjm@fractalide.com>",
    version = "0.1.0"
)]

struct Options {
    #[structopt(short = "c", long = "config", help = "Location of your fractalide config file")]
    config: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FractalideConfig {
    pub copernica_config_path: String,
}

impl FractalideConfig {
    pub fn new() -> FractalideConfig {
        let mut data_dir = dirs::home_dir().unwrap();
        data_dir.push(".copernica");
        data_dir.push("copernica.json");
        FractalideConfig {
            copernica_config_path: data_dir.to_string_lossy().to_string(),
        }
    }
}

fn main() {
    let options = Options::from_args();
    let mut fractalide_config = FractalideConfig::new();
    if let Some(fractalide_config_path) = options.config {
        let file = fs::File::open(fractalide_config_path).unwrap();
        let reader = BufReader::new(file);
        fractalide_config = serde_json::from_reader(reader).unwrap();
    }
    let state: State = Arc::new(Mutex::new(
        FractalideState {
            copernica: CopernicaState {
                query_val: "".into(),
            },
            luceo: LuceoState {
                test_entry: "".into(),
            },
            whistle: WhistleState {
                test_entry: "".into(),
            },
            settings: SettingsState {
                val: "".into(),
            },
            identities: IdentitiesState {
                current_identity: "".into(),
            },
        }
    ));

    {
        use qt_widgets::QApplication;
        QApplication::init(|_| unsafe {
            let _form = crate::gui::Form::new();
            QApplication::exec()
        });
    }
}
