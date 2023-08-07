#using extension edgeql_http;
module default {
    type User {
        required email: str; 
        required password: str;
        required name: str {
            constraint exclusive;
        };
        multi clothes: Clothes;
        standard_location: Location;
    }

    type Session {
        required token: str;
        user: User {
            constraint exclusive;
        };
        session_date: datetime;
    }

    #TODO manage images
    type Clothes {
        required name: str {
            constraint exclusive;
        };
        color1: HexColor; 
        color2: HexColor; 
        color3: HexColor; 
        bodyPart: BodyPart;
        waterproof: bool;
        warmness: float32 {
            constraint max_value(5);
            constraint min_value(1);
        };
        mod_warmness: float32;
    }

    type Outfit {
        name: str;
        creationDate: datetime;
        creator: OutfitCreator;
        required clothes: Clothes;
        location: Location;
        rating: int32 {
            constraint max_value(5);
            constraint min_value(1);
        };
    }

    type Location {
        name: str;
        required latitude: float64;
        required longitude: float64;
    }

    type Options {
        population_size: int32;
        generation_limit: int32;
        iterations: int32;
        learning_rate: float32;
    }

    scalar type BodyPart extending enum<Hat, Shirt, Sweater, Jacket, Trousers, Socks, Shoes>;

    scalar type OutfitCreator extending enum<User, Auto>;

    scalar type HexColor extending str {
        constraint regexp(r'^#([A-Fa-f0-9]{6}|[A-Fa-f0-9]{3})$|^$');
    };

}
