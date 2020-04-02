use iui::prelude::*;
use iui::controls::{TabGroup, PasswordEntry, Button, HorizontalBox, VerticalBox, Group, Entry, Label};
use crate::state::{State};

pub fn my_ids(ui: &UI, g: &mut Group, state: State) -> Box<FnMut()> {
    let mut id_vb = VerticalBox::new(&ui);
    id_vb.set_padded(&ui, true);
    let mut current_id_lbl = Label::new(&ui, "Current ID:");
    let my_id_pw_entry_one = PasswordEntry::new(&ui);
    let mut my_id_pw_entry_two = PasswordEntry::new(&ui);
    let mut button = Button::new(&ui, "Create");
    my_id_pw_entry_two.on_changed(&ui, {
        let state = state.clone();
        move |val| { state.borrow_mut().identities.current_identity = val; }
    });
    button.on_clicked(&ui, {
        let ui = ui.clone();
        let state = state.clone();
        let mut lbl = current_id_lbl.clone();
        move |btn| {
            btn.set_text(&ui, "Clicked!");
            lbl.set_text(&ui, &format!("Current ID: {}", state.borrow_mut().identities.current_identity));
            state.borrow_mut().whistle.test_entry = "clicked".into();
        }
    });
    id_vb.append(&ui, current_id_lbl, LayoutStrategy::Compact);
    id_vb.append(&ui, my_id_pw_entry_one, LayoutStrategy::Compact);
    id_vb.append(&ui, my_id_pw_entry_two, LayoutStrategy::Compact);
    id_vb.append(&ui, button, LayoutStrategy::Compact);
    g.set_child(&ui, id_vb);
    let ui = ui.clone();
    let function =  move || {
        let state = state.borrow();
    };
    Box::new(function)
}

pub fn their_ids(ui: &UI, g: &mut Group, state: State) -> Box<FnMut()> {
    let mut id_vb = VerticalBox::new(&ui);
    id_vb.set_padded(&ui, true);
    let my_id_pw_entry_one = PasswordEntry::new(&ui);
    let my_id_pw_entry_two = PasswordEntry::new(&ui);
    let mut button = Button::new(&ui, "Create");
    button.on_clicked(&ui, {
        let ui = ui.clone();
        move |btn| {
            btn.set_text(&ui, "Clicked!");
        }
    });
    id_vb.append(&ui, my_id_pw_entry_one, LayoutStrategy::Compact);
    id_vb.append(&ui, my_id_pw_entry_two, LayoutStrategy::Compact);
    id_vb.append(&ui, button, LayoutStrategy::Compact);
    g.set_child(&ui, id_vb);
    let ui = ui.clone();
    let function =  move || {
        let state = state.borrow();
    };
    Box::new(function)
}

pub fn setup(ui: &UI, identity_tg: &mut TabGroup, state: State) -> Box<FnMut()> {
    let mut my_id_g = Group::new(&ui, "");
    let mut their_id_g = Group::new(&ui, "");
    let mut my_ids_events = my_ids(&ui, &mut my_id_g, state.clone());
    let mut their_ids_events = their_ids(&ui, &mut their_id_g, state.clone());
    identity_tg.append(&ui, "My Identities", my_id_g);
    identity_tg.append(&ui, "Their Identities", their_id_g);
    Box::new(move || { my_ids_events(); their_ids_events();})
}
