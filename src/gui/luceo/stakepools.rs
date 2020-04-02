use iui::prelude::*;
use iui::controls::{HorizontalBox, TabGroup};
use iui::controls::{VerticalBox, Group, Entry, Label};
use crate::state::{State};

pub fn setup(ui: &UI, tg: &mut TabGroup, state: State) -> Box<FnMut()>  {
    let mut summary_vb = VerticalBox::new(&ui);
    let send_hb = HorizontalBox::new(&ui);
    let receive_hb = HorizontalBox::new(&ui);
    let faucet_hb = HorizontalBox::new(&ui);
    let transactions_hb = HorizontalBox::new(&ui);
    let mut test_entry = Entry::new(&ui);
    let test_label = Label::new(&ui, "output:");
    summary_vb.append(&ui, test_label.clone(), LayoutStrategy::Compact);
    summary_vb.append(&ui, test_entry.clone(), LayoutStrategy::Compact);
    tg.append(&ui, "Summary", summary_vb);
    tg.append(&ui, "Send", send_hb);
    tg.append(&ui, "Receive", receive_hb);
    tg.append(&ui, "Transactions", transactions_hb);
    tg.append(&ui, "Faucet", faucet_hb);
    test_entry.on_changed(&ui, {
        let state = state.clone();
        move |val| { state.borrow_mut().luceo.test_entry = val; }
    });
    let ui = ui.clone();
    let mut test_label = test_label.clone();
    let function =  move || {
        let state = state.borrow();
        test_label.set_text(&ui, &format!("Text: {}", state.luceo.test_entry));
    };
    Box::new(function)
}

