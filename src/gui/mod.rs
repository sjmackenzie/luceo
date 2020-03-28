use iui::prelude::*;
use iui::controls::{Label, Entry, PasswordEntry, VerticalBox, HorizontalBox, HorizontalSeparator, Group, TabGroup, Spacer, Button};
mod identity;
mod luceo;
mod copernica;
mod whistle;
use {
    identity::{ identity },
    luceo::{ luceo },
    copernica::{ copernica },
    whistle::{ whistle },
};

pub fn setup() {
    let ui = UI::init().unwrap();
    // ************** Copernica ****************
    let mut copernica_tg = TabGroup::new(&ui);
    copernica(&ui, &mut copernica_tg);
    let mut tab_group = TabGroup::new(&ui);
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
    let settings_vb = VerticalBox::new(&ui);
    tab_group.append(&ui, "Settings", settings_vb);

    let mut window = Window::new(&ui, "Fractalide", 300, 150, WindowType::NoMenubar);
    window.set_child(&ui, tab_group);
    window.show(&ui);
    /*
    copernica_query.on_changed(&ui, {
        let copernica_state = copernica_state.clone();
        move |val| { copernica_state.borrow_mut().query_val = val; }
    });
    */
    let mut event_loop = ui.event_loop();
    /*
    event_loop.on_tick(&ui, {
        let ui = ui.clone();
        let mut text_label = text_label.clone();
        move || {
            let copernica_state = copernica_state.borrow();
            text_label.set_text(&ui, &format!("Text: {}", copernica_state.query_val));
        }
    });
*/
    event_loop.run(&ui);
}
