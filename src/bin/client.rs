use std::{ net::{SocketAddr, UdpSocket}, time::Duration};
use macroquad::prelude::*;
use rusty_time::timer::Timer;
use std::io::Write; // flush

//modulos locais
use net_tank::{input_jogador::InputJogador};

struct ConexaoServidor{
    apelido:String,
    socket:UdpSocket,
    server_addr: SocketAddr,
    timer: Timer,
}

impl ConexaoServidor {
    pub fn enviar(&self,msg:&String){
        let msg = self.apelido.clone()+";"+msg;
        self.enviar_raw(msg.as_bytes());
    }
    pub fn enviar_raw(&self, msg:&[u8]){
        self.socket.send_to(msg, &self.server_addr).expect("falha ao enviar dados ao servidor");
    }
}

fn hide_console_window() {
    use std::ptr;
    use winapi::um::wincon::GetConsoleWindow;
    use winapi::um::winuser::{ShowWindow, SW_HIDE};

    let window = unsafe {GetConsoleWindow()};
    // https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow
    if window != ptr::null_mut() {
        unsafe {
            ShowWindow(window, SW_HIDE);
        }
    }
}

#[macroquad::main("Cliente NetTank")]
async fn main() {
    let mut apelido = None;
    let mut server_addr: Option<SocketAddr> = None;

    let mut args = std::env::args();
    let mut it = args.next();

    // avalia args
    while let Some(arg) = it  {
        if arg.trim() == "-n"{
            it = args.next();
            if let Some(arg) = it{
                if arg.trim().len() > 20{
                    println!("Grande demais, escolha um nome menor");
                }
                else{
                    apelido = Some(arg.trim().to_string());
                }
            }
        }
        if arg.trim() == "-a"{
            it = args.next();
            if let Some(arg) = it{
                if let Ok(addr) = arg.trim().parse() {
                    server_addr=Some(addr);
                }
            }
        }
        it = args.next();
    }

    //abre um socket na maquina
    let socket = net_tank::open_some_port().expect("Falha ao abrir porta");

    //se não obteve endereço do servidor como arg, pede ele
    if let None = server_addr{
        loop{
            let mut line = String::new();
            print!("Endereço: ");
            std::io::stdout().flush().unwrap(); //força impressão
            std::io::stdin().read_line(&mut line).unwrap();
            //tenta traduzir endereço digitado
            if let Ok(addr) = line.trim().parse() {
                server_addr=Some(addr);
                break;
            }
            else{
                println!("Incorreto, tente novamente.");
            }
        }
    }
    
    //se não obteve nome do jogador como arg, pede ele
    if let None = apelido{
        let mut texto = String::new();
        loop{
            print!("Apelido: ");
            std::io::stdout().flush().unwrap(); //force flush
            std::io::stdin().read_line(&mut texto).unwrap();
            texto.truncate(texto.trim_end().len());
            //limita tamanho do nome
            if texto.chars().count() > 20{
                println!("Grande demais, escolha um nome com menos de 20 caracteres");
            }
            else{
                apelido = Some(texto);
                break;
            }
        }

    }
    
    let mut conexao = ConexaoServidor{
        apelido: apelido.unwrap(),
        socket,
        server_addr:server_addr.unwrap(),
        timer: Timer::from_millis(1000/30) //timer para max de 30 envios/seg
    };
    let mut input = InputJogador::new();

    //fecha o console
    hide_console_window();

    loop {
        //atualiza o timer que limita envios
        conexao.timer.update(Duration::from_secs_f32(get_frame_time()));

        //se o timer permitir
        if conexao.timer.ready {
            conexao.timer.reset();
            //busca input do usuario
            input.bool_input( is_key_down(KeyCode::W),  is_key_down(KeyCode::A),  is_key_down(KeyCode::S),  is_key_down(KeyCode::D),  is_key_down(KeyCode::Space));
            //envia input
            conexao.enviar(&input.0.to_string());
        }  
        //limpa a tela
        clear_background(RED);

        //calcula tamanho
        let pixels_arena = screen_width().min(screen_height());
        
        //desenha raw do input
        draw_text(&input.0.to_string(), 0.1*pixels_arena, 0.1*pixels_arena, 0.1*pixels_arena, BLACK);

        //desenha preenchimentos
        if is_key_down(KeyCode::W){
            draw_triangle(
                Vec2 { x: 0.5, y: 0.1 } *pixels_arena,
                Vec2 { x: 0.3, y: 0.3 } *pixels_arena,
                Vec2 { x: 0.7, y: 0.3 } *pixels_arena,
            GREEN);
        }
        if is_key_down(KeyCode::A){
            draw_triangle(
                Vec2 { x: 0.3, y: 0.3 } *pixels_arena,
                Vec2 { x: 0.1, y: 0.5 } *pixels_arena,
                Vec2 { x: 0.3, y: 0.7 } *pixels_arena,
            GREEN);
        }
        if is_key_down(KeyCode::S){
            draw_triangle(
                Vec2 { x: 0.5, y: 0.9 } *pixels_arena,
                Vec2 { x: 0.3, y: 0.7 } *pixels_arena,
                Vec2 { x: 0.7, y: 0.7 } *pixels_arena,
            GREEN);
        }
        if is_key_down(KeyCode::D){
            draw_triangle(
                Vec2 { x: 0.7, y: 0.3 } *pixels_arena,
                Vec2 { x: 0.9, y: 0.5 } *pixels_arena,
                Vec2 { x: 0.7, y: 0.7 } *pixels_arena,
            GREEN);
        }
        if is_key_down(KeyCode::Space){
            draw_rectangle(0.3*pixels_arena,0.3*pixels_arena,0.4*pixels_arena,0.4*pixels_arena,GREEN);
        }

        //desenha molduras
        draw_triangle_lines(
            Vec2 { x: 0.5, y: 0.1 } *pixels_arena,
            Vec2 { x: 0.3, y: 0.3 } *pixels_arena,
            Vec2 { x: 0.7, y: 0.3 } *pixels_arena,
        0.01*pixels_arena, BLACK);

        draw_triangle_lines(
            Vec2 { x: 0.3, y: 0.3 } *pixels_arena,
            Vec2 { x: 0.1, y: 0.5 } *pixels_arena,
            Vec2 { x: 0.3, y: 0.7 } *pixels_arena,
        0.01*pixels_arena, BLACK);
        
        draw_triangle_lines(
            Vec2 { x: 0.5, y: 0.9 } *pixels_arena,
            Vec2 { x: 0.3, y: 0.7 } *pixels_arena,
            Vec2 { x: 0.7, y: 0.7 } *pixels_arena,
        0.01*pixels_arena, BLACK);
        
        draw_triangle_lines(
            Vec2 { x: 0.7, y: 0.3 } *pixels_arena,
            Vec2 { x: 0.9, y: 0.5 } *pixels_arena,
            Vec2 { x: 0.7, y: 0.7 } *pixels_arena,
        0.01*pixels_arena, BLACK);

        next_frame().await
    }
}