use iui::prelude::*;
use iui::controls::{TabGroup};
use crate::state::{State};

mod stakepools;
mod wallet;

pub fn setup(ui: &UI, luceo_tg: &mut TabGroup, state: State) -> Box<FnMut()>  {
    let mut wallet_tg = TabGroup::new(&ui);
    let mut stakepools_tg = TabGroup::new(&ui);
    let mut wallet_events = wallet::setup(&ui, &mut wallet_tg, state.clone());
    let mut stakepool_events = stakepools::setup(&ui, &mut stakepools_tg, state.clone());
    luceo_tg.append(&ui, "Wallet", wallet_tg);
    luceo_tg.append(&ui, "Stakepools", stakepools_tg);
    Box::new(move || { wallet_events(); stakepool_events(); })
}
