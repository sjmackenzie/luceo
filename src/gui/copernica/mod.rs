use iui::prelude::*;
use iui::controls::{TabGroup, HorizontalBox, VerticalBox, Group, Entry, Label};

pub fn file_sharing(ui: &UI, hb: &mut HorizontalBox) {
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
/*
    copernica_query.on_changed(&ui, {
        let copernica_state = copernica_state.clone();
        move |val| { copernica_state.borrow_mut().query_val = val; }
    });
*/
}

pub fn copernica(ui: &UI, tg: &mut TabGroup) {
    let mut file_sharing_hb = HorizontalBox::new(&ui);
    file_sharing(&ui, &mut file_sharing_hb);
    tg.append(&ui, "File Sharing", file_sharing_hb);
    let copernica_store_hb = HorizontalBox::new(&ui);
    let copernica_chat_hb = HorizontalBox::new(&ui);
    tg.append(&ui, "Streaming", copernica_store_hb);
    tg.append(&ui, "Chat", copernica_chat_hb);
}
