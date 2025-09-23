use macroquad::prelude::*;
use ::rand::{Rng, thread_rng};

mod traits;
mod util;
mod organism {
    pub mod prey;
    pub mod predator;
}
mod sim;

use crate::organism::prey::{Prey, Species, Sex};
use crate::organism::predator::Predator;
use crate::sim::Simulation;
use crate::traits::Organismo;

#[macroquad::main("Simulación Presas/Depredador")]
async fn main() {
    // Población inicial, solo tengo vacas, cabras y conejos (los conejos se mueren con solo verlos)
    let mut animals: Vec<Box<dyn Organismo>> = Vec::new();
    animals.push(Box::new(Prey::new(1, Species::Cow, Sex::Female, 0.0005, 0.05)));
    animals.push(Box::new(Prey::new(2, Species::Goat, Sex::Female, 0.001, 0.05)));
    animals.push(Box::new(Prey::new(3, Species::Rabbit, Sex::Female, 0.002, 0.10)));
    animals.push(Box::new(Prey::new(7, Species::Cow, Sex::Female, 0.0005, 0.05)));
    animals.push(Box::new(Prey::new(8, Species::Goat, Sex::Female, 0.001, 0.05)));
    animals.push(Box::new(Prey::new(9, Species::Rabbit, Sex::Female, 0.002, 0.10)));
    animals.push(Box::new(Prey::new(10, Species::Cow, Sex::Female, 0.0005, 0.05)));
    animals.push(Box::new(Prey::new(11, Species::Goat, Sex::Female, 0.001, 0.05)));
    animals.push(Box::new(Prey::new(12, Species::Rabbit, Sex::Female, 0.002, 0.10)));
    animals.push(Box::new(Prey::new(13, Species::Cow, Sex::Female, 0.0005, 0.05)));
    animals.push(Box::new(Prey::new(14, Species::Goat, Sex::Female, 0.001, 0.05)));
    animals.push(Box::new(Prey::new(15, Species::Rabbit, Sex::Female, 0.002, 0.10)));



    animals.push(Box::new(Prey::new(4, Species::Cow, Sex::Male, 0.0005, 0.05)));
    animals.push(Box::new(Prey::new(5, Species::Goat, Sex::Male, 0.001, 0.05)));
    animals.push(Box::new(Prey::new(6, Species::Rabbit, Sex::Male, 0.002, 0.10)));

    
    let predator = Predator::new(
        /*min_reserve=*/10.0,
        /*opt_reserve=*/30.0,
        /*sacrifice_age_days=*/280,
        3000.0,
    );

    let mut sim = Simulation::new(animals, predator);

   

    
    let mut history_cow: Vec<usize> = Vec::new();
    let mut history_goat: Vec<usize> = Vec::new();
    let mut history_rabbit: Vec<usize> = Vec::new();

    let total_dias = 500;
    for _ in 0..total_dias {
        clear_background(LIGHTGRAY);
        sim.simulate_day();

        
        let mut cow_count = 0;
        let mut goat_count = 0;
        let mut rabbit_count = 0;
        for presa in sim.get_animals() {
            match presa.especie() {
                Some(Species::Cow) => cow_count += 1,
                Some(Species::Goat) => goat_count += 1,
                Some(Species::Rabbit) => rabbit_count += 1,
                _ => {}
            }
        }
        history_cow.push(cow_count);
        history_goat.push(goat_count);
        history_rabbit.push(rabbit_count);

        
        let graph_x0 = 60.0;
        let graph_y0 = screen_height() - 60.0;
        let graph_width = screen_width() - 120.0;
        let graph_height = screen_height() - 120.0;
        let max_y = history_cow.iter().chain(history_goat.iter()).chain(history_rabbit.iter()).max().cloned().unwrap_or(1) as f32;
        let scale_x = graph_width / total_dias as f32;
        let scale_y = if max_y > 0.0 { graph_height / max_y } else { 1.0 };

        
        draw_line(graph_x0, graph_y0, graph_x0 + graph_width, graph_y0, 2.0, BLACK);
        draw_line(graph_x0, graph_y0, graph_x0, graph_y0 - graph_height, 2.0, BLACK);

        
        for (i, &count) in history_cow.iter().enumerate().skip(1) {
            let x1 = graph_x0 + (i as f32 - 1.0) * scale_x;
            let y1 = graph_y0 - (history_cow[i - 1] as f32) * scale_y;
            let x2 = graph_x0 + (i as f32) * scale_x;
            let y2 = graph_y0 - (count as f32) * scale_y;
            draw_line(x1, y1, x2, y2, 2.0, BLUE);
        }
        for (i, &count) in history_goat.iter().enumerate().skip(1) {
            let x1 = graph_x0 + (i as f32 - 1.0) * scale_x;
            let y1 = graph_y0 - (history_goat[i - 1] as f32) * scale_y;
            let x2 = graph_x0 + (i as f32) * scale_x;
            let y2 = graph_y0 - (count as f32) * scale_y;
            draw_line(x1, y1, x2, y2, 2.0, ORANGE);
        }
        for (i, &count) in history_rabbit.iter().enumerate().skip(1) {
            let x1 = graph_x0 + (i as f32 - 1.0) * scale_x;
            let y1 = graph_y0 - (history_rabbit[i - 1] as f32) * scale_y;
            let x2 = graph_x0 + (i as f32) * scale_x;
            let y2 = graph_y0 - (count as f32) * scale_y;
            draw_line(x1, y1, x2, y2, 2.0, PURPLE);
        }

        
    draw_text(&format!("Cow: {}", cow_count), graph_x0 + graph_width - 100.0, graph_y0 - graph_height + 20.0, 24.0, BLUE);
    draw_text(&format!("Goat: {}", goat_count), graph_x0 + graph_width - 100.0, graph_y0 - graph_height + 50.0, 24.0, ORANGE);
    draw_text(&format!("Rabbit: {}", rabbit_count), graph_x0 + graph_width - 100.0, graph_y0 - graph_height + 80.0, 24.0, PURPLE);

        draw_text(
            &format!("Día: {}  |  Reserva: {:.1}", sim.get_day(), sim.get_predator().current_reserve()),
            20.0,
            30.0,
            24.0,
            BLACK,
        );

        next_frame().await;
    }

    
    loop {
        clear_background(LIGHTGRAY);
        draw_text("Simulación finalizada", 200.0, 200.0, 40.0, RED);
        
        let graph_x0 = 60.0;
        let graph_y0 = screen_height() - 60.0;
        let graph_width = screen_width() - 120.0;
        let graph_height = screen_height() - 120.0;
        let max_y = history_cow.iter().chain(history_goat.iter()).chain(history_rabbit.iter()).max().cloned().unwrap_or(1) as f32;
        let scale_x = graph_width / total_dias as f32;
        let scale_y = if max_y > 0.0 { graph_height / max_y } else { 1.0 };
        draw_line(graph_x0, graph_y0, graph_x0 + graph_width, graph_y0, 2.0, BLACK);
        draw_line(graph_x0, graph_y0, graph_x0, graph_y0 - graph_height, 2.0, BLACK);
        for (i, &count) in history_cow.iter().enumerate().skip(1) {
            let x1 = graph_x0 + (i as f32 - 1.0) * scale_x;
            let y1 = graph_y0 - (history_cow[i - 1] as f32) * scale_y;
            let x2 = graph_x0 + (i as f32) * scale_x;
            let y2 = graph_y0 - (count as f32) * scale_y;
            draw_line(x1, y1, x2, y2, 2.0, BLUE);
        }
        for (i, &count) in history_goat.iter().enumerate().skip(1) {
            let x1 = graph_x0 + (i as f32 - 1.0) * scale_x;
            let y1 = graph_y0 - (history_goat[i - 1] as f32) * scale_y;
            let x2 = graph_x0 + (i as f32) * scale_x;
            let y2 = graph_y0 - (count as f32) * scale_y;
            draw_line(x1, y1, x2, y2, 2.0, ORANGE);
        }
        for (i, &count) in history_rabbit.iter().enumerate().skip(1) {
            let x1 = graph_x0 + (i as f32 - 1.0) * scale_x;
            let y1 = graph_y0 - (history_rabbit[i - 1] as f32) * scale_y;
            let x2 = graph_x0 + (i as f32) * scale_x;
            let y2 = graph_y0 - (count as f32) * scale_y;
            draw_line(x1, y1, x2, y2, 2.0, PURPLE);
        }
    let final_cow = *history_cow.last().unwrap_or(&0);
    let final_goat = *history_goat.last().unwrap_or(&0);
    let final_rabbit = *history_rabbit.last().unwrap_or(&0);
    draw_text(&format!("Cow: {}", final_cow), graph_x0 + graph_width - 100.0, graph_y0 - graph_height + 20.0, 24.0, BLUE);
    draw_text(&format!("Goat: {}", final_goat), graph_x0 + graph_width - 100.0, graph_y0 - graph_height + 50.0, 24.0, ORANGE);
    draw_text(&format!("Rabbit: {}", final_rabbit), graph_x0 + graph_width - 100.0, graph_y0 - graph_height + 80.0, 24.0, PURPLE);
        next_frame().await;
    }
}
