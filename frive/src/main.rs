mod copernica;
mod luceo;
mod whistle;
use {
    structopt::StructOpt,
    libfractalide::{gstreamer},
    cursive_tabs::TabPanel,
    cursive::{
        Cursive,
        align::HAlign,
        traits::*,
        views::{Dialog, Panel, Button, TextView, EditView, DummyView, LinearLayout, RadioGroup},
    },
};

#[derive(StructOpt, Debug)]
#[structopt(
    name    = "frive",
    about   = "A cursive/ncurses interface to Copernica, Luceo and Whistle",
    author  = "Stewart Mackenzie <sjm@fractalide.com>",
    version = "0.1.0"
)]

struct Options {
    #[structopt(short = "c", long = "config", help = "Location of your fractalide config file")]
    config: Option<String>,
}
/*
fn main() {
    let mut siv = Cursive::default();
    siv.add_global_callback('q', |s| s.quit());
    let copernica_tp = TabPanel::new()
        .with_tab("Search", copernica::search::dialog())
        .with_tab("Manage", copernica::manage::dialog())
        .with_tab("Settings", copernica::settings::dialog());

    let luceo_overview_tp = luceo::wallet_overview::dialog();

    let whistle_tp = TabPanel::new()
        .with_tab("Create", whistle::create::dialog())
        .with_tab("Search", whistle::search::dialog())
        .with_tab("Settings", whistle::settings::dialog());

    let mut top_tp = TabPanel::new()
        .with_tab("Copernica", copernica_tp)
        .with_tab("Luceo", luceo_overview_tp)
        .with_tab("Whistle", whistle_tp);

    siv.add_layer(top_tp);
    siv.run();
}
*/

fn main() {
    gstreamer()
}
