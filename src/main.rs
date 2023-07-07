use std::os::fd::RawFd;

fn read_from_fd(fd: RawFd) -> Option<Vec<u8>> {
    unimplemented!()
}

fn spawn_pty_with_shell(default_shell: String) -> RawFd {
    unimplemented!()
}

fn main() {
    let default_shell = std::env::var("SHELL").expect("SHELL environment variable not set");
    let stdout_fd = spawn_pty_with_shell(default_shell);
    let mut read_buffer = vec![];
    loop {
        match read_from_fd(stdout_fd) {
            Some(mut read_bytes) => {
                read_buffer.append(&mut read_bytes);
            }
            None => {
                println!("{:?}", String::from_utf8(read_buffer).unwrap());
                std::process::exit(0);
            }
        }
    }
}
