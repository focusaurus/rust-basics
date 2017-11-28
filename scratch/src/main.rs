enum Thing {
    Box, Fox, Socks, Docks
}

#[derive(PartialEq)]
enum Species {
    Cat, Dog, Fox, Pig
}

struct Animal {
    species: Species
}

fn is_kind(thing: Thing, kind: Thing) -> bool{
    match thing {
        kind => true,
        _ => false
    }
}

fn is_species(animal: Animal, species: Species) -> bool {
    animal.species == species
}

fn main() {
    println!("{:?}", is_species(Animal{species: Species::Dog}, Species::Cat));
    println!("{:?}", is_species(Animal{species: Species::Dog}, Species::Dog));
}
