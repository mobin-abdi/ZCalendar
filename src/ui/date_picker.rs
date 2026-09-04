use gtk4::prelude::*;
use jalali_calendar::JalaliDate;
use std::rc::Rc;

pub fn show_date_picker(on_date_selected: Rc<dyn Fn(JalaliDate)>) {
    let window = gtk4::Window::builder()
        .title("رفتن به تاریخ")
        .default_width(400)
        .default_height(150)
        .build();

    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 15);

    main_box.set_margin_top(20);
    main_box.set_margin_bottom(20);
    main_box.set_margin_start(20);
    main_box.set_margin_end(20);

    let title = gtk4::Label::new(Some("رفتن به تاریخ"));

    title.add_css_class("title-2");

    let inputs = gtk4::Box::new(gtk4::Orientation::Horizontal, 10);

    inputs.set_halign(gtk4::Align::Center);

    let year_entry = gtk4::Entry::new();
    year_entry.set_placeholder_text(Some("سال"));

    let month_entry = gtk4::Entry::new();
    month_entry.set_placeholder_text(Some("ماه"));

    let day_entry = gtk4::Entry::new();
    day_entry.set_placeholder_text(Some("روز"));

    inputs.append(&year_entry);
    inputs.append(&month_entry);
    inputs.append(&day_entry);

    let go_button = gtk4::Button::with_label("برو");

    go_button.set_halign(gtk4::Align::Center);

    let year_entry_clone = year_entry.clone();
    let month_entry_clone = month_entry.clone();
    let day_entry_clone = day_entry.clone();

    let window_clone = window.clone();

    let callback = Rc::clone(&on_date_selected);

    go_button.connect_clicked(move |_| {
        let year = year_entry_clone.text().parse::<i32>();

        let month = month_entry_clone.text().parse::<u32>();

        let day = day_entry_clone.text().parse::<u32>();

        if let (Ok(year), Ok(month), Ok(day)) = (year, month, day) {
            if let Ok(date) = JalaliDate::new(year, month, day) {
                callback(date);

                window_clone.close();
            }
        }
    });

    main_box.append(&title);
    main_box.append(&inputs);
    main_box.append(&go_button);

    window.set_child(Some(&main_box));

    window.present();
}
