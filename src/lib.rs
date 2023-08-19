enum ItemType {
    File,
    Folder,
}

struct Item {
    pub name: std::ffi::OsString,
    pub item_type: ItemType,
}

const COLUMN_WIDTH: u32 = 5;

fn display_items(items: Vec<Item>) {
    let mut column_count: u32 = 0;

    for item in items.iter() {
        if column_count >= COLUMN_WIDTH {
            println!();
            column_count = 0;
        } 

        match item.item_type {
            ItemType::File => print!("\x1b[38;2;129;138;253m{}\x1b[0m", item.name.to_str().unwrap()),
            ItemType::Folder => print!("\x1b[38;2;193;66;202;4m{}\x1b[0m", item.name.to_str().unwrap()), 
        };

        print!("{: <10}", "");

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
        items.push(Item {
            name: item.as_ref().unwrap().file_name(),
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
        });
    }

    display_items(items);
}
