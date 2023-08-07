use GA::functions::{selection_pair, mutation, single_pair_crossover, fitness, generate_population};
use edgedb_protocol::*;
use edgedb_tokio::Builder;
use value::*;
mod GA;
use GA::types::*;
use GA::*;
use GA::functions::colorFitness;

use std::time::Instant;
//genome: [hat, shirt, sweater, jacket, trousers, socks, shoes]
//item: {name, warmness, color1, color2, color3 }

const GET_USER_CLOTHES: &str = "
with filteredClothes := ( 
  select Clothes filter .<clothes[is User].name = <str>$0
)
select {
  hat := (select filteredClothes { name, warmness, color1, color2, color3 } filter .bodyPart = BodyPart.Hat),
  shirt := (select filteredClothes { name, warmness, color1, color2, color3 } filter .bodyPart = BodyPart.Shirt),
  sweater := (select filteredClothes { name, warmness, color1, color2, color3 } filter .bodyPart = BodyPart.Sweater),
  jacket := (select filteredClothes { name, warmness, color1, color2, color3 } filter .bodyPart = BodyPart.Jacket),
  trousers := (select filteredClothes { name, warmness, color1, color2, color3 } filter .bodyPart = BodyPart.Trousers),
  socks := (select filteredClothes { name, warmness, color1, color2, color3 } filter .bodyPart = BodyPart.Socks),
  shoes := (select filteredClothes { name, warmness, color1, color2, color3 } filter .bodyPart = BodyPart.Shoes)
} limit 1;
";

/*
const GET_USER_CLOTHES: &str = "
with groups := ( 
  group ( select Clothes filter .<clothes[is User].name = <str>$0 ) 
  by .bodyPart 
)
select groups {
  items := .elements { 
    name,
    warmness,
    color1,
    color2,
    color3
  } 
} 
";
*/



#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Builder::new()
        .database("edgedb")
        .unwrap()
        .instance("nothing2wear")
        .unwrap()
        .build_env()
        .await?;
    let conn = edgedb_tokio::Client::new(&config);
    conn.ensure_connected().await?;

    //let conn = edgedb_tokio::create_client().await?;
    let username = "test";
    let clothes_list = conn.query_single::<GeneList, _>(GET_USER_CLOTHES, &(username,)).await?;

    let items: GeneList = 
        match clothes_list {
            Some(clothes) => clothes,
            None => panic!("NO")
        };

    let size = 10;
    let max_warmness = 20.0;
    let generation_limit = 100;
    let mutation_number = 1;
    let mutation_prob = 0.5;

    println!("Starting...");
    let time = Instant::now();
    let result = run_evolution(
        items,
        size,
        generation_limit,
        max_warmness,
        mutation_number,
        mutation_prob,
        fitness,
        generate_population,
        selection_pair,
        mutation,
        single_pair_crossover
        );

    println!("Time {:.2?}", time.elapsed());
    println!("Accuracy: {}", accuracy(result[0].clone(), max_warmness, fitness));
    println!("Color fitness: {}", colorFitness(&result[0], 10));
    for gene in result[0].iter() {
        println!("{:?}", gene.name);
    }
    Ok(())
}

