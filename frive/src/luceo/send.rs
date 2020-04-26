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
                        .child(TextView::new("Receiver Address"))
                        .child(DummyView.fixed_width(1))
                        .child(
                            EditView::new()
                                .with_name("luceo_receiver_address")
                                .fixed_width(50)
                        )
                )
                .child(DummyView.fixed_height(1))
                .child(
                    LinearLayout::horizontal()
                        .child(TextView::new("Amount"))
                        .child(DummyView.fixed_width(11))
                        .child(
                            EditView::new()
                                .with_name("luceo_sending_amount")
                                .fixed_width(50)
                        )
                )
                .child(DummyView.fixed_height(1))
                .child(
                    LinearLayout::horizontal()
                        .child(TextView::new("Password"))
                        .child(DummyView.fixed_width(9))
                        .child(
                            EditView::new()
                                .secret()
                                .with_name("luceo_sending_password")
                                .fixed_width(50)
                        )
                )
                .child(DummyView.fixed_height(1))
        )
        .button("Send", |s| s.quit())
        .h_align(HAlign::Center)
}
