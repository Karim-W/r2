use std::env;
use std::fs::{create_dir_all, File};
fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: r2 <input>");
        return;
    }
    let current_path = env::current_dir().unwrap().to_str().unwrap().to_string();
    let input = &args[1];
    let tree = input.split("/").collect::<Vec<_>>();
    let last = tree.last().unwrap();
    let total_path = format!("{}/{}", current_path, tree[..tree.len() - 1].join("/"));
    create_dir_all(total_path).unwrap();
    if last.contains(".") {
        let fullpath = format!("{}/{}", current_path, input);
        let file = File::create(fullpath).ok();
        if file.is_none() {
            println!("failed to create file {}", input);
            return;
        }
    } else {
        let res = create_dir_all(&format!("{}/{}", current_path, input)).ok();
        if res.is_none() {
            println!("failed to create");
            return;
        }
    }
}
