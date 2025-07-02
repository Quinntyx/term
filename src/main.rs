use gtk4::gio;
use gtk4::{
    Application, ApplicationWindow, Button, HeaderBar, Label, Notebook, ScrolledWindow, prelude::*,
};
use std::env;
use vte4::{PtyFlags, Terminal, TerminalExtManual};

fn spawn_shell(term: &Terminal) {
    let shell = env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
    term.spawn_async(
        PtyFlags::DEFAULT,
        None::<&str>,
        &[&shell],
        &[],
        glib::SpawnFlags::DEFAULT,
        || {},
        -1,
        None::<&gio::Cancellable>,
        |_| {},
    );
}

fn add_tab(notebook: &Notebook) {
    let term = Terminal::new();
    spawn_shell(&term);
    let label = Label::new(Some("Shell"));
    let scrolled = ScrolledWindow::builder().child(&term).build();
    notebook.append_page(&scrolled, Some(&label));
    notebook.set_tab_reorderable(&scrolled, true);
    notebook.set_tab_detachable(&scrolled, true);
}

fn main() {
    let app = Application::builder()
        .application_id("com.example.term")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Terminal")
            .default_width(800)
            .default_height(600)
            .build();

        let notebook = Notebook::new();
        window.set_child(Some(&notebook));

        let header = HeaderBar::builder()
            .title_widget(&Label::new(Some("Terminal")))
            .show_title_buttons(true)
            .build();
        let add_button = Button::builder().label("+").build();
        header.pack_end(&add_button);
        window.set_titlebar(Some(&header));

        let nb_clone = notebook.clone();
        add_button.connect_clicked(move |_| {
            add_tab(&nb_clone);
        });

        let app_clone = app.clone();
        notebook.connect_create_window(move |_, _| {
            let win = ApplicationWindow::builder()
                .application(&app_clone)
                .title("Terminal")
                .default_width(800)
                .default_height(600)
                .build();
            let nb = Notebook::new();
            win.set_child(Some(&nb));
            win.present();
            Some(nb)
        });

        add_tab(&notebook);
        window.present();
    });

    app.run();
}
