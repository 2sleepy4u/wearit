CREATE MIGRATION m1wpdcfbnk4j5dieael2hgszz6mwphsxkcq53x7xdqrewpyn5l6gna
    ONTO m1wcrw5srebcuqluty4ml5oodwtxlfgixh6tu3jwlclftdupcbhgia
{
  CREATE TYPE default::Session {
      CREATE LINK user: default::User;
      CREATE PROPERTY session_date: std::datetime;
      CREATE REQUIRED PROPERTY token: std::str;
  };
};
