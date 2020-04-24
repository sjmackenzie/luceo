use {
    cursive::{
        align::HAlign,
        traits::*,
        views::{Dialog, Panel, Button, TextView, EditView, DummyView, LinearLayout, RadioGroup},
    },
};

struct Transaction {
    value: String,
    tx_id: String,
    tx_time: String,
}

fn transactions() -> LinearLayout {
    let mut txs: Vec<LinearLayout> = vec![];
    let tx0 = Transaction {
        value:   "12".into(),
        tx_id:   "123123".into(),
        tx_time: "2020212".into(),
    };
    let tx1 = Transaction {
        value:   "24".into(),
        tx_id:   "1423432".into(),
        tx_time: "2020315".into(),
    };
    let mut tx_view = LinearLayout::horizontal();
    tx_view.add_child(TextView::new("CEO Sent"));
    tx_view.add_child(DummyView.fixed_width(5));
    tx_view.add_child(TextView::new(tx0.value));
    txs.push(tx_view);
    let mut tx_view = LinearLayout::horizontal();
    tx_view.add_child(TextView::new("CEO Sent"));
    tx_view.add_child(DummyView.fixed_width(5));
    tx_view.add_child(TextView::new(tx1.value));
    txs.push(tx_view);
    let mut txs_view = LinearLayout::vertical();
    for tx in txs {
        txs_view.add_child(tx);
    }
    txs_view
}

pub fn dialog() -> Dialog {
    Dialog::new()
        .content(
            LinearLayout::vertical()
                .child(DummyView.fixed_height(1))
                .child(
                    LinearLayout::horizontal()
                        .child(TextView::new("Total"))
                        .child(DummyView.fixed_width(5))
                        .child(TextView::new("34000"))
                )
                .child(DummyView.fixed_height(1))
                .child(transactions())
        )
        .button("Delete Wallet", |s| s.quit())
        .button("Delete Wallet", |s| s.quit())
        .h_align(HAlign::Center)
}
