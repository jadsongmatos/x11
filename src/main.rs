use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::COPY_DEPTH_FROM_PARENT;
use std::error::Error;
use std::{thread, time};

fn main() -> Result<(), Box<dyn Error>> {

    let red: u8 = 0xAB;
    let green: u8 = 0xCD;
    let blue: u8 = 0xF0;

    let frame_duration = time::Duration::from_millis(1000 / 2); // Duração de um quadro para 30 FPS

    let (conn, screen_num) = x11rb::connect(None)?;
    let screen = &conn.setup().roots[screen_num];

    let win_id = conn.generate_id()?;
    conn.create_window(
        COPY_DEPTH_FROM_PARENT,
        win_id,
        screen.root,
        0,
        0,
        800,
        600,
        0,
        WindowClass::INPUT_OUTPUT,
        0,
        &CreateWindowAux::new().background_pixel(screen.white_pixel),
    )?;
    conn.map_window(win_id)?;
    conn.flush()?;

    let gc = conn.generate_id()?;
    conn.create_gc(
        gc,
        win_id,
        &CreateGCAux::new(),
    )?;

    let pixmap_id = conn.generate_id()?;
    conn.create_pixmap(
        screen.root_depth,
        pixmap_id,
        win_id,
        800,
        600,
    )?;

    let mut offscreen_buffer = vec![0; 800 * 600 * 4]; // Seu buffer de textura

    let mut cor = false;

    loop {
        let start = time::Instant::now(); // Marca o início do quadro

        for pixel in offscreen_buffer.chunks_exact_mut(4) {
            if cor {
                pixel[0] = blue;
                pixel[1] = green;
                pixel[2] = red;
            } else {
                pixel[0] = 0xFF;
                pixel[1] = 0xFF;
                pixel[2] = 0xFF;
            }
        }
        cor = !cor;

        // Desenha os dados do buffer no Pixmap
        conn.put_image(
            ImageFormat::Z_PIXMAP,
            pixmap_id,
            gc,
            800,
            600,
            0,
            0,
            0,
            screen.root_depth,
            &offscreen_buffer,
        )?;

        // Copia o Pixmap para a janela
        conn.copy_area(
            pixmap_id,
            win_id,
            gc,
            0,
            0,
            0,
            0,
            800,
            600,
        )?;

        conn.flush()?;

        let elapsed = start.elapsed(); // Calcula o tempo gasto na renderização
        if elapsed < frame_duration {
            // Se a renderização foi mais rápida do que a duração de um quadro, pausa o loop
            thread::sleep(frame_duration - elapsed);
        } else {
            println!("lento: {:?}",elapsed);
        }
    }
}
