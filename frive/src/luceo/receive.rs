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
                        .child(TextView::new("Wallet Receive Address"))
                        .child(DummyView.fixed_width(5))
                        .child(TextView::new("ceo2h43nthuti4n5h6t4nih49348yy938ih39i33i34"))
                )
                .child(DummyView.fixed_height(1))
        )
        .h_align(HAlign::Center)
}
