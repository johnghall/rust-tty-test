mod test;

use nix::pty::forkpty;
use nix::unistd::{read, ForkResult};
use std::os::fd::RawFd;
use std::process::Command;
use vte::{Params, Parser, Perform};

struct Performer;

impl Perform for Performer {
    fn print(&mut self, c: char) {
        println!("[print] {:?}", c);
    }

    fn execute(&mut self, byte: u8) {
        println!("[execute] {:02x}", byte);
    }

    fn hook(&mut self, params: &Params, intermediates: &[u8], ignore: bool, c: char) {
        println!(
            "[hook] params={:?}, intermediates={:?}, ignore={:?}, char={:?}",
            params, intermediates, ignore, c
        );
    }

    fn put(&mut self, byte: u8) {
        println!("[put] {:02x}", byte);
    }

    fn unhook(&mut self) {
        println!("[unhook]");
    }

    fn osc_dispatch(&mut self, params: &[&[u8]], bell_terminated: bool) {
        println!(
            "[osc_dispatch] params={:?} bell_terminated={}",
            params, bell_terminated
        );
    }

    fn csi_dispatch(&mut self, params: &Params, intermediates: &[u8], ignore: bool, c: char) {
        println!(
            "[csi_dispatch] params={:#?}, intermediates={:?}, ignore={:?}, char={:?}",
            params, intermediates, ignore, c
        );
    }

    fn esc_dispatch(&mut self, intermediates: &[u8], ignore: bool, byte: u8) {
        println!(
            "[esc_dispatch] intermediates={:?}, ignore={:?}, byte={:02x}",
            intermediates, ignore, byte
        );
    }
}

fn parse_ansi_escape(read_bytes: Vec<u8>) -> () {
    let mut parser = Parser::new();

    let mut log = Performer;

    for byte in &read_bytes {
        parser.advance(&mut log, *byte)
    };
}

fn read_from_fd(fd: RawFd) -> Option<Vec<u8>> {
    let mut read_buffer = [0; 65536];
    let read_result = read(fd, &mut read_buffer);
    match read_result {
        Ok(bytes_read) => Some(read_buffer[..bytes_read].to_vec()),
        Err(_e) => None,
    }
}

fn spawn_pty_with_shell(default_shell: String) -> RawFd {
    unsafe {
        match forkpty(None, None) {
            Ok(fork_pty_res) => {
                let stdout_fd = fork_pty_res.master;
                if let ForkResult::Child = fork_pty_res.fork_result {
                    Command::new(&default_shell)
                        .spawn()
                        .expect("failed to spawn");
                    std::thread::sleep(std::time::Duration::from_millis(2000));
                    Command::new("ls").spawn().expect("failed ls");
                    std::thread::sleep(std::time::Duration::from_millis(2000));
                    std::process::exit(0);
                }
                stdout_fd
            }
            Err(e) => {
                panic!("forkpty failed: {}", e);
            }
        }
    }
}

fn main() {
    let default_shell = std::env::var("SHELL").expect("SHELL environment variable not set");
    let stdout_fd = spawn_pty_with_shell(default_shell);
    let mut read_buffer = vec![];
    loop {
        match read_from_fd(stdout_fd) {
            Some(mut read_bytes) => {
                // read_buffer.append(&mut read_bytes);
                parse_ansi_escape(read_bytes); 
            }
            None => {
                println!("{:?}", String::from_utf8(read_buffer).unwrap());
                std::process::exit(0);
            }
        }
    }
}
