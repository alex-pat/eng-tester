extern crate cursive;

use self::cursive::event::Key;
use self::cursive::traits::*;
use self::cursive::views::{Dialog, DummyView, EditView, LinearLayout, TextView};
use self::cursive::Cursive;
use std::cell::RefCell;

pub fn run(context: ::org_tester::Context) {
    let gform = context.get_guess_form();
    let cform = context.get_check_form();
    let gword = context.get_guess();

    let context = RefCell::new(context);

    let submit = move |s: &mut Cursive, input: &str| {
        let mut context = context.borrow_mut();
        if context.check(input) {
            s.add_layer(Dialog::text("Correct!").button("Next", |sx| drop(sx.pop_layer().unwrap())));
        } else {
            s.add_layer(
                Dialog::text(format!("Wrong!\n{:?}", context.last_error()))
                    .button("Next", |sx| drop(sx.pop_layer().unwrap())),
            );
        };
        s.call_on_id("guess_form", |view: &mut TextView| {
            view.set_content(context.get_guess_form())
        });
        s.call_on_id("check_form", |view: &mut TextView| {
            view.set_content(context.get_check_form())
        });
        s.call_on_id("guess_word", |view: &mut TextView| {
            view.set_content(context.get_guess())
        });
        s.call_on_id("check_word", |view: &mut EditView| view.set_content(""));
    };

    let mut siv = Cursive::default();
    siv.add_layer(
        Dialog::around(
            LinearLayout::horizontal()
                .child(
                    LinearLayout::vertical()
                        .child(TextView::new(gform).with_id("guess_form"))
                        .child(TextView::new(cform).with_id("check_form")),
                )
                .child(DummyView)
                .child(
                    LinearLayout::vertical()
                        .child(TextView::new(gword).with_id("guess_word"))
                        .child(EditView::new().on_submit(submit).with_id("check_word")),
                ),
        ).title("org_tester"),
    );

    siv.add_global_callback(Key::Esc, |s| s.quit());
    siv.add_global_callback(Key::F1, |s| {
        s.add_layer(
            Dialog::text("org_tester v.0.1.0\nAlexander Pateenok, 2017")
                .button("Ok", |s| drop(s.pop_layer().unwrap())),
        )
    });

    siv.run();
}
