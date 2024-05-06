use std::process::Command;
use std::str;

fn main() {
    // WiFi profillerini al
    let output = Command::new("netsh")
        .args(&["wlan", "show", "profiles"])
        .output()
        .expect("Failed to execute command");

    let output_str = str::from_utf8(&output.stdout).expect("Invalid UTF-8");

    // Her bir WiFi profili için şifreyi al
    for line in output_str.lines() {
        if line.contains("All User Profile") {
            let profile_name = line.split(':').nth(1).map(|s| s.trim()).unwrap_or("");

            let key_output = Command::new("netsh")
                .args(&["wlan", "show", "profile", profile_name, "key=clear"])
                .output()
                .expect("Failed to execute command");

            let key_output_str = str::from_utf8(&key_output.stdout).expect("Invalid UTF-8");

            // Şifreyi bul
            let key = key_output_str
                .lines()
                .filter(|line| line.contains("Key Content"))
                .map(|line| line.split(':').nth(1).map(|s| s.trim()).unwrap_or(""))
                .next()
                .unwrap_or("");

            println!("{:<30}|  {:<}", profile_name, key);
        }
    }

    // Programın kapanmaması için bir tuşa basılmasını bekle
    println!("Press Enter to exit...");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
}
