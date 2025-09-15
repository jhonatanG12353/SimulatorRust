use crate::traits::Organismo;
use crate::util::{crecimiento_gompertz}; 
use rand::rngs::ThreadRng;
use rand::Rng; 


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Species {
    Cow,
    Goat,
    Rabbit,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Sex {
    Male,
    Female,
}

pub struct Prey {
    pub id: u32,
    pub species: Species,
    pub sex: Sex,
    pub age_days: u32,
    pub weight_kg: f64,
    growth_fn: Box<dyn Fn(u32) -> f64>,
    pub prob_enfermar: f64,
    pub prob_muerte_enfermedad: f64,
    pub enfermo: bool,
}

impl Prey {
    pub fn new(
        id: u32,
        species: Species,
        sex: Sex,
        prob_enfermar: f64,
        prob_muerte_enfermedad: f64,
    ) -> Self {
        let (a, b, k) = match species {
            Species::Cow => (700.0, 3.0, 0.008),
            Species::Goat => (75.0, 2.8, 0.01),
            Species::Rabbit => (5.0, 2.5, 0.05),
        };
        let growth = Box::new(crecimiento_gompertz(a, b, k));
        let weight = growth(0);
        Prey {
            id,
            species,
            sex,
            age_days: 0,
            weight_kg: weight,
            growth_fn: growth,
            prob_enfermar,
            prob_muerte_enfermedad,
            enfermo: false,
        }
    }
}

impl Organismo for Prey {
    fn envejecer(&mut self) {
        self.age_days += 1;
        
        self.weight_kg = (self.growth_fn)(self.age_days);
    }

    fn reproducirse(
        &self,
        rng: &mut ThreadRng,
        poblacion: &Vec<Box<dyn Organismo>>
    ) -> Vec<Box<dyn Organismo>> {
        let mut hijos: Vec<Box<dyn Organismo>> = Vec::new();

        
        let existe_macho = poblacion.iter().any(|ind| {
            if let (Some(especie_i), Some(es_macho_i)) = (ind.especie(), ind.es_macho()) {
                especie_i == self.species && es_macho_i
            } else {
                false
            }
        });

        if !existe_macho {
            return hijos;
        }

        
        if self.sex == Sex::Female && self.age_days > 180 && rng.gen_bool(0.01) {
            let num_offspring = match self.species {
                Species::Cow => 1,
                Species::Goat => rng.gen_range(1..=3),
                Species::Rabbit => rng.gen_range(3..=8),
            };
            for i in 0..num_offspring {
                let sex = if rng.gen_bool(0.5) { Sex::Male } else { Sex::Female };
                hijos.push(Box::new( Prey::new(
                    self.id + 1000 + i,
                    self.species,
                    sex,
                    self.prob_enfermar,
                    self.prob_muerte_enfermedad,
                )));
            }
        }

        hijos
    }

    fn peso(&self) -> f64 {
        self.weight_kg
    }

    fn esta_vivo(&self) -> bool {
        
        let vivo_por_edad = match self.species {
            Species::Cow => self.age_days < 25 * 365,
            Species::Goat => self.age_days < 15 * 365,
            Species::Rabbit => self.age_days < 8 * 365,
        };

        
        if self.enfermo {
            true
        } else {
            vivo_por_edad
        }
    }

    fn nombre(&self) -> &str {
        match self.species {
            Species::Cow => "Vaca",
            Species::Goat => "Cabra",
            Species::Rabbit => "Conejo",
        }
    }

    fn especie(&self) -> Option<Species> {
        Some(self.species)
    }

    fn es_macho(&self) -> Option<bool> {
        Some(self.sex == Sex::Male)
    }

    fn prob_enfermar(&self) -> Option<f64> {
        Some(self.prob_enfermar)
    }

    fn prob_muerte_enfermedad(&self) -> Option<f64> {
        Some(self.prob_muerte_enfermedad)
    }

    fn tratar_enfermedad(&mut self, rng: &mut ThreadRng) -> bool {
        if self.enfermo {
            if rng.gen_bool(self.prob_muerte_enfermedad) {
                return true;  
            } else {
                return false; 
            }
        } else {
            if rng.gen_bool(self.prob_enfermar) {
                self.enfermo = true;
            }
            return false;
        }
    }
    fn edad(&self) -> Option<u32> {
        Some(self.age_days)
    }
}
