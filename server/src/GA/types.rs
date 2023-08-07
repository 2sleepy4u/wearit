use edgedb_tokio::Queryable;
use rocket::serde::{Deserialize, Serialize, json::Json};

#[derive(Clone, Debug, Queryable)]
pub struct GeneList {
    pub hat: Vec<Gene>,
    pub shirt: Vec<Gene>,
    pub sweater: Vec<Gene>,
    pub jacket: Vec<Gene>,
    pub trousers: Vec<Gene>,
    pub socks: Vec<Gene>,
    pub shoes: Vec<Gene>
}

#[serde(crate = "rocket::serde")]
#[derive(Deserialize, Serialize, Clone, Debug, Queryable, Default)]
pub struct Gene {
    pub name: String,
    pub warmness: f32,
    pub color1: String,
    pub color2: Option<String>,
    pub color3: Option<String>
}

pub type FitnessFunc = fn(genome: Genome, max_warmness: f32) -> f32;
pub type Population = Vec<Genome>;
pub type Genome = Vec<Gene>; 

