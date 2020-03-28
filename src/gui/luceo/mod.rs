use iui::prelude::*;
use iui::controls::{TabGroup};
mod stakepools;
mod wallet;

pub fn luceo(ui: &UI, luceo_tg: &mut TabGroup) {
    let mut wallet_tg = TabGroup::new(&ui);
    let mut stakepools_tg = TabGroup::new(&ui);
    wallet::wallet(&ui, &mut wallet_tg);
    stakepools::stakepools(&ui, &mut stakepools_tg);
    luceo_tg.append(&ui, "Wallet", wallet_tg);
    luceo_tg.append(&ui, "Stakepools", stakepools_tg);
}
