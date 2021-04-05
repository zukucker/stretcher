use notify_rust::Notification;
use std::time::{Instant, Duration};
use std::thread::sleep;

fn main() {
    send_message();
}

fn send_message() {
    let active = true;
    while active == true{
        if messure(){
            Notification::new()
                .summary("It's time for a stretch, Sir!")
                .body("Du hast dich seit über einer Stunde nicht bewegt. Es ist Zeit!")
                .show();
            println!("Send message");
        }
    }
}

fn messure() -> bool{
    let now = Instant::now();

    // notify every hour that you need to stetch
    sleep(Duration::new(60,0));

    return true;
}
