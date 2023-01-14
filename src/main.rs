#![allow(
    dead_code,
    unused_imports,
    unused_variables,
    unused_attributes,
    unused_mut
)]

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{TextureCreator, WindowCanvas};
use sdl2::sys::KeyCode;
use sdl2::video::WindowContext;

use std::path::Path;
use std::time::Duration;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

fn render(
    canvas: &mut WindowCanvas,
    texture_creator: &TextureCreator<WindowContext>,
    font: &sdl2::ttf::Font,
) -> Result<(), String> {
    let color = Color::RGB(0, 0, 0);
    canvas.set_draw_color(color);
    canvas.clear(); // limpa o novo frame com a cor setada

    let text = "Hello".to_string();
    
    let surface = font // colocando o texto em uma superficie
        .render(&text)
        .blended(Color::WHITE)
        .map_err(|e| e.to_string())?;
    
        let texture = texture_creator//colocando a superficie em uma textura
        .create_texture_from_surface(surface)
        .map_err(|e| e.to_string())?;

    let target = Rect::new(10 as i32, 0 as i32, 200 as u32, 100 as u32);
    canvas.copy(&texture, None, Some(target))?; //adicionando textura ao canvas

    canvas.present(); //chama o novo frame
    Ok(())
}

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
    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Falha ao iniciar canvas");

    let texture_creator = canvas.texture_creator();

    // Preparar fontes
    let ttf_ctx = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font_path: &Path =
        Path::new(&"/home/nailsonchagas/Projetos/asteroides_sdl2/assets/fonts/OpenSans-Bold.ttf");
    let mut font = ttf_ctx.load_font(font_path, 128)?;
    font.set_style(sdl2::ttf::FontStyle::BOLD);

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

        render(&mut canvas, &texture_creator, &font)?;
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60)); // tempo entre cada iteração
    }
    println!("... Encerrando ...");
    Ok(()) //Executou corretamente
}
