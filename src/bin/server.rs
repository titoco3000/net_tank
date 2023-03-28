//impede que o executavel abra em um terminal
#![windows_subsystem = "windows"]

use std::{f32::consts::PI, thread, sync::{Arc, Mutex}, path::Path};
use net_tank::{constantes::*, jogadores::{Jogadores}, tiro::Tiro, jogador::DadosJogador, input_jogador::InputJogador};
use macroquad::prelude::*;

//retorna qual o tamanho do quadrado na qual cabe um quadrado rotacionado
//usado para calculos de colisão
fn proporcao_quadrado_externo(angulo:f32)->f32{
     angulo.sin().abs()+angulo.cos().abs()
}

#[macroquad::main("Servidor NetTank")]
async fn main() {
    let mut rng = ::rand::thread_rng();
    //esses if é só para se desviar de problemas com o borrow checker do rust
    if let Some(path_to_assets) = &std::env::args().next() {
        let path_to_assets = Path::new(path_to_assets).parent().unwrap();
        
        let game_font = load_ttf_font(path_to_assets.join("assets\\fonts\\Fira_Sans\\FiraSans-Bold.ttf").to_str().unwrap()).await.unwrap();
        
        //abre socket
        let socket = net_tank::open_some_port().expect("Falha ao abrir porta");
        let local_addr_str:String = socket.local_addr().unwrap().to_string();
    
        let mut tiros = Vec::<Tiro>::with_capacity(100);
        let mut jogadores = Jogadores::with_capacity(20);
        let mut placar:Vec<(String,u32)> = Vec::with_capacity(20);

        //adiciona o jogador local, "admin"
        jogadores.push(DadosJogador::new("admin".to_string(),socket.local_addr().unwrap(),false));
        
        //MutexGuard para usar o obj jogadores em duas threads
        let jogadores =  Arc::new(Mutex::new(jogadores));
        let jogadores_na_thread = Arc::clone(&jogadores);
    
        thread::spawn(move || {
            //aloca buffer
            let mut buf = [0; 100];
            loop{
                //lê do socket (se for mais de 100 bytes, ignora o final)
                let (_amount, src) = socket.recv_from(&mut buf).expect("Erro ao aguardar dados");
                
                //tenta transformar bytes em String
                match String::from_utf8(buf.to_vec()) {
                    Ok(s) => {
                        //separa nome e valor
                        let v:Vec<&str> = s.trim_end_matches('\0').split(';').collect();
                        //tenta obter valor do input
                        if let Ok(val) = v[1].parse() {
                            //se obteve sucesso, pede acesso ao valor protegido pela mutex
                            (*jogadores_na_thread).lock().unwrap().add_or_update(src, v[0], InputJogador(val));
                        }
                    },
                    Err(e) => println!("Ignorando sequencia UTF-8 inválida: {}", e),
                };
                buf.fill(0);
            }
        });
        
        loop {
            //esses valores de tamanho (em pixels) são calculados no loop por causa de redimensionamento da janela

            //calcula tamanho da arena
            let pixels_arena = screen_width().min(screen_height()-ALTURA_MIN_HEADER);
            //calcula tamanho do header (que exibe o IpAddr)
            let header_size = screen_height()-pixels_arena;
            //tamanhos do jogador
            let tamanho_jogador = pixels_arena*PROPORCAO_JOGADOR;
            let comprimento_canhao = tamanho_jogador*0.8;
            let largura_canhao = tamanho_jogador*0.2;
            
            //limpa a tela
            clear_background(RED);
            //escreve endereço no topo
            draw_text(&local_addr_str, 10.0, (header_size) * 0.8, header_size, WHITE);
            //escreve fps
            draw_text(&get_fps().to_string(), 300.0, (header_size) * 0.8, header_size, WHITE);
            //desenha a arena
            draw_rectangle_lines(0.0, header_size, pixels_arena, pixels_arena, 1.0, BLACK);
            
            //pede acesso ao obj de jogadores
            match jogadores.lock(){
                Ok(mut jogadores) => {
                    //caso tenha obtido o acesso

                    //realiza input do jogador local
                    jogadores[0].input.bool_input( is_key_down(KeyCode::W),  is_key_down(KeyCode::A),  is_key_down(KeyCode::S),  is_key_down(KeyCode::D),  is_key_down(KeyCode::Space));
                    
                    //limpa o placar
                    placar.truncate(0
                    );
                    // para cada jogador
                    for i in (0..jogadores.len()).rev(){
                        //atualiza valores insternos do jogador e passa vetor de tiros para ele adicionar, se precisar
                        jogadores[i].update(get_frame_time(), &mut tiros);

                        //retira jogadores que estejam desconectados
                        if let Some(timeout) = &mut jogadores[i].timeout {
                            if timeout.ready{
                                jogadores.swap_remove(i);
                                continue; //passa para proxima iteração
                            }
                        }

                        //adiciona esse jogador no placar
                        placar.push((jogadores[i].nome.clone(), jogadores[i].pontos));


                        if jogadores[i].vivo(){
                            //se estiver visivel, desenha o jogador
                            if jogadores[i].visivel(){
                                //corpo
                                draw_line(
                                    jogadores[i].x*pixels_arena-jogadores[i].dir.cos()*tamanho_jogador*0.5, 
                                    header_size + jogadores[i].y*pixels_arena-jogadores[i].dir.sin()*tamanho_jogador*0.5, 
                                    jogadores[i].x*pixels_arena+jogadores[i].dir.cos()*tamanho_jogador*0.5, 
                                    header_size + jogadores[i].y*pixels_arena+jogadores[i].dir.sin()*tamanho_jogador*0.5, 
                                tamanho_jogador, BLUE);
                                //canhão
                                draw_line(
                                    jogadores[i].x*pixels_arena, 
                                    header_size + jogadores[i].y*pixels_arena, 
                                    jogadores[i].x*pixels_arena+jogadores[i].dir.cos()*comprimento_canhao, 
                                    header_size + jogadores[i].y*pixels_arena+jogadores[i].dir.sin()*comprimento_canhao, 
                                largura_canhao, GRAY);
                                //nome
                                let font_scale = PROPORCAO_JOGADOR * pixels_arena * 0.01;
                                //precisa das medidas para centralizar
                                let text_measures = measure_text(&jogadores[i].nome, Some(game_font), 22, font_scale);
                                draw_text_ex(
                                    &jogadores[i].nome, 
                                    jogadores[i].x*pixels_arena - text_measures.width*0.5, 
                                    header_size + (jogadores[i].y-PROPORCAO_JOGADOR*0.8)*pixels_arena,
                                    TextParams { 
                                        font: game_font, 
                                        font_size: 22, 
                                        font_scale, 
                                        font_scale_aspect: 1.0, 
                                        rotation: 0.0, 
                                        color: WHITE }
                                );
                            }
                            //para cada tiro
                            for j in (0..tiros.len()).rev(){
                                //se o tiro não pertence ao jogador e o jogador está alvejavel
                                if jogadores[i].addr != tiros[j].origem && jogadores[i].alvejavel(){
                                    //rotação que aponta do tiro ao jogador
                                    let rel_dir = f32::atan2(tiros[j].y - jogadores[i].y, jogadores[i].x-tiros[j].x);
                                    
                                    let mut rots_somadas = rel_dir + jogadores[i].dir;
                                    // normaliza o angulo para ficar entre 0º-45º em relação ao plano virado para o tiro
                                    while rots_somadas < PI*0.5 {
                                        rots_somadas += PI*0.5;
                                    }
                                    while rots_somadas > PI*0.5 {
                                        rots_somadas -= PI*0.5;
                                    }
                                    if rots_somadas > PI*0.25{
                                        rots_somadas = PI*0.25 - ((rots_somadas-PI*0.25).abs());
                                    }

                                    //tamanho do quadrado externo ao tanque
                                    let quad_ext = proporcao_quadrado_externo(rots_somadas);
                                    let y = 0.7071067811865476; //meia diagonal de um quadrado de tamanho 1.0
                                    let z = quad_ext*0.5;
                                    let w = ((y*y)-(z*z)).sqrt(); //distancia do vertice do quadrado interno ao centro da aresta do quadrado externo
                                    //distancia do centro da aresta do quadrado externo até o interno
                                    let x = rots_somadas.tan()* w;
                                    //distancia do tiro até o quadrado interno
                                    let d = ((tiros[j].x-jogadores[i].x).powi(2) + (tiros[j].y-jogadores[i].y).powi(2)).sqrt() - PROPORCAO_TIRO*1.2; //leve exagero no tamanho do tiro
                                    //raio da hitbox alinhada
                                    let raio = (quad_ext*0.5 -x) * PROPORCAO_JOGADOR;
                                    
                                    if d < raio{
                                        if let Some(j) = &mut jogadores.jogador_com_addr(&tiros[j].origem){
                                            j.pontos+=1;
                                        }
                                        tiros.swap_remove(j);
                                        jogadores[i].matar(&mut rng);
                                    }
                                    //descomente esse bloco para linhas de depuração
                                    /*
                                    draw_line(
                                        tiros[j].x*pixels_arena,
                                        tiros[j].y*pixels_arena + header_size,
                                        (tiros[j].x + rel_dir.cos()*100.0)*pixels_arena,
                                        (tiros[j].y - rel_dir.sin()*100.0)*pixels_arena + header_size,
                                        2.0,
                                        YELLOW
                                    );
                                    draw_circle_lines(
                                        jogadores[i].x * pixels_arena,
                                        jogadores[i].y * pixels_arena + header_size,
                                        raio * pixels_arena,
                                        2.0, YELLOW
                                    );
                                     */
                                }
                            }
                        }
                    }
                }
                Err(e) => println!("Mutex falhou por :\"{e}\"")
            }
            
            //ordena o placar
            placar.sort_unstable_by(|a, b| b.1.cmp(&a.1));
            //escreve o placar na tela
            for (i, item) in placar.iter().enumerate(){
                draw_text(&format!("{}|{}",item.1,item.0), pixels_arena + 4.0, header_size * (i+1) as f32 , header_size, WHITE);
            }
            
            //para cada tiro
            for i in (0..tiros.len()).rev(){
                //move o tiro
                tiros[i].x+=VELOCIDADE_TIRO*get_frame_time()*tiros[i].dir.cos();
                tiros[i].y+=VELOCIDADE_TIRO*get_frame_time()*tiros[i].dir.sin();
                
                //verifica se o tiro saiu da arena
                if tiros[i].x < 0.0 || tiros[i].x > 1.0 || tiros[i].y < 0.0 || tiros[i].y > 1.0{
                    tiros.swap_remove(i);
                }
                else{
                    //desenha o tiro
                    draw_circle(
                        tiros[i].x*pixels_arena,
                        header_size + tiros[i].y*pixels_arena,
                        PROPORCAO_TIRO* pixels_arena,
                    BLACK);
    
                }
            }
            next_frame().await
        }
    }
}