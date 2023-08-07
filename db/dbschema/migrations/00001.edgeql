CREATE MIGRATION m1chwfrvtedqdenhxdq6evid242vub7g2xhvxf7vgke3mgbgmwpvzq
    ONTO initial
{
  CREATE SCALAR TYPE default::HexColor EXTENDING std::str {
      CREATE CONSTRAINT std::regexp('^#([A-Fa-f0-9]{6}|[A-Fa-f0-9]{3})$');
  };
  CREATE TYPE default::Clothes {
      CREATE PROPERTY color1: default::HexColor;
      CREATE PROPERTY color2: default::HexColor;
      CREATE PROPERTY color3: default::HexColor;
      CREATE REQUIRED PROPERTY name: std::str;
      CREATE PROPERTY warmness: std::float32 {
          CREATE CONSTRAINT std::max_value(5);
          CREATE CONSTRAINT std::min_value(1);
      };
  };
  CREATE SCALAR TYPE default::OutfitCreator EXTENDING enum<User, Auto>;
  CREATE TYPE default::Outfit {
      CREATE REQUIRED LINK clothes: default::Clothes;
      CREATE PROPERTY creationDate: std::datetime;
      CREATE PROPERTY creator: default::OutfitCreator;
      CREATE PROPERTY name: std::str;
      CREATE PROPERTY rating: std::int32 {
          CREATE CONSTRAINT std::max_value(5);
          CREATE CONSTRAINT std::min_value(1);
      };
  };
  CREATE TYPE default::User {
      CREATE LINK clothes: default::Clothes;
      CREATE REQUIRED PROPERTY email: std::str;
      CREATE REQUIRED PROPERTY name: std::str {
          CREATE CONSTRAINT std::exclusive;
      };
      CREATE REQUIRED PROPERTY password: std::str;
  };
};
