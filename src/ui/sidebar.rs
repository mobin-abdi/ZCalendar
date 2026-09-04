use gtk4::prelude::*;
use jalali_calendar::JalaliDate;
use std::rc::Rc;

use crate::ui::date_picker;

pub fn build_sidebar<F>(stack: &gtk4::Stack, on_date_selected: F) -> gtk4::Revealer where F: Fn(JalaliDate) + 'static, {
    let revealer = gtk4::Revealer::new();

    revealer.set_transition_type(gtk4::RevealerTransitionType::SlideLeft);

    revealer.set_transition_duration(250);

    let sidebar = gtk4::Box::new(gtk4::Orientation::Vertical, 10);

    sidebar.set_margin_top(20);
    sidebar.set_margin_bottom(20);
    sidebar.set_margin_start(20);
    sidebar.set_margin_end(20);

    let converter_button = gtk4::Button::with_label("تبدیل تاریخ");

    let go_to_date_button = gtk4::Button::with_label("رفتن به تاریخ");

    let stack_converter = stack.clone();

    converter_button.connect_clicked(move |_| {
        stack_converter.set_visible_child_name("converter");
    });

    let on_date_selected = Rc::new(on_date_selected);

    let callback_for_button = Rc::clone(&on_date_selected);

    go_to_date_button.connect_clicked(move |_| {
        let callback_for_picker = Rc::clone(&callback_for_button);

        date_picker::show_date_picker(callback_for_picker);
    });

    sidebar.append(&converter_button);
    sidebar.append(&go_to_date_button);

    sidebar.add_css_class("sidebar");

    revealer.set_child(Some(&sidebar));

    revealer
}
