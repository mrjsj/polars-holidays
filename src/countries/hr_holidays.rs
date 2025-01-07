use crate::countries::constants::*;
use phf::phf_map;

pub static HR_HOLIDAYS: phf::Map<i32, &'static str> = phf_map! {
    10957_i32 => _NEW_YEAR_S_DAY,
    10962_i32 => _EPIPHANY,
    11071_i32 => _EASTER_MONDAY,
    11078_i32 => _LABOR_DAY,
    11107_i32 => _STATEHOOD_DAY,
    11130_i32 => _ANTI_FASCIST_STRUGGLE_DAY,
    11174_i32 => _VICTORY_AND_HOMELAND_THANKSGIVING_DAY,
    11184_i32 => _ASSUMPTION_DAY,
    11262_i32 => _ALL_SAINTS__DAY,
    11316_i32 => _CHRISTMAS_DAY,
    11317_i32 => _SAINT_STEPHEN_S_DAY,
    11323_i32 => _NEW_YEAR_S_DAY,
    11328_i32 => _EPIPHANY,
    11428_i32 => _EASTER_MONDAY,
    11443_i32 => _LABOR_DAY,
    11472_i32 => _STATEHOOD_DAY,
    11495_i32 => _ANTI_FASCIST_STRUGGLE_DAY,
    11539_i32 => _VICTORY_AND_HOMELAND_THANKSGIVING_DAY,
    11549_i32 => _ASSUMPTION_DAY,
    11627_i32 => _ALL_SAINTS__DAY,
    11681_i32 => _CHRISTMAS_DAY,
    11682_i32 => _SAINT_STEPHEN_S_DAY,
    11688_i32 => _NEW_YEAR_S_DAY,
    11778_i32 => _EASTER_MONDAY,
    11808_i32 => _LABOR_DAY,
    11837_i32 => _CORPUS_CHRISTI,
    11860_i32 => _ANTI_FASCIST_STRUGGLE_DAY,
    11863_i32 => _STATEHOOD_DAY,
    11904_i32 => _VICTORY_AND_HOMELAND_THANKSGIVING_DAY,
    11914_i32 => _ASSUMPTION_DAY,
    11968_i32 => _INDEPENDENCE_DAY,
    11992_i32 => _ALL_SAINTS__DAY,
    12046_i32 => _CHRISTMAS_DAY,
    12047_i32 => _SAINT_STEPHEN_S_DAY,
    12053_i32 => _NEW_YEAR_S_DAY,
    12058_i32 => _EPIPHANY,
    12163_i32 => _EASTER_MONDAY,
    12173_i32 => _LABOR_DAY,
    12222_i32 => _CORPUS_CHRISTI,
    12225_i32 => _ANTI_FASCIST_STRUGGLE_DAY,
    12228_i32 => _STATEHOOD_DAY,
    12269_i32 => _VICTORY_AND_HOMELAND_THANKSGIVING_DAY,
    12279_i32 => _ASSUMPTION_DAY,
    12333_i32 => _INDEPENDENCE_DAY,
    12357_i32 => _ALL_SAINTS__DAY,
    12411_i32 => _CHRISTMAS_DAY,
    12412_i32 => _SAINT_STEPHEN_S_DAY,
    12418_i32 => _NEW_YEAR_S_DAY,
    12423_i32 => _EPIPHANY,
    12520_i32 => _EASTER_MONDAY,
    12539_i32 => _LABOR_DAY,
    12579_i32 => _CORPUS_CHRISTI,
    12591_i32 => _ANTI_FASCIST_STRUGGLE_DAY,
    12594_i32 => _STATEHOOD_DAY,
    12635_i32 => _VICTORY_AND_HOMELAND_THANKSGIVING_DAY,
    12645_i32 => _ASSUMPTION_DAY,
    12699_i32 => _INDEPENDENCE_DAY,
    12723_i32 => _ALL_SAINTS__DAY,
    12777_i32 => _CHRISTMAS_DAY,
    12778_i32 => _SAINT_STEPHEN_S_DAY,
    12784_i32 => _NEW_YEAR_S_DAY,
    12789_i32 => _EPIPHANY,
    12870_i32 => _EASTER_MONDAY,
    12904_i32 => _LABOR_DAY,
    12929_i32 => _CORPUS_CHRISTI,
    12956_i32 => _ANTI_FASCIST_STRUGGLE_DAY,
    12959_i32 => _STATEHOOD_DAY,
    13000_i32 => _VICTORY_AND_HOMELAND_THANKSGIVING_DAY,
    13010_i32 => _ASSUMPTION_DAY,
    13064_i32 => _INDEPENDENCE_DAY,
    13088_i32 => _ALL_SAINTS__DAY,
    13142_i32 => _CHRISTMAS_DAY,
    13143_i32 => _SAINT_STEPHEN_S_DAY,
    13149_i32 => _NEW_YEAR_S_DAY,
    13154_i32 => _EPIPHANY,
    13255_i32 => _EASTER_MONDAY,
    13269_i32 => _LABOR_DAY,
    13314_i32 => _CORPUS_CHRISTI,
    13321_i32 => _ANTI_FASCIST_STRUGGLE_DAY,
    13324_i32 => _STATEHOOD_DAY,
    13365_i32 => _VICTORY_AND_HOMELAND_THANKSGIVING_DAY,
    13375_i32 => _ASSUMPTION_DAY,
    13429_i32 => _INDEPENDENCE_DAY,
    13453_i32 => _ALL_SAINTS__DAY,
    13507_i32 => _CHRISTMAS_DAY,
    13508_i32 => _SAINT_STEPHEN_S_DAY,
    13514_i32 => _NEW_YEAR_S_DAY,
    13519_i32 => _EPIPHANY,
    13612_i32 => _EASTER_MONDAY,
    13634_i32 => _LABOR_DAY,
    13671_i32 => _CORPUS_CHRISTI,
    13686_i32 => _ANTI_FASCIST_STRUGGLE_DAY,
    13689_i32 => _STATEHOOD_DAY,
    13730_i32 => _VICTORY_AND_HOMELAND_THANKSGIVING_DAY,
    13740_i32 => _ASSUMPTION_DAY,
    13794_i32 => _INDEPENDENCE_DAY,
    13818_i32 => _ALL_SAINTS__DAY,
    13872_i32 => _CHRISTMAS_DAY,
    13873_i32 => _SAINT_STEPHEN_S_DAY,
    13879_i32 => _NEW_YEAR_S_DAY,
    13884_i32 => _EPIPHANY,
    13962_i32 => _EASTER_MONDAY,
    14000_i32 => _LABOR_DAY,
    14021_i32 => _CORPUS_CHRISTI,
    14052_i32 => _ANTI_FASCIST_STRUGGLE_DAY,
    14055_i32 => _STATEHOOD_DAY,
    14096_i32 => _VICTORY_AND_HOMELAND_THANKSGIVING_DAY_AND_CROATIAN_VETERANS_DAY,
    14106_i32 => _ASSUMPTION_DAY,
    14160_i32 => _INDEPENDENCE_DAY,
    14184_i32 => _ALL_SAINTS__DAY,
    14238_i32 => _CHRISTMAS_DAY,
    14239_i32 => _SAINT_STEPHEN_S_DAY,
    14245_i32 => _NEW_YEAR_S_DAY,
    14250_i32 => _EPIPHANY,
    14346_i32 => _EASTER_SUNDAY,
    14347_i32 => _EASTER_MONDAY,
    14365_i32 => _LABOR_DAY,
    14406_i32 => _CORPUS_CHRISTI,
    14417_i32 => _ANTI_FASCIST_STRUGGLE_DAY,
    14420_i32 => _STATEHOOD_DAY,
    14461_i32 => _VICTORY_AND_HOMELAND_THANKSGIVING_DAY_AND_CROATIAN_VETERANS_DAY,
    14471_i32 => _ASSUMPTION_DAY,
    14525_i32 => _INDEPENDENCE_DAY,
    14549_i32 => _ALL_SAINTS__DAY,
    14603_i32 => _CHRISTMAS_DAY,
    14604_i32 => _SAINT_STEPHEN_S_DAY,
    14610_i32 => _NEW_YEAR_S_DAY,
    14615_i32 => _EPIPHANY,
    14703_i32 => _EASTER_SUNDAY,
    14704_i32 => _EASTER_MONDAY,
    14730_i32 => _LABOR_DAY,
    14763_i32 => _CORPUS_CHRISTI,
    14782_i32 => _ANTI_FASCIST_STRUGGLE_DAY,
    14785_i32 => _STATEHOOD_DAY,
    14826_i32 => _VICTORY_AND_HOMELAND_THANKSGIVING_DAY_AND_CROATIAN_VETERANS_DAY,
    14836_i32 => _ASSUMPTION_DAY,
    14890_i32 => _INDEPENDENCE_DAY,
    14914_i32 => _ALL_SAINTS__DAY,
    14968_i32 => _CHRISTMAS_DAY,
    14969_i32 => _SAINT_STEPHEN_S_DAY,
    14975_i32 => _NEW_YEAR_S_DAY,
    14980_i32 => _EPIPHANY,
    15088_i32 => _EASTER_SUNDAY,
    15089_i32 => _EASTER_MONDAY,
    15095_i32 => _LABOR_DAY,
    15147_i32 => _ANTI_FASCIST_STRUGGLE_DAY,
    15148_i32 => _CORPUS_CHRISTI,
    15150_i32 => _STATEHOOD_DAY,
    15191_i32 => _VICTORY_AND_HOMELAND_THANKSGIVING_DAY_AND_CROATIAN_VETERANS_DAY,
    15201_i32 => _ASSUMPTION_DAY,
    15255_i32 => _INDEPENDENCE_DAY,
    15279_i32 => _ALL_SAINTS__DAY,
    15333_i32 => _CHRISTMAS_DAY,
    15334_i32 => _SAINT_STEPHEN_S_DAY,
    15340_i32 => _NEW_YEAR_S_DAY,
    15345_i32 => _EPIPHANY,
    15438_i32 => _EASTER_SUNDAY,
    15439_i32 => _EASTER_MONDAY,
    15461_i32 => _LABOR_DAY,
    15498_i32 => _CORPUS_CHRISTI,
    15513_i32 => _ANTI_FASCIST_STRUGGLE_DAY,
    15516_i32 => _STATEHOOD_DAY,
    15557_i32 => _VICTORY_AND_HOMELAND_THANKSGIVING_DAY_AND_CROATIAN_VETERANS_DAY,
    15567_i32 => _ASSUMPTION_DAY,
    15621_i32 => _INDEPENDENCE_DAY,
    15645_i32 => _ALL_SAINTS__DAY,
    15699_i32 => _CHRISTMAS_DAY,
    15700_i32 => _SAINT_STEPHEN_S_DAY,
    15706_i32 => _NEW_YEAR_S_DAY,
    15711_i32 => _EPIPHANY,
    15795_i32 => _EASTER_SUNDAY,
    15796_i32 => _EASTER_MONDAY,
    15826_i32 => _LABOR_DAY,
    15855_i32 => _CORPUS_CHRISTI,
    15878_i32 => _ANTI_FASCIST_STRUGGLE_DAY,
    15881_i32 => _STATEHOOD_DAY,
    15922_i32 => _VICTORY_AND_HOMELAND_THANKSGIVING_DAY_AND_CROATIAN_VETERANS_DAY,
    15932_i32 => _ASSUMPTION_DAY,
    15986_i32 => _INDEPENDENCE_DAY,
    16010_i32 => _ALL_SAINTS__DAY,
    16064_i32 => _CHRISTMAS_DAY,
    16065_i32 => _SAINT_STEPHEN_S_DAY,
    16071_i32 => _NEW_YEAR_S_DAY,
    16076_i32 => _EPIPHANY,
    16180_i32 => _EASTER_SUNDAY,
    16181_i32 => _EASTER_MONDAY,
    16191_i32 => _LABOR_DAY,
    16240_i32 => _CORPUS_CHRISTI,
    16243_i32 => _ANTI_FASCIST_STRUGGLE_DAY,
    16246_i32 => _STATEHOOD_DAY,
    16287_i32 => _VICTORY_AND_HOMELAND_THANKSGIVING_DAY_AND_CROATIAN_VETERANS_DAY,
    16297_i32 => _ASSUMPTION_DAY,
    16351_i32 => _INDEPENDENCE_DAY,
    16375_i32 => _ALL_SAINTS__DAY,
    16429_i32 => _CHRISTMAS_DAY,
    16430_i32 => _SAINT_STEPHEN_S_DAY,
    16436_i32 => _NEW_YEAR_S_DAY,
    16441_i32 => _EPIPHANY,
    16530_i32 => _EASTER_SUNDAY,
    16531_i32 => _EASTER_MONDAY,
    16556_i32 => _LABOR_DAY,
    16590_i32 => _CORPUS_CHRISTI,
    16608_i32 => _ANTI_FASCIST_STRUGGLE_DAY,
    16611_i32 => _STATEHOOD_DAY,
    16652_i32 => _VICTORY_AND_HOMELAND_THANKSGIVING_DAY_AND_CROATIAN_VETERANS_DAY,
    16662_i32 => _ASSUMPTION_DAY,
    16716_i32 => _INDEPENDENCE_DAY,
    16740_i32 => _ALL_SAINTS__DAY,
    16794_i32 => _CHRISTMAS_DAY,
    16795_i32 => _SAINT_STEPHEN_S_DAY,
    16801_i32 => _NEW_YEAR_S_DAY,
    16806_i32 => _EPIPHANY,
    16887_i32 => _EASTER_SUNDAY,
    16888_i32 => _EASTER_MONDAY,
    16922_i32 => _LABOR_DAY,
    16947_i32 => _CORPUS_CHRISTI,
    16974_i32 => _ANTI_FASCIST_STRUGGLE_DAY,
    16977_i32 => _STATEHOOD_DAY,
    17018_i32 => _VICTORY_AND_HOMELAND_THANKSGIVING_DAY_AND_CROATIAN_VETERANS_DAY,
    17028_i32 => _ASSUMPTION_DAY,
    17082_i32 => _INDEPENDENCE_DAY,
    17106_i32 => _ALL_SAINTS__DAY,
    17160_i32 => _CHRISTMAS_DAY,
    17161_i32 => _SAINT_STEPHEN_S_DAY,
    17167_i32 => _NEW_YEAR_S_DAY,
    17172_i32 => _EPIPHANY,
    17272_i32 => _EASTER_SUNDAY,
    17273_i32 => _EASTER_MONDAY,
    17287_i32 => _LABOR_DAY,
    17332_i32 => _CORPUS_CHRISTI,
    17339_i32 => _ANTI_FASCIST_STRUGGLE_DAY,
    17342_i32 => _STATEHOOD_DAY,
    17383_i32 => _VICTORY_AND_HOMELAND_THANKSGIVING_DAY_AND_CROATIAN_VETERANS_DAY,
    17393_i32 => _ASSUMPTION_DAY,
    17447_i32 => _INDEPENDENCE_DAY,
    17471_i32 => _ALL_SAINTS__DAY,
    17525_i32 => _CHRISTMAS_DAY,
    17526_i32 => _SAINT_STEPHEN_S_DAY,
    17532_i32 => _NEW_YEAR_S_DAY,
    17537_i32 => _EPIPHANY,
    17622_i32 => _EASTER_SUNDAY,
    17623_i32 => _EASTER_MONDAY,
    17652_i32 => _LABOR_DAY,
    17682_i32 => _CORPUS_CHRISTI,
    17704_i32 => _ANTI_FASCIST_STRUGGLE_DAY,
    17707_i32 => _STATEHOOD_DAY,
    17748_i32 => _VICTORY_AND_HOMELAND_THANKSGIVING_DAY_AND_CROATIAN_VETERANS_DAY,
    17758_i32 => _ASSUMPTION_DAY,
    17812_i32 => _INDEPENDENCE_DAY,
    17836_i32 => _ALL_SAINTS__DAY,
    17890_i32 => _CHRISTMAS_DAY,
    17891_i32 => _SAINT_STEPHEN_S_DAY,
    17897_i32 => _NEW_YEAR_S_DAY,
    17902_i32 => _EPIPHANY,
    18007_i32 => _EASTER_SUNDAY,
    18008_i32 => _EASTER_MONDAY,
    18017_i32 => _LABOR_DAY,
    18067_i32 => _CORPUS_CHRISTI,
    18069_i32 => _ANTI_FASCIST_STRUGGLE_DAY,
    18072_i32 => _STATEHOOD_DAY,
    18113_i32 => _VICTORY_AND_HOMELAND_THANKSGIVING_DAY_AND_CROATIAN_VETERANS_DAY,
    18123_i32 => _ASSUMPTION_DAY,
    18177_i32 => _INDEPENDENCE_DAY,
    18201_i32 => _ALL_SAINTS__DAY,
    18255_i32 => _CHRISTMAS_DAY,
    18256_i32 => _SAINT_STEPHEN_S_DAY,
    18262_i32 => _NEW_YEAR_S_DAY,
    18267_i32 => _EPIPHANY,
    18364_i32 => _EASTER_SUNDAY,
    18365_i32 => _EASTER_MONDAY,
    18383_i32 => _LABOR_DAY,
    18412_i32 => _STATEHOOD_DAY,
    18424_i32 => _CORPUS_CHRISTI,
    18435_i32 => _ANTI_FASCIST_STRUGGLE_DAY,
    18479_i32 => _VICTORY_AND_HOMELAND_THANKSGIVING_DAY_AND_CROATIAN_VETERANS_DAY,
    18489_i32 => _ASSUMPTION_DAY,
    18567_i32 => _ALL_SAINTS__DAY,
    18584_i32 => _REMEMBRANCE_DAY,
    18621_i32 => _CHRISTMAS_DAY,
    18622_i32 => _SAINT_STEPHEN_S_DAY,
    18628_i32 => _NEW_YEAR_S_DAY,
    18633_i32 => _EPIPHANY,
    18721_i32 => _EASTER_SUNDAY,
    18722_i32 => _EASTER_MONDAY,
    18748_i32 => _LABOR_DAY,
    18777_i32 => _STATEHOOD_DAY,
    18781_i32 => _CORPUS_CHRISTI,
    18800_i32 => _ANTI_FASCIST_STRUGGLE_DAY,
    18844_i32 => _VICTORY_AND_HOMELAND_THANKSGIVING_DAY_AND_CROATIAN_VETERANS_DAY,
    18854_i32 => _ASSUMPTION_DAY,
    18932_i32 => _ALL_SAINTS__DAY,
    18949_i32 => _REMEMBRANCE_DAY,
    18986_i32 => _CHRISTMAS_DAY,
    18987_i32 => _SAINT_STEPHEN_S_DAY,
    18993_i32 => _NEW_YEAR_S_DAY,
    18998_i32 => _EPIPHANY,
    19099_i32 => _EASTER_SUNDAY,
    19100_i32 => _EASTER_MONDAY,
    19113_i32 => _LABOR_DAY,
    19142_i32 => _STATEHOOD_DAY,
    19159_i32 => _CORPUS_CHRISTI,
    19165_i32 => _ANTI_FASCIST_STRUGGLE_DAY,
    19209_i32 => _VICTORY_AND_HOMELAND_THANKSGIVING_DAY_AND_CROATIAN_VETERANS_DAY,
    19219_i32 => _ASSUMPTION_DAY,
    19297_i32 => _ALL_SAINTS__DAY,
    19314_i32 => _REMEMBRANCE_DAY,
    19351_i32 => _CHRISTMAS_DAY,
    19352_i32 => _SAINT_STEPHEN_S_DAY,
    19358_i32 => _NEW_YEAR_S_DAY,
    19363_i32 => _EPIPHANY,
    19456_i32 => _EASTER_SUNDAY,
    19457_i32 => _EASTER_MONDAY,
    19478_i32 => _LABOR_DAY,
    19507_i32 => _STATEHOOD_DAY,
    19516_i32 => _CORPUS_CHRISTI,
    19530_i32 => _ANTI_FASCIST_STRUGGLE_DAY,
    19574_i32 => _VICTORY_AND_HOMELAND_THANKSGIVING_DAY_AND_CROATIAN_VETERANS_DAY,
    19584_i32 => _ASSUMPTION_DAY,
    19662_i32 => _ALL_SAINTS__DAY,
    19679_i32 => _REMEMBRANCE_DAY,
    19716_i32 => _CHRISTMAS_DAY,
    19717_i32 => _SAINT_STEPHEN_S_DAY,
    19723_i32 => _NEW_YEAR_S_DAY,
    19728_i32 => _EPIPHANY,
    19813_i32 => _EASTER_SUNDAY,
    19814_i32 => _EASTER_MONDAY,
    19844_i32 => _LABOR_DAY,
    19873_i32 => _CORPUS_CHRISTI__STATEHOOD_DAY,
    19896_i32 => _ANTI_FASCIST_STRUGGLE_DAY,
    19940_i32 => _VICTORY_AND_HOMELAND_THANKSGIVING_DAY_AND_CROATIAN_VETERANS_DAY,
    19950_i32 => _ASSUMPTION_DAY,
    20028_i32 => _ALL_SAINTS__DAY,
    20045_i32 => _REMEMBRANCE_DAY,
    20082_i32 => _CHRISTMAS_DAY,
    20083_i32 => _SAINT_STEPHEN_S_DAY,
    20089_i32 => _NEW_YEAR_S_DAY,
    20094_i32 => _EPIPHANY,
    20198_i32 => _EASTER_SUNDAY,
    20199_i32 => _EASTER_MONDAY,
    20209_i32 => _LABOR_DAY,
    20238_i32 => _STATEHOOD_DAY,
    20258_i32 => _CORPUS_CHRISTI,
    20261_i32 => _ANTI_FASCIST_STRUGGLE_DAY,
    20305_i32 => _VICTORY_AND_HOMELAND_THANKSGIVING_DAY_AND_CROATIAN_VETERANS_DAY,
    20315_i32 => _ASSUMPTION_DAY,
    20393_i32 => _ALL_SAINTS__DAY,
    20410_i32 => _REMEMBRANCE_DAY,
    20447_i32 => _CHRISTMAS_DAY,
    20448_i32 => _SAINT_STEPHEN_S_DAY,
    20454_i32 => _NEW_YEAR_S_DAY,
    20459_i32 => _EPIPHANY,
    20548_i32 => _EASTER_SUNDAY,
    20549_i32 => _EASTER_MONDAY,
    20574_i32 => _LABOR_DAY,
    20603_i32 => _STATEHOOD_DAY,
    20608_i32 => _CORPUS_CHRISTI,
    20626_i32 => _ANTI_FASCIST_STRUGGLE_DAY,
    20670_i32 => _VICTORY_AND_HOMELAND_THANKSGIVING_DAY_AND_CROATIAN_VETERANS_DAY,
    20680_i32 => _ASSUMPTION_DAY,
    20758_i32 => _ALL_SAINTS__DAY,
    20775_i32 => _REMEMBRANCE_DAY,
    20812_i32 => _CHRISTMAS_DAY,
    20813_i32 => _SAINT_STEPHEN_S_DAY,
    20819_i32 => _NEW_YEAR_S_DAY,
    20824_i32 => _EPIPHANY,
    20905_i32 => _EASTER_SUNDAY,
    20906_i32 => _EASTER_MONDAY,
    20939_i32 => _LABOR_DAY,
    20965_i32 => _CORPUS_CHRISTI,
    20968_i32 => _STATEHOOD_DAY,
    20991_i32 => _ANTI_FASCIST_STRUGGLE_DAY,
    21035_i32 => _VICTORY_AND_HOMELAND_THANKSGIVING_DAY_AND_CROATIAN_VETERANS_DAY,
    21045_i32 => _ASSUMPTION_DAY,
    21123_i32 => _ALL_SAINTS__DAY,
    21140_i32 => _REMEMBRANCE_DAY,
    21177_i32 => _CHRISTMAS_DAY,
    21178_i32 => _SAINT_STEPHEN_S_DAY,
    21184_i32 => _NEW_YEAR_S_DAY,
    21189_i32 => _EPIPHANY,
    21290_i32 => _EASTER_SUNDAY,
    21291_i32 => _EASTER_MONDAY,
    21305_i32 => _LABOR_DAY,
    21334_i32 => _STATEHOOD_DAY,
    21350_i32 => _CORPUS_CHRISTI,
    21357_i32 => _ANTI_FASCIST_STRUGGLE_DAY,
    21401_i32 => _VICTORY_AND_HOMELAND_THANKSGIVING_DAY_AND_CROATIAN_VETERANS_DAY,
    21411_i32 => _ASSUMPTION_DAY,
    21489_i32 => _ALL_SAINTS__DAY,
    21506_i32 => _REMEMBRANCE_DAY,
    21543_i32 => _CHRISTMAS_DAY,
    21544_i32 => _SAINT_STEPHEN_S_DAY,
    21550_i32 => _NEW_YEAR_S_DAY,
    21555_i32 => _EPIPHANY,
    21640_i32 => _EASTER_SUNDAY,
    21641_i32 => _EASTER_MONDAY,
    21670_i32 => _LABOR_DAY,
    21699_i32 => _STATEHOOD_DAY,
    21700_i32 => _CORPUS_CHRISTI,
    21722_i32 => _ANTI_FASCIST_STRUGGLE_DAY,
    21766_i32 => _VICTORY_AND_HOMELAND_THANKSGIVING_DAY_AND_CROATIAN_VETERANS_DAY,
    21776_i32 => _ASSUMPTION_DAY,
    21854_i32 => _ALL_SAINTS__DAY,
    21871_i32 => _REMEMBRANCE_DAY,
    21908_i32 => _CHRISTMAS_DAY,
    21909_i32 => _SAINT_STEPHEN_S_DAY,
    21915_i32 => _NEW_YEAR_S_DAY,
    21920_i32 => _EPIPHANY,
    22025_i32 => _EASTER_SUNDAY,
    22026_i32 => _EASTER_MONDAY,
    22035_i32 => _LABOR_DAY,
    22064_i32 => _STATEHOOD_DAY,
    22085_i32 => _CORPUS_CHRISTI,
    22087_i32 => _ANTI_FASCIST_STRUGGLE_DAY,
    22131_i32 => _VICTORY_AND_HOMELAND_THANKSGIVING_DAY_AND_CROATIAN_VETERANS_DAY,
    22141_i32 => _ASSUMPTION_DAY,
    22219_i32 => _ALL_SAINTS__DAY,
    22236_i32 => _REMEMBRANCE_DAY,
    22273_i32 => _CHRISTMAS_DAY,
    22274_i32 => _SAINT_STEPHEN_S_DAY,
    22280_i32 => _NEW_YEAR_S_DAY,
    22285_i32 => _EPIPHANY,
    22382_i32 => _EASTER_SUNDAY,
    22383_i32 => _EASTER_MONDAY,
    22400_i32 => _LABOR_DAY,
    22429_i32 => _STATEHOOD_DAY,
    22442_i32 => _CORPUS_CHRISTI,
    22452_i32 => _ANTI_FASCIST_STRUGGLE_DAY,
    22496_i32 => _VICTORY_AND_HOMELAND_THANKSGIVING_DAY_AND_CROATIAN_VETERANS_DAY,
    22506_i32 => _ASSUMPTION_DAY,
    22584_i32 => _ALL_SAINTS__DAY,
    22601_i32 => _REMEMBRANCE_DAY,
    22638_i32 => _CHRISTMAS_DAY,
    22639_i32 => _SAINT_STEPHEN_S_DAY,
    22645_i32 => _NEW_YEAR_S_DAY,
    22650_i32 => _EPIPHANY,
    22732_i32 => _EASTER_SUNDAY,
    22733_i32 => _EASTER_MONDAY,
    22766_i32 => _LABOR_DAY,
    22792_i32 => _CORPUS_CHRISTI,
    22795_i32 => _STATEHOOD_DAY,
    22818_i32 => _ANTI_FASCIST_STRUGGLE_DAY,
    22862_i32 => _VICTORY_AND_HOMELAND_THANKSGIVING_DAY_AND_CROATIAN_VETERANS_DAY,
    22872_i32 => _ASSUMPTION_DAY,
    22950_i32 => _ALL_SAINTS__DAY,
    22967_i32 => _REMEMBRANCE_DAY,
    23004_i32 => _CHRISTMAS_DAY,
    23005_i32 => _SAINT_STEPHEN_S_DAY,
    23011_i32 => _NEW_YEAR_S_DAY,
    23016_i32 => _EPIPHANY,
    23117_i32 => _EASTER_SUNDAY,
    23118_i32 => _EASTER_MONDAY,
    23131_i32 => _LABOR_DAY,
    23160_i32 => _STATEHOOD_DAY,
    23177_i32 => _CORPUS_CHRISTI,
    23183_i32 => _ANTI_FASCIST_STRUGGLE_DAY,
    23227_i32 => _VICTORY_AND_HOMELAND_THANKSGIVING_DAY_AND_CROATIAN_VETERANS_DAY,
    23237_i32 => _ASSUMPTION_DAY,
    23315_i32 => _ALL_SAINTS__DAY,
    23332_i32 => _REMEMBRANCE_DAY,
    23369_i32 => _CHRISTMAS_DAY,
    23370_i32 => _SAINT_STEPHEN_S_DAY,
    23376_i32 => _NEW_YEAR_S_DAY,
    23381_i32 => _EPIPHANY,
    23474_i32 => _EASTER_SUNDAY,
    23475_i32 => _EASTER_MONDAY,
    23496_i32 => _LABOR_DAY,
    23525_i32 => _STATEHOOD_DAY,
    23534_i32 => _CORPUS_CHRISTI,
    23548_i32 => _ANTI_FASCIST_STRUGGLE_DAY,
    23592_i32 => _VICTORY_AND_HOMELAND_THANKSGIVING_DAY_AND_CROATIAN_VETERANS_DAY,
    23602_i32 => _ASSUMPTION_DAY,
    23680_i32 => _ALL_SAINTS__DAY,
    23697_i32 => _REMEMBRANCE_DAY,
    23734_i32 => _CHRISTMAS_DAY,
    23735_i32 => _SAINT_STEPHEN_S_DAY,
    23741_i32 => _NEW_YEAR_S_DAY,
    23746_i32 => _EPIPHANY,
    23824_i32 => _EASTER_SUNDAY,
    23825_i32 => _EASTER_MONDAY,
    23861_i32 => _LABOR_DAY,
    23884_i32 => _CORPUS_CHRISTI,
    23890_i32 => _STATEHOOD_DAY,
    23913_i32 => _ANTI_FASCIST_STRUGGLE_DAY,
    23957_i32 => _VICTORY_AND_HOMELAND_THANKSGIVING_DAY_AND_CROATIAN_VETERANS_DAY,
    23967_i32 => _ASSUMPTION_DAY,
    24045_i32 => _ALL_SAINTS__DAY,
    24062_i32 => _REMEMBRANCE_DAY,
    24099_i32 => _CHRISTMAS_DAY,
    24100_i32 => _SAINT_STEPHEN_S_DAY,
    24106_i32 => _NEW_YEAR_S_DAY,
    24111_i32 => _EPIPHANY,
    24209_i32 => _EASTER_SUNDAY,
    24210_i32 => _EASTER_MONDAY,
    24227_i32 => _LABOR_DAY,
    24256_i32 => _STATEHOOD_DAY,
    24269_i32 => _CORPUS_CHRISTI,
    24279_i32 => _ANTI_FASCIST_STRUGGLE_DAY,
    24323_i32 => _VICTORY_AND_HOMELAND_THANKSGIVING_DAY_AND_CROATIAN_VETERANS_DAY,
    24333_i32 => _ASSUMPTION_DAY,
    24411_i32 => _ALL_SAINTS__DAY,
    24428_i32 => _REMEMBRANCE_DAY,
    24465_i32 => _CHRISTMAS_DAY,
    24466_i32 => _SAINT_STEPHEN_S_DAY,
    24472_i32 => _NEW_YEAR_S_DAY,
    24477_i32 => _EPIPHANY,
    24566_i32 => _EASTER_SUNDAY,
    24567_i32 => _EASTER_MONDAY,
    24592_i32 => _LABOR_DAY,
    24621_i32 => _STATEHOOD_DAY,
    24626_i32 => _CORPUS_CHRISTI,
    24644_i32 => _ANTI_FASCIST_STRUGGLE_DAY,
    24688_i32 => _VICTORY_AND_HOMELAND_THANKSGIVING_DAY_AND_CROATIAN_VETERANS_DAY,
    24698_i32 => _ASSUMPTION_DAY,
    24776_i32 => _ALL_SAINTS__DAY,
    24793_i32 => _REMEMBRANCE_DAY,
    24830_i32 => _CHRISTMAS_DAY,
    24831_i32 => _SAINT_STEPHEN_S_DAY,
    24837_i32 => _NEW_YEAR_S_DAY,
    24842_i32 => _EPIPHANY,
    24951_i32 => _EASTER_SUNDAY,
    24952_i32 => _EASTER_MONDAY,
    24957_i32 => _LABOR_DAY,
    24986_i32 => _STATEHOOD_DAY,
    25009_i32 => _ANTI_FASCIST_STRUGGLE_DAY,
    25011_i32 => _CORPUS_CHRISTI,
    25053_i32 => _VICTORY_AND_HOMELAND_THANKSGIVING_DAY_AND_CROATIAN_VETERANS_DAY,
    25063_i32 => _ASSUMPTION_DAY,
    25141_i32 => _ALL_SAINTS__DAY,
    25158_i32 => _REMEMBRANCE_DAY,
    25195_i32 => _CHRISTMAS_DAY,
    25196_i32 => _SAINT_STEPHEN_S_DAY,
    25202_i32 => _NEW_YEAR_S_DAY,
    25207_i32 => _EPIPHANY,
    25301_i32 => _EASTER_SUNDAY,
    25302_i32 => _EASTER_MONDAY,
    25322_i32 => _LABOR_DAY,
    25351_i32 => _STATEHOOD_DAY,
    25361_i32 => _CORPUS_CHRISTI,
    25374_i32 => _ANTI_FASCIST_STRUGGLE_DAY,
    25418_i32 => _VICTORY_AND_HOMELAND_THANKSGIVING_DAY_AND_CROATIAN_VETERANS_DAY,
    25428_i32 => _ASSUMPTION_DAY,
    25506_i32 => _ALL_SAINTS__DAY,
    25523_i32 => _REMEMBRANCE_DAY,
    25560_i32 => _CHRISTMAS_DAY,
    25561_i32 => _SAINT_STEPHEN_S_DAY,
    25567_i32 => _NEW_YEAR_S_DAY,
    25572_i32 => _EPIPHANY,
    25658_i32 => _EASTER_SUNDAY,
    25659_i32 => _EASTER_MONDAY,
    25688_i32 => _LABOR_DAY,
    25717_i32 => _STATEHOOD_DAY,
    25718_i32 => _CORPUS_CHRISTI,
    25740_i32 => _ANTI_FASCIST_STRUGGLE_DAY,
    25784_i32 => _VICTORY_AND_HOMELAND_THANKSGIVING_DAY_AND_CROATIAN_VETERANS_DAY,
    25794_i32 => _ASSUMPTION_DAY,
    25872_i32 => _ALL_SAINTS__DAY,
    25889_i32 => _REMEMBRANCE_DAY,
    25926_i32 => _CHRISTMAS_DAY,
    25927_i32 => _SAINT_STEPHEN_S_DAY,
    25933_i32 => _NEW_YEAR_S_DAY,
    25938_i32 => _EPIPHANY,
    26043_i32 => _EASTER_SUNDAY,
    26044_i32 => _EASTER_MONDAY,
    26053_i32 => _LABOR_DAY,
    26082_i32 => _STATEHOOD_DAY,
    26103_i32 => _CORPUS_CHRISTI,
    26105_i32 => _ANTI_FASCIST_STRUGGLE_DAY,
    26149_i32 => _VICTORY_AND_HOMELAND_THANKSGIVING_DAY_AND_CROATIAN_VETERANS_DAY,
    26159_i32 => _ASSUMPTION_DAY,
    26237_i32 => _ALL_SAINTS__DAY,
    26254_i32 => _REMEMBRANCE_DAY,
    26291_i32 => _CHRISTMAS_DAY,
    26292_i32 => _SAINT_STEPHEN_S_DAY,
    26298_i32 => _NEW_YEAR_S_DAY,
    26303_i32 => _EPIPHANY,
    26393_i32 => _EASTER_SUNDAY,
    26394_i32 => _EASTER_MONDAY,
    26418_i32 => _LABOR_DAY,
    26447_i32 => _STATEHOOD_DAY,
    26453_i32 => _CORPUS_CHRISTI,
    26470_i32 => _ANTI_FASCIST_STRUGGLE_DAY,
    26514_i32 => _VICTORY_AND_HOMELAND_THANKSGIVING_DAY_AND_CROATIAN_VETERANS_DAY,
    26524_i32 => _ASSUMPTION_DAY,
    26602_i32 => _ALL_SAINTS__DAY,
    26619_i32 => _REMEMBRANCE_DAY,
    26656_i32 => _CHRISTMAS_DAY,
    26657_i32 => _SAINT_STEPHEN_S_DAY,
    26663_i32 => _NEW_YEAR_S_DAY,
    26668_i32 => _EPIPHANY,
    26750_i32 => _EASTER_SUNDAY,
    26751_i32 => _EASTER_MONDAY,
    26783_i32 => _LABOR_DAY,
    26810_i32 => _CORPUS_CHRISTI,
    26812_i32 => _STATEHOOD_DAY,
    26835_i32 => _ANTI_FASCIST_STRUGGLE_DAY,
    26879_i32 => _VICTORY_AND_HOMELAND_THANKSGIVING_DAY_AND_CROATIAN_VETERANS_DAY,
    26889_i32 => _ASSUMPTION_DAY,
    26967_i32 => _ALL_SAINTS__DAY,
    26984_i32 => _REMEMBRANCE_DAY,
    27021_i32 => _CHRISTMAS_DAY,
    27022_i32 => _SAINT_STEPHEN_S_DAY,
    27028_i32 => _NEW_YEAR_S_DAY,
    27033_i32 => _EPIPHANY,
    27135_i32 => _EASTER_SUNDAY,
    27136_i32 => _EASTER_MONDAY,
    27149_i32 => _LABOR_DAY,
    27178_i32 => _STATEHOOD_DAY,
    27195_i32 => _CORPUS_CHRISTI,
    27201_i32 => _ANTI_FASCIST_STRUGGLE_DAY,
    27245_i32 => _VICTORY_AND_HOMELAND_THANKSGIVING_DAY_AND_CROATIAN_VETERANS_DAY,
    27255_i32 => _ASSUMPTION_DAY,
    27333_i32 => _ALL_SAINTS__DAY,
    27350_i32 => _REMEMBRANCE_DAY,
    27387_i32 => _CHRISTMAS_DAY,
    27388_i32 => _SAINT_STEPHEN_S_DAY,
    27394_i32 => _NEW_YEAR_S_DAY,
    27399_i32 => _EPIPHANY,
    27492_i32 => _EASTER_SUNDAY,
    27493_i32 => _EASTER_MONDAY,
    27514_i32 => _LABOR_DAY,
    27543_i32 => _STATEHOOD_DAY,
    27552_i32 => _CORPUS_CHRISTI,
    27566_i32 => _ANTI_FASCIST_STRUGGLE_DAY,
    27610_i32 => _VICTORY_AND_HOMELAND_THANKSGIVING_DAY_AND_CROATIAN_VETERANS_DAY,
    27620_i32 => _ASSUMPTION_DAY,
    27698_i32 => _ALL_SAINTS__DAY,
    27715_i32 => _REMEMBRANCE_DAY,
    27752_i32 => _CHRISTMAS_DAY,
    27753_i32 => _SAINT_STEPHEN_S_DAY,
    27759_i32 => _NEW_YEAR_S_DAY,
    27764_i32 => _EPIPHANY,
    27842_i32 => _EASTER_SUNDAY,
    27843_i32 => _EASTER_MONDAY,
    27879_i32 => _LABOR_DAY,
    27902_i32 => _CORPUS_CHRISTI,
    27908_i32 => _STATEHOOD_DAY,
    27931_i32 => _ANTI_FASCIST_STRUGGLE_DAY,
    27975_i32 => _VICTORY_AND_HOMELAND_THANKSGIVING_DAY_AND_CROATIAN_VETERANS_DAY,
    27985_i32 => _ASSUMPTION_DAY,
    28063_i32 => _ALL_SAINTS__DAY,
    28080_i32 => _REMEMBRANCE_DAY,
    28117_i32 => _CHRISTMAS_DAY,
    28118_i32 => _SAINT_STEPHEN_S_DAY,
    28124_i32 => _NEW_YEAR_S_DAY,
    28129_i32 => _EPIPHANY,
    28227_i32 => _EASTER_SUNDAY,
    28228_i32 => _EASTER_MONDAY,
    28244_i32 => _LABOR_DAY,
    28273_i32 => _STATEHOOD_DAY,
    28287_i32 => _CORPUS_CHRISTI,
    28296_i32 => _ANTI_FASCIST_STRUGGLE_DAY,
    28340_i32 => _VICTORY_AND_HOMELAND_THANKSGIVING_DAY_AND_CROATIAN_VETERANS_DAY,
    28350_i32 => _ASSUMPTION_DAY,
    28428_i32 => _ALL_SAINTS__DAY,
    28445_i32 => _REMEMBRANCE_DAY,
    28482_i32 => _CHRISTMAS_DAY,
    28483_i32 => _SAINT_STEPHEN_S_DAY,
    28489_i32 => _NEW_YEAR_S_DAY,
    28494_i32 => _EPIPHANY,
    28584_i32 => _EASTER_SUNDAY,
    28585_i32 => _EASTER_MONDAY,
    28610_i32 => _LABOR_DAY,
    28639_i32 => _STATEHOOD_DAY,
    28644_i32 => _CORPUS_CHRISTI,
    28662_i32 => _ANTI_FASCIST_STRUGGLE_DAY,
    28706_i32 => _VICTORY_AND_HOMELAND_THANKSGIVING_DAY_AND_CROATIAN_VETERANS_DAY,
    28716_i32 => _ASSUMPTION_DAY,
    28794_i32 => _ALL_SAINTS__DAY,
    28811_i32 => _REMEMBRANCE_DAY,
    28848_i32 => _CHRISTMAS_DAY,
    28849_i32 => _SAINT_STEPHEN_S_DAY,
    28855_i32 => _NEW_YEAR_S_DAY,
    28860_i32 => _EPIPHANY,
    28962_i32 => _EASTER_SUNDAY,
    28963_i32 => _EASTER_MONDAY,
    28975_i32 => _LABOR_DAY,
    29004_i32 => _STATEHOOD_DAY,
    29022_i32 => _CORPUS_CHRISTI,
    29027_i32 => _ANTI_FASCIST_STRUGGLE_DAY,
    29071_i32 => _VICTORY_AND_HOMELAND_THANKSGIVING_DAY_AND_CROATIAN_VETERANS_DAY,
    29081_i32 => _ASSUMPTION_DAY,
    29159_i32 => _ALL_SAINTS__DAY,
    29176_i32 => _REMEMBRANCE_DAY,
    29213_i32 => _CHRISTMAS_DAY,
    29214_i32 => _SAINT_STEPHEN_S_DAY,
    29220_i32 => _NEW_YEAR_S_DAY,
};
