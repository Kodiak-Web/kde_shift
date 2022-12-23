use regex::Regex;
use std::process::Command;
fn main() {
    shifter();
    let playername: String = strip_playerout();
    qdbus_send_application(playername);
}

fn shifter() {
    Command::new("dbus-send")
        .args([
            "--type=method_call",
            "--dest=org.mpris.MediaPlayer2.playerctld",
            "/org/mpris/MediaPlayer2",
            "com.github.altdesktop.playerctld.Shift",
        ])
        .output();
}

fn strip_playerout() -> String {
    let playerctl_list = Command::new("playerctl").arg("-l").output().unwrap();
    let players = format!("{:?}", std::str::from_utf8(&playerctl_list.stdout));

    let player_filter = Regex::new(r#"(Ok\(")(\w+)"#).unwrap();
    let caps = player_filter.captures(&players).unwrap();
    let playername = caps.get(2).unwrap().as_str();
    return playername.to_string();
}

fn qdbus_send_application(appname: String) {
    Command::new("/usr/bin/qdbus")
        .args([
            "org.kde.plasmashell",
            "/org/kde/osdService",
            "org.kde.osdService.showText",
            &appname.as_str(),
            &appname.as_str(),
        ])
        .output().expect("qdbus is not installed");
}
