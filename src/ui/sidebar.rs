use gtk4::prelude::*;

pub fn build_sidebar(stack: &gtk4::Stack) -> gtk4::Revealer {
    let revealer = gtk4::Revealer::new();

    revealer.set_transition_type(gtk4::RevealerTransitionType::SlideLeft);
    revealer.set_transition_duration(250);

    let sidebar = gtk4::Box::new(gtk4::Orientation::Vertical, 10);

    sidebar.set_margin_top(20);
    sidebar.set_margin_bottom(20);
    sidebar.set_margin_start(20);
    sidebar.set_margin_end(20);

    let button = gtk4::Button::with_label("تبدیل تاریخ");

    let stack_converter = stack.clone();

    button.connect_clicked(move |_| {
        stack_converter.set_visible_child_name("converter");
    });

    sidebar.append(&button);

    sidebar.add_css_class("sidebar");

    revealer.set_child(Some(&sidebar));

    revealer
}
