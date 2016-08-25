extern crate gtk;

use gtk::Inhibit;
use gtk::traits::*;

pub fn create_menu<FS: Fn() + 'static, FQ: Fn() + 'static>(save_action: FS,
                                                           quit_action: FQ)
                                                           -> gtk::MenuBar {
    let bar = gtk::MenuBar::new();
    let file = gtk::MenuItem::new_with_mnemonic("_File");
    let f_menu = gtk::Menu::new();
    let f_new = gtk::MenuItem::new_with_mnemonic("_New");
    let f_save = gtk::MenuItem::new_with_mnemonic("_Save");
    let f_quit = gtk::MenuItem::new_with_mnemonic("_Quit");

    f_save.connect_activate(move |_| {
        save_action();
        Inhibit(false);
    });
    f_quit.connect_activate(move |_| {
        quit_action();
        Inhibit(false);
    });
    let help = gtk::MenuItem::new_with_mnemonic("_Help");
    let h_menu = gtk::Menu::new();
    let h_about = gtk::MenuItem::new_with_mnemonic("A_bout");

    f_menu.append(&f_new);
    f_menu.append(&f_save);
    f_menu.append(&f_quit);
    file.set_submenu(Some(&f_menu));
    bar.append(&file);

    h_menu.append(&h_about);
    help.set_submenu(Some(&h_menu));
    bar.append(&help);
    bar
}