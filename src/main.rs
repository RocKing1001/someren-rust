use adw::prelude::*;
use adw::{glib, Application, ApplicationWindow};
use gtk::Orientation;

const APP_ID: &str = "me.piguy.inholland.someren";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    return app.run();
}

fn build_ui(app: &Application) {
    // Create a button with label and margins
    let button = gtk::Button::builder().label("Dashboard").build();

    let content = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .build();

    let stack = adw::ViewStack::builder().build();

    stack.add_titled_with_icon(
        &gtk::Label::new(Some("Rooms")),
        Some("People"),
        "People",
        "money-clip-symbolic",
    );
    stack.add_titled_with_icon(
        &gtk::Label::new(Some("Rooms")),
        Some("Rooms"),
        "Rooms",
        "Page 2",
    );
    stack.add_titled_with_icon(
        &gtk::Label::new(Some("Activities")),
        Some("Activities"),
        "Activities",
        "Page 2",
    );
    stack.add_titled_with_icon(
        &gtk::Label::new(Some("Shop")),
        Some("Shop"),
        "Shop",
        "Page 2",
    );

    stack.set_vexpand(true);

    let v = adw::ViewSwitcher::builder()
        .policy(adw::ViewSwitcherPolicy::Wide)
        .stack(&stack)
        .build();

    let header = adw::HeaderBar::builder().title_widget(&v).build();

    header.pack_start(&button);

    content.append(&header);
    content.append(&stack);

    let window = ApplicationWindow::builder()
        .decorated(false)
        .application(app)
        .title("Someren")
        .content(&content)
        .build();

    window.present();
}
