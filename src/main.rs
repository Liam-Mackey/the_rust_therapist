extern crate rand;
use std::io;
use rand::Rng;

fn main() {
    println!("Welcome to the Rust Therapist's office. \n");
    println!("How are you feeling today? \n");


    let mut resp = String::new();

    io::stdin().read_line(&mut resp)
        .expect("Failed to read line");

    if resp.len() < 10 {
        println!("Why don't you tell me a little more about that? \n");
        let mut descison = String::new();
        io::stdin().read_line(&mut descison)
            .expect("Failed to read line");
        let mut i = 0;
        while i < 10 {
            let ans = therapist();
            println!("{}", ans);

            io::stdin().read_line(&mut descison)
                .expect("Failed to read line");
            i += 1;
        }
    } else {
        println!("Wow sounds like you have a lot on your mind. Wanna share? \n");
        let mut descison = String::new();
        io::stdin().read_line(&mut descison)
            .expect("Failed to read line");
        let mut i = 0;
        while i < 10 {
            let ans = therapist();
            println!("{}", ans);
            io::stdin().read_line(&mut descison)
                .expect("Failed to read line");
            i += 1;
        }
    }
    println!("I think this session went really well come back soon! \n");
}


fn therapist() -> &'static str  {
    let ans = rand::thread_rng().gen_range(1, 3);
    let therapist_resp = ["Wow that must cause a wide array of feelings and possible situations for you? Want to elaborate more? \n",
    "That's very interesting. How do you think you should deal with that? \n",
    "That's intense. How do you think you should proceed? \n"];
    let resp =  therapist_resp[ans];
    return resp;
}