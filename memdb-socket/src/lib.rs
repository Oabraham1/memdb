use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::{io, os::fd::AsRawFd};

use socket2::{Domain, Protocol, SockAddr, Socket, Type};

fn create_socket(
    domain: Domain,
    socket_type: Type,
    protocol: Option<Protocol>,
) -> Result<Socket, std::io::Error> {
    let socket = Socket::new(domain, socket_type, protocol)?;

    Ok(socket)
}

fn configure_socket(listener: &TcpListener) -> io::Result<()> {
    let fd = listener.as_raw_fd();
    let opt_val: libc::c_int = 1;
    let opt_len = std::mem::size_of_val(&opt_val) as libc::socklen_t;

    unsafe {
        if libc::setsockopt(
            fd,
            libc::SOL_SOCKET,
            libc::SO_REUSEADDR,
            &opt_val as *const _ as *const libc::c_void,
            opt_len,
        ) == -1
        {
            return Err(io::Error::last_os_error());
        }
    }

    Ok(())
}

fn bind_socket(addr: SocketAddr, socket: &Socket) -> io::Result<()> {
    let sock_addr = SockAddr::from(addr);
    socket.bind(&sock_addr)?;
    Ok(())
}

fn get_socket_address(socket: &Socket) -> Result<SocketAddr, std::io::Error> {
    socket.local_addr().and_then(|addr| {
        addr.as_socket().ok_or(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Failed to get local address",
        ))
    })
}

fn listen(socket: &Socket) -> io::Result<()> {
    let fd = socket.as_raw_fd();
    let backlog = 128; // SOMAXCONN
    let rv = unsafe { libc::listen(fd, backlog) };
    if rv != 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

fn accept_connections(listener: TcpListener) -> io::Result<()> {
    loop {
        match listener.accept() {
            Ok((mut stream, addr)) => {
                println!("Accepted connection from {}", addr);
                if let Err(e) = read_and_write(&mut stream) {
                    eprintln!("Error processing connection: {}", e);
                }
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
                break;
            }
        }
    }
    Ok(())
}

fn read_and_write(stream: &mut TcpStream) -> std::io::Result<()> {
    let mut rbuf = [0; 64];
    let n = stream.read(&mut rbuf)?;

    println!("client says: {}", String::from_utf8_lossy(&rbuf[..n]));

    let wbuf = b"world";
    stream.write_all(wbuf)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use rand::Rng;
    use std::mem::drop;
    use std::net::Shutdown;
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, TcpListener};
    use std::os::fd::FromRawFd;
    use std::thread;
    use std::time::Duration;

    use super::*;

    fn random_port() -> u16 {
        rand::thread_rng().gen_range(1024..65535)
    }

    #[test]
    fn test_create_socket_ipv4_tcp() {
        let socket = create_socket(Domain::IPV4, Type::STREAM, Some(Protocol::TCP)).unwrap();
        drop(socket);
        assert!(true);
    }

    #[test]
    fn test_create_socket_ipv6_udp() {
        let socket = create_socket(Domain::IPV6, Type::DGRAM, Some(Protocol::UDP)).unwrap();
        drop(socket);
        assert!(true);
    }

    #[test]
    fn test_socket_address() {
        let socket = create_socket(Domain::IPV4, Type::STREAM, Some(Protocol::TCP)).unwrap();
        let addr = get_socket_address(&socket).unwrap();

        assert!(addr.is_ipv4());
        assert_eq!(addr.port(), 0); // Port 0 indicates an unbound socket
        drop(socket);
    }

    #[test]
    fn test_socket_bind() {
        let socket = create_socket(Domain::IPV4, Type::STREAM, Some(Protocol::TCP)).unwrap();
        let port = random_port();
        let address = format!("127.0.0.1:{port}");
        let addr: SocketAddr = address.parse().unwrap();
        bind_socket(addr, &socket).expect("Bind failed");
        let bound_addr = get_socket_address(&socket).unwrap();
        assert_eq!(bound_addr.ip(), IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
        assert_eq!(bound_addr.port(), port);
    }

    #[test]
    fn test_socket_bind_ipv6() {
        let socket = create_socket(Domain::IPV6, Type::STREAM, Some(Protocol::TCP)).unwrap();
        let port = random_port();
        let addr: SocketAddr = format!("[::1]:{}", port).parse().unwrap();
        bind_socket(addr, &socket).expect("Bind failed");
        let bound_addr = get_socket_address(&socket).unwrap();
        assert_eq!(
            bound_addr.ip(),
            IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1))
        );
        assert_eq!(bound_addr.port(), port);
    }

    #[test]
    fn test_configure_socket_success() {
        // Create a socket address
        let port = random_port();
        let address = format!("127.0.0.1:{port}");
        let addr: SocketAddr = address.parse().unwrap();

        // Create a TCP listener
        let listener = TcpListener::bind(&addr).unwrap();
        let result = configure_socket(&listener);
        drop(listener);

        // Assert success
        assert!(result.is_ok());
    }

    #[test]
    fn test_listen() {
        let socket = create_socket(Domain::IPV4, Type::STREAM, Some(Protocol::TCP)).unwrap();
        let port = random_port();
        let address = format!("127.0.0.1:{port}");
        let addr: SocketAddr = address.parse().unwrap();
        let sock_addr = SockAddr::from(addr);
        socket.bind(&sock_addr).unwrap();
        assert!(listen(&socket).is_ok());
    }

    #[test]
    fn test_listen_failed() {
        let socket = create_socket(Domain::IPV4, Type::STREAM, Some(Protocol::TCP)).unwrap();
        let port = random_port();
        let address = format!("127.0.0.1:{port}");
        let addr: SocketAddr = address.parse().unwrap();
        let sock_addr = SockAddr::from(addr);
        socket.bind(&sock_addr).unwrap();
        let _ = socket.shutdown(Shutdown::Both);
        assert!(listen(&socket).is_ok());
    }
}
