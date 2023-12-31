extern crate termsize;

enum ItemType {
    File,
    Folder,
}

struct Item {
    pub name: String,
    pub item_type: ItemType,
}

const CHARACTER_LIMIT: usize = 25;
const COLUMN_WIDTH: usize = 35;

fn cutoff_long_item_names(items: &mut Vec<Item>) {
    for item in items.iter_mut() {
        if item.name.len() > CHARACTER_LIMIT {
            let buffer_name = item.name.clone();
            item.name = buffer_name[0..(CHARACTER_LIMIT-3)].to_string();
            item.name.push_str("...");
        }
    }
}

fn calculate_column_max() -> u32 {
    let size: termsize::Size = termsize::get().unwrap();

    (size.cols/(COLUMN_WIDTH as u16)).into()
}

fn display_items(items: Vec<Item>) {
    let mut column_count: u32 = 0;
    let column_max = calculate_column_max();

    for item in items.iter() {
        if column_count >= column_max {
            println!();
            column_count = 0;
        } 

        match item.item_type {
            ItemType::File => print!("\x1b[38;2;129;138;253m{}\x1b[0m", item.name),
            ItemType::Folder => print!("\x1b[38;2;193;66;202;4m{}\x1b[0m", item.name), 
        };

        print!("{: <1$}", "", COLUMN_WIDTH-item.name.len());

        column_count += 1;
    }
}

pub fn run() {
    let dir = std::env::current_dir().unwrap_or_else(|_e| {
        eprintln!("Error when retrieving current directory!");
        std::process::exit(1);
    });

    let raw_items = std::fs::read_dir(dir).unwrap();
    let mut items: Vec<Item> = vec![];

    for item in raw_items {
        let buffer_item = Item {
            name: item.as_ref().unwrap().file_name().into_string().unwrap(),
            item_type: match item.as_ref().unwrap().file_type() {
                Ok(x) => {
                    if x.is_dir() {
                        ItemType::Folder
                    } else {
                        ItemType::File
                    }
                }
                Err(e) => panic!("Error reading file type: {}", e),
            },
        };

        match buffer_item.item_type {
            ItemType::File => items.push(buffer_item),
            ItemType::Folder => items.insert(0, buffer_item),
        };
    }

    cutoff_long_item_names(&mut items);
    display_items(items);
}
