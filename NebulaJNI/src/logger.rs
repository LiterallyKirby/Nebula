
use std::fs::OpenOptions;
use std::io::Write;

pub fn log(msg: &str) {
    eprintln!("{}", msg);

    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("/tmp/NebulaJNI.log")
    {
        let _ = writeln!(file, "{}", msg);
        let _ = file.flush();
    }
}
