use gtk4::prelude::*;
use jalali_calendar::JalaliDate;

fn jalali_to_gregorian(year: i32, month:u32, day: u32) -> Result<(i32, u32, u32), jalali_calendar::Error> {
    let date = JalaliDate::new(year, month, day)?;
    Ok(date.to_gregorian())
}

pub fn build_converter_page(stack: &gtk4::Stack) -> gtk4::Box {
    let page = gtk4::Box::new(gtk4::Orientation::Vertical, 10);
    let header = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
    let year_entry = gtk4::Entry::new();
    let month_entry = gtk4::Entry::new();
    let day_entry = gtk4::Entry::new();

    year_entry.set_placeholder_text(Some("سال"));
    month_entry.set_placeholder_text(Some("ماه"));
    day_entry.set_placeholder_text(Some("روز"));
    year_entry.set_margin_top(10);
    month_entry.set_margin_top(10);
    day_entry.set_margin_top(10);
    year_entry.set_width_request(300);
    year_entry.set_height_request(45);
    month_entry.set_width_request(300);
    month_entry.set_height_request(45);
    day_entry.set_width_request(300);
    day_entry.set_height_request(45);
    year_entry.set_halign(gtk4::Align::Center);
    month_entry.set_halign(gtk4::Align::Center);
    day_entry.set_halign(gtk4::Align::Center);

    header.set_halign(gtk4::Align::End);

    page.set_margin_top(20);
    page.set_margin_bottom(20);
    page.set_margin_start(20);
    page.set_margin_end(20);
    
    let title = gtk4::Label::new(Some("تبدیل تاریخ جلالی به میلادی"));
    title.add_css_class("title-2");

    let back_button = gtk4::Button::with_label("برگشت");
    let stack_clone = stack.clone();

    back_button.set_width_request(100);
    back_button.set_height_request(40);

    back_button.connect_clicked(move |_| {
        stack_clone.set_visible_child_name("calendar");
    });

    let convert_button = gtk4::Button::with_label("تبدیل");
    convert_button.set_width_request(300);
    convert_button.set_height_request(45);
    convert_button.set_margin_top(10);
    convert_button.set_halign(gtk4::Align::Center);

    let year_entry_clone = year_entry.clone();
    let month_entry_clone = month_entry.clone();
    let day_entry_clone = day_entry.clone();

    let result_label = gtk4::Label::new(Some("نتیجه:"));
    result_label.add_css_class("title-3");

    let result_label_clone = result_label.clone();

    convert_button.connect_clicked(move |_| {
        let year = match year_entry_clone.text().parse::<i32>() {
            Ok(value) => value,
            Err(_) => {
                result_label_clone.set_text("سال باید عدد باشد");
                return;
            }
        };
        let month = match month_entry_clone.text().parse::<u32>() {
            Ok(value) => value,
            Err(_) => {
                result_label_clone.set_text("ماه باید عدد باشد");
                return;
            }
        };

        let day = match day_entry_clone.text().parse::<u32>() {
            Ok(value) => value,
            Err(_) => {
                result_label_clone.set_text("روز باید عدد باشد");
                return;
            }
        };

        match jalali_to_gregorian(year, month, day) {
            Ok(result) => {
                result_label_clone.set_text(
                    &format!("نتیجه: {}/{}/{}", result.0, result.1, result.2)
                );
            }
        
            Err(_) => {
                result_label_clone.set_text("تاریخ وارد شده معتبر نیست");
            }
        }
    });

    header.append(&back_button);

    page.append(&title);
    page.append(&back_button);
    page.append(&header);
    page.append(&year_entry);
    page.append(&month_entry);
    page.append(&day_entry);
    page.append(&convert_button);
    page.append(&result_label);
    
    page
}