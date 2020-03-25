use iui::prelude::*;
use iui::controls::{Label, Spinbox, Slider, Entry, PasswordEntry, MultilineEntry, VerticalBox, HorizontalBox, HorizontalSeparator, Group, TabGroup, Spacer, ProgressBar};
use std::rc::Rc;
use std::cell::RefCell;

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
    let copernica_notary_hb = HorizontalBox::new(&ui);
    let copernica_manage_hb = HorizontalBox::new(&ui);
    copernica_tg.append(&ui, "Query", copernica_query_hb);
    copernica_tg.append(&ui, "Store", copernica_store_hb);
    copernica_tg.append(&ui, "Notary", copernica_notary_hb);
    copernica_tg.append(&ui, "Manage", copernica_manage_hb);
    copernica_vb.append(&ui, copernica_tg, LayoutStrategy::Stretchy);

    let mut tab_group = TabGroup::new(&ui);
    tab_group.append(&ui, "Copernica Network", copernica_vb);

    let luceo_hb = HorizontalBox::new(&ui);
    tab_group.append(&ui, "Luceo Wallet", luceo_hb);
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
    event_loop.run(&ui);
}
