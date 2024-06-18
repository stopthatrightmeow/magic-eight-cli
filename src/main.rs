use std::io;
use rand::seq::SliceRandom;

fn main() {
    // Create the vars
    let mut name: String = String::new();
    let mut question: String = String::new();

    // Keep asking user their name until the variable isn't empty
    while name.trim().is_empty() {
        println!("What is your name?");
        io::stdin() 
            .read_line(&mut name)
            .expect("Failed to read line.");
    }
    // Keep asking user their question until the variable isn't empty
    while question.trim().is_empty() {
        println!("What do you want to know?");
        io::stdin()
            .read_line(&mut question)
            .expect("Failed to read line.");
    }

    // List of potential responses 
    let responses = vec!["Yes, definitely", "It is certain", "Without a doubt", 
    "Yes, absolutely", "You may rely on it", "As I see it, yes", "Most likely", 
    "Outlook good", "Yes", "Signs point to yes", "Reply hazy, try again", 
    "Ask again later", "Better not tell you now", "Cannot predict now", 
    "Concentrate and ask again", "Don't count on it", "My reply is no", 
    "My sources say no", "Outlook not so good", "Very doubtful", "Absolutely", 
    "Indubitably", "Positively", "Certainly", "Yes, indeed", "Sure thing", 
    "All signs say yes", "Decidedly so", "No doubt about it", "Affirmative", 
    "Absolutely not", "Definitely not", "Negative", "Unlikely", "Chances aren't good", 
    "I wouldn't bet on it", "Don't hold your breath", "My intuition says no"];
    
    // Re-state the prompt back to the user
    println!("{} asks {}\n", name.trim(), question.trim());
    
    // Get a random choice from the list
    let response = responses.choose(&mut rand::thread_rng());
    
    // Print the response to the user
    println!("{}", response.unwrap());

}
