#[derive(Debug)]
enum Menu {
    MainMenu,
    Start,
    Quit,
}

fn get_choice(choice: &str) -> Result<Menu, String> {
    match choice {
        "main_menu" => Ok(Menu::MainMenu),
        "start" => Ok(Menu::Start),
        "quit" => Ok(Menu::Quit),
        _ => Err("Menu choice not found".to_string())
    }
}

fn print_choice(choice: &Menu) {
    println!("choice = {:?}", choice);
}

fn pick_choice(input: &str) -> Result<(), String> {
    let choice: Menu = get_choice(input)?;
    print_choice(&choice);
    Ok(())
}

fn main() {
    let choice = pick_choice("leave");
    println!("choice = {:?}", choice);
}
