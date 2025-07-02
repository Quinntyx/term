use gtk4::gio;
use gtk4::{
    Application, ApplicationWindow, Box, Button, Label, Notebook, Orientation, Paned,
    ScrolledWindow, prelude::*,
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

        let header_box = Box::new(Orientation::Horizontal, 5);
        header_box.append(&Label::new(Some("Terminal")));
        let add_button = Button::builder().label("+").build();
        header_box.append(&add_button);

        let notebook = Notebook::new();
        let paned = Paned::new(Orientation::Horizontal);
        paned.set_start_child(Some(&notebook));

        let root_box = Box::new(Orientation::Vertical, 0);
        root_box.append(&header_box);
        root_box.append(&paned);
        window.set_child(Some(&root_box));

        let nb_clone = notebook.clone();
        add_button.connect_clicked(move |_| {
            add_tab(&nb_clone);
        });

        let paned_clone = paned.clone();
        notebook.connect_create_window(move |_, _| {
            let nb = Notebook::new();
            paned_clone.set_end_child(Some(&nb));
            Some(nb)
        });

        add_tab(&notebook);
        window.present();
    });

    app.run();
}
