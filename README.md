# Asteroids Clone
Feito seguindo o tutorial nessa playlist -> https://www.youtube.com/playlist?list=PLFOS-Gn3aXROnSfl26esPExssd-rQw6jD.  
**Obs:** Feito com o objetivo de aprender alguns conceitos de SDL2.  

## Dependências usadas
- Rust v1.66.1
- SDL2 v0.35.2
- rand v0.8.5
- kira v0.7.1
- once_cell v1.17.0
- specs v0.18.0
- specs-derive v0.4.1

### Para que serve cada dependência?
- **Rust:** Linguagem de programação multiparadigma compilada desenvolvida pela Mozilla Research. É projetada para ser "segura, concorrente e prática", mas diferente de outras linguagens seguras, Rust não usa coletor de lixo. Possui suporte nativo ao WebAssembly.
- **SDL2:** Simple DirectMedia Layer (SDL) é uma biblioteca multiplataforma (Windows, Linux, macOS, Android, iOS...), escrita em C, para aplicações multimídia. Ela provê funções para gerenciar vídeo, áudio, controle, entre outras.
- **rand:** Utilidades para gerar números pseudo aleatórios.
- **kira:** Biblioteca usada na manipulação de áudio para jogos. Ele fornece parâmetros para ajustar suavemente as propriedades dos sons, um mixer flexível para aplicar efeitos ao áudio e um sistema de tempo para sincronizar com precisão os eventos de áudio.
- **once_cell:** Fornece dois novos tipos semelhantes a Cell, unsync::OnceCell e sync::OnceCell.
- **Specs:** Esta biblioteca fornece uma variante ECS projetada para execução paralela. É altamente flexível quando se trata de dados de componentes reais e da maneira como são armazenados e acessados.
- **specs-derive:** Implementa os macros #[derive(Component)], #[derive(Saveload)] e o atributo #[component] para **specs**.

## Preparando o ambiente de dev
- SDL: ```sudo apt-get install libsdl2-dev```  
- SDL_Image: ```sudo apt install libjpeg-dev libwebp-dev libtiff5-dev libsdl2-image-dev libsdl2-image-2.0-0 -y;```  
- SDL_Sound: ```sudo apt install libmikmod-dev libfishsound1-dev libsmpeg-dev liboggz2-dev libflac-dev libfluidsynth-dev libsdl2-mixer-dev libsdl2-mixer-2.0-0 -y;```  
- SDL_Text: ```sudo apt install libfreetype6-dev libsdl2-ttf-dev libsdl2-ttf-2.0-0 -y;```  