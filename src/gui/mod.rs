use iui::prelude::*;
use iui::controls::{Label, Entry, PasswordEntry, VerticalBox, HorizontalBox, HorizontalSeparator, Group, TabGroup, Spacer, Button};
use std::rc::Rc;
use std::cell::RefCell;
mod identity;
mod luceo;
mod copernica;
mod whistle;
mod settings;
use {
    identity::{ identity },
    luceo::{ luceo },
    copernica::{ copernica },
    whistle::{ whistle },
    settings::{ settings },
    crate::state::{State, FractalideState, CopernicaState, LuceoState, SettingsState},
};

pub fn setup() {
    let mut state: State = Rc::new(RefCell::new(
        FractalideState {
            copernica: CopernicaState {
                query_val: "".into(),
            },
            luceo: LuceoState {
                query_val: "".into(),
            },
            settings: SettingsState {
                val: "".into(),
            },
        }
    ));
    let ui = UI::init().unwrap();
    let mut tab_group = TabGroup::new(&ui);

    // ************** Copernica ****************
    let mut copernica_tg = TabGroup::new(&ui);
    let mut copernica_events = copernica(&ui, &mut copernica_tg, state.clone());
    tab_group.append(&ui, "Copernica", copernica_tg);

    // ************** Luceo ****************
    let mut luceo_tg = TabGroup::new(&ui);
    luceo(&ui, &mut luceo_tg);
    tab_group.append(&ui, "Luceo", luceo_tg);

    // ************** Whistle ****************
    let mut whistle_tg = TabGroup::new(&ui);
    whistle(&ui, &mut whistle_tg);
    tab_group.append(&ui, "Whistle", whistle_tg);

    // ************** Identities ****************
    let mut identity_tg = TabGroup::new(&ui);
    identity(&ui, &mut identity_tg);
    tab_group.append(&ui, "Identities", identity_tg);

    // ************** Settings ****************
    let mut settings_tg = TabGroup::new(&ui);
    let mut settings_events = settings(&ui, &mut settings_tg, state.clone());
    tab_group.append(&ui, "Settings", settings_tg);

    let mut window = Window::new(&ui, "Fractalide", 300, 150, WindowType::NoMenubar);
    window.set_child(&ui, tab_group);
    window.show(&ui);
    let functions = move || { copernica_events(); settings_events(); };
    let mut event_loop = ui.event_loop();
    event_loop.on_tick(&ui, functions);
    event_loop.run(&ui);
}
