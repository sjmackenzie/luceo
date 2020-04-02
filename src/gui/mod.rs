use iui::prelude::*;
use iui::controls::{Label, Entry, PasswordEntry, VerticalBox, HorizontalBox, HorizontalSeparator, Group, TabGroup, Spacer, Button};
use std::rc::Rc;
use std::cell::RefCell;
mod identity;
mod luceo;
mod copernica;
mod whistle;
mod settings;
use { crate::state::{State, FractalideState, CopernicaState, LuceoState, WhistleState, SettingsState}, };

pub fn setup() {
    let state: State = Rc::new(RefCell::new(
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
        }
    ));
    let ui = UI::init().unwrap();
    let mut tab_group = TabGroup::new(&ui);

    // ************** Copernica ****************
    let mut copernica_tg = TabGroup::new(&ui);
    let mut copernica_events = copernica::setup(&ui, &mut copernica_tg, state.clone());
    tab_group.append(&ui, "Copernica", copernica_tg);

    // ************** Luceo ****************
    let mut luceo_tg = TabGroup::new(&ui);
    let mut luceo_events = luceo::setup(&ui, &mut luceo_tg, state.clone());
    tab_group.append(&ui, "Luceo", luceo_tg);

    // ************** Whistle ****************
    let mut whistle_tg = TabGroup::new(&ui);
    let mut whistle_events = whistle::setup(&ui, &mut whistle_tg, state.clone());
    tab_group.append(&ui, "Whistle", whistle_tg);

    // ************** Identities ****************
    let mut identity_tg = TabGroup::new(&ui);
    identity::setup(&ui, &mut identity_tg);
    tab_group.append(&ui, "Identities", identity_tg);

    // ************** Settings ****************
    let mut settings_tg = TabGroup::new(&ui);
    let mut settings_events = settings::setup(&ui, &mut settings_tg, state.clone());
    tab_group.append(&ui, "Settings", settings_tg);

    let mut window = Window::new(&ui, "Fractalide", 300, 150, WindowType::NoMenubar);
    window.set_child(&ui, tab_group);
    window.show(&ui);
    let functions = move || {
        copernica_events();
        luceo_events();
        whistle_events();
        settings_events();
    };
    let mut event_loop = ui.event_loop();
    event_loop.on_tick(&ui, functions);
    event_loop.run(&ui);
}
