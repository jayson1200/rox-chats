//Make the rest of my code aware of this rust crate
extern crate easy_http_request;

//Brings the stdin whihc is part of rust standard library into scope
use std::io::stdin;
//Brings the code from this httprequest library into scope
use easy_http_request::DefaultHttpRequest;

//Brings a type i'm using to parse json into scope
use serde_json::Map;

//Brings my random generator code into scope
use rand::Rng;

//The Api key that I use to access the cleverbot API
static API_KEY: &str = "CC9g3wS9yJIJDrJOd22Ecej4LKg";


//Where my program begins
fn main() {
    print_seperator();
    println!("WARNING: This will not work on school wifi");
    print_seperator();
    println!("Welcome to roxchats");
    print_seperator();
    println!("To talk to one of the chatbots, simply just say something and click enter");
    print_seperator();
    println!("The chatbot will respond with text and some emojis artwork to describe the chatbot's emotions. After the chatbot responds, you can respond by typing something in the command line and hitting enter");
    print_seperator();
    println!("This is a very intelliegent chatbot (You can really get into some deep conversations) so say anything you like");
    print_seperator();

    
    let mut user_input = get_user_input();
    let mut cleverbotState: Option<String> = None;
    let emojis = vec!["😀","🙂", "😀", "😃", "😄", "😁", "😅", "😆", "🤣", "😂", "🙃", "😉", "😊", "😇", "😎", "🤓", "🧐", "🥳", "🥰", "😍", "🤩", "😘", "😗"];

    while(true){

        //Generates a random nnumber between 0 and the length of the emoji vector
        let randomInt = rand::thread_rng().gen_range(0..emojis.len());
        
        //Controls the structure of the HTTP request url according to what stage the user is when using the program
        match &cleverbotState {
            Some(state) => {
                println!("{}{}{}", emojis.get(randomInt).unwrap(), get_chatbot_response(&user_input, state), emojis.get(randomInt).unwrap());
            }
            None => {
                let init_response = init_response(&user_input);

                cleverbotState = Some(init_response.1);

                println!("{}{}{}", emojis.get(randomInt).unwrap(), init_response.0, emojis.get(randomInt).unwrap());
            }
        }

        user_input = get_user_input();
    }
    
    
}

//Gets user input
fn get_user_input() -> String {
    let mut input_string = String::new();

    stdin().read_line(&mut input_string)
    	.ok()
        .expect("Failed to read line");

    input_string
}

//Is the first call to the web api that I make
//It returns a tuple with two strings
fn init_response(init_user_repsonse: &str) -> (String, String) {
    let url = format!("https://www.cleverbot.com/getreply?key={}&input={}", API_KEY, init_user_repsonse);

    //Sends the http request
    let response = DefaultHttpRequest::get_from_url_str(url).unwrap().send().unwrap();

    //converts the binary to a json string
    let json_string = String::from_utf8(response.body).unwrap();

    //converts the json into a map
    let message: Map<String, serde_json::Value> = serde_json::from_str(&json_string).unwrap();

    //returns the infromation in the form of a tuple
    (message.get("output").unwrap().to_string(), message.get("cs").unwrap().to_string())
}

//Handle all subsequent calls to the cleverbot chat web api
fn get_chatbot_response(input_text: &str, state: &str) -> String {
    let url = format!("https://www.cleverbot.com/getreply?key={}&input={}&cs={}", API_KEY, input_text, state);
    
    //Send the http request
    let response = DefaultHttpRequest::get_from_url_str(url).unwrap().send().unwrap();

    //Converts the binary into a json string
    let json_string = String::from_utf8(response.body).unwrap();
    //Converts the json string into a map
    let message: Map<String, serde_json::Value> = serde_json::from_str(&json_string).unwrap();

    //returns the information in the form of a string
    message.get("output").unwrap().to_string()
}

//Prints a seperator for organization
fn print_seperator() {
    println!("===========================================");
}
