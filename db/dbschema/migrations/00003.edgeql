CREATE MIGRATION m12cepal2o2hywjze5ppsi2hcv5apss7eshn5mvx6c6yqikdbsog3q
    ONTO m1e632f7owbgo4qoa5m464yiwbhc5ccmsqld4j7nwfjefgvgb4m2oa
{
  CREATE ALIAS default::ClothesFields := (
      default::Clothes {
          name,
          warmness,
          color1,
          color2,
          color3
      }
  );
};
