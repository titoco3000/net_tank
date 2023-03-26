use std::{net::SocketAddr};
use std::ops::{Index, IndexMut};

use crate::input_jogador::InputJogador;
use crate::jogador::DadosJogador;
/*
Wrapper sobre vetor de jogadores, para poder ter funções-membro
*/
pub struct Jogadores(Vec<DadosJogador>);

impl Jogadores {
    pub fn with_capacity(capacity:usize)->Jogadores{
        Jogadores(Vec::with_capacity(capacity))
    }
    pub fn push(&mut self, j:DadosJogador){
        self.0.push(j);
    }
    pub fn jogador_com_addr(&mut self,addr:&SocketAddr)->Option<&mut DadosJogador>{
        for j in &mut self.0{
            if j.addr == *addr{
                return Some(&mut *j);
            }
        }
        None
    }
    //se não tiver jogador com esse addr, adiciona; se não, atualiza os valores de nome e input
    pub fn add_or_update(&mut self, addr:SocketAddr, nome:&str, input:InputJogador){
        if let Some(j) = self.jogador_com_addr(&addr) {
            j.nome = nome.to_string();
            j.input = input;
            if let Some(timeout) = &mut j.timeout{
                timeout.reset();
            }
        }
        else{
            self.0.push(DadosJogador::new(nome.to_string(),addr, true));
        }
    }
    pub fn iter_mut(&mut self)->impl Iterator<Item=&mut DadosJogador>{
        self.0.iter_mut()
    }
    pub fn len(&self)->usize{
        self.0.len()
    }
    //remove item em O(1)
    pub fn swap_remove(&mut self, i:usize){
        self.0.swap_remove(i);
    }
}

//permitem indexação
impl Index<usize> for Jogadores {
    type Output = DadosJogador;
    fn index<'a>(&'a self, i: usize) -> &'a DadosJogador {
        &self.0[i]
    }
}
impl IndexMut<usize> for Jogadores {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut DadosJogador {
        &mut self.0[i]
    }
}