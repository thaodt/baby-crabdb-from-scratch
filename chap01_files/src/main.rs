extern crate libc;
use std::fs::OpenOptions;
use std::io::Write;
use std::os::unix::fs::OpenOptionsExt;

/// some drawbacks:
///  1. the file is truncated before updating. What if the file need to be read concurrently?
///  2. Writing data to files may not be atomic, depending on the size of the write. Concurrent readers might get incomplete data.
///  3. When is the data actually persisted to the disk?
///     The data is probably still in the operating system’s page cache after the write syscall returns.
///     What’s the state of the file when the system crashes and reboots?
#[allow(dead_code)]
fn persist_data_1<'a>(path: &'a str, data: &'a [u8]) -> std::io::Result<()> {
    let mut options = OpenOptions::new();
    options.mode(0o644).write(true);
    if cfg!(unix) {
        options.custom_flags(libc::O_WRONLY|libc::O_CREAT|libc::O_TRUNC);
    }
    let mut file = options.open(path)?;
    file.write_all(data)
}

fn persist_data_2(path: &str, data: &[u8]) -> std::io::Result<()> {
    let mut options = OpenOptions::new();
    options.mode(0o644).write(true);
    if cfg!(unix) {
        options.custom_flags(libc::O_WRONLY|libc::O_CREAT|libc::O_EXCL);
    }
    let tmp_fp = "/tmp/persist-tmp.txt";
    let mut file = options.open(tmp_fp)?;
    if file.write_all(data).is_ok() {
        println!("write ok");
        let relative_path = std::path::PathBuf::from(path);
        let mut absolute_path = std::env::current_dir()?;
        absolute_path.push(relative_path);
        println!("{}", absolute_path.display());
        std::fs::rename(tmp_fp, std::path::PathBuf::from(path))
    } else {
        println!("write failed");
        std::fs::remove_file(tmp_fp)?;
        println!("remove tmp file");
        Err(std::io::Error::new(std::io::ErrorKind::AlreadyExists, "file already exists"))
    }

}


fn main() {
    /*let path = "persis-t1.txt";
    let data = vec![1, 2, 3];
    let _ = persist_data_1(path, &data);*/

    let path = "persis-t2.txt";
    let data = vec![1, 2, 3];
    let _ = persist_data_2(path, &data);
}
