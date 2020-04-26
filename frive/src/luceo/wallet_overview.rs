use {
    cursive_tabs::TabPanel,
    cursive::{
        align::HAlign,
        Cursive,
        traits::*,
        views::{Dialog, Panel, Button, TextView, EditView, DummyView, LinearLayout, RadioGroup,
             ListView, SelectView, Checkbox},
    },
    crate::luceo,
};

pub fn dialog() -> Dialog {
    let select = SelectView::<String>::new()
        .on_submit(challenge)
        .with_name("select");
    let buttons = LinearLayout::vertical()
        .child(Button::new("Add new", add_name))
        .child(Button::new("Delete", delete_name))
        .child(DummyView)
        .child(Button::new("Quit", Cursive::quit));

    Dialog::around(LinearLayout::horizontal()
            .child(select)
            .child(DummyView)
            .child(buttons))
        .title("Select a wallet")
 }

fn add_name(s: &mut Cursive) {
    fn ok(s: &mut Cursive, name: &str) {
        s.call_on_name("select", |view: &mut SelectView<String>| {
            view.add_item_str(name)
        });
        s.pop_layer();
    }

    s.add_layer(Dialog::around(EditView::new()
            .on_submit(ok)
            .with_name("name")
            .fixed_width(10))
        .title("Enter a new name")
        .button("Ok", |s| {
            let name =
                s.call_on_name("name", |view: &mut EditView| {
                    view.get_content()
                }).unwrap();
            ok(s, &name);
        })
        .button("Cancel", |s| {
            s.pop_layer();
        }));
}

fn delete_name(s: &mut Cursive) {
    let mut select = s.find_name::<SelectView<String>>("select").unwrap();
    match select.selected_id() {
        None => s.add_layer(Dialog::info("No name to remove")),
        Some(focus) => {
            select.remove_item(focus);
        }
    }
}

fn challenge(s: &mut Cursive, name: &str) {
    s.add_layer(
        Dialog::new()
            .content(Dialog::around(EditView::new()
                .on_submit(wallet)
                .with_name("password")
                .fixed_width(10)))
            .title(format!("Insert password"))
            .button("Cancel", |s|{s.pop_layer();}));
}

fn wallet(s: &mut Cursive, password: &str) {
    let luceo_tp = TabPanel::new()
        .with_tab("Summary", luceo::summary::dialog())
        .with_tab("Send", luceo::send::dialog())
        .with_tab("Receive", luceo::receive::dialog())
        .with_tab("Transactions", luceo::transactions::dialog())
        .with_tab("Settings", luceo::settings::dialog());
    if password == "123" {
        s.add_layer(Dialog::new()
        .content(luceo_tp)
        .button("Close Wallet", |s|{s.pop_layer();}));
    } else {
        s.pop_layer();
    }
}
