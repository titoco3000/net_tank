use std::net::SocketAddr;
/*
Armazena dados do tiro
*/
pub struct Tiro{
    pub origem:SocketAddr,
    pub x:f32,      //entre 0.0 e 1.0
    pub y:f32,      //entre 0.0 e 1.0
    pub dir:f32,    //em radianos
}
