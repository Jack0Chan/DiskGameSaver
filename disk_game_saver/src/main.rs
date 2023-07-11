use std::path::Path;
mod file_manager;
mod spider;
mod toml_parser;

// fn main() {
//     match project_root::get_project_root() {
//         Ok(p) => {
//             // println!("Current project root is {:?}", p);
//             let source_folder = p.join(Path::new("tests/source_folder"));
//             let target_folder = p.join(Path::new("tests/target_folder"));
//             // println!("{:?}", source_folder);
//             // println!("{:?}", target_folder);
//             file_manager::sync_folders(
//                 source_folder.to_str().unwrap(),
//                 target_folder.to_str().unwrap(),
//             );
//         }
//         Err(e) => println!("Error obtaining project root {:?}", e),
//     };
// }

// Copyright © SixtyFPS GmbH <info@slint.dev>
// SPDX-License-Identifier: MIT

fn main() {
    MainWindow::new().unwrap().run().unwrap();
}

slint::slint! {
    component MemoryTile inherits Rectangle {
        width: 64px;
        height: 64px;
        background: #3960D5;

        Image {
            source: @image-url("icons/bus.png");
            width: parent.width;
            height: parent.height;
        }
    }

    export component MainWindow inherits Window {
        MemoryTile {}
    }

}
