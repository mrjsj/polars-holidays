use crate::countries::constants::*;
use phf::phf_map;

pub static MZ_HOLIDAYS: phf::Map<i32, &'static str> = phf_map! {
    10957_i32 => _INTERNATIONAL_FRATERNALISM_DAY,
    10990_i32 => _HEROES__DAY,
    11054_i32 => _WOMEN_S_DAY,
    11078_i32 => _INTERNATIONAL_WORKERS__DAY,
    11133_i32 => _INDEPENDENCE_DAY,
    11134_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    11207_i32 => _VICTORY_DAY,
    11225_i32 => _ARMED_FORCES_DAY,
    11234_i32 => _PEACE_AND_RECONCILIATION_DAY,
    11316_i32 => _FAMILY_DAY,
    11323_i32 => _INTERNATIONAL_FRATERNALISM_DAY,
    11356_i32 => _HEROES__DAY,
    11419_i32 => _WOMEN_S_DAY,
    11443_i32 => _INTERNATIONAL_WORKERS__DAY,
    11498_i32 => _INDEPENDENCE_DAY,
    11572_i32 => _VICTORY_DAY,
    11590_i32 => _ARMED_FORCES_DAY,
    11599_i32 => _PEACE_AND_RECONCILIATION_DAY,
    11681_i32 => _FAMILY_DAY,
    11688_i32 => _INTERNATIONAL_FRATERNALISM_DAY,
    11721_i32 => _HEROES__DAY,
    11722_i32 => _HEROES__DAY__OBSERVED_,
    11784_i32 => _WOMEN_S_DAY,
    11785_i32 => _WOMEN_S_DAY__OBSERVED_,
    11808_i32 => _INTERNATIONAL_WORKERS__DAY,
    11863_i32 => _INDEPENDENCE_DAY,
    11937_i32 => _VICTORY_DAY,
    11955_i32 => _ARMED_FORCES_DAY,
    11964_i32 => _PEACE_AND_RECONCILIATION_DAY,
    12046_i32 => _FAMILY_DAY,
    12053_i32 => _INTERNATIONAL_FRATERNALISM_DAY,
    12086_i32 => _HEROES__DAY,
    12149_i32 => _WOMEN_S_DAY,
    12173_i32 => _INTERNATIONAL_WORKERS__DAY,
    12228_i32 => _INDEPENDENCE_DAY,
    12302_i32 => _VICTORY_DAY,
    12303_i32 => _VICTORY_DAY__OBSERVED_,
    12320_i32 => _ARMED_FORCES_DAY,
    12329_i32 => _PEACE_AND_RECONCILIATION_DAY,
    12411_i32 => _FAMILY_DAY,
    12418_i32 => _INTERNATIONAL_FRATERNALISM_DAY,
    12451_i32 => _HEROES__DAY,
    12515_i32 => _WOMEN_S_DAY,
    12539_i32 => _INTERNATIONAL_WORKERS__DAY,
    12594_i32 => _INDEPENDENCE_DAY,
    12668_i32 => _VICTORY_DAY,
    12686_i32 => _ARMED_FORCES_DAY,
    12695_i32 => _PEACE_AND_RECONCILIATION_DAY,
    12777_i32 => _FAMILY_DAY,
    12784_i32 => _INTERNATIONAL_FRATERNALISM_DAY,
    12817_i32 => _HEROES__DAY,
    12880_i32 => _WOMEN_S_DAY,
    12904_i32 => _INTERNATIONAL_WORKERS__DAY,
    12905_i32 => _INTERNATIONAL_WORKERS__DAY__OBSERVED_,
    12959_i32 => _INDEPENDENCE_DAY,
    13033_i32 => _VICTORY_DAY,
    13051_i32 => _ARMED_FORCES_DAY,
    13052_i32 => _ARMED_FORCES_DAY__OBSERVED_,
    13060_i32 => _PEACE_AND_RECONCILIATION_DAY,
    13142_i32 => _FAMILY_DAY,
    13143_i32 => _FAMILY_DAY__OBSERVED_,
    13149_i32 => _INTERNATIONAL_FRATERNALISM_DAY,
    13150_i32 => _INTERNATIONAL_FRATERNALISM_DAY__OBSERVED_,
    13182_i32 => _HEROES__DAY,
    13245_i32 => _WOMEN_S_DAY,
    13269_i32 => _INTERNATIONAL_WORKERS__DAY,
    13324_i32 => _INDEPENDENCE_DAY,
    13325_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    13398_i32 => _VICTORY_DAY,
    13416_i32 => _ARMED_FORCES_DAY,
    13425_i32 => _PEACE_AND_RECONCILIATION_DAY,
    13507_i32 => _FAMILY_DAY,
    13514_i32 => _INTERNATIONAL_FRATERNALISM_DAY,
    13547_i32 => _HEROES__DAY,
    13610_i32 => _WOMEN_S_DAY,
    13634_i32 => _INTERNATIONAL_WORKERS__DAY,
    13689_i32 => _INDEPENDENCE_DAY,
    13763_i32 => _VICTORY_DAY,
    13781_i32 => _ARMED_FORCES_DAY,
    13790_i32 => _PEACE_AND_RECONCILIATION_DAY,
    13872_i32 => _FAMILY_DAY,
    13879_i32 => _INTERNATIONAL_FRATERNALISM_DAY,
    13912_i32 => _HEROES__DAY,
    13913_i32 => _HEROES__DAY__OBSERVED_,
    13976_i32 => _WOMEN_S_DAY,
    14000_i32 => _INTERNATIONAL_WORKERS__DAY,
    14055_i32 => _INDEPENDENCE_DAY,
    14129_i32 => _VICTORY_DAY,
    14130_i32 => _VICTORY_DAY__OBSERVED_,
    14147_i32 => _ARMED_FORCES_DAY,
    14156_i32 => _PEACE_AND_RECONCILIATION_DAY,
    14238_i32 => _FAMILY_DAY,
    14245_i32 => _INTERNATIONAL_FRATERNALISM_DAY,
    14278_i32 => _HEROES__DAY,
    14341_i32 => _WOMEN_S_DAY,
    14365_i32 => _INTERNATIONAL_WORKERS__DAY,
    14420_i32 => _INDEPENDENCE_DAY,
    14494_i32 => _VICTORY_DAY,
    14512_i32 => _ARMED_FORCES_DAY,
    14521_i32 => _PEACE_AND_RECONCILIATION_DAY,
    14522_i32 => _PEACE_AND_RECONCILIATION_DAY__OBSERVED_,
    14603_i32 => _FAMILY_DAY,
    14610_i32 => _INTERNATIONAL_FRATERNALISM_DAY,
    14643_i32 => _HEROES__DAY,
    14706_i32 => _WOMEN_S_DAY,
    14730_i32 => _INTERNATIONAL_WORKERS__DAY,
    14785_i32 => _INDEPENDENCE_DAY,
    14859_i32 => _VICTORY_DAY,
    14877_i32 => _ARMED_FORCES_DAY,
    14886_i32 => _PEACE_AND_RECONCILIATION_DAY,
    14968_i32 => _FAMILY_DAY,
    14975_i32 => _INTERNATIONAL_FRATERNALISM_DAY,
    15008_i32 => _HEROES__DAY,
    15071_i32 => _WOMEN_S_DAY,
    15095_i32 => _INTERNATIONAL_WORKERS__DAY,
    15096_i32 => _INTERNATIONAL_WORKERS__DAY__OBSERVED_,
    15150_i32 => _INDEPENDENCE_DAY,
    15224_i32 => _VICTORY_DAY,
    15242_i32 => _ARMED_FORCES_DAY,
    15243_i32 => _ARMED_FORCES_DAY__OBSERVED_,
    15251_i32 => _PEACE_AND_RECONCILIATION_DAY,
    15333_i32 => _FAMILY_DAY,
    15334_i32 => _FAMILY_DAY__OBSERVED_,
    15340_i32 => _INTERNATIONAL_FRATERNALISM_DAY,
    15341_i32 => _INTERNATIONAL_FRATERNALISM_DAY__OBSERVED_,
    15373_i32 => _HEROES__DAY,
    15437_i32 => _WOMEN_S_DAY,
    15461_i32 => _INTERNATIONAL_WORKERS__DAY,
    15516_i32 => _INDEPENDENCE_DAY,
    15590_i32 => _VICTORY_DAY,
    15608_i32 => _ARMED_FORCES_DAY,
    15617_i32 => _PEACE_AND_RECONCILIATION_DAY,
    15699_i32 => _FAMILY_DAY,
    15706_i32 => _INTERNATIONAL_FRATERNALISM_DAY,
    15739_i32 => _HEROES__DAY,
    15740_i32 => _HEROES__DAY__OBSERVED_,
    15802_i32 => _WOMEN_S_DAY,
    15803_i32 => _WOMEN_S_DAY__OBSERVED_,
    15826_i32 => _INTERNATIONAL_WORKERS__DAY,
    15881_i32 => _INDEPENDENCE_DAY,
    15955_i32 => _VICTORY_DAY,
    15973_i32 => _ARMED_FORCES_DAY,
    15982_i32 => _PEACE_AND_RECONCILIATION_DAY,
    16064_i32 => _FAMILY_DAY,
    16071_i32 => _INTERNATIONAL_FRATERNALISM_DAY,
    16104_i32 => _HEROES__DAY,
    16167_i32 => _WOMEN_S_DAY,
    16191_i32 => _INTERNATIONAL_WORKERS__DAY,
    16246_i32 => _INDEPENDENCE_DAY,
    16320_i32 => _VICTORY_DAY,
    16321_i32 => _VICTORY_DAY__OBSERVED_,
    16338_i32 => _ARMED_FORCES_DAY,
    16347_i32 => _PEACE_AND_RECONCILIATION_DAY,
    16429_i32 => _FAMILY_DAY,
    16436_i32 => _INTERNATIONAL_FRATERNALISM_DAY,
    16469_i32 => _HEROES__DAY,
    16532_i32 => _WOMEN_S_DAY,
    16556_i32 => _INTERNATIONAL_WORKERS__DAY,
    16611_i32 => _INDEPENDENCE_DAY,
    16685_i32 => _VICTORY_DAY,
    16703_i32 => _ARMED_FORCES_DAY,
    16712_i32 => _PEACE_AND_RECONCILIATION_DAY,
    16713_i32 => _PEACE_AND_RECONCILIATION_DAY__OBSERVED_,
    16794_i32 => _FAMILY_DAY,
    16801_i32 => _INTERNATIONAL_FRATERNALISM_DAY,
    16834_i32 => _HEROES__DAY,
    16898_i32 => _WOMEN_S_DAY,
    16922_i32 => _INTERNATIONAL_WORKERS__DAY,
    16923_i32 => _INTERNATIONAL_WORKERS__DAY__OBSERVED_,
    16977_i32 => _INDEPENDENCE_DAY,
    17051_i32 => _VICTORY_DAY,
    17069_i32 => _ARMED_FORCES_DAY,
    17070_i32 => _ARMED_FORCES_DAY__OBSERVED_,
    17078_i32 => _PEACE_AND_RECONCILIATION_DAY,
    17160_i32 => _FAMILY_DAY,
    17161_i32 => _FAMILY_DAY__OBSERVED_,
    17167_i32 => _INTERNATIONAL_FRATERNALISM_DAY,
    17168_i32 => _INTERNATIONAL_FRATERNALISM_DAY__OBSERVED_,
    17200_i32 => _HEROES__DAY,
    17263_i32 => _WOMEN_S_DAY,
    17287_i32 => _INTERNATIONAL_WORKERS__DAY,
    17342_i32 => _INDEPENDENCE_DAY,
    17343_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    17416_i32 => _VICTORY_DAY,
    17434_i32 => _ARMED_FORCES_DAY,
    17443_i32 => _PEACE_AND_RECONCILIATION_DAY,
    17525_i32 => _FAMILY_DAY,
    17532_i32 => _INTERNATIONAL_FRATERNALISM_DAY,
    17565_i32 => _HEROES__DAY,
    17628_i32 => _WOMEN_S_DAY,
    17652_i32 => _INTERNATIONAL_WORKERS__DAY,
    17707_i32 => _INDEPENDENCE_DAY,
    17781_i32 => _VICTORY_DAY,
    17799_i32 => _ARMED_FORCES_DAY,
    17808_i32 => _PEACE_AND_RECONCILIATION_DAY,
    17890_i32 => _FAMILY_DAY,
    17897_i32 => _INTERNATIONAL_FRATERNALISM_DAY,
    17930_i32 => _HEROES__DAY,
    17931_i32 => _HEROES__DAY__OBSERVED_,
    17993_i32 => _WOMEN_S_DAY,
    17994_i32 => _WOMEN_S_DAY__OBSERVED_,
    18017_i32 => _INTERNATIONAL_WORKERS__DAY,
    18072_i32 => _INDEPENDENCE_DAY,
    18146_i32 => _VICTORY_DAY,
    18164_i32 => _ARMED_FORCES_DAY,
    18173_i32 => _PEACE_AND_RECONCILIATION_DAY,
    18255_i32 => _FAMILY_DAY,
    18262_i32 => _INTERNATIONAL_FRATERNALISM_DAY,
    18295_i32 => _HEROES__DAY,
    18359_i32 => _WOMEN_S_DAY,
    18383_i32 => _INTERNATIONAL_WORKERS__DAY,
    18438_i32 => _INDEPENDENCE_DAY,
    18512_i32 => _VICTORY_DAY,
    18530_i32 => _ARMED_FORCES_DAY,
    18539_i32 => _PEACE_AND_RECONCILIATION_DAY,
    18540_i32 => _PEACE_AND_RECONCILIATION_DAY__OBSERVED_,
    18621_i32 => _FAMILY_DAY,
    18628_i32 => _INTERNATIONAL_FRATERNALISM_DAY,
    18661_i32 => _HEROES__DAY,
    18724_i32 => _WOMEN_S_DAY,
    18748_i32 => _INTERNATIONAL_WORKERS__DAY,
    18803_i32 => _INDEPENDENCE_DAY,
    18877_i32 => _VICTORY_DAY,
    18895_i32 => _ARMED_FORCES_DAY,
    18904_i32 => _PEACE_AND_RECONCILIATION_DAY,
    18986_i32 => _FAMILY_DAY,
    18993_i32 => _INTERNATIONAL_FRATERNALISM_DAY,
    19026_i32 => _HEROES__DAY,
    19089_i32 => _WOMEN_S_DAY,
    19113_i32 => _INTERNATIONAL_WORKERS__DAY,
    19114_i32 => _INTERNATIONAL_WORKERS__DAY__OBSERVED_,
    19168_i32 => _INDEPENDENCE_DAY,
    19242_i32 => _VICTORY_DAY,
    19260_i32 => _ARMED_FORCES_DAY,
    19261_i32 => _ARMED_FORCES_DAY__OBSERVED_,
    19269_i32 => _PEACE_AND_RECONCILIATION_DAY,
    19351_i32 => _FAMILY_DAY,
    19352_i32 => _FAMILY_DAY__OBSERVED_,
    19358_i32 => _INTERNATIONAL_FRATERNALISM_DAY,
    19359_i32 => _INTERNATIONAL_FRATERNALISM_DAY__OBSERVED_,
    19391_i32 => _HEROES__DAY,
    19454_i32 => _WOMEN_S_DAY,
    19478_i32 => _INTERNATIONAL_WORKERS__DAY,
    19533_i32 => _INDEPENDENCE_DAY,
    19534_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    19607_i32 => _VICTORY_DAY,
    19625_i32 => _ARMED_FORCES_DAY,
    19634_i32 => _PEACE_AND_RECONCILIATION_DAY,
    19716_i32 => _FAMILY_DAY,
    19723_i32 => _INTERNATIONAL_FRATERNALISM_DAY,
    19756_i32 => _HEROES__DAY,
    19820_i32 => _WOMEN_S_DAY,
    19821_i32 => _WOMEN_S_DAY__OBSERVED_,
    19844_i32 => _INTERNATIONAL_WORKERS__DAY,
    19899_i32 => _INDEPENDENCE_DAY,
    19973_i32 => _VICTORY_DAY,
    19991_i32 => _ARMED_FORCES_DAY,
    20000_i32 => _PEACE_AND_RECONCILIATION_DAY,
    20082_i32 => _FAMILY_DAY,
    20089_i32 => _INTERNATIONAL_FRATERNALISM_DAY,
    20122_i32 => _HEROES__DAY,
    20185_i32 => _WOMEN_S_DAY,
    20209_i32 => _INTERNATIONAL_WORKERS__DAY,
    20264_i32 => _INDEPENDENCE_DAY,
    20338_i32 => _VICTORY_DAY,
    20339_i32 => _VICTORY_DAY__OBSERVED_,
    20356_i32 => _ARMED_FORCES_DAY,
    20365_i32 => _PEACE_AND_RECONCILIATION_DAY,
    20447_i32 => _FAMILY_DAY,
    20454_i32 => _INTERNATIONAL_FRATERNALISM_DAY,
    20487_i32 => _HEROES__DAY,
    20550_i32 => _WOMEN_S_DAY,
    20574_i32 => _INTERNATIONAL_WORKERS__DAY,
    20629_i32 => _INDEPENDENCE_DAY,
    20703_i32 => _VICTORY_DAY,
    20721_i32 => _ARMED_FORCES_DAY,
    20730_i32 => _PEACE_AND_RECONCILIATION_DAY,
    20731_i32 => _PEACE_AND_RECONCILIATION_DAY__OBSERVED_,
    20812_i32 => _FAMILY_DAY,
    20819_i32 => _INTERNATIONAL_FRATERNALISM_DAY,
    20852_i32 => _HEROES__DAY,
    20915_i32 => _WOMEN_S_DAY,
    20939_i32 => _INTERNATIONAL_WORKERS__DAY,
    20994_i32 => _INDEPENDENCE_DAY,
    21068_i32 => _VICTORY_DAY,
    21086_i32 => _ARMED_FORCES_DAY,
    21095_i32 => _PEACE_AND_RECONCILIATION_DAY,
    21177_i32 => _FAMILY_DAY,
    21184_i32 => _INTERNATIONAL_FRATERNALISM_DAY,
    21217_i32 => _HEROES__DAY,
    21281_i32 => _WOMEN_S_DAY,
    21305_i32 => _INTERNATIONAL_WORKERS__DAY,
    21360_i32 => _INDEPENDENCE_DAY,
    21361_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    21434_i32 => _VICTORY_DAY,
    21452_i32 => _ARMED_FORCES_DAY,
    21461_i32 => _PEACE_AND_RECONCILIATION_DAY,
    21543_i32 => _FAMILY_DAY,
    21550_i32 => _INTERNATIONAL_FRATERNALISM_DAY,
    21583_i32 => _HEROES__DAY,
    21646_i32 => _WOMEN_S_DAY,
    21670_i32 => _INTERNATIONAL_WORKERS__DAY,
    21725_i32 => _INDEPENDENCE_DAY,
    21799_i32 => _VICTORY_DAY,
    21817_i32 => _ARMED_FORCES_DAY,
    21826_i32 => _PEACE_AND_RECONCILIATION_DAY,
    21908_i32 => _FAMILY_DAY,
    21915_i32 => _INTERNATIONAL_FRATERNALISM_DAY,
    21948_i32 => _HEROES__DAY,
    21949_i32 => _HEROES__DAY__OBSERVED_,
    22011_i32 => _WOMEN_S_DAY,
    22012_i32 => _WOMEN_S_DAY__OBSERVED_,
    22035_i32 => _INTERNATIONAL_WORKERS__DAY,
    22090_i32 => _INDEPENDENCE_DAY,
    22164_i32 => _VICTORY_DAY,
    22182_i32 => _ARMED_FORCES_DAY,
    22191_i32 => _PEACE_AND_RECONCILIATION_DAY,
    22273_i32 => _FAMILY_DAY,
    22280_i32 => _INTERNATIONAL_FRATERNALISM_DAY,
    22313_i32 => _HEROES__DAY,
    22376_i32 => _WOMEN_S_DAY,
    22400_i32 => _INTERNATIONAL_WORKERS__DAY,
    22455_i32 => _INDEPENDENCE_DAY,
    22529_i32 => _VICTORY_DAY,
    22530_i32 => _VICTORY_DAY__OBSERVED_,
    22547_i32 => _ARMED_FORCES_DAY,
    22556_i32 => _PEACE_AND_RECONCILIATION_DAY,
    22638_i32 => _FAMILY_DAY,
    22645_i32 => _INTERNATIONAL_FRATERNALISM_DAY,
    22678_i32 => _HEROES__DAY,
    22742_i32 => _WOMEN_S_DAY,
    22766_i32 => _INTERNATIONAL_WORKERS__DAY,
    22821_i32 => _INDEPENDENCE_DAY,
    22895_i32 => _VICTORY_DAY,
    22913_i32 => _ARMED_FORCES_DAY,
    22922_i32 => _PEACE_AND_RECONCILIATION_DAY,
    23004_i32 => _FAMILY_DAY,
    23011_i32 => _INTERNATIONAL_FRATERNALISM_DAY,
    23044_i32 => _HEROES__DAY,
    23107_i32 => _WOMEN_S_DAY,
    23131_i32 => _INTERNATIONAL_WORKERS__DAY,
    23132_i32 => _INTERNATIONAL_WORKERS__DAY__OBSERVED_,
    23186_i32 => _INDEPENDENCE_DAY,
    23260_i32 => _VICTORY_DAY,
    23278_i32 => _ARMED_FORCES_DAY,
    23279_i32 => _ARMED_FORCES_DAY__OBSERVED_,
    23287_i32 => _PEACE_AND_RECONCILIATION_DAY,
    23369_i32 => _FAMILY_DAY,
    23370_i32 => _FAMILY_DAY__OBSERVED_,
    23376_i32 => _INTERNATIONAL_FRATERNALISM_DAY,
    23377_i32 => _INTERNATIONAL_FRATERNALISM_DAY__OBSERVED_,
    23409_i32 => _HEROES__DAY,
    23472_i32 => _WOMEN_S_DAY,
    23496_i32 => _INTERNATIONAL_WORKERS__DAY,
    23551_i32 => _INDEPENDENCE_DAY,
    23552_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    23625_i32 => _VICTORY_DAY,
    23643_i32 => _ARMED_FORCES_DAY,
    23652_i32 => _PEACE_AND_RECONCILIATION_DAY,
    23734_i32 => _FAMILY_DAY,
    23741_i32 => _INTERNATIONAL_FRATERNALISM_DAY,
    23774_i32 => _HEROES__DAY,
    23837_i32 => _WOMEN_S_DAY,
    23861_i32 => _INTERNATIONAL_WORKERS__DAY,
    23916_i32 => _INDEPENDENCE_DAY,
    23990_i32 => _VICTORY_DAY,
    24008_i32 => _ARMED_FORCES_DAY,
    24017_i32 => _PEACE_AND_RECONCILIATION_DAY,
    24099_i32 => _FAMILY_DAY,
    24106_i32 => _INTERNATIONAL_FRATERNALISM_DAY,
    24139_i32 => _HEROES__DAY,
    24140_i32 => _HEROES__DAY__OBSERVED_,
    24203_i32 => _WOMEN_S_DAY,
    24227_i32 => _INTERNATIONAL_WORKERS__DAY,
    24282_i32 => _INDEPENDENCE_DAY,
    24356_i32 => _VICTORY_DAY,
    24357_i32 => _VICTORY_DAY__OBSERVED_,
    24374_i32 => _ARMED_FORCES_DAY,
    24383_i32 => _PEACE_AND_RECONCILIATION_DAY,
    24465_i32 => _FAMILY_DAY,
    24472_i32 => _INTERNATIONAL_FRATERNALISM_DAY,
    24505_i32 => _HEROES__DAY,
    24568_i32 => _WOMEN_S_DAY,
    24592_i32 => _INTERNATIONAL_WORKERS__DAY,
    24647_i32 => _INDEPENDENCE_DAY,
    24721_i32 => _VICTORY_DAY,
    24739_i32 => _ARMED_FORCES_DAY,
    24748_i32 => _PEACE_AND_RECONCILIATION_DAY,
    24749_i32 => _PEACE_AND_RECONCILIATION_DAY__OBSERVED_,
    24830_i32 => _FAMILY_DAY,
    24837_i32 => _INTERNATIONAL_FRATERNALISM_DAY,
    24870_i32 => _HEROES__DAY,
    24933_i32 => _WOMEN_S_DAY,
    24957_i32 => _INTERNATIONAL_WORKERS__DAY,
    25012_i32 => _INDEPENDENCE_DAY,
    25086_i32 => _VICTORY_DAY,
    25104_i32 => _ARMED_FORCES_DAY,
    25113_i32 => _PEACE_AND_RECONCILIATION_DAY,
    25195_i32 => _FAMILY_DAY,
    25202_i32 => _INTERNATIONAL_FRATERNALISM_DAY,
    25235_i32 => _HEROES__DAY,
    25298_i32 => _WOMEN_S_DAY,
    25322_i32 => _INTERNATIONAL_WORKERS__DAY,
    25323_i32 => _INTERNATIONAL_WORKERS__DAY__OBSERVED_,
    25377_i32 => _INDEPENDENCE_DAY,
    25451_i32 => _VICTORY_DAY,
    25469_i32 => _ARMED_FORCES_DAY,
    25470_i32 => _ARMED_FORCES_DAY__OBSERVED_,
    25478_i32 => _PEACE_AND_RECONCILIATION_DAY,
    25560_i32 => _FAMILY_DAY,
    25561_i32 => _FAMILY_DAY__OBSERVED_,
    25567_i32 => _INTERNATIONAL_FRATERNALISM_DAY,
    25568_i32 => _INTERNATIONAL_FRATERNALISM_DAY__OBSERVED_,
    25600_i32 => _HEROES__DAY,
    25664_i32 => _WOMEN_S_DAY,
    25688_i32 => _INTERNATIONAL_WORKERS__DAY,
    25743_i32 => _INDEPENDENCE_DAY,
    25817_i32 => _VICTORY_DAY,
    25835_i32 => _ARMED_FORCES_DAY,
    25844_i32 => _PEACE_AND_RECONCILIATION_DAY,
    25926_i32 => _FAMILY_DAY,
    25933_i32 => _INTERNATIONAL_FRATERNALISM_DAY,
    25966_i32 => _HEROES__DAY,
    25967_i32 => _HEROES__DAY__OBSERVED_,
    26029_i32 => _WOMEN_S_DAY,
    26030_i32 => _WOMEN_S_DAY__OBSERVED_,
    26053_i32 => _INTERNATIONAL_WORKERS__DAY,
    26108_i32 => _INDEPENDENCE_DAY,
    26182_i32 => _VICTORY_DAY,
    26200_i32 => _ARMED_FORCES_DAY,
    26209_i32 => _PEACE_AND_RECONCILIATION_DAY,
    26291_i32 => _FAMILY_DAY,
    26298_i32 => _INTERNATIONAL_FRATERNALISM_DAY,
    26331_i32 => _HEROES__DAY,
    26394_i32 => _WOMEN_S_DAY,
    26418_i32 => _INTERNATIONAL_WORKERS__DAY,
    26473_i32 => _INDEPENDENCE_DAY,
    26547_i32 => _VICTORY_DAY,
    26548_i32 => _VICTORY_DAY__OBSERVED_,
    26565_i32 => _ARMED_FORCES_DAY,
    26574_i32 => _PEACE_AND_RECONCILIATION_DAY,
    26656_i32 => _FAMILY_DAY,
    26663_i32 => _INTERNATIONAL_FRATERNALISM_DAY,
    26696_i32 => _HEROES__DAY,
    26759_i32 => _WOMEN_S_DAY,
    26783_i32 => _INTERNATIONAL_WORKERS__DAY,
    26838_i32 => _INDEPENDENCE_DAY,
    26912_i32 => _VICTORY_DAY,
    26930_i32 => _ARMED_FORCES_DAY,
    26939_i32 => _PEACE_AND_RECONCILIATION_DAY,
    26940_i32 => _PEACE_AND_RECONCILIATION_DAY__OBSERVED_,
    27021_i32 => _FAMILY_DAY,
    27028_i32 => _INTERNATIONAL_FRATERNALISM_DAY,
    27061_i32 => _HEROES__DAY,
    27125_i32 => _WOMEN_S_DAY,
    27149_i32 => _INTERNATIONAL_WORKERS__DAY,
    27150_i32 => _INTERNATIONAL_WORKERS__DAY__OBSERVED_,
    27204_i32 => _INDEPENDENCE_DAY,
    27278_i32 => _VICTORY_DAY,
    27296_i32 => _ARMED_FORCES_DAY,
    27297_i32 => _ARMED_FORCES_DAY__OBSERVED_,
    27305_i32 => _PEACE_AND_RECONCILIATION_DAY,
    27387_i32 => _FAMILY_DAY,
    27388_i32 => _FAMILY_DAY__OBSERVED_,
    27394_i32 => _INTERNATIONAL_FRATERNALISM_DAY,
    27395_i32 => _INTERNATIONAL_FRATERNALISM_DAY__OBSERVED_,
    27427_i32 => _HEROES__DAY,
    27490_i32 => _WOMEN_S_DAY,
    27514_i32 => _INTERNATIONAL_WORKERS__DAY,
    27569_i32 => _INDEPENDENCE_DAY,
    27570_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    27643_i32 => _VICTORY_DAY,
    27661_i32 => _ARMED_FORCES_DAY,
    27670_i32 => _PEACE_AND_RECONCILIATION_DAY,
    27752_i32 => _FAMILY_DAY,
    27759_i32 => _INTERNATIONAL_FRATERNALISM_DAY,
    27792_i32 => _HEROES__DAY,
    27855_i32 => _WOMEN_S_DAY,
    27879_i32 => _INTERNATIONAL_WORKERS__DAY,
    27934_i32 => _INDEPENDENCE_DAY,
    28008_i32 => _VICTORY_DAY,
    28026_i32 => _ARMED_FORCES_DAY,
    28035_i32 => _PEACE_AND_RECONCILIATION_DAY,
    28117_i32 => _FAMILY_DAY,
    28124_i32 => _INTERNATIONAL_FRATERNALISM_DAY,
    28157_i32 => _HEROES__DAY,
    28158_i32 => _HEROES__DAY__OBSERVED_,
    28220_i32 => _WOMEN_S_DAY,
    28221_i32 => _WOMEN_S_DAY__OBSERVED_,
    28244_i32 => _INTERNATIONAL_WORKERS__DAY,
    28299_i32 => _INDEPENDENCE_DAY,
    28373_i32 => _VICTORY_DAY,
    28391_i32 => _ARMED_FORCES_DAY,
    28400_i32 => _PEACE_AND_RECONCILIATION_DAY,
    28482_i32 => _FAMILY_DAY,
    28489_i32 => _INTERNATIONAL_FRATERNALISM_DAY,
    28522_i32 => _HEROES__DAY,
    28586_i32 => _WOMEN_S_DAY,
    28610_i32 => _INTERNATIONAL_WORKERS__DAY,
    28665_i32 => _INDEPENDENCE_DAY,
    28739_i32 => _VICTORY_DAY,
    28757_i32 => _ARMED_FORCES_DAY,
    28766_i32 => _PEACE_AND_RECONCILIATION_DAY,
    28767_i32 => _PEACE_AND_RECONCILIATION_DAY__OBSERVED_,
    28848_i32 => _FAMILY_DAY,
    28855_i32 => _INTERNATIONAL_FRATERNALISM_DAY,
    28888_i32 => _HEROES__DAY,
    28951_i32 => _WOMEN_S_DAY,
    28975_i32 => _INTERNATIONAL_WORKERS__DAY,
    29030_i32 => _INDEPENDENCE_DAY,
    29104_i32 => _VICTORY_DAY,
    29122_i32 => _ARMED_FORCES_DAY,
    29131_i32 => _PEACE_AND_RECONCILIATION_DAY,
    29213_i32 => _FAMILY_DAY,
    29220_i32 => _INTERNATIONAL_FRATERNALISM_DAY,
};
