use ssh2;
use std::net;
use std::io::{self, Read, Write};

pub type Error = ssh2::Error;
pub type Result<T> = std::result::Result<T, ssh2::Error>;

pub struct Session(ssh2::Session);

impl std::ops::Deref for Session {
    type Target = ssh2::Session;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Session {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Session {
    pub fn new() -> Result<Session> {
        let s = ssh2::Session::new()?;
        Ok(Session(s))
    }

    pub fn connect<A: net::ToSocketAddrs>(&mut self, addr: A) -> io::Result<()> {
        self.set_tcp_stream(std::net::TcpStream::connect(addr)?);
        self.handshake()?;
        Ok(())
    }

    pub fn exec(&self, cmd: &str) -> io::Result<String> {
        let mut channel = self.channel_session()?;

        println!("------------- exec start -------------");
        println!("{cmd}");

        channel.exec(cmd)?;

        let mut s = String::new();
        let mut stderr = channel.stderr();

        stderr.read_to_string(&mut s)?;
        println!("{}", s);

        channel.read_to_string(&mut s)?;
        println!("{}", s);

        _ = channel.wait_close();

        println!("exit status: {}", channel.exit_status().unwrap());

        println!("------------- exec end -------------\n");

        Ok(s.to_string())

    }

    pub fn upload_file(&self, remote_path: &str, data: &[u8]) -> io::Result<()> {
        let mut channel = self.channel_session()?;
        channel.exec(format!("cat > {remote_path}").as_str())?;

        channel.write_all(data)?;
        channel.close()?;

        Ok(())
    }
}
