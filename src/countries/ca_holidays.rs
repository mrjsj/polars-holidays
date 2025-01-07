use crate::countries::constants::*;
use phf::phf_map;

pub static CA_HOLIDAYS: phf::Map<i32, &'static str> = phf_map! {
    10957_i32 => _NEW_YEAR_S_DAY,
    10959_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    11068_i32 => _GOOD_FRIDAY,
    11139_i32 => _CANADA_DAY,
    11204_i32 => _LABOR_DAY,
    11316_i32 => _CHRISTMAS_DAY,
    11323_i32 => _NEW_YEAR_S_DAY,
    11425_i32 => _GOOD_FRIDAY,
    11504_i32 => _CANADA_DAY,
    11568_i32 => _LABOR_DAY,
    11681_i32 => _CHRISTMAS_DAY,
    11688_i32 => _NEW_YEAR_S_DAY,
    11775_i32 => _GOOD_FRIDAY,
    11869_i32 => _CANADA_DAY,
    11932_i32 => _LABOR_DAY,
    12046_i32 => _CHRISTMAS_DAY,
    12053_i32 => _NEW_YEAR_S_DAY,
    12160_i32 => _GOOD_FRIDAY,
    12234_i32 => _CANADA_DAY,
    12296_i32 => _LABOR_DAY,
    12411_i32 => _CHRISTMAS_DAY,
    12418_i32 => _NEW_YEAR_S_DAY,
    12517_i32 => _GOOD_FRIDAY,
    12600_i32 => _CANADA_DAY,
    12667_i32 => _LABOR_DAY,
    12777_i32 => _CHRISTMAS_DAY,
    12779_i32 => _CHRISTMAS_DAY__OBSERVED_,
    12784_i32 => _NEW_YEAR_S_DAY,
    12786_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    12867_i32 => _GOOD_FRIDAY,
    12965_i32 => _CANADA_DAY,
    13031_i32 => _LABOR_DAY,
    13142_i32 => _CHRISTMAS_DAY,
    13143_i32 => _CHRISTMAS_DAY__OBSERVED_,
    13149_i32 => _NEW_YEAR_S_DAY,
    13150_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    13252_i32 => _GOOD_FRIDAY,
    13330_i32 => _CANADA_DAY,
    13395_i32 => _LABOR_DAY,
    13507_i32 => _CHRISTMAS_DAY,
    13514_i32 => _NEW_YEAR_S_DAY,
    13609_i32 => _GOOD_FRIDAY,
    13695_i32 => _CANADA_DAY,
    13759_i32 => _LABOR_DAY,
    13872_i32 => _CHRISTMAS_DAY,
    13879_i32 => _NEW_YEAR_S_DAY,
    13959_i32 => _GOOD_FRIDAY,
    14061_i32 => _CANADA_DAY,
    14123_i32 => _LABOR_DAY,
    14238_i32 => _CHRISTMAS_DAY,
    14245_i32 => _NEW_YEAR_S_DAY,
    14344_i32 => _GOOD_FRIDAY,
    14426_i32 => _CANADA_DAY,
    14494_i32 => _LABOR_DAY,
    14603_i32 => _CHRISTMAS_DAY,
    14610_i32 => _NEW_YEAR_S_DAY,
    14701_i32 => _GOOD_FRIDAY,
    14791_i32 => _CANADA_DAY,
    14858_i32 => _LABOR_DAY,
    14968_i32 => _CHRISTMAS_DAY,
    14970_i32 => _CHRISTMAS_DAY__OBSERVED_,
    14975_i32 => _NEW_YEAR_S_DAY,
    14977_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    15086_i32 => _GOOD_FRIDAY,
    15156_i32 => _CANADA_DAY,
    15222_i32 => _LABOR_DAY,
    15333_i32 => _CHRISTMAS_DAY,
    15334_i32 => _CHRISTMAS_DAY__OBSERVED_,
    15340_i32 => _NEW_YEAR_S_DAY,
    15341_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    15436_i32 => _GOOD_FRIDAY,
    15522_i32 => _CANADA_DAY,
    15586_i32 => _LABOR_DAY,
    15699_i32 => _CHRISTMAS_DAY,
    15706_i32 => _NEW_YEAR_S_DAY,
    15793_i32 => _GOOD_FRIDAY,
    15887_i32 => _CANADA_DAY,
    15950_i32 => _LABOR_DAY,
    16064_i32 => _CHRISTMAS_DAY,
    16071_i32 => _NEW_YEAR_S_DAY,
    16178_i32 => _GOOD_FRIDAY,
    16252_i32 => _CANADA_DAY,
    16314_i32 => _LABOR_DAY,
    16429_i32 => _CHRISTMAS_DAY,
    16436_i32 => _NEW_YEAR_S_DAY,
    16528_i32 => _GOOD_FRIDAY,
    16617_i32 => _CANADA_DAY,
    16685_i32 => _LABOR_DAY,
    16794_i32 => _CHRISTMAS_DAY,
    16801_i32 => _NEW_YEAR_S_DAY,
    16885_i32 => _GOOD_FRIDAY,
    16983_i32 => _CANADA_DAY,
    17049_i32 => _LABOR_DAY,
    17160_i32 => _CHRISTMAS_DAY,
    17161_i32 => _CHRISTMAS_DAY__OBSERVED_,
    17167_i32 => _NEW_YEAR_S_DAY,
    17168_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    17270_i32 => _GOOD_FRIDAY,
    17348_i32 => _CANADA_DAY,
    17413_i32 => _LABOR_DAY,
    17525_i32 => _CHRISTMAS_DAY,
    17532_i32 => _NEW_YEAR_S_DAY,
    17620_i32 => _GOOD_FRIDAY,
    17713_i32 => _CANADA_DAY,
    17777_i32 => _LABOR_DAY,
    17890_i32 => _CHRISTMAS_DAY,
    17897_i32 => _NEW_YEAR_S_DAY,
    18005_i32 => _GOOD_FRIDAY,
    18078_i32 => _CANADA_DAY,
    18141_i32 => _LABOR_DAY,
    18255_i32 => _CHRISTMAS_DAY,
    18262_i32 => _NEW_YEAR_S_DAY,
    18362_i32 => _GOOD_FRIDAY,
    18444_i32 => _CANADA_DAY,
    18512_i32 => _LABOR_DAY,
    18621_i32 => _CHRISTMAS_DAY,
    18628_i32 => _NEW_YEAR_S_DAY,
    18719_i32 => _GOOD_FRIDAY,
    18809_i32 => _CANADA_DAY,
    18876_i32 => _LABOR_DAY,
    18986_i32 => _CHRISTMAS_DAY,
    18988_i32 => _CHRISTMAS_DAY__OBSERVED_,
    18993_i32 => _NEW_YEAR_S_DAY,
    18995_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    19097_i32 => _GOOD_FRIDAY,
    19174_i32 => _CANADA_DAY,
    19240_i32 => _LABOR_DAY,
    19351_i32 => _CHRISTMAS_DAY,
    19352_i32 => _CHRISTMAS_DAY__OBSERVED_,
    19358_i32 => _NEW_YEAR_S_DAY,
    19359_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    19454_i32 => _GOOD_FRIDAY,
    19539_i32 => _CANADA_DAY,
    19604_i32 => _LABOR_DAY,
    19716_i32 => _CHRISTMAS_DAY,
    19723_i32 => _NEW_YEAR_S_DAY,
    19811_i32 => _GOOD_FRIDAY,
    19905_i32 => _CANADA_DAY,
    19968_i32 => _LABOR_DAY,
    20082_i32 => _CHRISTMAS_DAY,
    20089_i32 => _NEW_YEAR_S_DAY,
    20196_i32 => _GOOD_FRIDAY,
    20270_i32 => _CANADA_DAY,
    20332_i32 => _LABOR_DAY,
    20447_i32 => _CHRISTMAS_DAY,
    20454_i32 => _NEW_YEAR_S_DAY,
    20546_i32 => _GOOD_FRIDAY,
    20635_i32 => _CANADA_DAY,
    20703_i32 => _LABOR_DAY,
    20812_i32 => _CHRISTMAS_DAY,
    20819_i32 => _NEW_YEAR_S_DAY,
    20903_i32 => _GOOD_FRIDAY,
    21000_i32 => _CANADA_DAY,
    21067_i32 => _LABOR_DAY,
    21177_i32 => _CHRISTMAS_DAY,
    21179_i32 => _CHRISTMAS_DAY__OBSERVED_,
    21184_i32 => _NEW_YEAR_S_DAY,
    21186_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    21288_i32 => _GOOD_FRIDAY,
    21366_i32 => _CANADA_DAY,
    21431_i32 => _LABOR_DAY,
    21543_i32 => _CHRISTMAS_DAY,
    21550_i32 => _NEW_YEAR_S_DAY,
    21638_i32 => _GOOD_FRIDAY,
    21731_i32 => _CANADA_DAY,
    21795_i32 => _LABOR_DAY,
    21908_i32 => _CHRISTMAS_DAY,
    21915_i32 => _NEW_YEAR_S_DAY,
    22023_i32 => _GOOD_FRIDAY,
    22096_i32 => _CANADA_DAY,
    22159_i32 => _LABOR_DAY,
    22273_i32 => _CHRISTMAS_DAY,
    22280_i32 => _NEW_YEAR_S_DAY,
    22380_i32 => _GOOD_FRIDAY,
    22461_i32 => _CANADA_DAY,
    22523_i32 => _LABOR_DAY,
    22638_i32 => _CHRISTMAS_DAY,
    22645_i32 => _NEW_YEAR_S_DAY,
    22730_i32 => _GOOD_FRIDAY,
    22827_i32 => _CANADA_DAY,
    22894_i32 => _LABOR_DAY,
    23004_i32 => _CHRISTMAS_DAY,
    23006_i32 => _CHRISTMAS_DAY__OBSERVED_,
    23011_i32 => _NEW_YEAR_S_DAY,
    23013_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    23115_i32 => _GOOD_FRIDAY,
    23192_i32 => _CANADA_DAY,
    23258_i32 => _LABOR_DAY,
    23369_i32 => _CHRISTMAS_DAY,
    23370_i32 => _CHRISTMAS_DAY__OBSERVED_,
    23376_i32 => _NEW_YEAR_S_DAY,
    23377_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    23472_i32 => _GOOD_FRIDAY,
    23557_i32 => _CANADA_DAY,
    23622_i32 => _LABOR_DAY,
    23734_i32 => _CHRISTMAS_DAY,
    23741_i32 => _NEW_YEAR_S_DAY,
    23822_i32 => _GOOD_FRIDAY,
    23922_i32 => _CANADA_DAY,
    23986_i32 => _LABOR_DAY,
    24099_i32 => _CHRISTMAS_DAY,
    24106_i32 => _NEW_YEAR_S_DAY,
    24207_i32 => _GOOD_FRIDAY,
    24288_i32 => _CANADA_DAY,
    24350_i32 => _LABOR_DAY,
    24465_i32 => _CHRISTMAS_DAY,
    24472_i32 => _NEW_YEAR_S_DAY,
    24564_i32 => _GOOD_FRIDAY,
    24653_i32 => _CANADA_DAY,
    24721_i32 => _LABOR_DAY,
    24830_i32 => _CHRISTMAS_DAY,
    24837_i32 => _NEW_YEAR_S_DAY,
    24949_i32 => _GOOD_FRIDAY,
    25018_i32 => _CANADA_DAY,
    25085_i32 => _LABOR_DAY,
    25195_i32 => _CHRISTMAS_DAY,
    25197_i32 => _CHRISTMAS_DAY__OBSERVED_,
    25202_i32 => _NEW_YEAR_S_DAY,
    25204_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    25299_i32 => _GOOD_FRIDAY,
    25383_i32 => _CANADA_DAY,
    25449_i32 => _LABOR_DAY,
    25560_i32 => _CHRISTMAS_DAY,
    25561_i32 => _CHRISTMAS_DAY__OBSERVED_,
    25567_i32 => _NEW_YEAR_S_DAY,
    25568_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    25656_i32 => _GOOD_FRIDAY,
    25749_i32 => _CANADA_DAY,
    25813_i32 => _LABOR_DAY,
    25926_i32 => _CHRISTMAS_DAY,
    25933_i32 => _NEW_YEAR_S_DAY,
    26041_i32 => _GOOD_FRIDAY,
    26114_i32 => _CANADA_DAY,
    26177_i32 => _LABOR_DAY,
    26291_i32 => _CHRISTMAS_DAY,
    26298_i32 => _NEW_YEAR_S_DAY,
    26391_i32 => _GOOD_FRIDAY,
    26479_i32 => _CANADA_DAY,
    26541_i32 => _LABOR_DAY,
    26656_i32 => _CHRISTMAS_DAY,
    26663_i32 => _NEW_YEAR_S_DAY,
    26748_i32 => _GOOD_FRIDAY,
    26844_i32 => _CANADA_DAY,
    26912_i32 => _LABOR_DAY,
    27021_i32 => _CHRISTMAS_DAY,
    27028_i32 => _NEW_YEAR_S_DAY,
    27133_i32 => _GOOD_FRIDAY,
    27210_i32 => _CANADA_DAY,
    27276_i32 => _LABOR_DAY,
    27387_i32 => _CHRISTMAS_DAY,
    27388_i32 => _CHRISTMAS_DAY__OBSERVED_,
    27394_i32 => _NEW_YEAR_S_DAY,
    27395_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    27490_i32 => _GOOD_FRIDAY,
    27575_i32 => _CANADA_DAY,
    27640_i32 => _LABOR_DAY,
    27752_i32 => _CHRISTMAS_DAY,
    27759_i32 => _NEW_YEAR_S_DAY,
    27840_i32 => _GOOD_FRIDAY,
    27940_i32 => _CANADA_DAY,
    28004_i32 => _LABOR_DAY,
    28117_i32 => _CHRISTMAS_DAY,
    28124_i32 => _NEW_YEAR_S_DAY,
    28225_i32 => _GOOD_FRIDAY,
    28305_i32 => _CANADA_DAY,
    28368_i32 => _LABOR_DAY,
    28482_i32 => _CHRISTMAS_DAY,
    28489_i32 => _NEW_YEAR_S_DAY,
    28582_i32 => _GOOD_FRIDAY,
    28671_i32 => _CANADA_DAY,
    28739_i32 => _LABOR_DAY,
    28848_i32 => _CHRISTMAS_DAY,
    28855_i32 => _NEW_YEAR_S_DAY,
    28960_i32 => _GOOD_FRIDAY,
    29036_i32 => _CANADA_DAY,
    29103_i32 => _LABOR_DAY,
    29213_i32 => _CHRISTMAS_DAY,
    29215_i32 => _CHRISTMAS_DAY__OBSERVED_,
    29220_i32 => _NEW_YEAR_S_DAY,
};
