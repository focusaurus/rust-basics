enum Thing {
    Box, Fox, Socks, Docks
}

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
    match animal.species {
        species => true,
        _ => false
    }
}

fn main() {
    println!("{:?}", is_species(Animal{species: Species::Dog}, Species::Cat));
    println!("{:?}", is_species(Animal{species: Species::Dog}, Species::Dog));
}
