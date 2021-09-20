use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx1, rx) = mpsc::channel();

    let tx2 = tx1.clone();
    thread::spawn(move || {
        let greetings = vec![
            String::from("Hi"),
            String::from("from"),
            String::from("Guilherme"),
            String::from("Jordi"),
            String::from("and"),
            String::from("Nicolle"),
            String::from(":)"),
        ];

        for greeting in greetings {
            tx2.send(greeting).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let letters = vec![
            String::from("S"),
            String::from("I"),
            String::from("S"),
            String::from("O"),
            String::from("P"),
        ];

        for letter in letters {
            tx1.send(letter).unwrap();
            thread::sleep(Duration::from_secs(2));
        }
    });

    for received in rx {
        println!("{}", received);
    }
}
