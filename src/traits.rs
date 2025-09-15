use crate::organism::prey::Species;
use rand::rngs::ThreadRng;

pub trait Organismo {
    fn envejecer(&mut self);
    fn reproducirse(&self, rng: &mut ThreadRng, poblacion: &Vec<Box<dyn Organismo>>)
        -> Vec<Box<dyn Organismo>>;
    fn peso(&self) -> f64;
    fn esta_vivo(&self) -> bool;
    fn nombre(&self) -> &str;

    fn especie(&self) -> Option<Species>;
    fn es_macho(&self) -> Option<bool>;

    fn prob_enfermar(&self) -> Option<f64>;
    fn prob_muerte_enfermedad(&self) -> Option<f64>;

    fn tratar_enfermedad(&mut self, rng: &mut ThreadRng) -> bool;

    fn edad(&self) -> Option<u32>;
}
