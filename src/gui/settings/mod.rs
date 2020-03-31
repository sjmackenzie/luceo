use iui::prelude::*;
use iui::controls::{TabGroup, HorizontalBox, VerticalBox, Group, Entry, Label};
use crate::state::{State};

pub fn file_sharing(ui: &UI, hb: &mut HorizontalBox, state: State) -> Box<FnMut()> {
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
        move |val| { state.borrow_mut().settings.val = val; }
    });
    let ui = ui.clone();
    let mut text_label = text_label.clone();
    let function =  move || {
        let state = state.borrow();
        text_label.set_text(&ui, &format!("Text: {}", state.settings.val));
    };
    Box::new(function)
}

pub fn settings(ui: &UI, tg: &mut TabGroup, state: State) -> Box<FnMut()> {
    let mut copernica_hb = HorizontalBox::new(&ui);
    let fns = file_sharing(&ui, &mut copernica_hb, state.clone());
    tg.append(&ui, "Copernica", copernica_hb);
    let luceo_hb = HorizontalBox::new(&ui);
    let whistle_hb = HorizontalBox::new(&ui);
    let identities_hb = HorizontalBox::new(&ui);
    tg.append(&ui, "Luceo", luceo_hb);
    tg.append(&ui, "Whistle", whistle_hb);
    tg.append(&ui, "Identities", identities_hb);
    fns
}
