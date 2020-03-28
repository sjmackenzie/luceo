use iui::prelude::*;
use iui::controls::{PasswordEntry, VerticalBox, Group, Button};

pub fn their_ids(ui: &UI, g: &mut Group) {
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
}

