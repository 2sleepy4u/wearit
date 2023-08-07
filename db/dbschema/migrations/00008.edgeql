CREATE MIGRATION m1pgocr5op5yr7yjpnduyno3zyxbtdwbmdyievjggxcokzg63xiinq
    ONTO m1wamjk7bod7t6sbdxvxl5om75s4lvrigv7tjbnt7vwdphw53sw2ga
{
  ALTER SCALAR TYPE default::HexColor {
      CREATE CONSTRAINT std::regexp('^#([A-Fa-f0-9]{6}|[A-Fa-f0-9]{3})$|^$');
  };
  ALTER SCALAR TYPE default::HexColor {
      DROP CONSTRAINT std::regexp('^#([A-Fa-f0-9]{6}|[A-Fa-f0-9]{3})$');
  };
};
