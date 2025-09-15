use crate::traits::Organismo;
use crate::organism::prey::Species;  
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;          

pub struct Predator {
    pub min_reserve: f64,        
    pub opt_reserve: f64,        
    pub current_reserve: f64,    
    pub sacrifice_age_days: u32, 
    pub enfermo: bool,           
    pub consumido_hoy: f64,      
}

impl Predator {
    
    pub fn new(
        min_reserve: f64,
        opt_reserve: f64,
        sacrifice_age_days: u32,
        initial_reserve: f64,
    ) -> Self {
        Predator {
            min_reserve,
            opt_reserve,
            current_reserve: initial_reserve,
            sacrifice_age_days,
            enfermo: false,
            consumido_hoy: 0.0,
        }
    }

    pub fn current_reserve(&self) -> f64 {
        self.current_reserve
    }

    pub fn comenzar_dia(&mut self) {
        self.consumido_hoy = 0.0;
    }

    
    pub fn chequear_al_final_del_dia(&mut self) {
        if self.consumido_hoy < self.min_reserve {
            let falta = self.min_reserve - self.consumido_hoy;
            if self.current_reserve >= falta {
                
                self.current_reserve -= falta;
                self.consumido_hoy = self.min_reserve;
                println!(
                    "Depredador usó {:.2} de reserva para alcanzar el mínimo. Reserva restante: {:.2}",
                    falta, self.current_reserve
                );
                self.enfermo = false;
            } else {
                
                self.enfermo = true;
                println!(
                    "Depredador no alcanzó el mínimo diario ({:.2} < {:.2}), enferma/muere.",
                    self.consumido_hoy + self.current_reserve,
                    self.min_reserve
                );
            }
        } else {
            self.enfermo = false;
        }
    }

    
    pub fn cazar(&mut self, poblacion: &mut Vec<Box<dyn Organismo>>, rng: &mut ThreadRng) {
        if self.consumido_hoy >= self.opt_reserve {
            return;
        }

        // Buscar candidatos
        let candidatos: Vec<(usize, f64)> = poblacion
            .iter()
            .enumerate()
            .filter_map(|(i, ind)| {
                if let Some(_especie) = ind.especie() {
                    if let Some(edad_ind) = ind.edad() {
                        if edad_ind >= self.sacrifice_age_days {
                            return Some((i, ind.peso()));
                        }
                    }
                }
                None
            })
            .collect();

        if candidatos.is_empty() {
            return;
        }

        
        let max_peso = candidatos.iter().map(|(_, p)| *p).fold(f64::MIN, f64::max);
        let mas_pesados: Vec<usize> = candidatos
            .iter()
            .filter(|(_, p)| (*p - max_peso).abs() < std::f64::EPSILON)
            .map(|(i, _)| *i)
            .collect();
        let elegido_idx = *mas_pesados.choose(rng).unwrap();

        let peso_pres = poblacion[elegido_idx].peso();
        let faltar_para_optimo = self.opt_reserve - self.consumido_hoy;

        if peso_pres <= faltar_para_optimo {
            
            self.consumido_hoy += peso_pres;
            println!(
                "Depredador cazó presa completa ({:.2}). Consumido hoy: {:.2}.",
                peso_pres, self.consumido_hoy
            );
        } else {
            
            self.consumido_hoy += faltar_para_optimo;
            let sobrante = peso_pres - faltar_para_optimo;
            self.current_reserve += sobrante;
            println!(
                "Depredador cazó presa de {:.2}, consumió {:.2} y guardó {:.2} en reserva. \
                 Consumido hoy: {:.2}, Reserva: {:.2}",
                peso_pres,
                faltar_para_optimo,
                sobrante,
                self.consumido_hoy,
                self.current_reserve
            );
        }

        
        poblacion.remove(elegido_idx);
    }
}

impl Organismo for Predator {
    fn envejecer(&mut self) {}
    fn reproducirse(&self, _rng: &mut ThreadRng, _poblacion: &Vec<Box<dyn Organismo>>) -> Vec<Box<dyn Organismo>> { Vec::new() }
    fn peso(&self) -> f64 { 0.0 }
    fn esta_vivo(&self) -> bool { !self.enfermo }
    fn nombre(&self) -> &str { "Depredador" }
    fn especie(&self) -> Option<Species> { None }
    fn es_macho(&self) -> Option<bool> { None }
    fn prob_enfermar(&self) -> Option<f64> { None }
    fn prob_muerte_enfermedad(&self) -> Option<f64> { None }
    fn tratar_enfermedad(&mut self, _rng: &mut ThreadRng) -> bool { false }
    fn edad(&self) -> Option<u32> { None }
}
