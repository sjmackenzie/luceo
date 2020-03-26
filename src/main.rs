use iui::prelude::*;
use iui::controls::{Label, Spinbox, Slider, Entry, PasswordEntry, MultilineEntry, VerticalBox, HorizontalBox, HorizontalSeparator, Group, TabGroup, Spacer, ProgressBar};
use std::rc::Rc;
use std::cell::RefCell;
use {
    libcopernica::{Router, read_config_file},
};
use std::thread;
struct CopernicaState {
    query_val: String,
}

fn main() {
    let ui = UI::init().unwrap();
    let copernica_state = Rc::new(RefCell::new(CopernicaState { query_val: "".into()}));
    let mut copernica_vb = VerticalBox::new(&ui);
    let mut copernica_tg = TabGroup::new(&ui);
    let mut copernica_query_hb = HorizontalBox::new(&ui);
    let (input_group, mut entry) = {
        let mut input_group = Group::new(&ui, "");
        let mut input_vbox = VerticalBox::new(&ui);
        input_vbox.set_padded(&ui, true);
        let entry = Entry::new(&ui);
        let text_label = Label::new(&ui, "Query:");
        input_vbox.append(&ui, text_label.clone(), LayoutStrategy::Compact);
        input_vbox.append(&ui, entry.clone(), LayoutStrategy::Compact);
        input_group.set_child(&ui, input_vbox);
        (input_group, entry)
    };
    let (output_group, text_label) = {
        let mut output_group = Group::new(&ui, "");
        let mut output_vbox = VerticalBox::new(&ui);
        output_vbox.set_padded(&ui, true);
        let text_label = Label::new(&ui, "");
        output_vbox.append(&ui, text_label.clone(), LayoutStrategy::Compact);
        output_group.set_child(&ui, output_vbox);
        (output_group, text_label)
    };
    copernica_query_hb.append(&ui, input_group, LayoutStrategy::Stretchy);
    copernica_query_hb.append(&ui, output_group, LayoutStrategy::Stretchy);

    let copernica_store_hb = HorizontalBox::new(&ui);
    let copernica_chat_hb = HorizontalBox::new(&ui);
    let copernica_settings_hb = HorizontalBox::new(&ui);
    copernica_tg.append(&ui, "File Sharing", copernica_store_hb);
    copernica_tg.append(&ui, "Streaming", copernica_query_hb);
    copernica_tg.append(&ui, "Chat", copernica_chat_hb);
    copernica_tg.append(&ui, "Settings", copernica_settings_hb);
    copernica_vb.append(&ui, copernica_tg, LayoutStrategy::Stretchy);

    let mut tab_group = TabGroup::new(&ui);
    tab_group.append(&ui, "Copernica", copernica_vb);

    let mut luceo_wallet_vb = VerticalBox::new(&ui);
    let mut luceo_wallet_tg = TabGroup::new(&ui);
    let luceo_summary_hb = HorizontalBox::new(&ui);
    let luceo_send_hb = HorizontalBox::new(&ui);
    let luceo_receive_hb = HorizontalBox::new(&ui);
    let luceo_faucet_hb = HorizontalBox::new(&ui);
    let luceo_transactions_hb = HorizontalBox::new(&ui);
    let luceo_settings_hb = HorizontalBox::new(&ui);
    luceo_wallet_tg.append(&ui, "Summary", luceo_summary_hb);
    luceo_wallet_tg.append(&ui, "Send", luceo_send_hb);
    luceo_wallet_tg.append(&ui, "Receive", luceo_receive_hb);
    luceo_wallet_tg.append(&ui, "Transactions", luceo_transactions_hb);
    luceo_wallet_tg.append(&ui, "Faucet", luceo_faucet_hb);
    luceo_wallet_tg.append(&ui, "Settings", luceo_settings_hb);
    luceo_wallet_vb.append(&ui, luceo_wallet_tg, LayoutStrategy::Stretchy);

    let luceo_stakepools_vb = VerticalBox::new(&ui);

    let mut luceo_tg = TabGroup::new(&ui);
    luceo_tg.append(&ui, "Wallet", luceo_wallet_vb);
    luceo_tg.append(&ui, "Stakepools", luceo_stakepools_vb);
    tab_group.append(&ui, "Luceo", luceo_tg);

    let notary_hb = HorizontalBox::new(&ui);
    let mut notary_vb = VerticalBox::new(&ui);
    let mut notary_tg = TabGroup::new(&ui);
    let notary_summary_hb = HorizontalBox::new(&ui);
    let notary_send_hb = HorizontalBox::new(&ui);
    let notary_settings_hb = HorizontalBox::new(&ui);
    notary_tg.append(&ui, "Search", notary_summary_hb);
    notary_tg.append(&ui, "Create", notary_send_hb);
    notary_tg.append(&ui, "Settings", notary_settings_hb);
    notary_vb.append(&ui, notary_tg, LayoutStrategy::Stretchy);
    tab_group.append(&ui, "Whistle", notary_vb);


    let identity_hb = HorizontalBox::new(&ui);
    let mut identity_vb = VerticalBox::new(&ui);
    let mut identity_tg = TabGroup::new(&ui);
    let identity_summary_hb = HorizontalBox::new(&ui);
    let identity_send_hb = HorizontalBox::new(&ui);
    let identity_settings_hb = HorizontalBox::new(&ui);
    identity_tg.append(&ui, "Search", identity_summary_hb);
    identity_tg.append(&ui, "Create", identity_send_hb);
    identity_tg.append(&ui, "Settings", identity_settings_hb);
    identity_vb.append(&ui, identity_tg, LayoutStrategy::Stretchy);
    tab_group.append(&ui, "Identity", identity_vb);

    let mut window = Window::new(&ui, "Fractalide", 300, 150, WindowType::NoMenubar);
    window.set_child(&ui, tab_group);
    window.show(&ui);
    entry.on_changed(&ui, {
        let copernica_state = copernica_state.clone();
        move |val| { copernica_state.borrow_mut().query_val = val; }
    });
    let mut event_loop = ui.event_loop();
    event_loop.on_tick(&ui, {
        let ui = ui.clone();
        let mut text_label = text_label.clone();
        move || {
            let copernica_state = copernica_state.borrow();
            text_label.set_text(&ui, &format!("Text: {}", copernica_state.query_val));
        }
    });

    let mut r = Router::new();
    thread::spawn(move || { r.run(); });
    event_loop.run(&ui);
}
