CREATE MIGRATION m1kxlpwvcol5kaihummt4rsnmgnma5bdxlxlgpk6flr4ffzv46ge3q
    ONTO m1wpdcfbnk4j5dieael2hgszz6mwphsxkcq53x7xdqrewpyn5l6gna
{
  ALTER TYPE default::Session {
      ALTER LINK user {
          CREATE CONSTRAINT std::exclusive;
      };
  };
};
