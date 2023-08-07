CREATE MIGRATION m1wamjk7bod7t6sbdxvxl5om75s4lvrigv7tjbnt7vwdphw53sw2ga
    ONTO m1kxlpwvcol5kaihummt4rsnmgnma5bdxlxlgpk6flr4ffzv46ge3q
{
  ALTER TYPE default::Clothes {
      ALTER PROPERTY name {
          CREATE CONSTRAINT std::exclusive;
      };
  };
};
