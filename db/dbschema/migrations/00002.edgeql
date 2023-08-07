CREATE MIGRATION m1e632f7owbgo4qoa5m464yiwbhc5ccmsqld4j7nwfjefgvgb4m2oa
    ONTO m1chwfrvtedqdenhxdq6evid242vub7g2xhvxf7vgke3mgbgmwpvzq
{
  CREATE SCALAR TYPE default::BodyPart EXTENDING enum<Hat, Shirt, Sweater, Jacket, Trousers, Socks, Shoes>;
  ALTER TYPE default::Clothes {
      CREATE PROPERTY bodyPart: default::BodyPart;
  };
};
