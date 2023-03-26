/*
Lida com entrada do jogador, controlado por WASD + espaço
cada um desses 5 dados ocupa um bit de um byte
*/

pub struct InputJogador(pub u8);

impl InputJogador {
    pub fn new()->Self{
        InputJogador(0)
    }
    
    pub fn bool_input(&mut self, w:bool,a:bool,s:bool,d:bool,spc:bool){
        self.0 = 0;
        if w {
            self.0 |= 16;
        }
        if a {
            self.0 |= 8;
        }
        if s {
            self.0 |= 4;
        }
        if d {
            self.0 |= 2;
        }
        if spc {
            self.0 |= 1;
        }
    }

    pub fn up(&self)->bool{
        ((self.0 >> 4)&1) !=0
    }
    pub fn left(&self)->bool{
        ((self.0 >> 3)&1) !=0
    }
    pub fn down(&self)->bool{
        ((self.0 >> 2)&1) !=0
    }
    pub fn right(&self)->bool{
        ((self.0 >> 1)&1) !=0
    }
    pub fn action(&self)->bool{
        (self.0&1) !=0
    }
}