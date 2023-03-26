use std::net::UdpSocket;
use local_ip_address::local_ip;

pub mod constantes;
pub mod tiro;
pub mod input_jogador;
pub mod jogador;
pub mod jogadores;

//busca uma porta livre e retorna um socket udp nela
pub fn open_some_port()->Option<UdpSocket>{
    for port in 1025..65535 {
        match UdpSocket::bind((local_ip().unwrap(), port)) {
            Ok(l) => return Some(l),
            _ => {}
        }
    }
    None
}