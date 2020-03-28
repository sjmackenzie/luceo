use iui::prelude::*;
use iui::controls::{HorizontalBox, TabGroup};

pub fn stakepools(ui: &UI, tg: &mut TabGroup) {
    let summary_hb = HorizontalBox::new(&ui);
    let send_hb = HorizontalBox::new(&ui);
    let receive_hb = HorizontalBox::new(&ui);
    let faucet_hb = HorizontalBox::new(&ui);
    let transactions_hb = HorizontalBox::new(&ui);
    tg.append(&ui, "Summary", summary_hb);
    tg.append(&ui, "Send", send_hb);
    tg.append(&ui, "Receive", receive_hb);
    tg.append(&ui, "Transactions", transactions_hb);
    tg.append(&ui, "Faucet", faucet_hb);
}

