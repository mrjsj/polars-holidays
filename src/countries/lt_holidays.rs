use crate::countries::constants::*;
use phf::phf_map;

pub static LT_HOLIDAYS: phf::Map<i32, &'static str> = phf_map! {
    10957_i32 => _NEW_YEAR_S_DAY,
    11003_i32 => _DAY_OF_RESTORATION_OF_THE_STATE_OF_LITHUANIA,
    11027_i32 => _DAY_OF_RESTORATION_OF_INDEPENDENCE_OF_LITHUANIA,
    11070_i32 => _EASTER_SUNDAY,
    11071_i32 => _EASTER_MONDAY,
    11078_i32 => _INTERNATIONAL_WORKERS__DAY,
    11084_i32 => _MOTHER_S_DAY,
    11112_i32 => _FATHER_S_DAY,
    11144_i32 => _STATEHOOD_DAY,
    11184_i32 => _ASSUMPTION_DAY,
    11262_i32 => _ALL_SAINTS__DAY,
    11315_i32 => _CHRISTMAS_EVE,
    11316_i32 => _CHRISTMAS_DAY,
    11317_i32 => _SECOND_DAY_OF_CHRISTMAS,
    11323_i32 => _NEW_YEAR_S_DAY,
    11369_i32 => _DAY_OF_RESTORATION_OF_THE_STATE_OF_LITHUANIA,
    11392_i32 => _DAY_OF_RESTORATION_OF_INDEPENDENCE_OF_LITHUANIA,
    11427_i32 => _EASTER_SUNDAY,
    11428_i32 => _EASTER_MONDAY,
    11443_i32 => _INTERNATIONAL_WORKERS__DAY,
    11448_i32 => _MOTHER_S_DAY,
    11476_i32 => _FATHER_S_DAY,
    11509_i32 => _STATEHOOD_DAY,
    11549_i32 => _ASSUMPTION_DAY,
    11627_i32 => _ALL_SAINTS__DAY,
    11680_i32 => _CHRISTMAS_EVE,
    11681_i32 => _CHRISTMAS_DAY,
    11682_i32 => _SECOND_DAY_OF_CHRISTMAS,
    11688_i32 => _NEW_YEAR_S_DAY,
    11734_i32 => _DAY_OF_RESTORATION_OF_THE_STATE_OF_LITHUANIA,
    11757_i32 => _DAY_OF_RESTORATION_OF_INDEPENDENCE_OF_LITHUANIA,
    11777_i32 => _EASTER_SUNDAY,
    11778_i32 => _EASTER_MONDAY,
    11808_i32 => _INTERNATIONAL_WORKERS__DAY,
    11812_i32 => _MOTHER_S_DAY,
    11840_i32 => _FATHER_S_DAY,
    11874_i32 => _STATEHOOD_DAY,
    11914_i32 => _ASSUMPTION_DAY,
    11992_i32 => _ALL_SAINTS__DAY,
    12045_i32 => _CHRISTMAS_EVE,
    12046_i32 => _CHRISTMAS_DAY,
    12047_i32 => _SECOND_DAY_OF_CHRISTMAS,
    12053_i32 => _NEW_YEAR_S_DAY,
    12099_i32 => _DAY_OF_RESTORATION_OF_THE_STATE_OF_LITHUANIA,
    12122_i32 => _DAY_OF_RESTORATION_OF_INDEPENDENCE_OF_LITHUANIA,
    12162_i32 => _EASTER_SUNDAY,
    12163_i32 => _EASTER_MONDAY,
    12173_i32 => _INTERNATIONAL_WORKERS__DAY,
    12176_i32 => _MOTHER_S_DAY,
    12204_i32 => _FATHER_S_DAY,
    12227_i32 => _DAY_OF_DEW_AND_SAINT_JOHN,
    12239_i32 => _STATEHOOD_DAY,
    12279_i32 => _ASSUMPTION_DAY,
    12357_i32 => _ALL_SAINTS__DAY,
    12410_i32 => _CHRISTMAS_EVE,
    12411_i32 => _CHRISTMAS_DAY,
    12412_i32 => _SECOND_DAY_OF_CHRISTMAS,
    12418_i32 => _NEW_YEAR_S_DAY,
    12464_i32 => _DAY_OF_RESTORATION_OF_THE_STATE_OF_LITHUANIA,
    12488_i32 => _DAY_OF_RESTORATION_OF_INDEPENDENCE_OF_LITHUANIA,
    12519_i32 => _EASTER_SUNDAY,
    12520_i32 => _EASTER_MONDAY,
    12539_i32 => _INTERNATIONAL_WORKERS__DAY,
    12540_i32 => _MOTHER_S_DAY,
    12575_i32 => _FATHER_S_DAY,
    12593_i32 => _DAY_OF_DEW_AND_SAINT_JOHN,
    12605_i32 => _STATEHOOD_DAY,
    12645_i32 => _ASSUMPTION_DAY,
    12723_i32 => _ALL_SAINTS__DAY,
    12776_i32 => _CHRISTMAS_EVE,
    12777_i32 => _CHRISTMAS_DAY,
    12778_i32 => _SECOND_DAY_OF_CHRISTMAS,
    12784_i32 => _NEW_YEAR_S_DAY,
    12830_i32 => _DAY_OF_RESTORATION_OF_THE_STATE_OF_LITHUANIA,
    12853_i32 => _DAY_OF_RESTORATION_OF_INDEPENDENCE_OF_LITHUANIA,
    12869_i32 => _EASTER_SUNDAY,
    12870_i32 => _EASTER_MONDAY,
    12904_i32 => _INTERNATIONAL_WORKERS__DAY__MOTHER_S_DAY,
    12939_i32 => _FATHER_S_DAY,
    12958_i32 => _DAY_OF_DEW_AND_SAINT_JOHN,
    12970_i32 => _STATEHOOD_DAY,
    13010_i32 => _ASSUMPTION_DAY,
    13088_i32 => _ALL_SAINTS__DAY,
    13141_i32 => _CHRISTMAS_EVE,
    13142_i32 => _CHRISTMAS_DAY,
    13143_i32 => _SECOND_DAY_OF_CHRISTMAS,
    13149_i32 => _NEW_YEAR_S_DAY,
    13195_i32 => _DAY_OF_RESTORATION_OF_THE_STATE_OF_LITHUANIA,
    13218_i32 => _DAY_OF_RESTORATION_OF_INDEPENDENCE_OF_LITHUANIA,
    13254_i32 => _EASTER_SUNDAY,
    13255_i32 => _EASTER_MONDAY,
    13269_i32 => _INTERNATIONAL_WORKERS__DAY,
    13275_i32 => _MOTHER_S_DAY,
    13303_i32 => _FATHER_S_DAY,
    13323_i32 => _DAY_OF_DEW_AND_SAINT_JOHN,
    13335_i32 => _STATEHOOD_DAY,
    13375_i32 => _ASSUMPTION_DAY,
    13453_i32 => _ALL_SAINTS__DAY,
    13506_i32 => _CHRISTMAS_EVE,
    13507_i32 => _CHRISTMAS_DAY,
    13508_i32 => _SECOND_DAY_OF_CHRISTMAS,
    13514_i32 => _NEW_YEAR_S_DAY,
    13560_i32 => _DAY_OF_RESTORATION_OF_THE_STATE_OF_LITHUANIA,
    13583_i32 => _DAY_OF_RESTORATION_OF_INDEPENDENCE_OF_LITHUANIA,
    13611_i32 => _EASTER_SUNDAY,
    13612_i32 => _EASTER_MONDAY,
    13634_i32 => _INTERNATIONAL_WORKERS__DAY,
    13639_i32 => _MOTHER_S_DAY,
    13667_i32 => _FATHER_S_DAY,
    13688_i32 => _DAY_OF_DEW_AND_SAINT_JOHN,
    13700_i32 => _STATEHOOD_DAY,
    13740_i32 => _ASSUMPTION_DAY,
    13818_i32 => _ALL_SAINTS__DAY,
    13871_i32 => _CHRISTMAS_EVE,
    13872_i32 => _CHRISTMAS_DAY,
    13873_i32 => _SECOND_DAY_OF_CHRISTMAS,
    13879_i32 => _NEW_YEAR_S_DAY,
    13925_i32 => _DAY_OF_RESTORATION_OF_THE_STATE_OF_LITHUANIA,
    13949_i32 => _DAY_OF_RESTORATION_OF_INDEPENDENCE_OF_LITHUANIA,
    13961_i32 => _EASTER_SUNDAY,
    13962_i32 => _EASTER_MONDAY,
    14000_i32 => _INTERNATIONAL_WORKERS__DAY,
    14003_i32 => _MOTHER_S_DAY,
    14031_i32 => _FATHER_S_DAY,
    14054_i32 => _DAY_OF_DEW_AND_SAINT_JOHN,
    14066_i32 => _STATEHOOD_DAY,
    14106_i32 => _ASSUMPTION_DAY,
    14184_i32 => _ALL_SAINTS__DAY,
    14237_i32 => _CHRISTMAS_EVE,
    14238_i32 => _CHRISTMAS_DAY,
    14239_i32 => _SECOND_DAY_OF_CHRISTMAS,
    14245_i32 => _NEW_YEAR_S_DAY,
    14291_i32 => _DAY_OF_RESTORATION_OF_THE_STATE_OF_LITHUANIA,
    14314_i32 => _DAY_OF_RESTORATION_OF_INDEPENDENCE_OF_LITHUANIA,
    14346_i32 => _EASTER_SUNDAY,
    14347_i32 => _EASTER_MONDAY,
    14365_i32 => _INTERNATIONAL_WORKERS__DAY,
    14367_i32 => _MOTHER_S_DAY,
    14402_i32 => _FATHER_S_DAY,
    14419_i32 => _DAY_OF_DEW_AND_SAINT_JOHN,
    14431_i32 => _STATEHOOD_DAY,
    14471_i32 => _ASSUMPTION_DAY,
    14549_i32 => _ALL_SAINTS__DAY,
    14602_i32 => _CHRISTMAS_EVE,
    14603_i32 => _CHRISTMAS_DAY,
    14604_i32 => _SECOND_DAY_OF_CHRISTMAS,
    14610_i32 => _NEW_YEAR_S_DAY,
    14656_i32 => _DAY_OF_RESTORATION_OF_THE_STATE_OF_LITHUANIA,
    14679_i32 => _DAY_OF_RESTORATION_OF_INDEPENDENCE_OF_LITHUANIA,
    14703_i32 => _EASTER_SUNDAY,
    14704_i32 => _EASTER_MONDAY,
    14730_i32 => _INTERNATIONAL_WORKERS__DAY,
    14731_i32 => _MOTHER_S_DAY,
    14766_i32 => _FATHER_S_DAY,
    14784_i32 => _DAY_OF_DEW_AND_SAINT_JOHN,
    14796_i32 => _STATEHOOD_DAY,
    14836_i32 => _ASSUMPTION_DAY,
    14914_i32 => _ALL_SAINTS__DAY,
    14967_i32 => _CHRISTMAS_EVE,
    14968_i32 => _CHRISTMAS_DAY,
    14969_i32 => _SECOND_DAY_OF_CHRISTMAS,
    14975_i32 => _NEW_YEAR_S_DAY,
    15021_i32 => _DAY_OF_RESTORATION_OF_THE_STATE_OF_LITHUANIA,
    15044_i32 => _DAY_OF_RESTORATION_OF_INDEPENDENCE_OF_LITHUANIA,
    15088_i32 => _EASTER_SUNDAY,
    15089_i32 => _EASTER_MONDAY,
    15095_i32 => _INTERNATIONAL_WORKERS__DAY__MOTHER_S_DAY,
    15130_i32 => _FATHER_S_DAY,
    15149_i32 => _DAY_OF_DEW_AND_SAINT_JOHN,
    15161_i32 => _STATEHOOD_DAY,
    15201_i32 => _ASSUMPTION_DAY,
    15279_i32 => _ALL_SAINTS__DAY,
    15332_i32 => _CHRISTMAS_EVE,
    15333_i32 => _CHRISTMAS_DAY,
    15334_i32 => _SECOND_DAY_OF_CHRISTMAS,
    15340_i32 => _NEW_YEAR_S_DAY,
    15386_i32 => _DAY_OF_RESTORATION_OF_THE_STATE_OF_LITHUANIA,
    15410_i32 => _DAY_OF_RESTORATION_OF_INDEPENDENCE_OF_LITHUANIA,
    15438_i32 => _EASTER_SUNDAY,
    15439_i32 => _EASTER_MONDAY,
    15461_i32 => _INTERNATIONAL_WORKERS__DAY,
    15466_i32 => _MOTHER_S_DAY,
    15494_i32 => _FATHER_S_DAY,
    15515_i32 => _DAY_OF_DEW_AND_SAINT_JOHN,
    15527_i32 => _STATEHOOD_DAY,
    15567_i32 => _ASSUMPTION_DAY,
    15645_i32 => _ALL_SAINTS__DAY,
    15698_i32 => _CHRISTMAS_EVE,
    15699_i32 => _CHRISTMAS_DAY,
    15700_i32 => _SECOND_DAY_OF_CHRISTMAS,
    15706_i32 => _NEW_YEAR_S_DAY,
    15752_i32 => _DAY_OF_RESTORATION_OF_THE_STATE_OF_LITHUANIA,
    15775_i32 => _DAY_OF_RESTORATION_OF_INDEPENDENCE_OF_LITHUANIA,
    15795_i32 => _EASTER_SUNDAY,
    15796_i32 => _EASTER_MONDAY,
    15826_i32 => _INTERNATIONAL_WORKERS__DAY,
    15830_i32 => _MOTHER_S_DAY,
    15858_i32 => _FATHER_S_DAY,
    15880_i32 => _DAY_OF_DEW_AND_SAINT_JOHN,
    15892_i32 => _STATEHOOD_DAY,
    15932_i32 => _ASSUMPTION_DAY,
    16010_i32 => _ALL_SAINTS__DAY,
    16063_i32 => _CHRISTMAS_EVE,
    16064_i32 => _CHRISTMAS_DAY,
    16065_i32 => _SECOND_DAY_OF_CHRISTMAS,
    16071_i32 => _NEW_YEAR_S_DAY,
    16117_i32 => _DAY_OF_RESTORATION_OF_THE_STATE_OF_LITHUANIA,
    16140_i32 => _DAY_OF_RESTORATION_OF_INDEPENDENCE_OF_LITHUANIA,
    16180_i32 => _EASTER_SUNDAY,
    16181_i32 => _EASTER_MONDAY,
    16191_i32 => _INTERNATIONAL_WORKERS__DAY,
    16194_i32 => _MOTHER_S_DAY,
    16222_i32 => _FATHER_S_DAY,
    16245_i32 => _DAY_OF_DEW_AND_SAINT_JOHN,
    16257_i32 => _STATEHOOD_DAY,
    16297_i32 => _ASSUMPTION_DAY,
    16375_i32 => _ALL_SAINTS__DAY,
    16428_i32 => _CHRISTMAS_EVE,
    16429_i32 => _CHRISTMAS_DAY,
    16430_i32 => _SECOND_DAY_OF_CHRISTMAS,
    16436_i32 => _NEW_YEAR_S_DAY,
    16482_i32 => _DAY_OF_RESTORATION_OF_THE_STATE_OF_LITHUANIA,
    16505_i32 => _DAY_OF_RESTORATION_OF_INDEPENDENCE_OF_LITHUANIA,
    16530_i32 => _EASTER_SUNDAY,
    16531_i32 => _EASTER_MONDAY,
    16556_i32 => _INTERNATIONAL_WORKERS__DAY,
    16558_i32 => _MOTHER_S_DAY,
    16593_i32 => _FATHER_S_DAY,
    16610_i32 => _DAY_OF_DEW_AND_SAINT_JOHN,
    16622_i32 => _STATEHOOD_DAY,
    16662_i32 => _ASSUMPTION_DAY,
    16740_i32 => _ALL_SAINTS__DAY,
    16793_i32 => _CHRISTMAS_EVE,
    16794_i32 => _CHRISTMAS_DAY,
    16795_i32 => _SECOND_DAY_OF_CHRISTMAS,
    16801_i32 => _NEW_YEAR_S_DAY,
    16847_i32 => _DAY_OF_RESTORATION_OF_THE_STATE_OF_LITHUANIA,
    16871_i32 => _DAY_OF_RESTORATION_OF_INDEPENDENCE_OF_LITHUANIA,
    16887_i32 => _EASTER_SUNDAY,
    16888_i32 => _EASTER_MONDAY,
    16922_i32 => _INTERNATIONAL_WORKERS__DAY__MOTHER_S_DAY,
    16957_i32 => _FATHER_S_DAY,
    16976_i32 => _DAY_OF_DEW_AND_SAINT_JOHN,
    16988_i32 => _STATEHOOD_DAY,
    17028_i32 => _ASSUMPTION_DAY,
    17106_i32 => _ALL_SAINTS__DAY,
    17159_i32 => _CHRISTMAS_EVE,
    17160_i32 => _CHRISTMAS_DAY,
    17161_i32 => _SECOND_DAY_OF_CHRISTMAS,
    17167_i32 => _NEW_YEAR_S_DAY,
    17213_i32 => _DAY_OF_RESTORATION_OF_THE_STATE_OF_LITHUANIA,
    17236_i32 => _DAY_OF_RESTORATION_OF_INDEPENDENCE_OF_LITHUANIA,
    17272_i32 => _EASTER_SUNDAY,
    17273_i32 => _EASTER_MONDAY,
    17287_i32 => _INTERNATIONAL_WORKERS__DAY,
    17293_i32 => _MOTHER_S_DAY,
    17321_i32 => _FATHER_S_DAY,
    17341_i32 => _DAY_OF_DEW_AND_SAINT_JOHN,
    17353_i32 => _STATEHOOD_DAY,
    17393_i32 => _ASSUMPTION_DAY,
    17471_i32 => _ALL_SAINTS__DAY,
    17524_i32 => _CHRISTMAS_EVE,
    17525_i32 => _CHRISTMAS_DAY,
    17526_i32 => _SECOND_DAY_OF_CHRISTMAS,
    17532_i32 => _NEW_YEAR_S_DAY,
    17578_i32 => _DAY_OF_RESTORATION_OF_THE_STATE_OF_LITHUANIA,
    17601_i32 => _DAY_OF_RESTORATION_OF_INDEPENDENCE_OF_LITHUANIA,
    17622_i32 => _EASTER_SUNDAY,
    17623_i32 => _EASTER_MONDAY,
    17652_i32 => _INTERNATIONAL_WORKERS__DAY,
    17657_i32 => _MOTHER_S_DAY,
    17685_i32 => _FATHER_S_DAY,
    17706_i32 => _DAY_OF_DEW_AND_SAINT_JOHN,
    17718_i32 => _STATEHOOD_DAY,
    17758_i32 => _ASSUMPTION_DAY,
    17836_i32 => _ALL_SAINTS__DAY,
    17889_i32 => _CHRISTMAS_EVE,
    17890_i32 => _CHRISTMAS_DAY,
    17891_i32 => _SECOND_DAY_OF_CHRISTMAS,
    17897_i32 => _NEW_YEAR_S_DAY,
    17943_i32 => _DAY_OF_RESTORATION_OF_THE_STATE_OF_LITHUANIA,
    17966_i32 => _DAY_OF_RESTORATION_OF_INDEPENDENCE_OF_LITHUANIA,
    18007_i32 => _EASTER_SUNDAY,
    18008_i32 => _EASTER_MONDAY,
    18017_i32 => _INTERNATIONAL_WORKERS__DAY,
    18021_i32 => _MOTHER_S_DAY,
    18049_i32 => _FATHER_S_DAY,
    18071_i32 => _DAY_OF_DEW_AND_SAINT_JOHN,
    18083_i32 => _STATEHOOD_DAY,
    18123_i32 => _ASSUMPTION_DAY,
    18201_i32 => _ALL_SAINTS__DAY,
    18254_i32 => _CHRISTMAS_EVE,
    18255_i32 => _CHRISTMAS_DAY,
    18256_i32 => _SECOND_DAY_OF_CHRISTMAS,
    18262_i32 => _NEW_YEAR_S_DAY,
    18308_i32 => _DAY_OF_RESTORATION_OF_THE_STATE_OF_LITHUANIA,
    18332_i32 => _DAY_OF_RESTORATION_OF_INDEPENDENCE_OF_LITHUANIA,
    18364_i32 => _EASTER_SUNDAY,
    18365_i32 => _EASTER_MONDAY,
    18383_i32 => _INTERNATIONAL_WORKERS__DAY,
    18385_i32 => _MOTHER_S_DAY,
    18420_i32 => _FATHER_S_DAY,
    18437_i32 => _DAY_OF_DEW_AND_SAINT_JOHN,
    18449_i32 => _STATEHOOD_DAY,
    18489_i32 => _ASSUMPTION_DAY,
    18567_i32 => _ALL_SAINTS__DAY,
    18568_i32 => _ALL_SOULS__DAY,
    18620_i32 => _CHRISTMAS_EVE,
    18621_i32 => _CHRISTMAS_DAY,
    18622_i32 => _SECOND_DAY_OF_CHRISTMAS,
    18628_i32 => _NEW_YEAR_S_DAY,
    18674_i32 => _DAY_OF_RESTORATION_OF_THE_STATE_OF_LITHUANIA,
    18697_i32 => _DAY_OF_RESTORATION_OF_INDEPENDENCE_OF_LITHUANIA,
    18721_i32 => _EASTER_SUNDAY,
    18722_i32 => _EASTER_MONDAY,
    18748_i32 => _INTERNATIONAL_WORKERS__DAY,
    18749_i32 => _MOTHER_S_DAY,
    18784_i32 => _FATHER_S_DAY,
    18802_i32 => _DAY_OF_DEW_AND_SAINT_JOHN,
    18814_i32 => _STATEHOOD_DAY,
    18854_i32 => _ASSUMPTION_DAY,
    18932_i32 => _ALL_SAINTS__DAY,
    18933_i32 => _ALL_SOULS__DAY,
    18985_i32 => _CHRISTMAS_EVE,
    18986_i32 => _CHRISTMAS_DAY,
    18987_i32 => _SECOND_DAY_OF_CHRISTMAS,
    18993_i32 => _NEW_YEAR_S_DAY,
    19039_i32 => _DAY_OF_RESTORATION_OF_THE_STATE_OF_LITHUANIA,
    19062_i32 => _DAY_OF_RESTORATION_OF_INDEPENDENCE_OF_LITHUANIA,
    19099_i32 => _EASTER_SUNDAY,
    19100_i32 => _EASTER_MONDAY,
    19113_i32 => _INTERNATIONAL_WORKERS__DAY__MOTHER_S_DAY,
    19148_i32 => _FATHER_S_DAY,
    19167_i32 => _DAY_OF_DEW_AND_SAINT_JOHN,
    19179_i32 => _STATEHOOD_DAY,
    19219_i32 => _ASSUMPTION_DAY,
    19297_i32 => _ALL_SAINTS__DAY,
    19298_i32 => _ALL_SOULS__DAY,
    19350_i32 => _CHRISTMAS_EVE,
    19351_i32 => _CHRISTMAS_DAY,
    19352_i32 => _SECOND_DAY_OF_CHRISTMAS,
    19358_i32 => _NEW_YEAR_S_DAY,
    19404_i32 => _DAY_OF_RESTORATION_OF_THE_STATE_OF_LITHUANIA,
    19427_i32 => _DAY_OF_RESTORATION_OF_INDEPENDENCE_OF_LITHUANIA,
    19456_i32 => _EASTER_SUNDAY,
    19457_i32 => _EASTER_MONDAY,
    19478_i32 => _INTERNATIONAL_WORKERS__DAY,
    19484_i32 => _MOTHER_S_DAY,
    19512_i32 => _FATHER_S_DAY,
    19532_i32 => _DAY_OF_DEW_AND_SAINT_JOHN,
    19544_i32 => _STATEHOOD_DAY,
    19584_i32 => _ASSUMPTION_DAY,
    19662_i32 => _ALL_SAINTS__DAY,
    19663_i32 => _ALL_SOULS__DAY,
    19715_i32 => _CHRISTMAS_EVE,
    19716_i32 => _CHRISTMAS_DAY,
    19717_i32 => _SECOND_DAY_OF_CHRISTMAS,
    19723_i32 => _NEW_YEAR_S_DAY,
    19769_i32 => _DAY_OF_RESTORATION_OF_THE_STATE_OF_LITHUANIA,
    19793_i32 => _DAY_OF_RESTORATION_OF_INDEPENDENCE_OF_LITHUANIA,
    19813_i32 => _EASTER_SUNDAY,
    19814_i32 => _EASTER_MONDAY,
    19844_i32 => _INTERNATIONAL_WORKERS__DAY,
    19848_i32 => _MOTHER_S_DAY,
    19876_i32 => _FATHER_S_DAY,
    19898_i32 => _DAY_OF_DEW_AND_SAINT_JOHN,
    19910_i32 => _STATEHOOD_DAY,
    19950_i32 => _ASSUMPTION_DAY,
    20028_i32 => _ALL_SAINTS__DAY,
    20029_i32 => _ALL_SOULS__DAY,
    20081_i32 => _CHRISTMAS_EVE,
    20082_i32 => _CHRISTMAS_DAY,
    20083_i32 => _SECOND_DAY_OF_CHRISTMAS,
    20089_i32 => _NEW_YEAR_S_DAY,
    20135_i32 => _DAY_OF_RESTORATION_OF_THE_STATE_OF_LITHUANIA,
    20158_i32 => _DAY_OF_RESTORATION_OF_INDEPENDENCE_OF_LITHUANIA,
    20198_i32 => _EASTER_SUNDAY,
    20199_i32 => _EASTER_MONDAY,
    20209_i32 => _INTERNATIONAL_WORKERS__DAY,
    20212_i32 => _MOTHER_S_DAY,
    20240_i32 => _FATHER_S_DAY,
    20263_i32 => _DAY_OF_DEW_AND_SAINT_JOHN,
    20275_i32 => _STATEHOOD_DAY,
    20315_i32 => _ASSUMPTION_DAY,
    20393_i32 => _ALL_SAINTS__DAY,
    20394_i32 => _ALL_SOULS__DAY,
    20446_i32 => _CHRISTMAS_EVE,
    20447_i32 => _CHRISTMAS_DAY,
    20448_i32 => _SECOND_DAY_OF_CHRISTMAS,
    20454_i32 => _NEW_YEAR_S_DAY,
    20500_i32 => _DAY_OF_RESTORATION_OF_THE_STATE_OF_LITHUANIA,
    20523_i32 => _DAY_OF_RESTORATION_OF_INDEPENDENCE_OF_LITHUANIA,
    20548_i32 => _EASTER_SUNDAY,
    20549_i32 => _EASTER_MONDAY,
    20574_i32 => _INTERNATIONAL_WORKERS__DAY,
    20576_i32 => _MOTHER_S_DAY,
    20611_i32 => _FATHER_S_DAY,
    20628_i32 => _DAY_OF_DEW_AND_SAINT_JOHN,
    20640_i32 => _STATEHOOD_DAY,
    20680_i32 => _ASSUMPTION_DAY,
    20758_i32 => _ALL_SAINTS__DAY,
    20759_i32 => _ALL_SOULS__DAY,
    20811_i32 => _CHRISTMAS_EVE,
    20812_i32 => _CHRISTMAS_DAY,
    20813_i32 => _SECOND_DAY_OF_CHRISTMAS,
    20819_i32 => _NEW_YEAR_S_DAY,
    20865_i32 => _DAY_OF_RESTORATION_OF_THE_STATE_OF_LITHUANIA,
    20888_i32 => _DAY_OF_RESTORATION_OF_INDEPENDENCE_OF_LITHUANIA,
    20905_i32 => _EASTER_SUNDAY,
    20906_i32 => _EASTER_MONDAY,
    20939_i32 => _INTERNATIONAL_WORKERS__DAY,
    20940_i32 => _MOTHER_S_DAY,
    20975_i32 => _FATHER_S_DAY,
    20993_i32 => _DAY_OF_DEW_AND_SAINT_JOHN,
    21005_i32 => _STATEHOOD_DAY,
    21045_i32 => _ASSUMPTION_DAY,
    21123_i32 => _ALL_SAINTS__DAY,
    21124_i32 => _ALL_SOULS__DAY,
    21176_i32 => _CHRISTMAS_EVE,
    21177_i32 => _CHRISTMAS_DAY,
    21178_i32 => _SECOND_DAY_OF_CHRISTMAS,
    21184_i32 => _NEW_YEAR_S_DAY,
    21230_i32 => _DAY_OF_RESTORATION_OF_THE_STATE_OF_LITHUANIA,
    21254_i32 => _DAY_OF_RESTORATION_OF_INDEPENDENCE_OF_LITHUANIA,
    21290_i32 => _EASTER_SUNDAY,
    21291_i32 => _EASTER_MONDAY,
    21305_i32 => _INTERNATIONAL_WORKERS__DAY,
    21311_i32 => _MOTHER_S_DAY,
    21339_i32 => _FATHER_S_DAY,
    21359_i32 => _DAY_OF_DEW_AND_SAINT_JOHN,
    21371_i32 => _STATEHOOD_DAY,
    21411_i32 => _ASSUMPTION_DAY,
    21489_i32 => _ALL_SAINTS__DAY,
    21490_i32 => _ALL_SOULS__DAY,
    21542_i32 => _CHRISTMAS_EVE,
    21543_i32 => _CHRISTMAS_DAY,
    21544_i32 => _SECOND_DAY_OF_CHRISTMAS,
    21550_i32 => _NEW_YEAR_S_DAY,
    21596_i32 => _DAY_OF_RESTORATION_OF_THE_STATE_OF_LITHUANIA,
    21619_i32 => _DAY_OF_RESTORATION_OF_INDEPENDENCE_OF_LITHUANIA,
    21640_i32 => _EASTER_SUNDAY,
    21641_i32 => _EASTER_MONDAY,
    21670_i32 => _INTERNATIONAL_WORKERS__DAY,
    21675_i32 => _MOTHER_S_DAY,
    21703_i32 => _FATHER_S_DAY,
    21724_i32 => _DAY_OF_DEW_AND_SAINT_JOHN,
    21736_i32 => _STATEHOOD_DAY,
    21776_i32 => _ASSUMPTION_DAY,
    21854_i32 => _ALL_SAINTS__DAY,
    21855_i32 => _ALL_SOULS__DAY,
    21907_i32 => _CHRISTMAS_EVE,
    21908_i32 => _CHRISTMAS_DAY,
    21909_i32 => _SECOND_DAY_OF_CHRISTMAS,
    21915_i32 => _NEW_YEAR_S_DAY,
    21961_i32 => _DAY_OF_RESTORATION_OF_THE_STATE_OF_LITHUANIA,
    21984_i32 => _DAY_OF_RESTORATION_OF_INDEPENDENCE_OF_LITHUANIA,
    22025_i32 => _EASTER_SUNDAY,
    22026_i32 => _EASTER_MONDAY,
    22035_i32 => _INTERNATIONAL_WORKERS__DAY,
    22039_i32 => _MOTHER_S_DAY,
    22067_i32 => _FATHER_S_DAY,
    22089_i32 => _DAY_OF_DEW_AND_SAINT_JOHN,
    22101_i32 => _STATEHOOD_DAY,
    22141_i32 => _ASSUMPTION_DAY,
    22219_i32 => _ALL_SAINTS__DAY,
    22220_i32 => _ALL_SOULS__DAY,
    22272_i32 => _CHRISTMAS_EVE,
    22273_i32 => _CHRISTMAS_DAY,
    22274_i32 => _SECOND_DAY_OF_CHRISTMAS,
    22280_i32 => _NEW_YEAR_S_DAY,
    22326_i32 => _DAY_OF_RESTORATION_OF_THE_STATE_OF_LITHUANIA,
    22349_i32 => _DAY_OF_RESTORATION_OF_INDEPENDENCE_OF_LITHUANIA,
    22382_i32 => _EASTER_SUNDAY,
    22383_i32 => _EASTER_MONDAY,
    22400_i32 => _INTERNATIONAL_WORKERS__DAY,
    22403_i32 => _MOTHER_S_DAY,
    22431_i32 => _FATHER_S_DAY,
    22454_i32 => _DAY_OF_DEW_AND_SAINT_JOHN,
    22466_i32 => _STATEHOOD_DAY,
    22506_i32 => _ASSUMPTION_DAY,
    22584_i32 => _ALL_SAINTS__DAY,
    22585_i32 => _ALL_SOULS__DAY,
    22637_i32 => _CHRISTMAS_EVE,
    22638_i32 => _CHRISTMAS_DAY,
    22639_i32 => _SECOND_DAY_OF_CHRISTMAS,
    22645_i32 => _NEW_YEAR_S_DAY,
    22691_i32 => _DAY_OF_RESTORATION_OF_THE_STATE_OF_LITHUANIA,
    22715_i32 => _DAY_OF_RESTORATION_OF_INDEPENDENCE_OF_LITHUANIA,
    22732_i32 => _EASTER_SUNDAY,
    22733_i32 => _EASTER_MONDAY,
    22766_i32 => _INTERNATIONAL_WORKERS__DAY,
    22767_i32 => _MOTHER_S_DAY,
    22802_i32 => _FATHER_S_DAY,
    22820_i32 => _DAY_OF_DEW_AND_SAINT_JOHN,
    22832_i32 => _STATEHOOD_DAY,
    22872_i32 => _ASSUMPTION_DAY,
    22950_i32 => _ALL_SAINTS__DAY,
    22951_i32 => _ALL_SOULS__DAY,
    23003_i32 => _CHRISTMAS_EVE,
    23004_i32 => _CHRISTMAS_DAY,
    23005_i32 => _SECOND_DAY_OF_CHRISTMAS,
    23011_i32 => _NEW_YEAR_S_DAY,
    23057_i32 => _DAY_OF_RESTORATION_OF_THE_STATE_OF_LITHUANIA,
    23080_i32 => _DAY_OF_RESTORATION_OF_INDEPENDENCE_OF_LITHUANIA,
    23117_i32 => _EASTER_SUNDAY,
    23118_i32 => _EASTER_MONDAY,
    23131_i32 => _INTERNATIONAL_WORKERS__DAY__MOTHER_S_DAY,
    23166_i32 => _FATHER_S_DAY,
    23185_i32 => _DAY_OF_DEW_AND_SAINT_JOHN,
    23197_i32 => _STATEHOOD_DAY,
    23237_i32 => _ASSUMPTION_DAY,
    23315_i32 => _ALL_SAINTS__DAY,
    23316_i32 => _ALL_SOULS__DAY,
    23368_i32 => _CHRISTMAS_EVE,
    23369_i32 => _CHRISTMAS_DAY,
    23370_i32 => _SECOND_DAY_OF_CHRISTMAS,
    23376_i32 => _NEW_YEAR_S_DAY,
    23422_i32 => _DAY_OF_RESTORATION_OF_THE_STATE_OF_LITHUANIA,
    23445_i32 => _DAY_OF_RESTORATION_OF_INDEPENDENCE_OF_LITHUANIA,
    23474_i32 => _EASTER_SUNDAY,
    23475_i32 => _EASTER_MONDAY,
    23496_i32 => _INTERNATIONAL_WORKERS__DAY,
    23502_i32 => _MOTHER_S_DAY,
    23530_i32 => _FATHER_S_DAY,
    23550_i32 => _DAY_OF_DEW_AND_SAINT_JOHN,
    23562_i32 => _STATEHOOD_DAY,
    23602_i32 => _ASSUMPTION_DAY,
    23680_i32 => _ALL_SAINTS__DAY,
    23681_i32 => _ALL_SOULS__DAY,
    23733_i32 => _CHRISTMAS_EVE,
    23734_i32 => _CHRISTMAS_DAY,
    23735_i32 => _SECOND_DAY_OF_CHRISTMAS,
    23741_i32 => _NEW_YEAR_S_DAY,
    23787_i32 => _DAY_OF_RESTORATION_OF_THE_STATE_OF_LITHUANIA,
    23810_i32 => _DAY_OF_RESTORATION_OF_INDEPENDENCE_OF_LITHUANIA,
    23824_i32 => _EASTER_SUNDAY,
    23825_i32 => _EASTER_MONDAY,
    23861_i32 => _INTERNATIONAL_WORKERS__DAY,
    23866_i32 => _MOTHER_S_DAY,
    23894_i32 => _FATHER_S_DAY,
    23915_i32 => _DAY_OF_DEW_AND_SAINT_JOHN,
    23927_i32 => _STATEHOOD_DAY,
    23967_i32 => _ASSUMPTION_DAY,
    24045_i32 => _ALL_SAINTS__DAY,
    24046_i32 => _ALL_SOULS__DAY,
    24098_i32 => _CHRISTMAS_EVE,
    24099_i32 => _CHRISTMAS_DAY,
    24100_i32 => _SECOND_DAY_OF_CHRISTMAS,
    24106_i32 => _NEW_YEAR_S_DAY,
    24152_i32 => _DAY_OF_RESTORATION_OF_THE_STATE_OF_LITHUANIA,
    24176_i32 => _DAY_OF_RESTORATION_OF_INDEPENDENCE_OF_LITHUANIA,
    24209_i32 => _EASTER_SUNDAY,
    24210_i32 => _EASTER_MONDAY,
    24227_i32 => _INTERNATIONAL_WORKERS__DAY,
    24230_i32 => _MOTHER_S_DAY,
    24258_i32 => _FATHER_S_DAY,
    24281_i32 => _DAY_OF_DEW_AND_SAINT_JOHN,
    24293_i32 => _STATEHOOD_DAY,
    24333_i32 => _ASSUMPTION_DAY,
    24411_i32 => _ALL_SAINTS__DAY,
    24412_i32 => _ALL_SOULS__DAY,
    24464_i32 => _CHRISTMAS_EVE,
    24465_i32 => _CHRISTMAS_DAY,
    24466_i32 => _SECOND_DAY_OF_CHRISTMAS,
    24472_i32 => _NEW_YEAR_S_DAY,
    24518_i32 => _DAY_OF_RESTORATION_OF_THE_STATE_OF_LITHUANIA,
    24541_i32 => _DAY_OF_RESTORATION_OF_INDEPENDENCE_OF_LITHUANIA,
    24566_i32 => _EASTER_SUNDAY,
    24567_i32 => _EASTER_MONDAY,
    24592_i32 => _INTERNATIONAL_WORKERS__DAY,
    24594_i32 => _MOTHER_S_DAY,
    24629_i32 => _FATHER_S_DAY,
    24646_i32 => _DAY_OF_DEW_AND_SAINT_JOHN,
    24658_i32 => _STATEHOOD_DAY,
    24698_i32 => _ASSUMPTION_DAY,
    24776_i32 => _ALL_SAINTS__DAY,
    24777_i32 => _ALL_SOULS__DAY,
    24829_i32 => _CHRISTMAS_EVE,
    24830_i32 => _CHRISTMAS_DAY,
    24831_i32 => _SECOND_DAY_OF_CHRISTMAS,
    24837_i32 => _NEW_YEAR_S_DAY,
    24883_i32 => _DAY_OF_RESTORATION_OF_THE_STATE_OF_LITHUANIA,
    24906_i32 => _DAY_OF_RESTORATION_OF_INDEPENDENCE_OF_LITHUANIA,
    24951_i32 => _EASTER_SUNDAY,
    24952_i32 => _EASTER_MONDAY,
    24957_i32 => _INTERNATIONAL_WORKERS__DAY,
    24958_i32 => _MOTHER_S_DAY,
    24993_i32 => _FATHER_S_DAY,
    25011_i32 => _DAY_OF_DEW_AND_SAINT_JOHN,
    25023_i32 => _STATEHOOD_DAY,
    25063_i32 => _ASSUMPTION_DAY,
    25141_i32 => _ALL_SAINTS__DAY,
    25142_i32 => _ALL_SOULS__DAY,
    25194_i32 => _CHRISTMAS_EVE,
    25195_i32 => _CHRISTMAS_DAY,
    25196_i32 => _SECOND_DAY_OF_CHRISTMAS,
    25202_i32 => _NEW_YEAR_S_DAY,
    25248_i32 => _DAY_OF_RESTORATION_OF_THE_STATE_OF_LITHUANIA,
    25271_i32 => _DAY_OF_RESTORATION_OF_INDEPENDENCE_OF_LITHUANIA,
    25301_i32 => _EASTER_SUNDAY,
    25302_i32 => _EASTER_MONDAY,
    25322_i32 => _INTERNATIONAL_WORKERS__DAY__MOTHER_S_DAY,
    25357_i32 => _FATHER_S_DAY,
    25376_i32 => _DAY_OF_DEW_AND_SAINT_JOHN,
    25388_i32 => _STATEHOOD_DAY,
    25428_i32 => _ASSUMPTION_DAY,
    25506_i32 => _ALL_SAINTS__DAY,
    25507_i32 => _ALL_SOULS__DAY,
    25559_i32 => _CHRISTMAS_EVE,
    25560_i32 => _CHRISTMAS_DAY,
    25561_i32 => _SECOND_DAY_OF_CHRISTMAS,
    25567_i32 => _NEW_YEAR_S_DAY,
    25613_i32 => _DAY_OF_RESTORATION_OF_THE_STATE_OF_LITHUANIA,
    25637_i32 => _DAY_OF_RESTORATION_OF_INDEPENDENCE_OF_LITHUANIA,
    25658_i32 => _EASTER_SUNDAY,
    25659_i32 => _EASTER_MONDAY,
    25688_i32 => _INTERNATIONAL_WORKERS__DAY,
    25693_i32 => _MOTHER_S_DAY,
    25721_i32 => _FATHER_S_DAY,
    25742_i32 => _DAY_OF_DEW_AND_SAINT_JOHN,
    25754_i32 => _STATEHOOD_DAY,
    25794_i32 => _ASSUMPTION_DAY,
    25872_i32 => _ALL_SAINTS__DAY,
    25873_i32 => _ALL_SOULS__DAY,
    25925_i32 => _CHRISTMAS_EVE,
    25926_i32 => _CHRISTMAS_DAY,
    25927_i32 => _SECOND_DAY_OF_CHRISTMAS,
    25933_i32 => _NEW_YEAR_S_DAY,
    25979_i32 => _DAY_OF_RESTORATION_OF_THE_STATE_OF_LITHUANIA,
    26002_i32 => _DAY_OF_RESTORATION_OF_INDEPENDENCE_OF_LITHUANIA,
    26043_i32 => _EASTER_SUNDAY,
    26044_i32 => _EASTER_MONDAY,
    26053_i32 => _INTERNATIONAL_WORKERS__DAY,
    26057_i32 => _MOTHER_S_DAY,
    26085_i32 => _FATHER_S_DAY,
    26107_i32 => _DAY_OF_DEW_AND_SAINT_JOHN,
    26119_i32 => _STATEHOOD_DAY,
    26159_i32 => _ASSUMPTION_DAY,
    26237_i32 => _ALL_SAINTS__DAY,
    26238_i32 => _ALL_SOULS__DAY,
    26290_i32 => _CHRISTMAS_EVE,
    26291_i32 => _CHRISTMAS_DAY,
    26292_i32 => _SECOND_DAY_OF_CHRISTMAS,
    26298_i32 => _NEW_YEAR_S_DAY,
    26344_i32 => _DAY_OF_RESTORATION_OF_THE_STATE_OF_LITHUANIA,
    26367_i32 => _DAY_OF_RESTORATION_OF_INDEPENDENCE_OF_LITHUANIA,
    26393_i32 => _EASTER_SUNDAY,
    26394_i32 => _EASTER_MONDAY,
    26418_i32 => _INTERNATIONAL_WORKERS__DAY,
    26421_i32 => _MOTHER_S_DAY,
    26449_i32 => _FATHER_S_DAY,
    26472_i32 => _DAY_OF_DEW_AND_SAINT_JOHN,
    26484_i32 => _STATEHOOD_DAY,
    26524_i32 => _ASSUMPTION_DAY,
    26602_i32 => _ALL_SAINTS__DAY,
    26603_i32 => _ALL_SOULS__DAY,
    26655_i32 => _CHRISTMAS_EVE,
    26656_i32 => _CHRISTMAS_DAY,
    26657_i32 => _SECOND_DAY_OF_CHRISTMAS,
    26663_i32 => _NEW_YEAR_S_DAY,
    26709_i32 => _DAY_OF_RESTORATION_OF_THE_STATE_OF_LITHUANIA,
    26732_i32 => _DAY_OF_RESTORATION_OF_INDEPENDENCE_OF_LITHUANIA,
    26750_i32 => _EASTER_SUNDAY,
    26751_i32 => _EASTER_MONDAY,
    26783_i32 => _INTERNATIONAL_WORKERS__DAY,
    26785_i32 => _MOTHER_S_DAY,
    26820_i32 => _FATHER_S_DAY,
    26837_i32 => _DAY_OF_DEW_AND_SAINT_JOHN,
    26849_i32 => _STATEHOOD_DAY,
    26889_i32 => _ASSUMPTION_DAY,
    26967_i32 => _ALL_SAINTS__DAY,
    26968_i32 => _ALL_SOULS__DAY,
    27020_i32 => _CHRISTMAS_EVE,
    27021_i32 => _CHRISTMAS_DAY,
    27022_i32 => _SECOND_DAY_OF_CHRISTMAS,
    27028_i32 => _NEW_YEAR_S_DAY,
    27074_i32 => _DAY_OF_RESTORATION_OF_THE_STATE_OF_LITHUANIA,
    27098_i32 => _DAY_OF_RESTORATION_OF_INDEPENDENCE_OF_LITHUANIA,
    27135_i32 => _EASTER_SUNDAY,
    27136_i32 => _EASTER_MONDAY,
    27149_i32 => _INTERNATIONAL_WORKERS__DAY__MOTHER_S_DAY,
    27184_i32 => _FATHER_S_DAY,
    27203_i32 => _DAY_OF_DEW_AND_SAINT_JOHN,
    27215_i32 => _STATEHOOD_DAY,
    27255_i32 => _ASSUMPTION_DAY,
    27333_i32 => _ALL_SAINTS__DAY,
    27334_i32 => _ALL_SOULS__DAY,
    27386_i32 => _CHRISTMAS_EVE,
    27387_i32 => _CHRISTMAS_DAY,
    27388_i32 => _SECOND_DAY_OF_CHRISTMAS,
    27394_i32 => _NEW_YEAR_S_DAY,
    27440_i32 => _DAY_OF_RESTORATION_OF_THE_STATE_OF_LITHUANIA,
    27463_i32 => _DAY_OF_RESTORATION_OF_INDEPENDENCE_OF_LITHUANIA,
    27492_i32 => _EASTER_SUNDAY,
    27493_i32 => _EASTER_MONDAY,
    27514_i32 => _INTERNATIONAL_WORKERS__DAY,
    27520_i32 => _MOTHER_S_DAY,
    27548_i32 => _FATHER_S_DAY,
    27568_i32 => _DAY_OF_DEW_AND_SAINT_JOHN,
    27580_i32 => _STATEHOOD_DAY,
    27620_i32 => _ASSUMPTION_DAY,
    27698_i32 => _ALL_SAINTS__DAY,
    27699_i32 => _ALL_SOULS__DAY,
    27751_i32 => _CHRISTMAS_EVE,
    27752_i32 => _CHRISTMAS_DAY,
    27753_i32 => _SECOND_DAY_OF_CHRISTMAS,
    27759_i32 => _NEW_YEAR_S_DAY,
    27805_i32 => _DAY_OF_RESTORATION_OF_THE_STATE_OF_LITHUANIA,
    27828_i32 => _DAY_OF_RESTORATION_OF_INDEPENDENCE_OF_LITHUANIA,
    27842_i32 => _EASTER_SUNDAY,
    27843_i32 => _EASTER_MONDAY,
    27879_i32 => _INTERNATIONAL_WORKERS__DAY,
    27884_i32 => _MOTHER_S_DAY,
    27912_i32 => _FATHER_S_DAY,
    27933_i32 => _DAY_OF_DEW_AND_SAINT_JOHN,
    27945_i32 => _STATEHOOD_DAY,
    27985_i32 => _ASSUMPTION_DAY,
    28063_i32 => _ALL_SAINTS__DAY,
    28064_i32 => _ALL_SOULS__DAY,
    28116_i32 => _CHRISTMAS_EVE,
    28117_i32 => _CHRISTMAS_DAY,
    28118_i32 => _SECOND_DAY_OF_CHRISTMAS,
    28124_i32 => _NEW_YEAR_S_DAY,
    28170_i32 => _DAY_OF_RESTORATION_OF_THE_STATE_OF_LITHUANIA,
    28193_i32 => _DAY_OF_RESTORATION_OF_INDEPENDENCE_OF_LITHUANIA,
    28227_i32 => _EASTER_SUNDAY,
    28228_i32 => _EASTER_MONDAY,
    28244_i32 => _INTERNATIONAL_WORKERS__DAY,
    28248_i32 => _MOTHER_S_DAY,
    28276_i32 => _FATHER_S_DAY,
    28298_i32 => _DAY_OF_DEW_AND_SAINT_JOHN,
    28310_i32 => _STATEHOOD_DAY,
    28350_i32 => _ASSUMPTION_DAY,
    28428_i32 => _ALL_SAINTS__DAY,
    28429_i32 => _ALL_SOULS__DAY,
    28481_i32 => _CHRISTMAS_EVE,
    28482_i32 => _CHRISTMAS_DAY,
    28483_i32 => _SECOND_DAY_OF_CHRISTMAS,
    28489_i32 => _NEW_YEAR_S_DAY,
    28535_i32 => _DAY_OF_RESTORATION_OF_THE_STATE_OF_LITHUANIA,
    28559_i32 => _DAY_OF_RESTORATION_OF_INDEPENDENCE_OF_LITHUANIA,
    28584_i32 => _EASTER_SUNDAY,
    28585_i32 => _EASTER_MONDAY,
    28610_i32 => _INTERNATIONAL_WORKERS__DAY,
    28612_i32 => _MOTHER_S_DAY,
    28647_i32 => _FATHER_S_DAY,
    28664_i32 => _DAY_OF_DEW_AND_SAINT_JOHN,
    28676_i32 => _STATEHOOD_DAY,
    28716_i32 => _ASSUMPTION_DAY,
    28794_i32 => _ALL_SAINTS__DAY,
    28795_i32 => _ALL_SOULS__DAY,
    28847_i32 => _CHRISTMAS_EVE,
    28848_i32 => _CHRISTMAS_DAY,
    28849_i32 => _SECOND_DAY_OF_CHRISTMAS,
    28855_i32 => _NEW_YEAR_S_DAY,
    28901_i32 => _DAY_OF_RESTORATION_OF_THE_STATE_OF_LITHUANIA,
    28924_i32 => _DAY_OF_RESTORATION_OF_INDEPENDENCE_OF_LITHUANIA,
    28962_i32 => _EASTER_SUNDAY,
    28963_i32 => _EASTER_MONDAY,
    28975_i32 => _INTERNATIONAL_WORKERS__DAY,
    28976_i32 => _MOTHER_S_DAY,
    29011_i32 => _FATHER_S_DAY,
    29029_i32 => _DAY_OF_DEW_AND_SAINT_JOHN,
    29041_i32 => _STATEHOOD_DAY,
    29081_i32 => _ASSUMPTION_DAY,
    29159_i32 => _ALL_SAINTS__DAY,
    29160_i32 => _ALL_SOULS__DAY,
    29212_i32 => _CHRISTMAS_EVE,
    29213_i32 => _CHRISTMAS_DAY,
    29214_i32 => _SECOND_DAY_OF_CHRISTMAS,
    29220_i32 => _NEW_YEAR_S_DAY,
};
