use e_macros::value;

// Define the Config enum
#[value]
#[derive(Debug, PartialEq)]
enum Config {
    #[e(value = "database")]
    Database { url: String, port: u16 },
    #[e(value = "api")]
    Api { endpoint: String },
    #[e(value = "logging")]
    Logging(LogLevel),
}

// Define the LogLevel enum
#[value]
#[derive(Debug, PartialEq)]
enum LogLevel {
    #[e(value = "debug")]
    Debug,
    #[e(value = "info")]
    Info,
}

// Define the LinkedList enum
#[value]
#[derive(Debug, PartialEq)]
enum LinkedList {
    #[e(value = "cons")]
    Cons(i32, Box<LinkedList>),
    #[e(value = "nil")]
    Nil,
}

fn main() {
    // Create a database config instance
    let config = Config::Database {
        url: "localhost".to_string(),
        port: 5432,
    };

    // Print debug and display output for config
    println!("Debug output: {:?}", config);
    println!("Display output: {}", config);

    // Create a log level instance
    let log_level = LogLevel::Debug;
    println!("LogLevel Debug: {:?}", log_level);
    println!("LogLevel Display: {}", log_level);

    // Create API config and logging config instances
    let api_config = Config::Api {
        endpoint: "https://api.example.com".to_string(),
    };
    let logging_config = Config::Logging(LogLevel::Info);
    println!("api {api_config:#?}");
    println!("logging {logging_config:#?}");

    // Create a linked list instance
    let list = LinkedList::Cons(
        1,
        Box::new(LinkedList::Cons(
            2,
            Box::new(LinkedList::Cons(3, Box::new(LinkedList::Nil))),
        )),
    );
    // Print different formats of the linked list
    println!("LinkedList Debug: {:?}", list);
    println!("LinkedList Display: {}", list);
    println!("LinkedList Pretty Debug: {:#?}", list);
}
