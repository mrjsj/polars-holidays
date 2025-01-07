use crate::countries::constants::*;
use phf::phf_map;

pub static BZ_HOLIDAYS: phf::Map<i32, &'static str> = phf_map! {
    10957_i32 => _NEW_YEAR_S_DAY,
    11022_i32 => _NATIONAL_HEROES_AND_BENEFACTORS_DAY__OBSERVED_,
    11068_i32 => _GOOD_FRIDAY,
    11069_i32 => _HOLY_SATURDAY,
    11071_i32 => _EASTER_MONDAY,
    11078_i32 => _LABOUR_DAY,
    11099_i32 => _COMMONWEALTH_DAY__OBSERVED_,
    11211_i32 => _SAINT_GEORGE_S_CAYE_DAY__OBSERVED_,
    11221_i32 => _INDEPENDENCE_DAY,
    11239_i32 => _PAN_AMERICAN_DAY__OBSERVED_,
    11281_i32 => _GARIFUNA_SETTLEMENT_DAY__OBSERVED_,
    11316_i32 => _CHRISTMAS_DAY,
    11317_i32 => _BOXING_DAY,
    11323_i32 => _NEW_YEAR_S_DAY,
    11393_i32 => _NATIONAL_HEROES_AND_BENEFACTORS_DAY__OBSERVED_,
    11425_i32 => _GOOD_FRIDAY,
    11426_i32 => _HOLY_SATURDAY,
    11428_i32 => _EASTER_MONDAY,
    11443_i32 => _LABOUR_DAY,
    11463_i32 => _COMMONWEALTH_DAY__OBSERVED_,
    11575_i32 => _SAINT_GEORGE_S_CAYE_DAY,
    11586_i32 => _INDEPENDENCE_DAY,
    11610_i32 => _PAN_AMERICAN_DAY__OBSERVED_,
    11645_i32 => _GARIFUNA_SETTLEMENT_DAY,
    11681_i32 => _CHRISTMAS_DAY,
    11682_i32 => _BOXING_DAY,
    11688_i32 => _NEW_YEAR_S_DAY,
    11755_i32 => _NATIONAL_HEROES_AND_BENEFACTORS_DAY,
    11775_i32 => _GOOD_FRIDAY,
    11776_i32 => _HOLY_SATURDAY,
    11778_i32 => _EASTER_MONDAY,
    11808_i32 => _LABOUR_DAY,
    11834_i32 => _COMMONWEALTH_DAY__OBSERVED_,
    11940_i32 => _SAINT_GEORGE_S_CAYE_DAY,
    11951_i32 => _INDEPENDENCE_DAY,
    11972_i32 => _PAN_AMERICAN_DAY,
    12010_i32 => _GARIFUNA_SETTLEMENT_DAY,
    12046_i32 => _CHRISTMAS_DAY,
    12047_i32 => _BOXING_DAY,
    12053_i32 => _NEW_YEAR_S_DAY,
    12121_i32 => _NATIONAL_HEROES_AND_BENEFACTORS_DAY__OBSERVED_,
    12160_i32 => _GOOD_FRIDAY,
    12161_i32 => _HOLY_SATURDAY,
    12163_i32 => _EASTER_MONDAY,
    12173_i32 => _LABOUR_DAY,
    12196_i32 => _COMMONWEALTH_DAY,
    12305_i32 => _SAINT_GEORGE_S_CAYE_DAY,
    12317_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    12338_i32 => _PAN_AMERICAN_DAY__OBSERVED_,
    12375_i32 => _GARIFUNA_SETTLEMENT_DAY,
    12411_i32 => _CHRISTMAS_DAY,
    12412_i32 => _BOXING_DAY,
    12418_i32 => _NEW_YEAR_S_DAY,
    12485_i32 => _NATIONAL_HEROES_AND_BENEFACTORS_DAY__OBSERVED_,
    12517_i32 => _GOOD_FRIDAY,
    12518_i32 => _HOLY_SATURDAY,
    12520_i32 => _EASTER_MONDAY,
    12539_i32 => _LABOUR_DAY,
    12562_i32 => _COMMONWEALTH_DAY,
    12671_i32 => _SAINT_GEORGE_S_CAYE_DAY,
    12682_i32 => _INDEPENDENCE_DAY,
    12702_i32 => _PAN_AMERICAN_DAY__OBSERVED_,
    12741_i32 => _GARIFUNA_SETTLEMENT_DAY,
    12777_i32 => _CHRISTMAS_DAY,
    12779_i32 => _BOXING_DAY__OBSERVED_,
    12784_i32 => _NEW_YEAR_S_DAY,
    12849_i32 => _NATIONAL_HEROES_AND_BENEFACTORS_DAY__OBSERVED_,
    12867_i32 => _GOOD_FRIDAY,
    12868_i32 => _HOLY_SATURDAY,
    12870_i32 => _EASTER_MONDAY,
    12905_i32 => _LABOUR_DAY__OBSERVED_,
    12926_i32 => _COMMONWEALTH_DAY__OBSERVED_,
    13036_i32 => _SAINT_GEORGE_S_CAYE_DAY,
    13047_i32 => _INDEPENDENCE_DAY,
    13066_i32 => _PAN_AMERICAN_DAY__OBSERVED_,
    13106_i32 => _GARIFUNA_SETTLEMENT_DAY,
    13142_i32 => _CHRISTMAS_DAY,
    13143_i32 => _BOXING_DAY,
    13150_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    13213_i32 => _NATIONAL_HEROES_AND_BENEFACTORS_DAY__OBSERVED_,
    13252_i32 => _GOOD_FRIDAY,
    13253_i32 => _HOLY_SATURDAY,
    13255_i32 => _EASTER_MONDAY,
    13269_i32 => _LABOUR_DAY,
    13290_i32 => _COMMONWEALTH_DAY__OBSERVED_,
    13402_i32 => _SAINT_GEORGE_S_CAYE_DAY__OBSERVED_,
    13412_i32 => _INDEPENDENCE_DAY,
    13430_i32 => _PAN_AMERICAN_DAY__OBSERVED_,
    13472_i32 => _GARIFUNA_SETTLEMENT_DAY__OBSERVED_,
    13507_i32 => _CHRISTMAS_DAY,
    13508_i32 => _BOXING_DAY,
    13514_i32 => _NEW_YEAR_S_DAY,
    13584_i32 => _NATIONAL_HEROES_AND_BENEFACTORS_DAY__OBSERVED_,
    13609_i32 => _GOOD_FRIDAY,
    13610_i32 => _HOLY_SATURDAY,
    13612_i32 => _EASTER_MONDAY,
    13634_i32 => _LABOUR_DAY,
    13654_i32 => _COMMONWEALTH_DAY__OBSERVED_,
    13766_i32 => _SAINT_GEORGE_S_CAYE_DAY,
    13777_i32 => _INDEPENDENCE_DAY,
    13801_i32 => _PAN_AMERICAN_DAY__OBSERVED_,
    13836_i32 => _GARIFUNA_SETTLEMENT_DAY,
    13872_i32 => _CHRISTMAS_DAY,
    13873_i32 => _BOXING_DAY,
    13879_i32 => _NEW_YEAR_S_DAY,
    13948_i32 => _NATIONAL_HEROES_AND_BENEFACTORS_DAY__OBSERVED_,
    13959_i32 => _GOOD_FRIDAY,
    13960_i32 => _HOLY_SATURDAY,
    13962_i32 => _EASTER_MONDAY,
    14000_i32 => _LABOUR_DAY,
    14023_i32 => _COMMONWEALTH_DAY,
    14132_i32 => _SAINT_GEORGE_S_CAYE_DAY,
    14144_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    14165_i32 => _PAN_AMERICAN_DAY__OBSERVED_,
    14202_i32 => _GARIFUNA_SETTLEMENT_DAY,
    14238_i32 => _CHRISTMAS_DAY,
    14239_i32 => _BOXING_DAY,
    14245_i32 => _NEW_YEAR_S_DAY,
    14312_i32 => _NATIONAL_HEROES_AND_BENEFACTORS_DAY,
    14344_i32 => _GOOD_FRIDAY,
    14345_i32 => _HOLY_SATURDAY,
    14347_i32 => _EASTER_MONDAY,
    14365_i32 => _LABOUR_DAY,
    14389_i32 => _COMMONWEALTH_DAY__OBSERVED_,
    14497_i32 => _SAINT_GEORGE_S_CAYE_DAY,
    14508_i32 => _INDEPENDENCE_DAY,
    14529_i32 => _PAN_AMERICAN_DAY,
    14567_i32 => _GARIFUNA_SETTLEMENT_DAY,
    14603_i32 => _CHRISTMAS_DAY,
    14604_i32 => _BOXING_DAY,
    14610_i32 => _NEW_YEAR_S_DAY,
    14676_i32 => _NATIONAL_HEROES_AND_BENEFACTORS_DAY__OBSERVED_,
    14701_i32 => _GOOD_FRIDAY,
    14702_i32 => _HOLY_SATURDAY,
    14704_i32 => _EASTER_MONDAY,
    14730_i32 => _LABOUR_DAY,
    14753_i32 => _COMMONWEALTH_DAY,
    14862_i32 => _SAINT_GEORGE_S_CAYE_DAY,
    14873_i32 => _INDEPENDENCE_DAY,
    14893_i32 => _PAN_AMERICAN_DAY__OBSERVED_,
    14932_i32 => _GARIFUNA_SETTLEMENT_DAY,
    14968_i32 => _CHRISTMAS_DAY,
    14970_i32 => _BOXING_DAY__OBSERVED_,
    14975_i32 => _NEW_YEAR_S_DAY,
    15040_i32 => _NATIONAL_HEROES_AND_BENEFACTORS_DAY__OBSERVED_,
    15086_i32 => _GOOD_FRIDAY,
    15087_i32 => _HOLY_SATURDAY,
    15089_i32 => _EASTER_MONDAY,
    15096_i32 => _LABOUR_DAY__OBSERVED_,
    15117_i32 => _COMMONWEALTH_DAY__OBSERVED_,
    15227_i32 => _SAINT_GEORGE_S_CAYE_DAY,
    15238_i32 => _INDEPENDENCE_DAY,
    15257_i32 => _PAN_AMERICAN_DAY__OBSERVED_,
    15297_i32 => _GARIFUNA_SETTLEMENT_DAY,
    15333_i32 => _CHRISTMAS_DAY,
    15334_i32 => _BOXING_DAY,
    15341_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    15411_i32 => _NATIONAL_HEROES_AND_BENEFACTORS_DAY__OBSERVED_,
    15436_i32 => _GOOD_FRIDAY,
    15437_i32 => _HOLY_SATURDAY,
    15439_i32 => _EASTER_MONDAY,
    15461_i32 => _LABOUR_DAY,
    15481_i32 => _COMMONWEALTH_DAY__OBSERVED_,
    15593_i32 => _SAINT_GEORGE_S_CAYE_DAY,
    15604_i32 => _INDEPENDENCE_DAY,
    15628_i32 => _PAN_AMERICAN_DAY__OBSERVED_,
    15663_i32 => _GARIFUNA_SETTLEMENT_DAY,
    15699_i32 => _CHRISTMAS_DAY,
    15700_i32 => _BOXING_DAY,
    15706_i32 => _NEW_YEAR_S_DAY,
    15773_i32 => _NATIONAL_HEROES_AND_BENEFACTORS_DAY,
    15793_i32 => _GOOD_FRIDAY,
    15794_i32 => _HOLY_SATURDAY,
    15796_i32 => _EASTER_MONDAY,
    15826_i32 => _LABOUR_DAY,
    15852_i32 => _COMMONWEALTH_DAY__OBSERVED_,
    15958_i32 => _SAINT_GEORGE_S_CAYE_DAY,
    15969_i32 => _INDEPENDENCE_DAY,
    15990_i32 => _PAN_AMERICAN_DAY,
    16028_i32 => _GARIFUNA_SETTLEMENT_DAY,
    16064_i32 => _CHRISTMAS_DAY,
    16065_i32 => _BOXING_DAY,
    16071_i32 => _NEW_YEAR_S_DAY,
    16139_i32 => _NATIONAL_HEROES_AND_BENEFACTORS_DAY__OBSERVED_,
    16178_i32 => _GOOD_FRIDAY,
    16179_i32 => _HOLY_SATURDAY,
    16181_i32 => _EASTER_MONDAY,
    16191_i32 => _LABOUR_DAY,
    16214_i32 => _COMMONWEALTH_DAY,
    16323_i32 => _SAINT_GEORGE_S_CAYE_DAY,
    16335_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    16356_i32 => _PAN_AMERICAN_DAY__OBSERVED_,
    16393_i32 => _GARIFUNA_SETTLEMENT_DAY,
    16429_i32 => _CHRISTMAS_DAY,
    16430_i32 => _BOXING_DAY,
    16436_i32 => _NEW_YEAR_S_DAY,
    16503_i32 => _NATIONAL_HEROES_AND_BENEFACTORS_DAY,
    16528_i32 => _GOOD_FRIDAY,
    16529_i32 => _HOLY_SATURDAY,
    16531_i32 => _EASTER_MONDAY,
    16556_i32 => _LABOUR_DAY,
    16580_i32 => _COMMONWEALTH_DAY__OBSERVED_,
    16688_i32 => _SAINT_GEORGE_S_CAYE_DAY,
    16699_i32 => _INDEPENDENCE_DAY,
    16720_i32 => _PAN_AMERICAN_DAY,
    16758_i32 => _GARIFUNA_SETTLEMENT_DAY,
    16794_i32 => _CHRISTMAS_DAY,
    16795_i32 => _BOXING_DAY,
    16801_i32 => _NEW_YEAR_S_DAY,
    16867_i32 => _NATIONAL_HEROES_AND_BENEFACTORS_DAY__OBSERVED_,
    16885_i32 => _GOOD_FRIDAY,
    16886_i32 => _HOLY_SATURDAY,
    16888_i32 => _EASTER_MONDAY,
    16923_i32 => _LABOUR_DAY__OBSERVED_,
    16944_i32 => _COMMONWEALTH_DAY__OBSERVED_,
    17054_i32 => _SAINT_GEORGE_S_CAYE_DAY,
    17065_i32 => _INDEPENDENCE_DAY,
    17084_i32 => _PAN_AMERICAN_DAY__OBSERVED_,
    17124_i32 => _GARIFUNA_SETTLEMENT_DAY,
    17160_i32 => _CHRISTMAS_DAY,
    17161_i32 => _BOXING_DAY,
    17168_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    17231_i32 => _NATIONAL_HEROES_AND_BENEFACTORS_DAY__OBSERVED_,
    17270_i32 => _GOOD_FRIDAY,
    17271_i32 => _HOLY_SATURDAY,
    17273_i32 => _EASTER_MONDAY,
    17287_i32 => _LABOUR_DAY,
    17308_i32 => _COMMONWEALTH_DAY__OBSERVED_,
    17420_i32 => _SAINT_GEORGE_S_CAYE_DAY__OBSERVED_,
    17430_i32 => _INDEPENDENCE_DAY,
    17448_i32 => _PAN_AMERICAN_DAY__OBSERVED_,
    17490_i32 => _GARIFUNA_SETTLEMENT_DAY__OBSERVED_,
    17525_i32 => _CHRISTMAS_DAY,
    17526_i32 => _BOXING_DAY,
    17532_i32 => _NEW_YEAR_S_DAY,
    17602_i32 => _NATIONAL_HEROES_AND_BENEFACTORS_DAY__OBSERVED_,
    17620_i32 => _GOOD_FRIDAY,
    17621_i32 => _HOLY_SATURDAY,
    17623_i32 => _EASTER_MONDAY,
    17652_i32 => _LABOUR_DAY,
    17672_i32 => _COMMONWEALTH_DAY__OBSERVED_,
    17784_i32 => _SAINT_GEORGE_S_CAYE_DAY,
    17795_i32 => _INDEPENDENCE_DAY,
    17819_i32 => _PAN_AMERICAN_DAY__OBSERVED_,
    17854_i32 => _GARIFUNA_SETTLEMENT_DAY,
    17890_i32 => _CHRISTMAS_DAY,
    17891_i32 => _BOXING_DAY,
    17897_i32 => _NEW_YEAR_S_DAY,
    17964_i32 => _NATIONAL_HEROES_AND_BENEFACTORS_DAY,
    18005_i32 => _GOOD_FRIDAY,
    18006_i32 => _HOLY_SATURDAY,
    18008_i32 => _EASTER_MONDAY,
    18017_i32 => _LABOUR_DAY,
    18043_i32 => _COMMONWEALTH_DAY__OBSERVED_,
    18149_i32 => _SAINT_GEORGE_S_CAYE_DAY,
    18160_i32 => _INDEPENDENCE_DAY,
    18181_i32 => _PAN_AMERICAN_DAY,
    18219_i32 => _GARIFUNA_SETTLEMENT_DAY,
    18255_i32 => _CHRISTMAS_DAY,
    18256_i32 => _BOXING_DAY,
    18262_i32 => _NEW_YEAR_S_DAY,
    18330_i32 => _NATIONAL_HEROES_AND_BENEFACTORS_DAY,
    18362_i32 => _GOOD_FRIDAY,
    18363_i32 => _HOLY_SATURDAY,
    18365_i32 => _EASTER_MONDAY,
    18383_i32 => _LABOUR_DAY,
    18407_i32 => _COMMONWEALTH_DAY__OBSERVED_,
    18515_i32 => _SAINT_GEORGE_S_CAYE_DAY,
    18526_i32 => _INDEPENDENCE_DAY,
    18547_i32 => _PAN_AMERICAN_DAY,
    18585_i32 => _GARIFUNA_SETTLEMENT_DAY,
    18621_i32 => _CHRISTMAS_DAY,
    18622_i32 => _BOXING_DAY,
    18628_i32 => _NEW_YEAR_S_DAY,
    18642_i32 => _GEORGE_PRICE_DAY,
    18694_i32 => _NATIONAL_HEROES_AND_BENEFACTORS_DAY__OBSERVED_,
    18719_i32 => _GOOD_FRIDAY,
    18720_i32 => _HOLY_SATURDAY,
    18722_i32 => _EASTER_MONDAY,
    18748_i32 => _LABOUR_DAY,
    18771_i32 => _COMMONWEALTH_DAY,
    18841_i32 => _EMANCIPATION_DAY__OBSERVED_,
    18880_i32 => _SAINT_GEORGE_S_CAYE_DAY,
    18891_i32 => _INDEPENDENCE_DAY,
    18911_i32 => _INDIGENOUS_PEOPLES__RESISTANCE_DAY__OBSERVED_,
    18950_i32 => _GARIFUNA_SETTLEMENT_DAY,
    18986_i32 => _CHRISTMAS_DAY,
    18988_i32 => _BOXING_DAY__OBSERVED_,
    18993_i32 => _NEW_YEAR_S_DAY,
    19007_i32 => _GEORGE_PRICE_DAY,
    19058_i32 => _NATIONAL_HEROES_AND_BENEFACTORS_DAY__OBSERVED_,
    19097_i32 => _GOOD_FRIDAY,
    19098_i32 => _HOLY_SATURDAY,
    19100_i32 => _EASTER_MONDAY,
    19114_i32 => _LABOUR_DAY__OBSERVED_,
    19205_i32 => _EMANCIPATION_DAY,
    19245_i32 => _SAINT_GEORGE_S_CAYE_DAY,
    19256_i32 => _INDEPENDENCE_DAY,
    19275_i32 => _INDIGENOUS_PEOPLES__RESISTANCE_DAY__OBSERVED_,
    19315_i32 => _GARIFUNA_SETTLEMENT_DAY,
    19351_i32 => _CHRISTMAS_DAY,
    19352_i32 => _BOXING_DAY,
    19359_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    19373_i32 => _GEORGE_PRICE_DAY__OBSERVED_,
    19422_i32 => _NATIONAL_HEROES_AND_BENEFACTORS_DAY__OBSERVED_,
    19454_i32 => _GOOD_FRIDAY,
    19455_i32 => _HOLY_SATURDAY,
    19457_i32 => _EASTER_MONDAY,
    19478_i32 => _LABOUR_DAY,
    19569_i32 => _EMANCIPATION_DAY__OBSERVED_,
    19611_i32 => _SAINT_GEORGE_S_CAYE_DAY__OBSERVED_,
    19621_i32 => _INDEPENDENCE_DAY,
    19639_i32 => _INDIGENOUS_PEOPLES__RESISTANCE_DAY__OBSERVED_,
    19681_i32 => _GARIFUNA_SETTLEMENT_DAY__OBSERVED_,
    19716_i32 => _CHRISTMAS_DAY,
    19717_i32 => _BOXING_DAY,
    19723_i32 => _NEW_YEAR_S_DAY,
    19737_i32 => _GEORGE_PRICE_DAY,
    19791_i32 => _NATIONAL_HEROES_AND_BENEFACTORS_DAY,
    19811_i32 => _GOOD_FRIDAY,
    19812_i32 => _HOLY_SATURDAY,
    19814_i32 => _EASTER_MONDAY,
    19844_i32 => _LABOUR_DAY,
    19933_i32 => _EMANCIPATION_DAY__OBSERVED_,
    19976_i32 => _SAINT_GEORGE_S_CAYE_DAY,
    19987_i32 => _INDEPENDENCE_DAY,
    20008_i32 => _INDIGENOUS_PEOPLES__RESISTANCE_DAY,
    20046_i32 => _GARIFUNA_SETTLEMENT_DAY,
    20082_i32 => _CHRISTMAS_DAY,
    20083_i32 => _BOXING_DAY,
    20089_i32 => _NEW_YEAR_S_DAY,
    20103_i32 => _GEORGE_PRICE_DAY,
    20157_i32 => _NATIONAL_HEROES_AND_BENEFACTORS_DAY__OBSERVED_,
    20196_i32 => _GOOD_FRIDAY,
    20197_i32 => _HOLY_SATURDAY,
    20199_i32 => _EASTER_MONDAY,
    20209_i32 => _LABOUR_DAY,
    20304_i32 => _EMANCIPATION_DAY__OBSERVED_,
    20341_i32 => _SAINT_GEORGE_S_CAYE_DAY,
    20353_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    20374_i32 => _INDIGENOUS_PEOPLES__RESISTANCE_DAY__OBSERVED_,
    20411_i32 => _GARIFUNA_SETTLEMENT_DAY,
    20447_i32 => _CHRISTMAS_DAY,
    20448_i32 => _BOXING_DAY,
    20454_i32 => _NEW_YEAR_S_DAY,
    20468_i32 => _GEORGE_PRICE_DAY,
    20521_i32 => _NATIONAL_HEROES_AND_BENEFACTORS_DAY,
    20546_i32 => _GOOD_FRIDAY,
    20547_i32 => _HOLY_SATURDAY,
    20549_i32 => _EASTER_MONDAY,
    20574_i32 => _LABOUR_DAY,
    20666_i32 => _EMANCIPATION_DAY,
    20706_i32 => _SAINT_GEORGE_S_CAYE_DAY,
    20717_i32 => _INDEPENDENCE_DAY,
    20738_i32 => _INDIGENOUS_PEOPLES__RESISTANCE_DAY,
    20776_i32 => _GARIFUNA_SETTLEMENT_DAY,
    20812_i32 => _CHRISTMAS_DAY,
    20813_i32 => _BOXING_DAY,
    20819_i32 => _NEW_YEAR_S_DAY,
    20833_i32 => _GEORGE_PRICE_DAY,
    20885_i32 => _NATIONAL_HEROES_AND_BENEFACTORS_DAY__OBSERVED_,
    20903_i32 => _GOOD_FRIDAY,
    20904_i32 => _HOLY_SATURDAY,
    20906_i32 => _EASTER_MONDAY,
    20939_i32 => _LABOUR_DAY,
    21032_i32 => _EMANCIPATION_DAY__OBSERVED_,
    21071_i32 => _SAINT_GEORGE_S_CAYE_DAY,
    21082_i32 => _INDEPENDENCE_DAY,
    21102_i32 => _INDIGENOUS_PEOPLES__RESISTANCE_DAY__OBSERVED_,
    21141_i32 => _GARIFUNA_SETTLEMENT_DAY,
    21177_i32 => _CHRISTMAS_DAY,
    21179_i32 => _BOXING_DAY__OBSERVED_,
    21184_i32 => _NEW_YEAR_S_DAY,
    21198_i32 => _GEORGE_PRICE_DAY,
    21249_i32 => _NATIONAL_HEROES_AND_BENEFACTORS_DAY__OBSERVED_,
    21288_i32 => _GOOD_FRIDAY,
    21289_i32 => _HOLY_SATURDAY,
    21291_i32 => _EASTER_MONDAY,
    21305_i32 => _LABOUR_DAY,
    21396_i32 => _EMANCIPATION_DAY__OBSERVED_,
    21438_i32 => _SAINT_GEORGE_S_CAYE_DAY__OBSERVED_,
    21448_i32 => _INDEPENDENCE_DAY,
    21466_i32 => _INDIGENOUS_PEOPLES__RESISTANCE_DAY__OBSERVED_,
    21508_i32 => _GARIFUNA_SETTLEMENT_DAY__OBSERVED_,
    21543_i32 => _CHRISTMAS_DAY,
    21544_i32 => _BOXING_DAY,
    21550_i32 => _NEW_YEAR_S_DAY,
    21564_i32 => _GEORGE_PRICE_DAY,
    21620_i32 => _NATIONAL_HEROES_AND_BENEFACTORS_DAY__OBSERVED_,
    21638_i32 => _GOOD_FRIDAY,
    21639_i32 => _HOLY_SATURDAY,
    21641_i32 => _EASTER_MONDAY,
    21670_i32 => _LABOUR_DAY,
    21760_i32 => _EMANCIPATION_DAY__OBSERVED_,
    21802_i32 => _SAINT_GEORGE_S_CAYE_DAY,
    21813_i32 => _INDEPENDENCE_DAY,
    21837_i32 => _INDIGENOUS_PEOPLES__RESISTANCE_DAY__OBSERVED_,
    21872_i32 => _GARIFUNA_SETTLEMENT_DAY,
    21908_i32 => _CHRISTMAS_DAY,
    21909_i32 => _BOXING_DAY,
    21915_i32 => _NEW_YEAR_S_DAY,
    21929_i32 => _GEORGE_PRICE_DAY,
    21982_i32 => _NATIONAL_HEROES_AND_BENEFACTORS_DAY,
    22023_i32 => _GOOD_FRIDAY,
    22024_i32 => _HOLY_SATURDAY,
    22026_i32 => _EASTER_MONDAY,
    22035_i32 => _LABOUR_DAY,
    22124_i32 => _EMANCIPATION_DAY__OBSERVED_,
    22167_i32 => _SAINT_GEORGE_S_CAYE_DAY,
    22178_i32 => _INDEPENDENCE_DAY,
    22199_i32 => _INDIGENOUS_PEOPLES__RESISTANCE_DAY,
    22237_i32 => _GARIFUNA_SETTLEMENT_DAY,
    22273_i32 => _CHRISTMAS_DAY,
    22274_i32 => _BOXING_DAY,
    22280_i32 => _NEW_YEAR_S_DAY,
    22294_i32 => _GEORGE_PRICE_DAY,
    22348_i32 => _NATIONAL_HEROES_AND_BENEFACTORS_DAY__OBSERVED_,
    22380_i32 => _GOOD_FRIDAY,
    22381_i32 => _HOLY_SATURDAY,
    22383_i32 => _EASTER_MONDAY,
    22400_i32 => _LABOUR_DAY,
    22495_i32 => _EMANCIPATION_DAY__OBSERVED_,
    22532_i32 => _SAINT_GEORGE_S_CAYE_DAY,
    22544_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    22565_i32 => _INDIGENOUS_PEOPLES__RESISTANCE_DAY__OBSERVED_,
    22602_i32 => _GARIFUNA_SETTLEMENT_DAY,
    22638_i32 => _CHRISTMAS_DAY,
    22639_i32 => _BOXING_DAY,
    22645_i32 => _NEW_YEAR_S_DAY,
    22659_i32 => _GEORGE_PRICE_DAY,
    22712_i32 => _NATIONAL_HEROES_AND_BENEFACTORS_DAY__OBSERVED_,
    22730_i32 => _GOOD_FRIDAY,
    22731_i32 => _HOLY_SATURDAY,
    22733_i32 => _EASTER_MONDAY,
    22766_i32 => _LABOUR_DAY,
    22859_i32 => _EMANCIPATION_DAY__OBSERVED_,
    22898_i32 => _SAINT_GEORGE_S_CAYE_DAY,
    22909_i32 => _INDEPENDENCE_DAY,
    22929_i32 => _INDIGENOUS_PEOPLES__RESISTANCE_DAY__OBSERVED_,
    22968_i32 => _GARIFUNA_SETTLEMENT_DAY,
    23004_i32 => _CHRISTMAS_DAY,
    23006_i32 => _BOXING_DAY__OBSERVED_,
    23011_i32 => _NEW_YEAR_S_DAY,
    23025_i32 => _GEORGE_PRICE_DAY,
    23076_i32 => _NATIONAL_HEROES_AND_BENEFACTORS_DAY__OBSERVED_,
    23115_i32 => _GOOD_FRIDAY,
    23116_i32 => _HOLY_SATURDAY,
    23118_i32 => _EASTER_MONDAY,
    23132_i32 => _LABOUR_DAY__OBSERVED_,
    23223_i32 => _EMANCIPATION_DAY,
    23263_i32 => _SAINT_GEORGE_S_CAYE_DAY,
    23274_i32 => _INDEPENDENCE_DAY,
    23293_i32 => _INDIGENOUS_PEOPLES__RESISTANCE_DAY__OBSERVED_,
    23333_i32 => _GARIFUNA_SETTLEMENT_DAY,
    23369_i32 => _CHRISTMAS_DAY,
    23370_i32 => _BOXING_DAY,
    23377_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    23391_i32 => _GEORGE_PRICE_DAY__OBSERVED_,
    23440_i32 => _NATIONAL_HEROES_AND_BENEFACTORS_DAY__OBSERVED_,
    23472_i32 => _GOOD_FRIDAY,
    23473_i32 => _HOLY_SATURDAY,
    23475_i32 => _EASTER_MONDAY,
    23496_i32 => _LABOUR_DAY,
    23587_i32 => _EMANCIPATION_DAY__OBSERVED_,
    23629_i32 => _SAINT_GEORGE_S_CAYE_DAY__OBSERVED_,
    23639_i32 => _INDEPENDENCE_DAY,
    23657_i32 => _INDIGENOUS_PEOPLES__RESISTANCE_DAY__OBSERVED_,
    23699_i32 => _GARIFUNA_SETTLEMENT_DAY__OBSERVED_,
    23734_i32 => _CHRISTMAS_DAY,
    23735_i32 => _BOXING_DAY,
    23741_i32 => _NEW_YEAR_S_DAY,
    23755_i32 => _GEORGE_PRICE_DAY,
    23811_i32 => _NATIONAL_HEROES_AND_BENEFACTORS_DAY__OBSERVED_,
    23822_i32 => _GOOD_FRIDAY,
    23823_i32 => _HOLY_SATURDAY,
    23825_i32 => _EASTER_MONDAY,
    23861_i32 => _LABOUR_DAY,
    23951_i32 => _EMANCIPATION_DAY__OBSERVED_,
    23993_i32 => _SAINT_GEORGE_S_CAYE_DAY,
    24004_i32 => _INDEPENDENCE_DAY,
    24028_i32 => _INDIGENOUS_PEOPLES__RESISTANCE_DAY__OBSERVED_,
    24063_i32 => _GARIFUNA_SETTLEMENT_DAY,
    24099_i32 => _CHRISTMAS_DAY,
    24100_i32 => _BOXING_DAY,
    24106_i32 => _NEW_YEAR_S_DAY,
    24120_i32 => _GEORGE_PRICE_DAY,
    24175_i32 => _NATIONAL_HEROES_AND_BENEFACTORS_DAY__OBSERVED_,
    24207_i32 => _GOOD_FRIDAY,
    24208_i32 => _HOLY_SATURDAY,
    24210_i32 => _EASTER_MONDAY,
    24227_i32 => _LABOUR_DAY,
    24322_i32 => _EMANCIPATION_DAY__OBSERVED_,
    24359_i32 => _SAINT_GEORGE_S_CAYE_DAY,
    24371_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    24392_i32 => _INDIGENOUS_PEOPLES__RESISTANCE_DAY__OBSERVED_,
    24429_i32 => _GARIFUNA_SETTLEMENT_DAY,
    24465_i32 => _CHRISTMAS_DAY,
    24466_i32 => _BOXING_DAY,
    24472_i32 => _NEW_YEAR_S_DAY,
    24486_i32 => _GEORGE_PRICE_DAY,
    24539_i32 => _NATIONAL_HEROES_AND_BENEFACTORS_DAY,
    24564_i32 => _GOOD_FRIDAY,
    24565_i32 => _HOLY_SATURDAY,
    24567_i32 => _EASTER_MONDAY,
    24592_i32 => _LABOUR_DAY,
    24684_i32 => _EMANCIPATION_DAY,
    24724_i32 => _SAINT_GEORGE_S_CAYE_DAY,
    24735_i32 => _INDEPENDENCE_DAY,
    24756_i32 => _INDIGENOUS_PEOPLES__RESISTANCE_DAY,
    24794_i32 => _GARIFUNA_SETTLEMENT_DAY,
    24830_i32 => _CHRISTMAS_DAY,
    24831_i32 => _BOXING_DAY,
    24837_i32 => _NEW_YEAR_S_DAY,
    24851_i32 => _GEORGE_PRICE_DAY,
    24903_i32 => _NATIONAL_HEROES_AND_BENEFACTORS_DAY__OBSERVED_,
    24949_i32 => _GOOD_FRIDAY,
    24950_i32 => _HOLY_SATURDAY,
    24952_i32 => _EASTER_MONDAY,
    24957_i32 => _LABOUR_DAY,
    25050_i32 => _EMANCIPATION_DAY__OBSERVED_,
    25089_i32 => _SAINT_GEORGE_S_CAYE_DAY,
    25100_i32 => _INDEPENDENCE_DAY,
    25120_i32 => _INDIGENOUS_PEOPLES__RESISTANCE_DAY__OBSERVED_,
    25159_i32 => _GARIFUNA_SETTLEMENT_DAY,
    25195_i32 => _CHRISTMAS_DAY,
    25197_i32 => _BOXING_DAY__OBSERVED_,
    25202_i32 => _NEW_YEAR_S_DAY,
    25216_i32 => _GEORGE_PRICE_DAY,
    25267_i32 => _NATIONAL_HEROES_AND_BENEFACTORS_DAY__OBSERVED_,
    25299_i32 => _GOOD_FRIDAY,
    25300_i32 => _HOLY_SATURDAY,
    25302_i32 => _EASTER_MONDAY,
    25323_i32 => _LABOUR_DAY__OBSERVED_,
    25414_i32 => _EMANCIPATION_DAY,
    25454_i32 => _SAINT_GEORGE_S_CAYE_DAY,
    25465_i32 => _INDEPENDENCE_DAY,
    25484_i32 => _INDIGENOUS_PEOPLES__RESISTANCE_DAY__OBSERVED_,
    25524_i32 => _GARIFUNA_SETTLEMENT_DAY,
    25560_i32 => _CHRISTMAS_DAY,
    25561_i32 => _BOXING_DAY,
    25568_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    25582_i32 => _GEORGE_PRICE_DAY__OBSERVED_,
    25638_i32 => _NATIONAL_HEROES_AND_BENEFACTORS_DAY__OBSERVED_,
    25656_i32 => _GOOD_FRIDAY,
    25657_i32 => _HOLY_SATURDAY,
    25659_i32 => _EASTER_MONDAY,
    25688_i32 => _LABOUR_DAY,
    25778_i32 => _EMANCIPATION_DAY__OBSERVED_,
    25820_i32 => _SAINT_GEORGE_S_CAYE_DAY,
    25831_i32 => _INDEPENDENCE_DAY,
    25855_i32 => _INDIGENOUS_PEOPLES__RESISTANCE_DAY__OBSERVED_,
    25890_i32 => _GARIFUNA_SETTLEMENT_DAY,
    25926_i32 => _CHRISTMAS_DAY,
    25927_i32 => _BOXING_DAY,
    25933_i32 => _NEW_YEAR_S_DAY,
    25947_i32 => _GEORGE_PRICE_DAY,
    26000_i32 => _NATIONAL_HEROES_AND_BENEFACTORS_DAY,
    26041_i32 => _GOOD_FRIDAY,
    26042_i32 => _HOLY_SATURDAY,
    26044_i32 => _EASTER_MONDAY,
    26053_i32 => _LABOUR_DAY,
    26142_i32 => _EMANCIPATION_DAY__OBSERVED_,
    26185_i32 => _SAINT_GEORGE_S_CAYE_DAY,
    26196_i32 => _INDEPENDENCE_DAY,
    26217_i32 => _INDIGENOUS_PEOPLES__RESISTANCE_DAY,
    26255_i32 => _GARIFUNA_SETTLEMENT_DAY,
    26291_i32 => _CHRISTMAS_DAY,
    26292_i32 => _BOXING_DAY,
    26298_i32 => _NEW_YEAR_S_DAY,
    26312_i32 => _GEORGE_PRICE_DAY,
    26366_i32 => _NATIONAL_HEROES_AND_BENEFACTORS_DAY__OBSERVED_,
    26391_i32 => _GOOD_FRIDAY,
    26392_i32 => _HOLY_SATURDAY,
    26394_i32 => _EASTER_MONDAY,
    26418_i32 => _LABOUR_DAY,
    26513_i32 => _EMANCIPATION_DAY__OBSERVED_,
    26550_i32 => _SAINT_GEORGE_S_CAYE_DAY,
    26562_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    26583_i32 => _INDIGENOUS_PEOPLES__RESISTANCE_DAY__OBSERVED_,
    26620_i32 => _GARIFUNA_SETTLEMENT_DAY,
    26656_i32 => _CHRISTMAS_DAY,
    26657_i32 => _BOXING_DAY,
    26663_i32 => _NEW_YEAR_S_DAY,
    26677_i32 => _GEORGE_PRICE_DAY,
    26730_i32 => _NATIONAL_HEROES_AND_BENEFACTORS_DAY,
    26748_i32 => _GOOD_FRIDAY,
    26749_i32 => _HOLY_SATURDAY,
    26751_i32 => _EASTER_MONDAY,
    26783_i32 => _LABOUR_DAY,
    26875_i32 => _EMANCIPATION_DAY,
    26915_i32 => _SAINT_GEORGE_S_CAYE_DAY,
    26926_i32 => _INDEPENDENCE_DAY,
    26947_i32 => _INDIGENOUS_PEOPLES__RESISTANCE_DAY,
    26985_i32 => _GARIFUNA_SETTLEMENT_DAY,
    27021_i32 => _CHRISTMAS_DAY,
    27022_i32 => _BOXING_DAY,
    27028_i32 => _NEW_YEAR_S_DAY,
    27042_i32 => _GEORGE_PRICE_DAY,
    27094_i32 => _NATIONAL_HEROES_AND_BENEFACTORS_DAY__OBSERVED_,
    27133_i32 => _GOOD_FRIDAY,
    27134_i32 => _HOLY_SATURDAY,
    27136_i32 => _EASTER_MONDAY,
    27150_i32 => _LABOUR_DAY__OBSERVED_,
    27241_i32 => _EMANCIPATION_DAY,
    27281_i32 => _SAINT_GEORGE_S_CAYE_DAY,
    27292_i32 => _INDEPENDENCE_DAY,
    27311_i32 => _INDIGENOUS_PEOPLES__RESISTANCE_DAY__OBSERVED_,
    27351_i32 => _GARIFUNA_SETTLEMENT_DAY,
    27387_i32 => _CHRISTMAS_DAY,
    27388_i32 => _BOXING_DAY,
    27395_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    27409_i32 => _GEORGE_PRICE_DAY__OBSERVED_,
    27458_i32 => _NATIONAL_HEROES_AND_BENEFACTORS_DAY__OBSERVED_,
    27490_i32 => _GOOD_FRIDAY,
    27491_i32 => _HOLY_SATURDAY,
    27493_i32 => _EASTER_MONDAY,
    27514_i32 => _LABOUR_DAY,
    27605_i32 => _EMANCIPATION_DAY__OBSERVED_,
    27647_i32 => _SAINT_GEORGE_S_CAYE_DAY__OBSERVED_,
    27657_i32 => _INDEPENDENCE_DAY,
    27675_i32 => _INDIGENOUS_PEOPLES__RESISTANCE_DAY__OBSERVED_,
    27717_i32 => _GARIFUNA_SETTLEMENT_DAY__OBSERVED_,
    27752_i32 => _CHRISTMAS_DAY,
    27753_i32 => _BOXING_DAY,
    27759_i32 => _NEW_YEAR_S_DAY,
    27773_i32 => _GEORGE_PRICE_DAY,
    27829_i32 => _NATIONAL_HEROES_AND_BENEFACTORS_DAY__OBSERVED_,
    27840_i32 => _GOOD_FRIDAY,
    27841_i32 => _HOLY_SATURDAY,
    27843_i32 => _EASTER_MONDAY,
    27879_i32 => _LABOUR_DAY,
    27969_i32 => _EMANCIPATION_DAY__OBSERVED_,
    28011_i32 => _SAINT_GEORGE_S_CAYE_DAY,
    28022_i32 => _INDEPENDENCE_DAY,
    28046_i32 => _INDIGENOUS_PEOPLES__RESISTANCE_DAY__OBSERVED_,
    28081_i32 => _GARIFUNA_SETTLEMENT_DAY,
    28117_i32 => _CHRISTMAS_DAY,
    28118_i32 => _BOXING_DAY,
    28124_i32 => _NEW_YEAR_S_DAY,
    28138_i32 => _GEORGE_PRICE_DAY,
    28191_i32 => _NATIONAL_HEROES_AND_BENEFACTORS_DAY,
    28225_i32 => _GOOD_FRIDAY,
    28226_i32 => _HOLY_SATURDAY,
    28228_i32 => _EASTER_MONDAY,
    28244_i32 => _LABOUR_DAY,
    28333_i32 => _EMANCIPATION_DAY__OBSERVED_,
    28376_i32 => _SAINT_GEORGE_S_CAYE_DAY,
    28387_i32 => _INDEPENDENCE_DAY,
    28408_i32 => _INDIGENOUS_PEOPLES__RESISTANCE_DAY,
    28446_i32 => _GARIFUNA_SETTLEMENT_DAY,
    28482_i32 => _CHRISTMAS_DAY,
    28483_i32 => _BOXING_DAY,
    28489_i32 => _NEW_YEAR_S_DAY,
    28503_i32 => _GEORGE_PRICE_DAY,
    28557_i32 => _NATIONAL_HEROES_AND_BENEFACTORS_DAY,
    28582_i32 => _GOOD_FRIDAY,
    28583_i32 => _HOLY_SATURDAY,
    28585_i32 => _EASTER_MONDAY,
    28610_i32 => _LABOUR_DAY,
    28702_i32 => _EMANCIPATION_DAY,
    28742_i32 => _SAINT_GEORGE_S_CAYE_DAY,
    28753_i32 => _INDEPENDENCE_DAY,
    28774_i32 => _INDIGENOUS_PEOPLES__RESISTANCE_DAY,
    28812_i32 => _GARIFUNA_SETTLEMENT_DAY,
    28848_i32 => _CHRISTMAS_DAY,
    28849_i32 => _BOXING_DAY,
    28855_i32 => _NEW_YEAR_S_DAY,
    28869_i32 => _GEORGE_PRICE_DAY,
    28921_i32 => _NATIONAL_HEROES_AND_BENEFACTORS_DAY__OBSERVED_,
    28960_i32 => _GOOD_FRIDAY,
    28961_i32 => _HOLY_SATURDAY,
    28963_i32 => _EASTER_MONDAY,
    28975_i32 => _LABOUR_DAY,
    29068_i32 => _EMANCIPATION_DAY__OBSERVED_,
    29107_i32 => _SAINT_GEORGE_S_CAYE_DAY,
    29118_i32 => _INDEPENDENCE_DAY,
    29138_i32 => _INDIGENOUS_PEOPLES__RESISTANCE_DAY__OBSERVED_,
    29177_i32 => _GARIFUNA_SETTLEMENT_DAY,
    29213_i32 => _CHRISTMAS_DAY,
    29215_i32 => _BOXING_DAY__OBSERVED_,
    29220_i32 => _NEW_YEAR_S_DAY,
};
