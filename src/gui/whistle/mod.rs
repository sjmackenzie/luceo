use iui::prelude::*;
use iui::controls::{TabGroup, HorizontalBox, VerticalBox, Group, Entry, Label};
use crate::state::{State};

pub fn create(ui: &UI, hb: &mut HorizontalBox, state: State) -> Box<FnMut()>  {
    let (input_group, mut copernica_query) = {
        let mut input_group = Group::new(&ui, "");
        let mut input_vbox = VerticalBox::new(&ui);
        input_vbox.set_padded(&ui, true);
        let copernica_query = Entry::new(&ui);
        let text_label = Label::new(&ui, "Query:");
        input_vbox.append(&ui, text_label.clone(), LayoutStrategy::Compact);
        input_vbox.append(&ui, copernica_query.clone(), LayoutStrategy::Compact);
        input_group.set_child(&ui, input_vbox);
        (input_group, copernica_query)
    };
    let (output_group, text_label) = {
        let mut output_group = Group::new(&ui, "");
        let mut output_vbox = VerticalBox::new(&ui);
        output_vbox.set_padded(&ui, true);
        let text_label = Label::new(&ui, "Output");
        output_vbox.append(&ui, text_label.clone(), LayoutStrategy::Compact);
        output_group.set_child(&ui, output_vbox);
        (output_group, text_label)
    };
    hb.append(&ui, input_group, LayoutStrategy::Stretchy);
    hb.append(&ui, output_group, LayoutStrategy::Stretchy);
    copernica_query.on_changed(&ui, {
        let state = state.clone();
        move |val| { state.borrow_mut().whistle.test_entry = val; }
    });
    let ui = ui.clone();
    let mut text_label = text_label.clone();
    let function =  move || {
        let state = state.borrow();
        text_label.set_text(&ui, &format!("Text: {}", state.whistle.test_entry));
    };
    Box::new(function)
}

pub fn setup(ui: &UI, tg: &mut TabGroup, state: State) -> Box<FnMut()> {
    let mut notary_search_hb = HorizontalBox::new(&ui);
    let notary_create_hb = HorizontalBox::new(&ui);
    let mut search_events = create(&ui, &mut notary_search_hb, state.clone());
    tg.append(&ui, "Search", notary_search_hb);
    tg.append(&ui, "Create", notary_create_hb);
    Box::new(move || { search_events(); })
}

