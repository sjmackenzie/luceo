use {
    cursive::{
        align::HAlign,
        traits::*,
        views::{Dialog, Panel, Button, TextView, EditView, DummyView, LinearLayout, RadioGroup},
    },
};

pub fn dialog() -> Dialog {
        Dialog::new()
        .content(
            LinearLayout::vertical()
                .child(DummyView.fixed_height(1))
                .child(
                    LinearLayout::horizontal()
                        .child(TextView::new("Wallet Name"))
                        .child(DummyView.fixed_width(5))
                        .child(
                            EditView::new()
                                .with_name("wallet_name")
                                .fixed_width(50)
                        )
                )
                .child(DummyView.fixed_height(1))
                .child(
                    LinearLayout::horizontal()
                        .child(TextView::new("Wallet Password"))
                        .child(DummyView.fixed_width(1))
                        .child(
                            EditView::new()
                                .with_name("wallet_password")
                                .fixed_width(50)
                        )
                )
                .child(DummyView.fixed_height(1))
        )
        .h_align(HAlign::Center)
}
