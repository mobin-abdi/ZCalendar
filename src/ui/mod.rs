mod sidebar;
mod converter;
mod calendar;

use gtk4::prelude::*;
use jalali_calendar::JalaliDate;
use std::cell::RefCell;
use std::rc::Rc;

fn build_calendar(date: JalaliDate, today: JalaliDate) -> gtk4::Grid {
    let grid = gtk4::Grid::new();

    grid.set_row_spacing(5);
    grid.set_column_spacing(5);

    grid.set_column_homogeneous(true);
    grid.set_row_homogeneous(true);

    let first_day = date.first_day_of_month();
    let first_weekday = first_day.weekday() as i32;

    let days_in_month = date.days_in_this_month();

    for day in 1..=days_in_month {
        let label = gtk4::Label::new(Some(&day.to_string()));

        label.set_hexpand(true);
        label.set_vexpand(true);

        label.set_halign(gtk4::Align::Center);
        label.set_valign(gtk4::Align::Center);

        if date.year() == today.year() && date.month() == today.month() && day == today.day(){
            label.add_css_class("today");
        }

        let position = first_weekday + (day - 1) as i32;

        let column = position % 7;
        let row = position / 7;

        grid.attach(
            &label,
            column,
            row,
            1,
            1,
        );
    }

    grid
}

fn build_ui(application: &gtk4::Application) {
    let window =
        gtk4::ApplicationWindow::builder()
            .application(application)
            .title("ZCalendar")
            .default_width(700)
            .default_height(500)
            .build();

    let today = JalaliDate::today();

    let displayed_date = Rc::new(RefCell::new(today.first_day_of_month()));

    let main_box = gtk4::Box::new(gtk4::Orientation::Vertical, 10);

    let stack = gtk4::Stack::new();

    main_box.set_margin_top(20);
    main_box.set_margin_bottom(20);
    main_box.set_margin_start(20);
    main_box.set_margin_end(20);

    let sidebar = sidebar::build_sidebar(&stack);

    let sidebar_toggle = sidebar.clone();

    let content_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);

    content_box.append(&main_box);
    content_box.append(&sidebar);

    let menu_button = gtk4::Button::with_label("☰");

    let navigation = gtk4::Box::new(gtk4::Orientation::Horizontal,10,);

    navigation.append(&menu_button);

    navigation.set_halign(
        gtk4::Align::Center
    );
    
    menu_button.connect_clicked(move |_| {
        sidebar_toggle.set_reveal_child(
            !sidebar_toggle.reveals_child()
        );
    });

    let previous_button = gtk4::Button::with_label("←");

    let next_button = gtk4::Button::with_label("→");


    let title = gtk4::Label::new(Some(&format!("{} {}", today.month_name(), today.year())));

    title.add_css_class("title-2");

    navigation.append(&previous_button);
    navigation.append(&title);
    navigation.append(&next_button);

    main_box.append(&navigation);

    let weekdays = gtk4::Grid::new();

    weekdays.set_column_homogeneous(true);

    let weekday_names = ["شنبه", "یکشنبه", "دوشنبه", "سه‌شنبه", "چهارشنبه", "پنجشنبه", "جمعه",
    ];

    for (column, name) in weekday_names.iter().enumerate() {

        let label = gtk4::Label::new(Some(name));

        label.set_hexpand(true);

        label.set_margin_bottom(8);

        weekdays.attach(&label, column as i32, 0, 1, 1);
    }

    main_box.append(&weekdays);

    let calendar_container = gtk4::Box::new(gtk4::Orientation::Vertical, 0);

    calendar_container.set_vexpand(true);
    calendar_container.set_hexpand(true);


    let calendar = build_calendar(*displayed_date.borrow(), today);

    calendar_container.append(&calendar);

    main_box.append(&calendar_container);

    let displayed_date_prev = Rc::clone(&displayed_date);

    let title_prev = title.clone();

    let calendar_container_prev = calendar_container.clone();


    previous_button.connect_clicked(
        move |_| {
            let current = *displayed_date_prev.borrow();

            let previous = current.add_months(-1);

            *displayed_date_prev.borrow_mut() = previous;

            title_prev.set_text(&format!("{} {}", previous.month_name(), previous.year()));

            while let Some(child) = calendar_container_prev.first_child() {
                calendar_container_prev.remove(
                    &child
                );
            }

            let new_calendar = build_calendar(previous, today);

            calendar_container_prev.append(&new_calendar);
        }
    );

    let displayed_date_next = Rc::clone(&displayed_date);
    let title_next = title.clone();
    let calendar_container_next = calendar_container.clone();

    next_button.connect_clicked(
        move |_| {
            let current = *displayed_date_next.borrow();

            let next = current.add_months(1);

            *displayed_date_next.borrow_mut() = next;

            title_next.set_text(&format!("{} {}", next.month_name(), next.year()));

            while let Some(child) = calendar_container_next.first_child(){
                calendar_container_next.remove(&child);
            }

            let new_calendar =build_calendar(next, today);

            calendar_container_next.append(&new_calendar);
        }
    );

    let css = gtk4::CssProvider::new();

    css.load_from_data(
        r#"
        * {
            font-family: "Vazir";
            font-size: 14px;
        }

        .sidebar {
            background: #3584e4;
            padding: 8px;
        }

        .sidebar button {
            color: black;
            background-color: #fff;
            margin: 4px;
            min-width: 100px;
        }

        .today {
            background: #3584e4;
            color: white;
            min-width: 48px;
            min-height: 48px;
            border-radius: 999px;
            padding: 0;
        }
        .title-2 {
            font-family: "Vazir";
        }
        "#,
    );

    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().unwrap(),
        &css,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    stack.add_named(&content_box, Some("calendar"));

    let converter_page = converter::build_converter_page(&stack);

    stack.add_named(&converter_page, Some("converter"));

    window.set_child(Some(&stack));

    window.show();
}

pub fn show_window() {
    let application = gtk4::Application::new(Some("ir.zcalendar.ZCalendar"), Default::default(),);
    application.connect_activate(build_ui);
    application.run();
}