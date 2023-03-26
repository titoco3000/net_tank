
use std::{net::SocketAddr,time::Duration};
use crate::constantes::*;
use crate::input_jogador::InputJogador;
use crate::tiro::Tiro;
use rusty_time::timer::Timer;
use std::f32::consts::PI;

/*
Armazena dados do jogador
*/
pub struct DadosJogador{
    pub nome:String,
    pub addr:SocketAddr,
    pub input:InputJogador,
    pub x:f32,      //entre 0.0 e 1.0
    pub y:f32,      //entre 0.0 e 1.0
    pub dir:f32, //em radianos
    pub impulso:f32,
    pub timer_tiro:Timer,
    pub timeout:Option<Timer>,
    pub pontos: u32,
    timer_morte:Timer,
    timer_inalvejavel:Timer,
    timer_invisivel: Timer,
    visivel:bool,
}
impl DadosJogador {
    pub fn new(nome:String,addr:SocketAddr, timeout:bool)->DadosJogador{
        DadosJogador{
            nome,
            addr,
            input:InputJogador::new(),
            x:0.5,
            y:0.5,
            dir:0.0,
            impulso:0.0,
            timer_tiro:Timer::from_millis(INTERVALO_TIROS),
            timeout: if timeout{Some(Timer::from_millis(TIMEOUT_JOGADOR))} else{None},
            pontos:0,
            timer_morte: Timer { duration: Duration::from_millis(TEMPO_MORTE), time_left: Duration::from_millis(0), ready: true },
            timer_inalvejavel: Timer { duration: Duration::from_millis(TEMPO_INVULNERAVEL), time_left: Duration::from_millis(0), ready: true },
            timer_invisivel: Timer { duration: Duration::from_millis(INTERVALO_PISCADA), time_left: Duration::from_millis(0), ready: true },
            visivel:true,
        }
    }
    pub fn update(&mut self, delta_time:f32, vetor_tiros:&mut Vec<Tiro>){
        self.timer_tiro.update(Duration::from_secs_f32(delta_time));
        if let Some(timeout) = &mut self.timeout {
            timeout.update(Duration::from_secs_f32(delta_time));
        }
        
        //se está vivo
        if self.timer_morte.ready{
            self.timer_inalvejavel.update(Duration::from_secs_f32(delta_time));
            
            //se esta alvejavel
            if self.alvejavel(){
               self.visivel = true; 
            }
            else{
                self.timer_invisivel.update(Duration::from_secs_f32(delta_time));
                if self.timer_invisivel.ready{
                    self.timer_invisivel.reset();
                    self.visivel = !self.visivel;
                }
            }

            //adiciona rotação
            if self.input.left(){
                self.dir-=VELOCIDADE_ROTACAO * delta_time;
            }
            if self.input.right(){
                self.dir+=VELOCIDADE_ROTACAO * delta_time;
            }
    
            //mantém a rotação entre 0.0 e 2*PI rad
            while self.dir < 0.0{
                self.dir+=PI *2.0;
            }
            while self.dir > PI*2.0{
                self.dir-=PI *2.0;
            }
    
            //adiciona ou retira aceleração
            if self.input.up(){
                self.impulso+=ACELERACAO*delta_time;
            }
            if self.input.down(){
                self.impulso-=ACELERACAO*delta_time;
            }
            //se não está acelerando nem pra frente nem pra tras, desacelera
            else if !self.input.up() {
                let fator = DESACELERACAO.min(self.impulso.abs()*delta_time);
                if self.impulso<0.0{
                    self.impulso+=fator;
                }
                else{
                    self.impulso-=fator;
                }
            }
            //limita o impulso
            self.impulso=self.impulso.clamp(-VELOCIDADE, VELOCIDADE);
    
            // aplica o impulso que tem atualmente
            self.x+=self.dir.cos()*self.impulso*delta_time;
            self.y+=self.dir.sin()*self.impulso*delta_time;
            
            let largura = (self.dir.sin().abs() + self.dir.cos().abs())*PROPORCAO_JOGADOR;
            //verifica colisão com as paredes
            self.x = self.x.clamp(largura*0.5, 1.0-largura*0.5);
            self.y = self.y.clamp(largura*0.5, 1.0-largura*0.5);
    
            //se puder e quiser, atirar
            if self.input.action() && self.timer_tiro.ready{
                vetor_tiros.push(Tiro{
                    origem:self.addr,
                    x: self.x + self.dir.cos() * DISTANCIA_INICIAL_TIRO,
                    y: self.y + self.dir.sin() * DISTANCIA_INICIAL_TIRO,
                    dir:self.dir //copia a propria direção para o tiro
                });
                self.timer_tiro.reset();
            }
        }
        else{
            //atualiza o timer morto
            self.timer_morte.update(Duration::from_secs_f32(delta_time));
        }
        

    }

    pub fn matar(&mut self){
        self.impulso = 0.0;
        self.dir = 0.0;
        self.timer_morte.reset();
        self.timer_inalvejavel.reset();
        self.timer_invisivel.reset();
        self.pontos /= 2;
        self.x = 0.5;
        self.y = 0.5;
    }

    //booleanos de consulta
    pub fn alvejavel(&self)->bool{self.timer_inalvejavel.ready}
    pub fn vivo(&self)->bool{self.timer_morte.ready}
    pub fn visivel(&self)->bool{self.visivel}
}
