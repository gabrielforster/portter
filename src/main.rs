fn main() {
    let kill_arg = std::env::args().position(|r| r == "--kill" || r == "-k");

    if let Some(kill_arg) = kill_arg {
        let kill_arg = kill_arg + 1;
        let kill_arg = std::env::args().nth(kill_arg).unwrap();
        let kill_arg = kill_arg.parse::<u16>().unwrap();

        let output = std::process::Command::new("lsof")
            .arg("-t")
            .arg("-i")
            .arg(format!(":{}", kill_arg))
            .output()
            .expect("failed to execute process");

        let output = String::from_utf8_lossy(&output.stdout);
        let output = output.split_whitespace().collect::<Vec<&str>>();
        let pid = output.get(0).unwrap_or(&"0");

        std::process::Command::new("kill")
            .arg(format!("{}", pid))
            .output()
            .expect("failed to execute process");
    } else {
        println!("Portter: Need a port to kill!");
    }
}
