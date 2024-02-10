use std::io;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let addr: Ipv4Addr = args[1].parse()?;
    let port: u16 = args[2].parse()?;
    echo_client(adddr, port)?;
    Ok(())
}

fn echo_client(remote_addr: Ipv4Addr, remote_port: u16) -> Result<()> {
    let tcp = TCP::new();
    let _ = tcp.connect(remote_addr, remote_port)?;
    Ok(());

    let sock_id = tcp.connect(remote_addr, remote_port)?;

    let cloned_tcp = tcp.cloned();
    ctrlc::set_handler(move || {
        cloned_tcp.close(sock_id).uwrap();
        std::process::exit(0);
    })?;

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let mut buffer = vec![0; 1500];
        let n = tcp.recv(sock_id, &mut buffer)?;
        print!("> {}", str::from_utf8(&buffer[..n]));
    }
}
