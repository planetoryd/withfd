use std::{
    error::Error,
    fs::File,
    io::Read,
    os::{fd::AsFd, unix::net::UnixStream},
};

use nix::unistd::{fork, ForkResult};
use withfd::WithFdExt;

fn main() -> Result<(), Box<dyn Error>> {
    let (mut sp, mut sc) = UnixStream::pair()?;
    let mut k = [0; 1];
    match unsafe { fork() }? {
        ForkResult::Parent { child } => {
            let mut sp = sp.with_fd();

            sp.read_exact(&mut k)?;
            let f = sp.take_fds().next().unwrap();
            println!("fd {:?}", f);

            let mut f = File::from(f);
            let mut buf2 = [0; 5];
            f.read_exact(&mut buf2)?;
            println!("{}", String::from_utf8_lossy(&buf2));
        },
        ForkResult::Child => {
            let f = File::open("./Cargo.toml")?;
            let mut sc = sc.with_fd();
            sc.write_with_fd(&[0], &[f.as_fd()])?;
        },
    }
    Ok(())
}
