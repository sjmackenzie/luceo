use iui::prelude::*;
use iui::controls::{TabGroup, Group};
pub mod my;
pub mod theirs;

pub fn setup(ui: &UI, identity_tg: &mut TabGroup) {
    let mut my_id_g = Group::new(&ui, "");
    let mut their_id_g = Group::new(&ui, "");
    my::my_ids(&ui, &mut my_id_g);
    theirs::their_ids(&ui, &mut their_id_g);
    identity_tg.append(&ui, "My Identities", my_id_g);
    identity_tg.append(&ui, "Their Identities", their_id_g);
}
