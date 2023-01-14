#![allow(dead_code, unused_imports, unused_variables, unused_attributes, unused_mut)]

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::sys::KeyCode;

use std::time::Duration;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

/*
Retornando Result pode ser uma boa idéia para informar
o SO quando o programa não foi executado corretamente
*/
fn main() -> Result<(), String> {
    println!("... Iniciando ...");

    let sdl_ctx = sdl2::init()?; //inicializa a biblioteca SDL
    let video_subsystem = sdl_ctx.video()?; //iniciando substistema de video

    let window = video_subsystem
        .window("Asteroides Clone", WIDTH, HEIGHT) // Criando a janela
        .position_centered()
        .build()
        .expect("Não foi possivel criar a janela");
    let mut canvas = window.into_canvas()
        .build()
        .expect("Falha ao iniciar canvas");
    
    let texture_creator = canvas.texture_creator();

    // Preparar fontes
    let ttf_ctx = sdl2::ttf::init()
        .map_err(|e| e.to_string())?;

    // lidando com evento e main loop
    let mut event_pump = sdl_ctx.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    //evendo de clicar para fechar a janela
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {} //default
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60)); // tempo entre cada iteração
    }

    println!("... Encerrando ...");
    Ok(()) //Executou corretamente
}
