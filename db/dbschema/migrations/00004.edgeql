CREATE MIGRATION m1wcrw5srebcuqluty4ml5oodwtxlfgixh6tu3jwlclftdupcbhgia
    ONTO m12cepal2o2hywjze5ppsi2hcv5apss7eshn5mvx6c6yqikdbsog3q
{
  DROP ALIAS default::ClothesFields;
  ALTER TYPE default::Clothes {
      CREATE PROPERTY mod_warmness: std::float32;
      CREATE PROPERTY waterproof: std::bool;
  };
  CREATE TYPE default::Location {
      CREATE REQUIRED PROPERTY latitude: std::float64;
      CREATE REQUIRED PROPERTY longitude: std::float64;
      CREATE PROPERTY name: std::str;
  };
  ALTER TYPE default::Outfit {
      CREATE LINK location: default::Location;
  };
  ALTER TYPE default::User {
      CREATE LINK standard_location: default::Location;
      ALTER LINK clothes {
          SET MULTI;
      };
  };
  CREATE TYPE default::Options {
      CREATE PROPERTY generation_limit: std::int32;
      CREATE PROPERTY iterations: std::int32;
      CREATE PROPERTY learning_rate: std::float32;
      CREATE PROPERTY population_size: std::int32;
  };
};
