use iui::prelude::*;
use iui::controls::{HorizontalBox, TabGroup};

pub fn whistle(ui: &UI, tg: &mut TabGroup) {
    let notary_summary_hb = HorizontalBox::new(&ui);
    let notary_send_hb = HorizontalBox::new(&ui);
    tg.append(&ui, "Search", notary_summary_hb);
    tg.append(&ui, "Create", notary_send_hb);
}

