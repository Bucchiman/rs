/*
 * FileName:        main
 * Author:          8ucchiman
 * CreatedDate:     2023-08-30 15:30:48
 * LastModified:    2023-01-23 14:15:07 +0900
 * Reference:       https://qiita.com/koji_mats/items/62e85a87cc580e225796
 * Description:     ---
 */


use std::error::Error;
extern crate gtk;
extern crate gio;

use gtk::{ WidgetExt, WindowExt };
use gio::{ ApplicationExt };


fn main() -> Result<(), Box<dyn Error>> {
    //アプリケーションを生成する
    match gtk::Application::new("com.github.koji-m.vanilla_text", gio::APPLICATION_HANDLES_OPEN) {
        Ok(app) => {
            //アプリケーションへ、activateシグナルに対するハンドラを設定する
            app.connect_activate(|app| {
                //ウィンドウを生成する
                let win = gtk::ApplicationWindow::new(&app);
                //ウィンドウのタイトルに表示する文字列を設定する
                win.set_title("Hello Gtk-rs");
                //ウィンドウとその中身全てを可視状態にする
                win.show_all();
            });

            //アプリケーションを開始、アプリケーションへactivateシグナルがエミットされる
            app.run(&[""]);
        },
        Err(_) => {
            println!("Application start up error");
        }
    };
    Ok(())
}

