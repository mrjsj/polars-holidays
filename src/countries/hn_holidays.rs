use crate::countries::constants::*;
use phf::phf_map;

pub static HN_HOLIDAYS: phf::Map<i32, &'static str> = phf_map! {
    10957_i32 => _NEW_YEAR_S_DAY,
    11061_i32 => _PANAMERICAN_DAY,
    11067_i32 => _MAUNDY_THURSDAY,
    11068_i32 => _GOOD_FRIDAY,
    11069_i32 => _HOLY_SATURDAY,
    11078_i32 => _LABOR_DAY,
    11215_i32 => _INDEPENDENCE_DAY,
    11233_i32 => _MORAZAN_S_DAY,
    11242_i32 => _COLUMBUS_DAY,
    11251_i32 => _ARMY_DAY,
    11316_i32 => _CHRISTMAS_DAY,
    11323_i32 => _NEW_YEAR_S_DAY,
    11424_i32 => _MAUNDY_THURSDAY,
    11425_i32 => _GOOD_FRIDAY,
    11426_i32 => _HOLY_SATURDAY__PANAMERICAN_DAY,
    11443_i32 => _LABOR_DAY,
    11580_i32 => _INDEPENDENCE_DAY,
    11598_i32 => _MORAZAN_S_DAY,
    11607_i32 => _COLUMBUS_DAY,
    11616_i32 => _ARMY_DAY,
    11681_i32 => _CHRISTMAS_DAY,
    11688_i32 => _NEW_YEAR_S_DAY,
    11774_i32 => _MAUNDY_THURSDAY,
    11775_i32 => _GOOD_FRIDAY,
    11776_i32 => _HOLY_SATURDAY,
    11791_i32 => _PANAMERICAN_DAY,
    11808_i32 => _LABOR_DAY,
    11945_i32 => _INDEPENDENCE_DAY,
    11963_i32 => _MORAZAN_S_DAY,
    11972_i32 => _COLUMBUS_DAY,
    11981_i32 => _ARMY_DAY,
    12046_i32 => _CHRISTMAS_DAY,
    12053_i32 => _NEW_YEAR_S_DAY,
    12156_i32 => _PANAMERICAN_DAY,
    12159_i32 => _MAUNDY_THURSDAY,
    12160_i32 => _GOOD_FRIDAY,
    12161_i32 => _HOLY_SATURDAY,
    12173_i32 => _LABOR_DAY,
    12310_i32 => _INDEPENDENCE_DAY,
    12328_i32 => _MORAZAN_S_DAY,
    12337_i32 => _COLUMBUS_DAY,
    12346_i32 => _ARMY_DAY,
    12411_i32 => _CHRISTMAS_DAY,
    12418_i32 => _NEW_YEAR_S_DAY,
    12516_i32 => _MAUNDY_THURSDAY,
    12517_i32 => _GOOD_FRIDAY,
    12518_i32 => _HOLY_SATURDAY,
    12522_i32 => _PANAMERICAN_DAY,
    12539_i32 => _LABOR_DAY,
    12676_i32 => _INDEPENDENCE_DAY,
    12694_i32 => _MORAZAN_S_DAY,
    12703_i32 => _COLUMBUS_DAY,
    12712_i32 => _ARMY_DAY,
    12777_i32 => _CHRISTMAS_DAY,
    12784_i32 => _NEW_YEAR_S_DAY,
    12866_i32 => _MAUNDY_THURSDAY,
    12867_i32 => _GOOD_FRIDAY,
    12868_i32 => _HOLY_SATURDAY,
    12887_i32 => _PANAMERICAN_DAY,
    12904_i32 => _LABOR_DAY,
    13041_i32 => _INDEPENDENCE_DAY,
    13059_i32 => _MORAZAN_S_DAY,
    13068_i32 => _COLUMBUS_DAY,
    13077_i32 => _ARMY_DAY,
    13142_i32 => _CHRISTMAS_DAY,
    13149_i32 => _NEW_YEAR_S_DAY,
    13251_i32 => _MAUNDY_THURSDAY,
    13252_i32 => _GOOD_FRIDAY__PANAMERICAN_DAY,
    13253_i32 => _HOLY_SATURDAY,
    13269_i32 => _LABOR_DAY,
    13406_i32 => _INDEPENDENCE_DAY,
    13424_i32 => _MORAZAN_S_DAY,
    13433_i32 => _COLUMBUS_DAY,
    13442_i32 => _ARMY_DAY,
    13507_i32 => _CHRISTMAS_DAY,
    13514_i32 => _NEW_YEAR_S_DAY,
    13608_i32 => _MAUNDY_THURSDAY,
    13609_i32 => _GOOD_FRIDAY,
    13610_i32 => _HOLY_SATURDAY,
    13617_i32 => _PANAMERICAN_DAY,
    13634_i32 => _LABOR_DAY,
    13771_i32 => _INDEPENDENCE_DAY,
    13789_i32 => _MORAZAN_S_DAY,
    13798_i32 => _COLUMBUS_DAY,
    13807_i32 => _ARMY_DAY,
    13872_i32 => _CHRISTMAS_DAY,
    13879_i32 => _NEW_YEAR_S_DAY,
    13958_i32 => _MAUNDY_THURSDAY,
    13959_i32 => _GOOD_FRIDAY,
    13960_i32 => _HOLY_SATURDAY,
    13983_i32 => _PANAMERICAN_DAY,
    14000_i32 => _LABOR_DAY,
    14137_i32 => _INDEPENDENCE_DAY,
    14155_i32 => _MORAZAN_S_DAY,
    14164_i32 => _COLUMBUS_DAY,
    14173_i32 => _ARMY_DAY,
    14238_i32 => _CHRISTMAS_DAY,
    14245_i32 => _NEW_YEAR_S_DAY,
    14343_i32 => _MAUNDY_THURSDAY,
    14344_i32 => _GOOD_FRIDAY,
    14345_i32 => _HOLY_SATURDAY,
    14348_i32 => _PANAMERICAN_DAY,
    14365_i32 => _LABOR_DAY,
    14502_i32 => _INDEPENDENCE_DAY,
    14520_i32 => _MORAZAN_S_DAY,
    14529_i32 => _COLUMBUS_DAY,
    14538_i32 => _ARMY_DAY,
    14603_i32 => _CHRISTMAS_DAY,
    14610_i32 => _NEW_YEAR_S_DAY,
    14700_i32 => _MAUNDY_THURSDAY,
    14701_i32 => _GOOD_FRIDAY,
    14702_i32 => _HOLY_SATURDAY,
    14713_i32 => _PANAMERICAN_DAY,
    14730_i32 => _LABOR_DAY,
    14867_i32 => _INDEPENDENCE_DAY,
    14885_i32 => _MORAZAN_S_DAY,
    14894_i32 => _COLUMBUS_DAY,
    14903_i32 => _ARMY_DAY,
    14968_i32 => _CHRISTMAS_DAY,
    14975_i32 => _NEW_YEAR_S_DAY,
    15078_i32 => _PANAMERICAN_DAY,
    15085_i32 => _MAUNDY_THURSDAY,
    15086_i32 => _GOOD_FRIDAY,
    15087_i32 => _HOLY_SATURDAY,
    15095_i32 => _LABOR_DAY,
    15232_i32 => _INDEPENDENCE_DAY,
    15250_i32 => _MORAZAN_S_DAY,
    15259_i32 => _COLUMBUS_DAY,
    15268_i32 => _ARMY_DAY,
    15333_i32 => _CHRISTMAS_DAY,
    15340_i32 => _NEW_YEAR_S_DAY,
    15435_i32 => _MAUNDY_THURSDAY,
    15436_i32 => _GOOD_FRIDAY,
    15437_i32 => _HOLY_SATURDAY,
    15444_i32 => _PANAMERICAN_DAY,
    15461_i32 => _LABOR_DAY,
    15598_i32 => _INDEPENDENCE_DAY,
    15616_i32 => _MORAZAN_S_DAY,
    15625_i32 => _COLUMBUS_DAY,
    15634_i32 => _ARMY_DAY,
    15699_i32 => _CHRISTMAS_DAY,
    15706_i32 => _NEW_YEAR_S_DAY,
    15792_i32 => _MAUNDY_THURSDAY,
    15793_i32 => _GOOD_FRIDAY,
    15794_i32 => _HOLY_SATURDAY,
    15809_i32 => _PANAMERICAN_DAY,
    15826_i32 => _LABOR_DAY,
    15963_i32 => _INDEPENDENCE_DAY,
    15981_i32 => _MORAZAN_S_DAY,
    15990_i32 => _COLUMBUS_DAY,
    15999_i32 => _ARMY_DAY,
    16064_i32 => _CHRISTMAS_DAY,
    16071_i32 => _NEW_YEAR_S_DAY,
    16174_i32 => _PANAMERICAN_DAY,
    16177_i32 => _MAUNDY_THURSDAY,
    16178_i32 => _GOOD_FRIDAY,
    16179_i32 => _HOLY_SATURDAY,
    16191_i32 => _LABOR_DAY,
    16328_i32 => _INDEPENDENCE_DAY,
    16346_i32 => _MORAZAN_S_DAY,
    16355_i32 => _COLUMBUS_DAY,
    16364_i32 => _ARMY_DAY,
    16429_i32 => _CHRISTMAS_DAY,
    16436_i32 => _NEW_YEAR_S_DAY,
    16527_i32 => _MAUNDY_THURSDAY,
    16528_i32 => _GOOD_FRIDAY,
    16529_i32 => _HOLY_SATURDAY,
    16539_i32 => _PANAMERICAN_DAY,
    16556_i32 => _LABOR_DAY,
    16693_i32 => _INDEPENDENCE_DAY,
    16715_i32 => _MORAZAN_WEEKEND,
    16716_i32 => _MORAZAN_WEEKEND,
    16717_i32 => _MORAZAN_WEEKEND,
    16794_i32 => _CHRISTMAS_DAY,
    16801_i32 => _NEW_YEAR_S_DAY,
    16884_i32 => _MAUNDY_THURSDAY,
    16885_i32 => _GOOD_FRIDAY,
    16886_i32 => _HOLY_SATURDAY,
    16905_i32 => _PANAMERICAN_DAY,
    16922_i32 => _LABOR_DAY,
    17059_i32 => _INDEPENDENCE_DAY,
    17079_i32 => _MORAZAN_WEEKEND,
    17080_i32 => _MORAZAN_WEEKEND,
    17081_i32 => _MORAZAN_WEEKEND,
    17160_i32 => _CHRISTMAS_DAY,
    17167_i32 => _NEW_YEAR_S_DAY,
    17269_i32 => _MAUNDY_THURSDAY,
    17270_i32 => _GOOD_FRIDAY__PANAMERICAN_DAY,
    17271_i32 => _HOLY_SATURDAY,
    17287_i32 => _LABOR_DAY,
    17424_i32 => _INDEPENDENCE_DAY,
    17443_i32 => _MORAZAN_WEEKEND,
    17444_i32 => _MORAZAN_WEEKEND,
    17445_i32 => _MORAZAN_WEEKEND,
    17525_i32 => _CHRISTMAS_DAY,
    17532_i32 => _NEW_YEAR_S_DAY,
    17619_i32 => _MAUNDY_THURSDAY,
    17620_i32 => _GOOD_FRIDAY,
    17621_i32 => _HOLY_SATURDAY,
    17635_i32 => _PANAMERICAN_DAY,
    17652_i32 => _LABOR_DAY,
    17789_i32 => _INDEPENDENCE_DAY,
    17807_i32 => _MORAZAN_WEEKEND,
    17808_i32 => _MORAZAN_WEEKEND,
    17809_i32 => _MORAZAN_WEEKEND,
    17890_i32 => _CHRISTMAS_DAY,
    17897_i32 => _NEW_YEAR_S_DAY,
    18000_i32 => _PANAMERICAN_DAY,
    18004_i32 => _MAUNDY_THURSDAY,
    18005_i32 => _GOOD_FRIDAY,
    18006_i32 => _HOLY_SATURDAY,
    18017_i32 => _LABOR_DAY,
    18154_i32 => _INDEPENDENCE_DAY,
    18171_i32 => _MORAZAN_WEEKEND,
    18172_i32 => _MORAZAN_WEEKEND,
    18173_i32 => _MORAZAN_WEEKEND,
    18255_i32 => _CHRISTMAS_DAY,
    18262_i32 => _NEW_YEAR_S_DAY,
    18361_i32 => _MAUNDY_THURSDAY,
    18362_i32 => _GOOD_FRIDAY,
    18363_i32 => _HOLY_SATURDAY,
    18366_i32 => _PANAMERICAN_DAY,
    18383_i32 => _LABOR_DAY,
    18520_i32 => _INDEPENDENCE_DAY,
    18542_i32 => _MORAZAN_WEEKEND,
    18543_i32 => _MORAZAN_WEEKEND,
    18544_i32 => _MORAZAN_WEEKEND,
    18621_i32 => _CHRISTMAS_DAY,
    18628_i32 => _NEW_YEAR_S_DAY,
    18718_i32 => _MAUNDY_THURSDAY,
    18719_i32 => _GOOD_FRIDAY,
    18720_i32 => _HOLY_SATURDAY,
    18731_i32 => _PANAMERICAN_DAY,
    18748_i32 => _LABOR_DAY,
    18885_i32 => _INDEPENDENCE_DAY,
    18906_i32 => _MORAZAN_WEEKEND,
    18907_i32 => _MORAZAN_WEEKEND,
    18908_i32 => _MORAZAN_WEEKEND,
    18986_i32 => _CHRISTMAS_DAY,
    18993_i32 => _NEW_YEAR_S_DAY,
    19096_i32 => _MAUNDY_THURSDAY__PANAMERICAN_DAY,
    19097_i32 => _GOOD_FRIDAY,
    19098_i32 => _HOLY_SATURDAY,
    19113_i32 => _LABOR_DAY,
    19250_i32 => _INDEPENDENCE_DAY,
    19270_i32 => _MORAZAN_WEEKEND,
    19271_i32 => _MORAZAN_WEEKEND,
    19272_i32 => _MORAZAN_WEEKEND,
    19351_i32 => _CHRISTMAS_DAY,
    19358_i32 => _NEW_YEAR_S_DAY,
    19453_i32 => _MAUNDY_THURSDAY,
    19454_i32 => _GOOD_FRIDAY,
    19455_i32 => _HOLY_SATURDAY,
    19461_i32 => _PANAMERICAN_DAY,
    19478_i32 => _LABOR_DAY,
    19615_i32 => _INDEPENDENCE_DAY,
    19634_i32 => _MORAZAN_WEEKEND,
    19635_i32 => _MORAZAN_WEEKEND,
    19636_i32 => _MORAZAN_WEEKEND,
    19716_i32 => _CHRISTMAS_DAY,
    19723_i32 => _NEW_YEAR_S_DAY,
    19810_i32 => _MAUNDY_THURSDAY,
    19811_i32 => _GOOD_FRIDAY,
    19812_i32 => _HOLY_SATURDAY,
    19827_i32 => _PANAMERICAN_DAY,
    19844_i32 => _LABOR_DAY,
    19981_i32 => _INDEPENDENCE_DAY,
    19998_i32 => _MORAZAN_WEEKEND,
    19999_i32 => _MORAZAN_WEEKEND,
    20000_i32 => _MORAZAN_WEEKEND,
    20082_i32 => _CHRISTMAS_DAY,
    20089_i32 => _NEW_YEAR_S_DAY,
    20192_i32 => _PANAMERICAN_DAY,
    20195_i32 => _MAUNDY_THURSDAY,
    20196_i32 => _GOOD_FRIDAY,
    20197_i32 => _HOLY_SATURDAY,
    20209_i32 => _LABOR_DAY,
    20346_i32 => _INDEPENDENCE_DAY,
    20362_i32 => _MORAZAN_WEEKEND,
    20363_i32 => _MORAZAN_WEEKEND,
    20364_i32 => _MORAZAN_WEEKEND,
    20447_i32 => _CHRISTMAS_DAY,
    20454_i32 => _NEW_YEAR_S_DAY,
    20545_i32 => _MAUNDY_THURSDAY,
    20546_i32 => _GOOD_FRIDAY,
    20547_i32 => _HOLY_SATURDAY,
    20557_i32 => _PANAMERICAN_DAY,
    20574_i32 => _LABOR_DAY,
    20711_i32 => _INDEPENDENCE_DAY,
    20733_i32 => _MORAZAN_WEEKEND,
    20734_i32 => _MORAZAN_WEEKEND,
    20735_i32 => _MORAZAN_WEEKEND,
    20812_i32 => _CHRISTMAS_DAY,
    20819_i32 => _NEW_YEAR_S_DAY,
    20902_i32 => _MAUNDY_THURSDAY,
    20903_i32 => _GOOD_FRIDAY,
    20904_i32 => _HOLY_SATURDAY,
    20922_i32 => _PANAMERICAN_DAY,
    20939_i32 => _LABOR_DAY,
    21076_i32 => _INDEPENDENCE_DAY,
    21097_i32 => _MORAZAN_WEEKEND,
    21098_i32 => _MORAZAN_WEEKEND,
    21099_i32 => _MORAZAN_WEEKEND,
    21177_i32 => _CHRISTMAS_DAY,
    21184_i32 => _NEW_YEAR_S_DAY,
    21287_i32 => _MAUNDY_THURSDAY,
    21288_i32 => _GOOD_FRIDAY__PANAMERICAN_DAY,
    21289_i32 => _HOLY_SATURDAY,
    21305_i32 => _LABOR_DAY,
    21442_i32 => _INDEPENDENCE_DAY,
    21461_i32 => _MORAZAN_WEEKEND,
    21462_i32 => _MORAZAN_WEEKEND,
    21463_i32 => _MORAZAN_WEEKEND,
    21543_i32 => _CHRISTMAS_DAY,
    21550_i32 => _NEW_YEAR_S_DAY,
    21637_i32 => _MAUNDY_THURSDAY,
    21638_i32 => _GOOD_FRIDAY,
    21639_i32 => _HOLY_SATURDAY,
    21653_i32 => _PANAMERICAN_DAY,
    21670_i32 => _LABOR_DAY,
    21807_i32 => _INDEPENDENCE_DAY,
    21825_i32 => _MORAZAN_WEEKEND,
    21826_i32 => _MORAZAN_WEEKEND,
    21827_i32 => _MORAZAN_WEEKEND,
    21908_i32 => _CHRISTMAS_DAY,
    21915_i32 => _NEW_YEAR_S_DAY,
    22018_i32 => _PANAMERICAN_DAY,
    22022_i32 => _MAUNDY_THURSDAY,
    22023_i32 => _GOOD_FRIDAY,
    22024_i32 => _HOLY_SATURDAY,
    22035_i32 => _LABOR_DAY,
    22172_i32 => _INDEPENDENCE_DAY,
    22189_i32 => _MORAZAN_WEEKEND,
    22190_i32 => _MORAZAN_WEEKEND,
    22191_i32 => _MORAZAN_WEEKEND,
    22273_i32 => _CHRISTMAS_DAY,
    22280_i32 => _NEW_YEAR_S_DAY,
    22379_i32 => _MAUNDY_THURSDAY,
    22380_i32 => _GOOD_FRIDAY,
    22381_i32 => _HOLY_SATURDAY,
    22383_i32 => _PANAMERICAN_DAY,
    22400_i32 => _LABOR_DAY,
    22537_i32 => _INDEPENDENCE_DAY,
    22553_i32 => _MORAZAN_WEEKEND,
    22554_i32 => _MORAZAN_WEEKEND,
    22555_i32 => _MORAZAN_WEEKEND,
    22638_i32 => _CHRISTMAS_DAY,
    22645_i32 => _NEW_YEAR_S_DAY,
    22729_i32 => _MAUNDY_THURSDAY,
    22730_i32 => _GOOD_FRIDAY,
    22731_i32 => _HOLY_SATURDAY,
    22749_i32 => _PANAMERICAN_DAY,
    22766_i32 => _LABOR_DAY,
    22903_i32 => _INDEPENDENCE_DAY,
    22924_i32 => _MORAZAN_WEEKEND,
    22925_i32 => _MORAZAN_WEEKEND,
    22926_i32 => _MORAZAN_WEEKEND,
    23004_i32 => _CHRISTMAS_DAY,
    23011_i32 => _NEW_YEAR_S_DAY,
    23114_i32 => _MAUNDY_THURSDAY__PANAMERICAN_DAY,
    23115_i32 => _GOOD_FRIDAY,
    23116_i32 => _HOLY_SATURDAY,
    23131_i32 => _LABOR_DAY,
    23268_i32 => _INDEPENDENCE_DAY,
    23288_i32 => _MORAZAN_WEEKEND,
    23289_i32 => _MORAZAN_WEEKEND,
    23290_i32 => _MORAZAN_WEEKEND,
    23369_i32 => _CHRISTMAS_DAY,
    23376_i32 => _NEW_YEAR_S_DAY,
    23471_i32 => _MAUNDY_THURSDAY,
    23472_i32 => _GOOD_FRIDAY,
    23473_i32 => _HOLY_SATURDAY,
    23479_i32 => _PANAMERICAN_DAY,
    23496_i32 => _LABOR_DAY,
    23633_i32 => _INDEPENDENCE_DAY,
    23652_i32 => _MORAZAN_WEEKEND,
    23653_i32 => _MORAZAN_WEEKEND,
    23654_i32 => _MORAZAN_WEEKEND,
    23734_i32 => _CHRISTMAS_DAY,
    23741_i32 => _NEW_YEAR_S_DAY,
    23821_i32 => _MAUNDY_THURSDAY,
    23822_i32 => _GOOD_FRIDAY,
    23823_i32 => _HOLY_SATURDAY,
    23844_i32 => _PANAMERICAN_DAY,
    23861_i32 => _LABOR_DAY,
    23998_i32 => _INDEPENDENCE_DAY,
    24016_i32 => _MORAZAN_WEEKEND,
    24017_i32 => _MORAZAN_WEEKEND,
    24018_i32 => _MORAZAN_WEEKEND,
    24099_i32 => _CHRISTMAS_DAY,
    24106_i32 => _NEW_YEAR_S_DAY,
    24206_i32 => _MAUNDY_THURSDAY,
    24207_i32 => _GOOD_FRIDAY,
    24208_i32 => _HOLY_SATURDAY,
    24210_i32 => _PANAMERICAN_DAY,
    24227_i32 => _LABOR_DAY,
    24364_i32 => _INDEPENDENCE_DAY,
    24380_i32 => _MORAZAN_WEEKEND,
    24381_i32 => _MORAZAN_WEEKEND,
    24382_i32 => _MORAZAN_WEEKEND,
    24465_i32 => _CHRISTMAS_DAY,
    24472_i32 => _NEW_YEAR_S_DAY,
    24563_i32 => _MAUNDY_THURSDAY,
    24564_i32 => _GOOD_FRIDAY,
    24565_i32 => _HOLY_SATURDAY,
    24575_i32 => _PANAMERICAN_DAY,
    24592_i32 => _LABOR_DAY,
    24729_i32 => _INDEPENDENCE_DAY,
    24751_i32 => _MORAZAN_WEEKEND,
    24752_i32 => _MORAZAN_WEEKEND,
    24753_i32 => _MORAZAN_WEEKEND,
    24830_i32 => _CHRISTMAS_DAY,
    24837_i32 => _NEW_YEAR_S_DAY,
    24940_i32 => _PANAMERICAN_DAY,
    24948_i32 => _MAUNDY_THURSDAY,
    24949_i32 => _GOOD_FRIDAY,
    24950_i32 => _HOLY_SATURDAY,
    24957_i32 => _LABOR_DAY,
    25094_i32 => _INDEPENDENCE_DAY,
    25115_i32 => _MORAZAN_WEEKEND,
    25116_i32 => _MORAZAN_WEEKEND,
    25117_i32 => _MORAZAN_WEEKEND,
    25195_i32 => _CHRISTMAS_DAY,
    25202_i32 => _NEW_YEAR_S_DAY,
    25298_i32 => _MAUNDY_THURSDAY,
    25299_i32 => _GOOD_FRIDAY,
    25300_i32 => _HOLY_SATURDAY,
    25305_i32 => _PANAMERICAN_DAY,
    25322_i32 => _LABOR_DAY,
    25459_i32 => _INDEPENDENCE_DAY,
    25479_i32 => _MORAZAN_WEEKEND,
    25480_i32 => _MORAZAN_WEEKEND,
    25481_i32 => _MORAZAN_WEEKEND,
    25560_i32 => _CHRISTMAS_DAY,
    25567_i32 => _NEW_YEAR_S_DAY,
    25655_i32 => _MAUNDY_THURSDAY,
    25656_i32 => _GOOD_FRIDAY,
    25657_i32 => _HOLY_SATURDAY,
    25671_i32 => _PANAMERICAN_DAY,
    25688_i32 => _LABOR_DAY,
    25825_i32 => _INDEPENDENCE_DAY,
    25843_i32 => _MORAZAN_WEEKEND,
    25844_i32 => _MORAZAN_WEEKEND,
    25845_i32 => _MORAZAN_WEEKEND,
    25926_i32 => _CHRISTMAS_DAY,
    25933_i32 => _NEW_YEAR_S_DAY,
    26036_i32 => _PANAMERICAN_DAY,
    26040_i32 => _MAUNDY_THURSDAY,
    26041_i32 => _GOOD_FRIDAY,
    26042_i32 => _HOLY_SATURDAY,
    26053_i32 => _LABOR_DAY,
    26190_i32 => _INDEPENDENCE_DAY,
    26207_i32 => _MORAZAN_WEEKEND,
    26208_i32 => _MORAZAN_WEEKEND,
    26209_i32 => _MORAZAN_WEEKEND,
    26291_i32 => _CHRISTMAS_DAY,
    26298_i32 => _NEW_YEAR_S_DAY,
    26390_i32 => _MAUNDY_THURSDAY,
    26391_i32 => _GOOD_FRIDAY,
    26392_i32 => _HOLY_SATURDAY,
    26401_i32 => _PANAMERICAN_DAY,
    26418_i32 => _LABOR_DAY,
    26555_i32 => _INDEPENDENCE_DAY,
    26571_i32 => _MORAZAN_WEEKEND,
    26572_i32 => _MORAZAN_WEEKEND,
    26573_i32 => _MORAZAN_WEEKEND,
    26656_i32 => _CHRISTMAS_DAY,
    26663_i32 => _NEW_YEAR_S_DAY,
    26747_i32 => _MAUNDY_THURSDAY,
    26748_i32 => _GOOD_FRIDAY,
    26749_i32 => _HOLY_SATURDAY,
    26766_i32 => _PANAMERICAN_DAY,
    26783_i32 => _LABOR_DAY,
    26920_i32 => _INDEPENDENCE_DAY,
    26942_i32 => _MORAZAN_WEEKEND,
    26943_i32 => _MORAZAN_WEEKEND,
    26944_i32 => _MORAZAN_WEEKEND,
    27021_i32 => _CHRISTMAS_DAY,
    27028_i32 => _NEW_YEAR_S_DAY,
    27132_i32 => _MAUNDY_THURSDAY__PANAMERICAN_DAY,
    27133_i32 => _GOOD_FRIDAY,
    27134_i32 => _HOLY_SATURDAY,
    27149_i32 => _LABOR_DAY,
    27286_i32 => _INDEPENDENCE_DAY,
    27306_i32 => _MORAZAN_WEEKEND,
    27307_i32 => _MORAZAN_WEEKEND,
    27308_i32 => _MORAZAN_WEEKEND,
    27387_i32 => _CHRISTMAS_DAY,
    27394_i32 => _NEW_YEAR_S_DAY,
    27489_i32 => _MAUNDY_THURSDAY,
    27490_i32 => _GOOD_FRIDAY,
    27491_i32 => _HOLY_SATURDAY,
    27497_i32 => _PANAMERICAN_DAY,
    27514_i32 => _LABOR_DAY,
    27651_i32 => _INDEPENDENCE_DAY,
    27670_i32 => _MORAZAN_WEEKEND,
    27671_i32 => _MORAZAN_WEEKEND,
    27672_i32 => _MORAZAN_WEEKEND,
    27752_i32 => _CHRISTMAS_DAY,
    27759_i32 => _NEW_YEAR_S_DAY,
    27839_i32 => _MAUNDY_THURSDAY,
    27840_i32 => _GOOD_FRIDAY,
    27841_i32 => _HOLY_SATURDAY,
    27862_i32 => _PANAMERICAN_DAY,
    27879_i32 => _LABOR_DAY,
    28016_i32 => _INDEPENDENCE_DAY,
    28034_i32 => _MORAZAN_WEEKEND,
    28035_i32 => _MORAZAN_WEEKEND,
    28036_i32 => _MORAZAN_WEEKEND,
    28117_i32 => _CHRISTMAS_DAY,
    28124_i32 => _NEW_YEAR_S_DAY,
    28224_i32 => _MAUNDY_THURSDAY,
    28225_i32 => _GOOD_FRIDAY,
    28226_i32 => _HOLY_SATURDAY,
    28227_i32 => _PANAMERICAN_DAY,
    28244_i32 => _LABOR_DAY,
    28381_i32 => _INDEPENDENCE_DAY,
    28398_i32 => _MORAZAN_WEEKEND,
    28399_i32 => _MORAZAN_WEEKEND,
    28400_i32 => _MORAZAN_WEEKEND,
    28482_i32 => _CHRISTMAS_DAY,
    28489_i32 => _NEW_YEAR_S_DAY,
    28581_i32 => _MAUNDY_THURSDAY,
    28582_i32 => _GOOD_FRIDAY,
    28583_i32 => _HOLY_SATURDAY,
    28593_i32 => _PANAMERICAN_DAY,
    28610_i32 => _LABOR_DAY,
    28747_i32 => _INDEPENDENCE_DAY,
    28769_i32 => _MORAZAN_WEEKEND,
    28770_i32 => _MORAZAN_WEEKEND,
    28771_i32 => _MORAZAN_WEEKEND,
    28848_i32 => _CHRISTMAS_DAY,
    28855_i32 => _NEW_YEAR_S_DAY,
    28958_i32 => _PANAMERICAN_DAY,
    28959_i32 => _MAUNDY_THURSDAY,
    28960_i32 => _GOOD_FRIDAY,
    28961_i32 => _HOLY_SATURDAY,
    28975_i32 => _LABOR_DAY,
    29112_i32 => _INDEPENDENCE_DAY,
    29133_i32 => _MORAZAN_WEEKEND,
    29134_i32 => _MORAZAN_WEEKEND,
    29135_i32 => _MORAZAN_WEEKEND,
    29213_i32 => _CHRISTMAS_DAY,
    29220_i32 => _NEW_YEAR_S_DAY,
};
