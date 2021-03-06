#![cfg(all(feature = "os-poll", feature = "tcp"))]

use mio::net::TcpSocket;

#[test]
fn is_send_and_sync() {
    fn is_send<T: Send>() {}
    fn is_sync<T: Sync>() {}

    is_send::<TcpSocket>();
    is_sync::<TcpSocket>();
}

#[test]
fn set_reuseaddr() {
    let addr = "127.0.0.1:0".parse().unwrap();

    let socket = TcpSocket::new_v4().unwrap();
    socket.set_reuseaddr(true).unwrap();
    assert!(socket.get_reuseaddr().unwrap());

    socket.bind(addr).unwrap();

    let _ = socket.listen(128).unwrap();
}

#[cfg(all(unix, not(any(target_os = "solaris", target_os = "illumos"))))]
#[test]
fn set_reuseport() {
    let addr = "127.0.0.1:0".parse().unwrap();

    let socket = TcpSocket::new_v4().unwrap();
    socket.set_reuseport(true).unwrap();
    assert!(socket.get_reuseport().unwrap());

    socket.bind(addr).unwrap();

    let _ = socket.listen(128).unwrap();
}
