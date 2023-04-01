use libadwaita::{
    gtk::Orientation,
    prelude::{ApplicationExt, ApplicationExtManual, BoxExt, WidgetExt},
    Application, ApplicationWindow, HeaderBar, WindowTitle,
};

pub fn run() {
    let application = Application::new(Some("com.rain"), Default::default());

    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(application: &Application) {
    let content = libadwaita::gtk::Box::new(Orientation::Vertical, 0);

    content.append(&HeaderBar::builder()
        .title_widget(&WindowTitle::new("rain", ""))
        .build());

    let window = ApplicationWindow::builder()
        .application(application)
        .title("Rain")
        .default_height(250)
        .default_width(400)
        .content(&content)
        .build();

    window.show()
}
