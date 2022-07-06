use inquire::{MultiSelect, Select, Text};
use spinners::{Spinner, Spinners};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let status = Text::new("What are you thinking about?")
        .prompt()
        .expect("xx");

    let options: Vec<&str> = vec![
        "eslint-react",
        "eslint-react-jsx-runtime",
        "stylelint",
        "prettier",
    ];
    // let res = Select::new("请选择你希望接入的模块", options)
    //     .prompt()
    //     .expect("error");

    MultiSelect::new("xxx", options).prompt().expect("error");
    println!("{}", status);
    // another_function(5);
    let mut sp = Spinner::new(Spinners::Dots9, "Waiting for 3 seconds".into());
    sleep(Duration::from_secs(3));
    sp.stop();
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}
