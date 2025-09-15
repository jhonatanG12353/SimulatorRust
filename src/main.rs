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

   
    let mut rng = thread_rng();
    let mut posiciones: Vec<(f32, f32)> = (0..100)
        .map(|_| {
            (
                rng.gen_range(50.0..(screen_width() - 50.0)),
                rng.gen_range(50.0..(screen_height() - 50.0)),
            )
        })
        .collect();

    // Este son los días para simular   
    let total_dias = 500;

    for _ in 0..total_dias {
        clear_background(LIGHTGRAY);

        -
        sim.simulate_day();

        
        for (i, presa) in sim.get_animals().iter().enumerate() {
            let (x, y) = posiciones[i % posiciones.len()];
            let color = match presa.es_macho() {
                Some(true) => GREEN,
                Some(false) => PINK,
                None => WHITE,
            };

            draw_rectangle(x, y, 20.0, 20.0, color);
        }

        
        let x = screen_width() / 2.0;
        let y = screen_height() / 2.0;
        draw_circle(x, y, 25.0, RED);

        
        draw_text(
            &format!(
                "Día: {}  |  Reserva: {:.1}",
                sim.get_day(),
                sim.get_predator().current_reserve()
            ),
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
        next_frame().await;
    }
}
