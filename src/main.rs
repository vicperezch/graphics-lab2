mod framebuffer;
mod line;

use framebuffer::Framebuffer;
use raylib::prelude::*;
use std::{thread, time::Duration};

const WIDTH: i32 = 200;
const HEIGHT: i32 = 200;
const CELL_SIZE: i32 = 3; // para mostrarlo más grande
const DELAY_MS: u64 = 100;

fn count_live_neighbors(grid: &Vec<Vec<bool>>, x: i32, y: i32) -> u8 {
    let mut count = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = x + dx;
            let ny = y + dy;
            if nx >= 0 && nx < WIDTH && ny >= 0 && ny < HEIGHT {
                if grid[ny as usize][nx as usize] {
                    count += 1;
                }
            }
        }
    }
    count
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH * CELL_SIZE, HEIGHT * CELL_SIZE)
        .title("Conway's Game of Life")
        .build();

    rl.set_target_fps(60);

    let mut fb = Framebuffer::new(WIDTH * CELL_SIZE, HEIGHT * CELL_SIZE, Color::BLACK);

    // Inicializar una grilla con valores aleatorios
    let mut grid = vec![vec![false; WIDTH as usize]; HEIGHT as usize];
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            grid[y as usize][x as usize] = rand::random::<bool>() && rand::random::<bool>(); // menos densidad
        }
    }

    while !rl.window_should_close() {
        // Generar la siguiente generación
        let mut next_grid = grid.clone();
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let live_neighbors = count_live_neighbors(&grid, x, y);
                let alive = grid[y as usize][x as usize];
                next_grid[y as usize][x as usize] = match (alive, live_neighbors) {
                    (true, 2) | (true, 3) => true,
                    (false, 3) => true,
                    _ => false,
                };
            }
        }

        // Actualizar la grilla
        grid = next_grid;

        // Limpiar el framebuffer
        fb.clear();

        // Dibujar las células
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if grid[y as usize][x as usize] {
                    fb.set_current_color(Color::WHITE);
                    // Pintar una celda más grande si CELL_SIZE > 1
                    for dy in 0..CELL_SIZE {
                        for dx in 0..CELL_SIZE {
                            fb.set_pixel(x * CELL_SIZE + dx, y * CELL_SIZE + dy);
                        }
                    }
                }
            }
        }

        // Mostrarlo en la ventana
        fb.swap_buffers(&mut rl, &thread);

        // Delay para ver la animación
        thread::sleep(Duration::from_millis(DELAY_MS));
    }
}

