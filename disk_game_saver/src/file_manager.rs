use fs_extra::file;
use std::path::Path;
// 同步文件夹
// 1. 当`target_folder`里的dir不存在时，创建dir到`target_folder`.
// 2. 当`target_folder`里file不存在，或者日期较旧时，复制(overwrite)file到`target_folder`.
pub fn sync_folders(source_folder: &str, target_folder: &str) {
    let options = file::CopyOptions::new().overwrite(true);

    if let Ok(entries) = std::fs::read_dir(source_folder) {
        for entry in entries {
            if let Ok(entry) = entry {
                let source_item = entry.path();
                let target_item = Path::new(target_folder).join(entry.file_name());

                if source_item.is_dir() {
                    if !target_item.exists() {
                        std::fs::create_dir(&target_item).unwrap();
                    }
                    sync_folders(source_item.to_str().unwrap(), target_item.to_str().unwrap());
                } else if source_item.is_file() {
                    if !target_item.exists()
                        || source_item.metadata().unwrap().modified().unwrap()
                            > target_item.metadata().unwrap().modified().unwrap()
                    {
                        file::copy(&source_item, &target_item, &options).unwrap();
                        // println!("同步文件: {:?} -> {:?}", source_item, target_item);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod unit_tests {
    use fs_extra::dir;
    use lazy_static::lazy_static;
    use std::{
        fs::{self},
        path::{Path, PathBuf},
        str::FromStr,
    };

    use super::*;

    lazy_static! {
        static ref TEST_SOURCE_DIR: String = {
            let project_root = project_root::get_project_root().unwrap();
            let res = project_root.join(Path::new("tests/source_folder"));
            String::from(res.to_str().unwrap())
        };
        static ref TEST_TARGET_DIR: String = {
            let project_root = project_root::get_project_root().unwrap();
            let res = project_root.join(Path::new("tests/target_folder"));
            String::from(res.to_str().unwrap())
        };
        static ref TEST_HELLO0: String = {
            let project_root = project_root::get_project_root().unwrap();
            let res = project_root.join(Path::new("tests/target_folder/hello0.txt"));
            String::from(res.to_str().unwrap())
        };
        static ref TEST_HELLO1: String = {
            let project_root = project_root::get_project_root().unwrap();
            let res = project_root.join(Path::new("tests/target_folder/hellos/hello1.txt"));
            String::from(res.to_str().unwrap())
        };
        static ref TEST_HELLO2: String = {
            let project_root = project_root::get_project_root().unwrap();
            let res = project_root.join(Path::new("tests/target_folder/hellos/hello2.txt"));
            String::from(res.to_str().unwrap())
        };
    }

    fn initialize() {
        // 确保source的文件结构
        let project_root = project_root::get_project_root().unwrap();
        let hello0 = project_root.join("tests/source_folder/hello0.txt");
        let hello1 = project_root.join("tests/source_folder/hellos/hello1.txt");
        let hello2 = project_root.join("tests/source_folder/hellos/hello2.txt");
        assert!(hello0.exists());
        assert!(hello1.exists());
        assert!(hello2.exists());
        let default_content = "Hello World! in `hello2`";
        fs::write(hello2.clone(), default_content).unwrap();
        let content = fs::read_to_string(hello2.clone()).unwrap();
        assert_eq!(default_content, &content);
    }

    // 注意! rust test默认多线程，因此测试过程会出现读写冲突
    // 解决方案:
    // 1. cargo test -- --test-threads=1
    // 2. 多线程读写加lock. 我们目前不清楚需不需要多线程lock，因此建议使用方法1
    // 3. 或者像我现在这样，把test写在同一个函数里

    // 检查不存在的文件是否会同步
    fn copy_file() {
        initialize();
        // 清空目标目录
        dir::create(&*TEST_TARGET_DIR, true).unwrap();
        // 确认清空
        let entries = fs::read_dir(&*TEST_TARGET_DIR).unwrap();
        assert!(entries.count() == 0);
        // 同步文件
        sync_folders(&TEST_SOURCE_DIR, &TEST_TARGET_DIR);
        // 检查同步结果
        assert!(PathBuf::from_str(&TEST_HELLO1).unwrap().exists());
        assert!(PathBuf::from_str(&TEST_HELLO2).unwrap().exists());
        assert!(PathBuf::from_str(&TEST_HELLO2).unwrap().exists());
    }

    #[test]
    // 检查modified文件是否会同步
    fn overwrite_file() {
        copy_file();
        // 确认旧文件内容的正确性
        let test_hello2 = PathBuf::from_str(&TEST_HELLO2).unwrap();
        let default_content = "Hello World! in `hello2`";
        let content_old = fs::read_to_string(test_hello2.clone()).unwrap();
        assert_eq!(default_content, content_old);

        // 写入新内容
        let new_content = "Modified Hello World! in `hello2`";
        fs::write(test_hello2.clone(), new_content).unwrap();
        let content_new = fs::read_to_string(test_hello2.clone()).unwrap();
        assert_eq!(new_content, content_new);
    }
}
