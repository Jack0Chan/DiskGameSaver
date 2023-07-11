use std::fs;
use toml::Value;

#[derive(Debug)]
pub struct Game {
    pub id: i64,
    pub official_name: String,
    pub other_names: Vec<String>,
    pub figure: String,
}

#[test]
fn test() {
    println!("test_toml_parser");
    let root = project_root::get_project_root().unwrap();
    let toml_file = root.join("settings/games.toml");
    // 读取Toml文件
    let contents = fs::read_to_string(toml_file).expect("Failed to read file");

    // 解析Toml内容
    let parsed: Value = toml::from_str(&contents).expect("Failed to parse Toml");

    // 获取游戏列表
    let games = parsed
        .get("game")
        .and_then(|games| games.as_array())
        .expect("Failed to get games");
    // println!("{:?}", games.get(0));

    // 遍历游戏列表
    let mut game_list: Vec<Game> = vec![];

    for game in games {
        // println!("1");
        if let Some(game_table) = game.as_table() {
            // println!("{:?}", game_table);
            // println!("{:?}", game_table.get("id"));
            if let (Some(id), Some(official_name), Some(other_names), Some(figure)) = (
                game_table.get("id").and_then(|id| id.as_integer()),
                game_table
                    .get("official_name")
                    .and_then(|name| name.as_str()),
                game_table
                    .get("other_names")
                    .and_then(|names| names.as_array()),
                game_table.get("figure").and_then(|figure| figure.as_str()),
            ) {
                // let id_string = id.to_string();
                let official_name_string = official_name.to_string();
                let other_names_vec = other_names
                    .iter()
                    .filter_map(|name| name.as_str().map(|n| n.to_string()))
                    .collect();
                let figure_string = figure.to_string();

                let game = Game {
                    id: id,
                    official_name: official_name_string,
                    other_names: other_names_vec,
                    figure: figure_string,
                };
                game_list.push(game);
            }
        }
    }

    // 打印游戏列表
    for game in game_list {
        println!("{:#?}", game);
    }
}
