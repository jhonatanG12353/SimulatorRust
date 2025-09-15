use crate::traits::Organismo;
use crate::organism::predator::Predator;
use rand::rngs::ThreadRng;
use std::boxed::Box;

pub struct Simulation {
    day: u32,
    animals: Vec<Box<dyn Organismo>>,
    predator: Predator,
    next_id: u32,
}

impl Simulation {
    pub fn new(initial: Vec<Box<dyn Organismo>>, predator: Predator) -> Self {
        Simulation {
            day: 0,
            animals: initial,
            predator,
            next_id: 1000,
        }
    }

    
    pub fn get_day(&self) -> u32 {
        self.day
    }

    
    pub fn get_animals(&self) -> &Vec<Box<dyn Organismo>> {
        &self.animals
    }

    
    pub fn get_predator(&self) -> &Predator {
        &self.predator
    }

    pub fn simulate_day(&mut self) {
        self.day += 1;
        let mut rng: ThreadRng = rand::thread_rng();
        println!("\n--- Día {} ---", self.day);

        
        self.predator.comenzar_dia();

       
        self.animals.retain_mut(|a| {
            a.envejecer();

            if let Some(_pe) = a.prob_enfermar() {
                if a.tratar_enfermedad(&mut rng) {
                    println!("Un individuo de {} murió por enfermedad.", a.nombre());
                    return false;
                }
            }

            a.esta_vivo()
        });

        
        let mut nuevos: Vec<Box<dyn Organismo>> = Vec::new();
        {
            let poblacion_snapshot = &self.animals;
            for a in &self.animals {
                let hijos = a.reproducirse(&mut rng, poblacion_snapshot);
                nuevos.extend(hijos);
            }
        }
        self.animals.extend(nuevos);

        
        self.predator.cazar(&mut self.animals, &mut rng);

        
        self.predator.chequear_al_final_del_dia();
        if !self.predator.esta_vivo() {
            println!(
                "El depredador enfermó/murió por no alcanzar el mínimo diario ({:.2} < {:.2}).",
                self.predator.consumido_hoy,
                self.predator.min_reserve
            );
        }

        
        let mut conteo: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
        for a in &self.animals {
            *conteo.entry(a.nombre()).or_insert(0) += 1;
        }
        println!("Población actual: {} animales", self.animals.len());
        for (nombre, cantidad) in conteo {
            println!("{}: {}", nombre, cantidad);
        }

       
        println!(
            "Reserva acumulada del depredador: {:.2}, consumido hoy: {:.2}, estado: {}",
            self.predator.current_reserve(),
            self.predator.consumido_hoy,
            if self.predator.esta_vivo() { "vivo" } else { "enfermo/muerto" }
        );
    }
}
