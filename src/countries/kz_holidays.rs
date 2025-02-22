use crate::countries::constants::*;
use phf::phf_map;

pub static KZ_HOLIDAYS: phf::Map<i32, &'static str> = phf_map! {
    10957_i32 => _NEW_YEAR_S_DAY,
    10958_i32 => _NEW_YEAR_S_DAY,
    11024_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    11078_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY,
    11085_i32 => _DAY_OFF__SUBSTITUTED_FROM_05_06_2000_,
    11086_i32 => _VICTORY_DAY,
    11199_i32 => _CONSTITUTION_DAY,
    11255_i32 => _REPUBLIC_DAY,
    11307_i32 => _INDEPENDENCE_DAY,
    11323_i32 => _NEW_YEAR_S_DAY,
    11324_i32 => _NEW_YEAR_S_DAY,
    11389_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    11390_i32 => _DAY_OFF__SUBSTITUTED_FROM_03_11_2001_,
    11404_i32 => _DAY_OFF__SUBSTITUTED_FROM_03_25_2001_,
    11442_i32 => _DAY_OFF__SUBSTITUTED_FROM_04_28_2001_,
    11443_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY,
    11451_i32 => _VICTORY_DAY,
    11564_i32 => _CONSTITUTION_DAY,
    11620_i32 => _REPUBLIC_DAY,
    11672_i32 => _INDEPENDENCE_DAY,
    11687_i32 => _DAY_OFF__SUBSTITUTED_FROM_12_29_2001_,
    11688_i32 => _NEW_YEAR_S_DAY,
    11689_i32 => _NEW_YEAR_S_DAY,
    11754_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    11768_i32 => _NOWRUZ_HOLIDAY,
    11808_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY,
    11816_i32 => _VICTORY_DAY,
    11817_i32 => _DAY_OFF__SUBSTITUTED_FROM_05_12_2002_,
    11929_i32 => _CONSTITUTION_DAY,
    11985_i32 => _REPUBLIC_DAY,
    12037_i32 => _INDEPENDENCE_DAY,
    12038_i32 => _INDEPENDENCE_DAY,
    12053_i32 => _NEW_YEAR_S_DAY,
    12054_i32 => _NEW_YEAR_S_DAY,
    12119_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    12121_i32 => _INTERNATIONAL_WOMEN_S_DAY__OBSERVED_,
    12133_i32 => _NOWRUZ_HOLIDAY,
    12135_i32 => _NOWRUZ_HOLIDAY__OBSERVED_,
    12173_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY,
    12174_i32 => _DAY_OFF__SUBSTITUTED_FROM_05_04_2003_,
    12181_i32 => _VICTORY_DAY,
    12294_i32 => _CONSTITUTION_DAY,
    12296_i32 => _CONSTITUTION_DAY__OBSERVED_,
    12350_i32 => _REPUBLIC_DAY,
    12352_i32 => _REPUBLIC_DAY__OBSERVED_,
    12401_i32 => _DAY_OFF__SUBSTITUTED_FROM_12_13_2003_,
    12402_i32 => _INDEPENDENCE_DAY,
    12403_i32 => _INDEPENDENCE_DAY,
    12418_i32 => _NEW_YEAR_S_DAY,
    12419_i32 => _NEW_YEAR_S_DAY,
    12485_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    12499_i32 => _NOWRUZ_HOLIDAY,
    12539_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY,
    12541_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY__OBSERVED_,
    12547_i32 => _VICTORY_DAY,
    12548_i32 => _VICTORY_DAY__OBSERVED_,
    12660_i32 => _CONSTITUTION_DAY,
    12716_i32 => _REPUBLIC_DAY,
    12768_i32 => _INDEPENDENCE_DAY,
    12769_i32 => _INDEPENDENCE_DAY,
    12784_i32 => _NEW_YEAR_S_DAY,
    12785_i32 => _NEW_YEAR_S_DAY,
    12786_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    12787_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    12849_i32 => _DAY_OFF__SUBSTITUTED_FROM_03_05_2005_,
    12850_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    12863_i32 => _DAY_OFF__SUBSTITUTED_FROM_03_19_2005_,
    12864_i32 => _NOWRUZ_HOLIDAY,
    12904_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY,
    12905_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY__OBSERVED_,
    12912_i32 => _VICTORY_DAY,
    13024_i32 => _DAY_OFF__SUBSTITUTED_FROM_08_27_2005_,
    13025_i32 => _CONSTITUTION_DAY,
    13080_i32 => _DAY_OFF__SUBSTITUTED_FROM_10_22_2005_,
    13081_i32 => _REPUBLIC_DAY,
    13133_i32 => _INDEPENDENCE_DAY,
    13134_i32 => _INDEPENDENCE_DAY,
    13136_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    13149_i32 => _NEW_YEAR_S_DAY,
    13150_i32 => _NEW_YEAR_S_DAY,
    13151_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    13155_i32 => _ORTHODOX_CHRISTMAS,
    13158_i32 => _EID_AL_ADHA,
    13159_i32 => _DAY_OFF__SUBSTITUTED_FROM_01_14_2006_,
    13215_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    13229_i32 => _NOWRUZ_HOLIDAY,
    13269_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY,
    13276_i32 => _DAY_OFF__SUBSTITUTED_FROM_05_06_2006_,
    13277_i32 => _VICTORY_DAY,
    13390_i32 => _CONSTITUTION_DAY,
    13446_i32 => _REPUBLIC_DAY,
    13498_i32 => _INDEPENDENCE_DAY,
    13499_i32 => _INDEPENDENCE_DAY,
    13500_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    13501_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    13514_i32 => _NEW_YEAR_S_DAY,
    13515_i32 => _NEW_YEAR_S_DAY,
    13520_i32 => _ORTHODOX_CHRISTMAS,
    13580_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    13581_i32 => _DAY_OFF__SUBSTITUTED_FROM_03_11_2007_,
    13594_i32 => _NOWRUZ_HOLIDAY,
    13595_i32 => _DAY_OFF__SUBSTITUTED_FROM_03_25_2007_,
    13634_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY,
    13642_i32 => _VICTORY_DAY,
    13755_i32 => _CONSTITUTION_DAY,
    13756_i32 => _DAY_OFF__SUBSTITUTED_FROM_09_02_2007_,
    13811_i32 => _REPUBLIC_DAY,
    13812_i32 => _DAY_OFF__SUBSTITUTED_FROM_10_28_2007_,
    13863_i32 => _INDEPENDENCE_DAY,
    13864_i32 => _INDEPENDENCE_DAY,
    13865_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    13867_i32 => _EID_AL_ADHA,
    13878_i32 => _DAY_OFF__SUBSTITUTED_FROM_12_29_2007_,
    13879_i32 => _NEW_YEAR_S_DAY,
    13880_i32 => _NEW_YEAR_S_DAY,
    13885_i32 => _ORTHODOX_CHRISTMAS,
    13946_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    13948_i32 => _INTERNATIONAL_WOMEN_S_DAY__OBSERVED_,
    13960_i32 => _NOWRUZ_HOLIDAY,
    13962_i32 => _NOWRUZ_HOLIDAY__OBSERVED_,
    14000_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY,
    14001_i32 => _DAY_OFF__SUBSTITUTED_FROM_05_04_2008_,
    14008_i32 => _VICTORY_DAY,
    14121_i32 => _CONSTITUTION_DAY,
    14123_i32 => _CONSTITUTION_DAY__OBSERVED_,
    14177_i32 => _REPUBLIC_DAY,
    14179_i32 => _REPUBLIC_DAY__OBSERVED_,
    14221_i32 => _EID_AL_ADHA,
    14229_i32 => _INDEPENDENCE_DAY,
    14230_i32 => _INDEPENDENCE_DAY,
    14245_i32 => _NEW_YEAR_S_DAY,
    14246_i32 => _NEW_YEAR_S_DAY,
    14251_i32 => _ORTHODOX_CHRISTMAS,
    14311_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    14312_i32 => _INTERNATIONAL_WOMEN_S_DAY__OBSERVED_,
    14325_i32 => _NOWRUZ_HOLIDAY,
    14326_i32 => _NOWRUZ_HOLIDAY__OBSERVED_,
    14365_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY,
    14373_i32 => _VICTORY_DAY,
    14375_i32 => _VICTORY_DAY__OBSERVED_,
    14431_i32 => _CAPITAL_DAY,
    14486_i32 => _CONSTITUTION_DAY,
    14487_i32 => _CONSTITUTION_DAY__OBSERVED_,
    14575_i32 => _EID_AL_ADHA,
    14594_i32 => _INDEPENDENCE_DAY,
    14595_i32 => _INDEPENDENCE_DAY,
    14596_i32 => _DAY_OFF__SUBSTITUTED_FROM_12_20_2009_,
    14610_i32 => _NEW_YEAR_S_DAY,
    14611_i32 => _NEW_YEAR_S_DAY,
    14613_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    14616_i32 => _ORTHODOX_CHRISTMAS,
    14617_i32 => _DAY_OFF__SUBSTITUTED_FROM_01_10_2010_,
    14676_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    14689_i32 => _NOWRUZ_HOLIDAY,
    14690_i32 => _NOWRUZ_HOLIDAY,
    14691_i32 => _NOWRUZ_HOLIDAY,
    14692_i32 => _NOWRUZ_HOLIDAY__OBSERVED_,
    14730_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY,
    14732_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY__OBSERVED_,
    14738_i32 => _VICTORY_DAY,
    14739_i32 => _VICTORY_DAY__OBSERVED_,
    14795_i32 => _DAY_OFF__SUBSTITUTED_FROM_07_03_2010_,
    14796_i32 => _CAPITAL_DAY,
    14851_i32 => _CONSTITUTION_DAY,
    14929_i32 => _EID_AL_ADHA,
    14959_i32 => _INDEPENDENCE_DAY,
    14960_i32 => _INDEPENDENCE_DAY,
    14975_i32 => _NEW_YEAR_S_DAY,
    14976_i32 => _NEW_YEAR_S_DAY,
    14977_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    14978_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    14981_i32 => _ORTHODOX_CHRISTMAS,
    15040_i32 => _DAY_OFF__SUBSTITUTED_FROM_03_05_2011_,
    15041_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    15054_i32 => _NOWRUZ_HOLIDAY,
    15055_i32 => _NOWRUZ_HOLIDAY,
    15056_i32 => _NOWRUZ_HOLIDAY,
    15095_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY,
    15096_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY__OBSERVED_,
    15103_i32 => _VICTORY_DAY,
    15161_i32 => _CAPITAL_DAY,
    15215_i32 => _DAY_OFF__SUBSTITUTED_FROM_08_27_2011_,
    15216_i32 => _CONSTITUTION_DAY,
    15284_i32 => _EID_AL_ADHA,
    15324_i32 => _INDEPENDENCE_DAY,
    15325_i32 => _INDEPENDENCE_DAY,
    15327_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    15340_i32 => _NEW_YEAR_S_DAY,
    15341_i32 => _NEW_YEAR_S_DAY,
    15342_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    15346_i32 => _ORTHODOX_CHRISTMAS,
    15407_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    15408_i32 => _DAY_OFF__SUBSTITUTED_FROM_03_11_2012_,
    15420_i32 => _NOWRUZ_HOLIDAY,
    15421_i32 => _NOWRUZ_HOLIDAY,
    15422_i32 => _NOWRUZ_HOLIDAY,
    15460_i32 => _DAY_OFF__SUBSTITUTED_FROM_04_28_2012_,
    15461_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY,
    15469_i32 => _VICTORY_DAY,
    15527_i32 => _CAPITAL_DAY,
    15582_i32 => _CONSTITUTION_DAY,
    15639_i32 => _EID_AL_ADHA,
    15675_i32 => _FIRST_PRESIDENT_DAY,
    15677_i32 => _FIRST_PRESIDENT_DAY__OBSERVED_,
    15690_i32 => _INDEPENDENCE_DAY,
    15691_i32 => _INDEPENDENCE_DAY,
    15692_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    15705_i32 => _DAY_OFF__SUBSTITUTED_FROM_12_29_2012_,
    15706_i32 => _NEW_YEAR_S_DAY,
    15707_i32 => _NEW_YEAR_S_DAY,
    15712_i32 => _ORTHODOX_CHRISTMAS,
    15772_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    15785_i32 => _NOWRUZ_HOLIDAY,
    15786_i32 => _NOWRUZ_HOLIDAY,
    15787_i32 => _NOWRUZ_HOLIDAY,
    15789_i32 => _NOWRUZ_HOLIDAY__OBSERVED_,
    15826_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY,
    15832_i32 => _DEFENDER_OF_THE_FATHERLAND_DAY,
    15834_i32 => _VICTORY_DAY,
    15835_i32 => _DAY_OFF__SUBSTITUTED_FROM_05_04_2013_,
    15892_i32 => _CAPITAL_DAY,
    15894_i32 => _CAPITAL_DAY__OBSERVED_,
    15947_i32 => _CONSTITUTION_DAY,
    15992_i32 => _DAY_OFF__SUBSTITUTED_FROM_10_12_2013_,
    15993_i32 => _EID_AL_ADHA,
    16040_i32 => _FIRST_PRESIDENT_DAY,
    16041_i32 => _FIRST_PRESIDENT_DAY__OBSERVED_,
    16055_i32 => _INDEPENDENCE_DAY,
    16056_i32 => _INDEPENDENCE_DAY,
    16071_i32 => _NEW_YEAR_S_DAY,
    16072_i32 => _NEW_YEAR_S_DAY,
    16073_i32 => _DAY_OFF__SUBSTITUTED_FROM_12_28_2013_,
    16077_i32 => _ORTHODOX_CHRISTMAS,
    16137_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    16139_i32 => _INTERNATIONAL_WOMEN_S_DAY__OBSERVED_,
    16150_i32 => _NOWRUZ_HOLIDAY,
    16151_i32 => _NOWRUZ_HOLIDAY,
    16152_i32 => _NOWRUZ_HOLIDAY,
    16153_i32 => _NOWRUZ_HOLIDAY__OBSERVED_,
    16154_i32 => _NOWRUZ_HOLIDAY__OBSERVED_,
    16191_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY,
    16192_i32 => _DAY_OFF__SUBSTITUTED_FROM_05_04_2014_,
    16197_i32 => _DEFENDER_OF_THE_FATHERLAND_DAY,
    16198_i32 => _DAY_OFF__SUBSTITUTED_FROM_05_11_2014_,
    16199_i32 => _VICTORY_DAY,
    16257_i32 => _CAPITAL_DAY,
    16258_i32 => _CAPITAL_DAY__OBSERVED_,
    16312_i32 => _CONSTITUTION_DAY,
    16314_i32 => _CONSTITUTION_DAY__OBSERVED_,
    16347_i32 => _EID_AL_ADHA,
    16405_i32 => _FIRST_PRESIDENT_DAY,
    16420_i32 => _INDEPENDENCE_DAY,
    16421_i32 => _INDEPENDENCE_DAY,
    16436_i32 => _NEW_YEAR_S_DAY,
    16437_i32 => _NEW_YEAR_S_DAY,
    16442_i32 => _ORTHODOX_CHRISTMAS,
    16502_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    16503_i32 => _INTERNATIONAL_WOMEN_S_DAY__OBSERVED_,
    16515_i32 => _NOWRUZ_HOLIDAY,
    16516_i32 => _NOWRUZ_HOLIDAY,
    16517_i32 => _NOWRUZ_HOLIDAY,
    16518_i32 => _NOWRUZ_HOLIDAY__OBSERVED_,
    16519_i32 => _NOWRUZ_HOLIDAY__OBSERVED_,
    16556_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY,
    16562_i32 => _DEFENDER_OF_THE_FATHERLAND_DAY,
    16564_i32 => _VICTORY_DAY,
    16566_i32 => _VICTORY_DAY__OBSERVED_,
    16622_i32 => _CAPITAL_DAY,
    16677_i32 => _CONSTITUTION_DAY,
    16678_i32 => _CONSTITUTION_DAY__OBSERVED_,
    16702_i32 => _EID_AL_ADHA,
    16770_i32 => _FIRST_PRESIDENT_DAY,
    16785_i32 => _INDEPENDENCE_DAY,
    16786_i32 => _INDEPENDENCE_DAY,
    16801_i32 => _NEW_YEAR_S_DAY,
    16802_i32 => _NEW_YEAR_S_DAY,
    16804_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    16807_i32 => _ORTHODOX_CHRISTMAS,
    16867_i32 => _DAY_OFF__SUBSTITUTED_FROM_03_05_2016_,
    16868_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    16881_i32 => _NOWRUZ_HOLIDAY,
    16882_i32 => _NOWRUZ_HOLIDAY,
    16883_i32 => _NOWRUZ_HOLIDAY,
    16922_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY,
    16923_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY__OBSERVED_,
    16928_i32 => _DEFENDER_OF_THE_FATHERLAND_DAY,
    16930_i32 => _VICTORY_DAY,
    16931_i32 => _DEFENDER_OF_THE_FATHERLAND_DAY__OBSERVED_,
    16988_i32 => _CAPITAL_DAY,
    17043_i32 => _CONSTITUTION_DAY,
    17056_i32 => _EID_AL_ADHA,
    17136_i32 => _FIRST_PRESIDENT_DAY,
    17151_i32 => _INDEPENDENCE_DAY,
    17152_i32 => _INDEPENDENCE_DAY,
    17154_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    17167_i32 => _NEW_YEAR_S_DAY,
    17168_i32 => _NEW_YEAR_S_DAY,
    17169_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    17173_i32 => _ORTHODOX_CHRISTMAS,
    17233_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    17245_i32 => _DAY_OFF__SUBSTITUTED_FROM_03_18_2017_,
    17246_i32 => _NOWRUZ_HOLIDAY,
    17247_i32 => _NOWRUZ_HOLIDAY,
    17248_i32 => _NOWRUZ_HOLIDAY,
    17287_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY,
    17293_i32 => _DEFENDER_OF_THE_FATHERLAND_DAY,
    17294_i32 => _DEFENDER_OF_THE_FATHERLAND_DAY__OBSERVED_,
    17295_i32 => _VICTORY_DAY,
    17353_i32 => _CAPITAL_DAY,
    17354_i32 => _DAY_OFF__SUBSTITUTED_FROM_07_01_2017_,
    17408_i32 => _CONSTITUTION_DAY,
    17410_i32 => _EID_AL_ADHA,
    17501_i32 => _FIRST_PRESIDENT_DAY,
    17516_i32 => _INDEPENDENCE_DAY,
    17517_i32 => _INDEPENDENCE_DAY,
    17518_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    17519_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    17532_i32 => _NEW_YEAR_S_DAY,
    17533_i32 => _NEW_YEAR_S_DAY,
    17538_i32 => _ORTHODOX_CHRISTMAS,
    17598_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    17599_i32 => _DAY_OFF__SUBSTITUTED_FROM_03_03_2018_,
    17611_i32 => _NOWRUZ_HOLIDAY,
    17612_i32 => _NOWRUZ_HOLIDAY,
    17613_i32 => _NOWRUZ_HOLIDAY,
    17651_i32 => _DAY_OFF__SUBSTITUTED_FROM_04_28_2018_,
    17652_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY,
    17658_i32 => _DEFENDER_OF_THE_FATHERLAND_DAY,
    17659_i32 => _DAY_OFF__SUBSTITUTED_FROM_05_05_2018_,
    17660_i32 => _VICTORY_DAY,
    17718_i32 => _CAPITAL_DAY,
    17764_i32 => _EID_AL_ADHA,
    17773_i32 => _CONSTITUTION_DAY,
    17774_i32 => _DAY_OFF__SUBSTITUTED_FROM_08_25_2018_,
    17866_i32 => _FIRST_PRESIDENT_DAY,
    17868_i32 => _FIRST_PRESIDENT_DAY__OBSERVED_,
    17881_i32 => _INDEPENDENCE_DAY,
    17882_i32 => _INDEPENDENCE_DAY,
    17883_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    17896_i32 => _DAY_OFF__SUBSTITUTED_FROM_12_29_2018_,
    17897_i32 => _NEW_YEAR_S_DAY,
    17898_i32 => _NEW_YEAR_S_DAY,
    17903_i32 => _ORTHODOX_CHRISTMAS,
    17963_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    17976_i32 => _NOWRUZ_HOLIDAY,
    17977_i32 => _NOWRUZ_HOLIDAY,
    17978_i32 => _NOWRUZ_HOLIDAY,
    17980_i32 => _NOWRUZ_HOLIDAY__OBSERVED_,
    18017_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY,
    18023_i32 => _DEFENDER_OF_THE_FATHERLAND_DAY,
    18025_i32 => _VICTORY_DAY,
    18026_i32 => _DAY_OFF__SUBSTITUTED_FROM_05_04_2019_,
    18083_i32 => _CAPITAL_DAY,
    18085_i32 => _CAPITAL_DAY__OBSERVED_,
    18119_i32 => _EID_AL_ADHA,
    18138_i32 => _CONSTITUTION_DAY,
    18231_i32 => _FIRST_PRESIDENT_DAY,
    18232_i32 => _FIRST_PRESIDENT_DAY__OBSERVED_,
    18246_i32 => _INDEPENDENCE_DAY,
    18247_i32 => _INDEPENDENCE_DAY,
    18262_i32 => _NEW_YEAR_S_DAY,
    18263_i32 => _NEW_YEAR_S_DAY,
    18264_i32 => _DAY_OFF__SUBSTITUTED_FROM_01_05_2020_,
    18268_i32 => _ORTHODOX_CHRISTMAS,
    18329_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    18330_i32 => _INTERNATIONAL_WOMEN_S_DAY__OBSERVED_,
    18342_i32 => _NOWRUZ_HOLIDAY,
    18343_i32 => _NOWRUZ_HOLIDAY,
    18344_i32 => _NOWRUZ_HOLIDAY,
    18345_i32 => _NOWRUZ_HOLIDAY__OBSERVED_,
    18346_i32 => _NOWRUZ_HOLIDAY__OBSERVED_,
    18383_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY,
    18389_i32 => _DEFENDER_OF_THE_FATHERLAND_DAY,
    18390_i32 => _VICTORY_DAY__OBSERVED_,
    18391_i32 => _VICTORY_DAY,
    18449_i32 => _CAPITAL_DAY,
    18474_i32 => _EID_AL_ADHA,
    18504_i32 => _CONSTITUTION_DAY,
    18505_i32 => _CONSTITUTION_DAY__OBSERVED_,
    18597_i32 => _FIRST_PRESIDENT_DAY,
    18612_i32 => _INDEPENDENCE_DAY,
    18613_i32 => _INDEPENDENCE_DAY,
    18614_i32 => _DAY_OFF__SUBSTITUTED_FROM_12_20_2020_,
    18628_i32 => _NEW_YEAR_S_DAY,
    18629_i32 => _NEW_YEAR_S_DAY,
    18631_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    18634_i32 => _ORTHODOX_CHRISTMAS,
    18694_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    18707_i32 => _NOWRUZ_HOLIDAY,
    18708_i32 => _NOWRUZ_HOLIDAY,
    18709_i32 => _NOWRUZ_HOLIDAY,
    18710_i32 => _NOWRUZ_HOLIDAY__OBSERVED_,
    18748_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY,
    18750_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY__OBSERVED_,
    18754_i32 => _DEFENDER_OF_THE_FATHERLAND_DAY,
    18756_i32 => _VICTORY_DAY,
    18757_i32 => _VICTORY_DAY__OBSERVED_,
    18813_i32 => _DAY_OFF__SUBSTITUTED_FROM_07_03_2021_,
    18814_i32 => _CAPITAL_DAY,
    18828_i32 => _EID_AL_ADHA,
    18869_i32 => _CONSTITUTION_DAY,
    18962_i32 => _FIRST_PRESIDENT_DAY,
    18977_i32 => _INDEPENDENCE_DAY,
    18978_i32 => _INDEPENDENCE_DAY,
    18993_i32 => _NEW_YEAR_S_DAY,
    18994_i32 => _NEW_YEAR_S_DAY,
    18995_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    18996_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    18999_i32 => _ORTHODOX_CHRISTMAS,
    19058_i32 => _DAY_OFF__SUBSTITUTED_FROM_03_05_2022_,
    19059_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    19072_i32 => _NOWRUZ_HOLIDAY,
    19073_i32 => _NOWRUZ_HOLIDAY,
    19074_i32 => _NOWRUZ_HOLIDAY,
    19113_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY,
    19114_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY__OBSERVED_,
    19119_i32 => _DEFENDER_OF_THE_FATHERLAND_DAY,
    19121_i32 => _VICTORY_DAY,
    19122_i32 => _DEFENDER_OF_THE_FATHERLAND_DAY__OBSERVED_,
    19179_i32 => _CAPITAL_DAY,
    19182_i32 => _EID_AL_ADHA,
    19233_i32 => _DAY_OFF__SUBSTITUTED_FROM_08_27_2022_,
    19234_i32 => _CONSTITUTION_DAY,
    19289_i32 => _DAY_OFF__SUBSTITUTED_FROM_10_22_2022_,
    19290_i32 => _REPUBLIC_DAY,
    19342_i32 => _INDEPENDENCE_DAY,
    19358_i32 => _NEW_YEAR_S_DAY,
    19359_i32 => _NEW_YEAR_S_DAY,
    19360_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    19364_i32 => _ORTHODOX_CHRISTMAS,
    19424_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    19437_i32 => _NOWRUZ_HOLIDAY,
    19438_i32 => _NOWRUZ_HOLIDAY,
    19439_i32 => _NOWRUZ_HOLIDAY,
    19478_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY,
    19484_i32 => _DEFENDER_OF_THE_FATHERLAND_DAY,
    19485_i32 => _DEFENDER_OF_THE_FATHERLAND_DAY__OBSERVED_,
    19486_i32 => _VICTORY_DAY,
    19536_i32 => _EID_AL_ADHA,
    19544_i32 => _CAPITAL_DAY,
    19545_i32 => _DAY_OFF__SUBSTITUTED_FROM_07_01_2023_,
    19599_i32 => _CONSTITUTION_DAY,
    19655_i32 => _REPUBLIC_DAY,
    19707_i32 => _INDEPENDENCE_DAY,
    19709_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    19723_i32 => _NEW_YEAR_S_DAY,
    19724_i32 => _NEW_YEAR_S_DAY,
    19729_i32 => _ORTHODOX_CHRISTMAS,
    19790_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    19803_i32 => _NOWRUZ_HOLIDAY,
    19804_i32 => _NOWRUZ_HOLIDAY,
    19805_i32 => _NOWRUZ_HOLIDAY,
    19807_i32 => _NOWRUZ_HOLIDAY__OBSERVED_,
    19844_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY,
    19850_i32 => _DEFENDER_OF_THE_FATHERLAND_DAY,
    19851_i32 => _DAY_OFF__SUBSTITUTED_FROM_05_04_2024_,
    19852_i32 => _VICTORY_DAY,
    19890_i32 => _EID_AL_ADHA,
    19910_i32 => _CAPITAL_DAY,
    19912_i32 => _CAPITAL_DAY__OBSERVED_,
    19965_i32 => _CONSTITUTION_DAY,
    20021_i32 => _REPUBLIC_DAY,
    20073_i32 => _INDEPENDENCE_DAY,
    20089_i32 => _NEW_YEAR_S_DAY,
    20090_i32 => _NEW_YEAR_S_DAY,
    20091_i32 => _DAY_OFF__SUBSTITUTED_FROM_01_05_2025_,
    20095_i32 => _ORTHODOX_CHRISTMAS,
    20155_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    20157_i32 => _INTERNATIONAL_WOMEN_S_DAY__OBSERVED_,
    20168_i32 => _NOWRUZ_HOLIDAY,
    20169_i32 => _NOWRUZ_HOLIDAY,
    20170_i32 => _NOWRUZ_HOLIDAY,
    20171_i32 => _NOWRUZ_HOLIDAY__OBSERVED_,
    20172_i32 => _NOWRUZ_HOLIDAY__OBSERVED_,
    20209_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY,
    20215_i32 => _DEFENDER_OF_THE_FATHERLAND_DAY,
    20217_i32 => _VICTORY_DAY,
    20245_i32 => _EID_AL_ADHA,
    20275_i32 => _CAPITAL_DAY,
    20276_i32 => _CAPITAL_DAY__OBSERVED_,
    20330_i32 => _CONSTITUTION_DAY,
    20332_i32 => _CONSTITUTION_DAY__OBSERVED_,
    20386_i32 => _REPUBLIC_DAY,
    20388_i32 => _REPUBLIC_DAY__OBSERVED_,
    20438_i32 => _INDEPENDENCE_DAY,
    20454_i32 => _NEW_YEAR_S_DAY,
    20455_i32 => _NEW_YEAR_S_DAY,
    20460_i32 => _ORTHODOX_CHRISTMAS,
    20520_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    20521_i32 => _INTERNATIONAL_WOMEN_S_DAY__OBSERVED_,
    20533_i32 => _NOWRUZ_HOLIDAY,
    20534_i32 => _NOWRUZ_HOLIDAY,
    20535_i32 => _NOWRUZ_HOLIDAY,
    20536_i32 => _NOWRUZ_HOLIDAY__OBSERVED_,
    20537_i32 => _NOWRUZ_HOLIDAY__OBSERVED_,
    20574_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY,
    20580_i32 => _DEFENDER_OF_THE_FATHERLAND_DAY,
    20582_i32 => _VICTORY_DAY,
    20584_i32 => _VICTORY_DAY__OBSERVED_,
    20600_i32 => _EID_AL_ADHA__ESTIMATED_,
    20640_i32 => _CAPITAL_DAY,
    20695_i32 => _CONSTITUTION_DAY,
    20696_i32 => _CONSTITUTION_DAY__OBSERVED_,
    20751_i32 => _REPUBLIC_DAY,
    20752_i32 => _REPUBLIC_DAY__OBSERVED_,
    20803_i32 => _INDEPENDENCE_DAY,
    20819_i32 => _NEW_YEAR_S_DAY,
    20820_i32 => _NEW_YEAR_S_DAY,
    20822_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    20825_i32 => _ORTHODOX_CHRISTMAS,
    20885_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    20898_i32 => _NOWRUZ_HOLIDAY,
    20899_i32 => _NOWRUZ_HOLIDAY,
    20900_i32 => _NOWRUZ_HOLIDAY,
    20901_i32 => _NOWRUZ_HOLIDAY__OBSERVED_,
    20939_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY,
    20941_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY__OBSERVED_,
    20945_i32 => _DEFENDER_OF_THE_FATHERLAND_DAY,
    20947_i32 => _VICTORY_DAY,
    20948_i32 => _VICTORY_DAY__OBSERVED_,
    20954_i32 => _EID_AL_ADHA__ESTIMATED_,
    21005_i32 => _CAPITAL_DAY,
    21060_i32 => _CONSTITUTION_DAY,
    21116_i32 => _REPUBLIC_DAY,
    21168_i32 => _INDEPENDENCE_DAY,
    21184_i32 => _NEW_YEAR_S_DAY,
    21185_i32 => _NEW_YEAR_S_DAY,
    21186_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    21187_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    21190_i32 => _ORTHODOX_CHRISTMAS,
    21251_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    21264_i32 => _NOWRUZ_HOLIDAY,
    21265_i32 => _NOWRUZ_HOLIDAY,
    21266_i32 => _NOWRUZ_HOLIDAY,
    21305_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY,
    21309_i32 => _EID_AL_ADHA__ESTIMATED_,
    21311_i32 => _DEFENDER_OF_THE_FATHERLAND_DAY,
    21312_i32 => _DEFENDER_OF_THE_FATHERLAND_DAY__OBSERVED_,
    21313_i32 => _VICTORY_DAY,
    21371_i32 => _CAPITAL_DAY,
    21426_i32 => _CONSTITUTION_DAY,
    21482_i32 => _REPUBLIC_DAY,
    21534_i32 => _INDEPENDENCE_DAY,
    21536_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    21550_i32 => _NEW_YEAR_S_DAY,
    21551_i32 => _NEW_YEAR_S_DAY,
    21556_i32 => _ORTHODOX_CHRISTMAS,
    21616_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    21629_i32 => _NOWRUZ_HOLIDAY,
    21630_i32 => _NOWRUZ_HOLIDAY,
    21631_i32 => _NOWRUZ_HOLIDAY,
    21663_i32 => _EID_AL_ADHA__ESTIMATED_,
    21670_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY,
    21676_i32 => _DEFENDER_OF_THE_FATHERLAND_DAY,
    21678_i32 => _VICTORY_DAY,
    21736_i32 => _CAPITAL_DAY,
    21791_i32 => _CONSTITUTION_DAY,
    21847_i32 => _REPUBLIC_DAY,
    21899_i32 => _INDEPENDENCE_DAY,
    21900_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    21915_i32 => _NEW_YEAR_S_DAY,
    21916_i32 => _NEW_YEAR_S_DAY,
    21921_i32 => _ORTHODOX_CHRISTMAS,
    21981_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    21994_i32 => _NOWRUZ_HOLIDAY,
    21995_i32 => _NOWRUZ_HOLIDAY,
    21996_i32 => _NOWRUZ_HOLIDAY,
    21998_i32 => _NOWRUZ_HOLIDAY__OBSERVED_,
    22017_i32 => _EID_AL_ADHA__ESTIMATED_,
    22035_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY,
    22041_i32 => _DEFENDER_OF_THE_FATHERLAND_DAY,
    22043_i32 => _VICTORY_DAY,
    22101_i32 => _CAPITAL_DAY,
    22103_i32 => _CAPITAL_DAY__OBSERVED_,
    22156_i32 => _CONSTITUTION_DAY,
    22212_i32 => _REPUBLIC_DAY,
    22264_i32 => _INDEPENDENCE_DAY,
    22280_i32 => _NEW_YEAR_S_DAY,
    22281_i32 => _NEW_YEAR_S_DAY,
    22286_i32 => _ORTHODOX_CHRISTMAS,
    22346_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    22348_i32 => _INTERNATIONAL_WOMEN_S_DAY__OBSERVED_,
    22359_i32 => _NOWRUZ_HOLIDAY,
    22360_i32 => _NOWRUZ_HOLIDAY,
    22361_i32 => _NOWRUZ_HOLIDAY,
    22362_i32 => _NOWRUZ_HOLIDAY__OBSERVED_,
    22363_i32 => _NOWRUZ_HOLIDAY__OBSERVED_,
    22371_i32 => _EID_AL_ADHA__ESTIMATED_,
    22400_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY,
    22406_i32 => _DEFENDER_OF_THE_FATHERLAND_DAY,
    22408_i32 => _VICTORY_DAY,
    22466_i32 => _CAPITAL_DAY,
    22467_i32 => _CAPITAL_DAY__OBSERVED_,
    22521_i32 => _CONSTITUTION_DAY,
    22523_i32 => _CONSTITUTION_DAY__OBSERVED_,
    22577_i32 => _REPUBLIC_DAY,
    22579_i32 => _REPUBLIC_DAY__OBSERVED_,
    22629_i32 => _INDEPENDENCE_DAY,
    22645_i32 => _NEW_YEAR_S_DAY,
    22646_i32 => _NEW_YEAR_S_DAY,
    22651_i32 => _ORTHODOX_CHRISTMAS,
    22712_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    22725_i32 => _NOWRUZ_HOLIDAY,
    22726_i32 => _EID_AL_ADHA__ESTIMATED___NOWRUZ_HOLIDAY,
    22727_i32 => _NOWRUZ_HOLIDAY,
    22728_i32 => _NOWRUZ_HOLIDAY__OBSERVED_,
    22766_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY,
    22768_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY__OBSERVED_,
    22772_i32 => _DEFENDER_OF_THE_FATHERLAND_DAY,
    22774_i32 => _VICTORY_DAY,
    22775_i32 => _VICTORY_DAY__OBSERVED_,
    22832_i32 => _CAPITAL_DAY,
    22887_i32 => _CONSTITUTION_DAY,
    22943_i32 => _REPUBLIC_DAY,
    22995_i32 => _INDEPENDENCE_DAY,
    23011_i32 => _NEW_YEAR_S_DAY,
    23012_i32 => _NEW_YEAR_S_DAY,
    23013_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    23014_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    23017_i32 => _ORTHODOX_CHRISTMAS,
    23077_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    23080_i32 => _EID_AL_ADHA__ESTIMATED_,
    23090_i32 => _NOWRUZ_HOLIDAY,
    23091_i32 => _NOWRUZ_HOLIDAY,
    23092_i32 => _NOWRUZ_HOLIDAY,
    23131_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY,
    23132_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY__OBSERVED_,
    23137_i32 => _DEFENDER_OF_THE_FATHERLAND_DAY,
    23139_i32 => _VICTORY_DAY,
    23140_i32 => _DEFENDER_OF_THE_FATHERLAND_DAY__OBSERVED_,
    23197_i32 => _CAPITAL_DAY,
    23252_i32 => _CONSTITUTION_DAY,
    23308_i32 => _REPUBLIC_DAY,
    23360_i32 => _INDEPENDENCE_DAY,
    23376_i32 => _NEW_YEAR_S_DAY,
    23377_i32 => _NEW_YEAR_S_DAY,
    23378_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    23382_i32 => _ORTHODOX_CHRISTMAS,
    23435_i32 => _EID_AL_ADHA__ESTIMATED_,
    23442_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    23455_i32 => _NOWRUZ_HOLIDAY,
    23456_i32 => _NOWRUZ_HOLIDAY,
    23457_i32 => _NOWRUZ_HOLIDAY,
    23496_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY,
    23502_i32 => _DEFENDER_OF_THE_FATHERLAND_DAY,
    23503_i32 => _DEFENDER_OF_THE_FATHERLAND_DAY__OBSERVED_,
    23504_i32 => _VICTORY_DAY,
    23562_i32 => _CAPITAL_DAY,
    23617_i32 => _CONSTITUTION_DAY,
    23673_i32 => _REPUBLIC_DAY,
    23725_i32 => _INDEPENDENCE_DAY,
    23727_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    23741_i32 => _NEW_YEAR_S_DAY,
    23742_i32 => _NEW_YEAR_S_DAY,
    23747_i32 => _ORTHODOX_CHRISTMAS,
    23789_i32 => _EID_AL_ADHA__ESTIMATED_,
    23807_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    23820_i32 => _NOWRUZ_HOLIDAY,
    23821_i32 => _NOWRUZ_HOLIDAY,
    23822_i32 => _NOWRUZ_HOLIDAY,
    23861_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY,
    23867_i32 => _DEFENDER_OF_THE_FATHERLAND_DAY,
    23869_i32 => _VICTORY_DAY,
    23927_i32 => _CAPITAL_DAY,
    23982_i32 => _CONSTITUTION_DAY,
    24038_i32 => _REPUBLIC_DAY,
    24090_i32 => _INDEPENDENCE_DAY,
    24091_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    24106_i32 => _NEW_YEAR_S_DAY,
    24107_i32 => _NEW_YEAR_S_DAY,
    24112_i32 => _ORTHODOX_CHRISTMAS,
    24143_i32 => _EID_AL_ADHA__ESTIMATED_,
    24173_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    24175_i32 => _INTERNATIONAL_WOMEN_S_DAY__OBSERVED_,
    24186_i32 => _NOWRUZ_HOLIDAY,
    24187_i32 => _NOWRUZ_HOLIDAY,
    24188_i32 => _NOWRUZ_HOLIDAY,
    24189_i32 => _NOWRUZ_HOLIDAY__OBSERVED_,
    24190_i32 => _NOWRUZ_HOLIDAY__OBSERVED_,
    24227_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY,
    24233_i32 => _DEFENDER_OF_THE_FATHERLAND_DAY,
    24235_i32 => _VICTORY_DAY,
    24293_i32 => _CAPITAL_DAY,
    24294_i32 => _CAPITAL_DAY__OBSERVED_,
    24348_i32 => _CONSTITUTION_DAY,
    24350_i32 => _CONSTITUTION_DAY__OBSERVED_,
    24404_i32 => _REPUBLIC_DAY,
    24406_i32 => _REPUBLIC_DAY__OBSERVED_,
    24456_i32 => _INDEPENDENCE_DAY,
    24472_i32 => _NEW_YEAR_S_DAY,
    24473_i32 => _NEW_YEAR_S_DAY,
    24478_i32 => _ORTHODOX_CHRISTMAS,
    24497_i32 => _EID_AL_ADHA__ESTIMATED_,
    24538_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    24539_i32 => _INTERNATIONAL_WOMEN_S_DAY__OBSERVED_,
    24551_i32 => _NOWRUZ_HOLIDAY,
    24552_i32 => _NOWRUZ_HOLIDAY,
    24553_i32 => _NOWRUZ_HOLIDAY,
    24554_i32 => _NOWRUZ_HOLIDAY__OBSERVED_,
    24555_i32 => _NOWRUZ_HOLIDAY__OBSERVED_,
    24592_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY,
    24598_i32 => _DEFENDER_OF_THE_FATHERLAND_DAY,
    24600_i32 => _VICTORY_DAY,
    24602_i32 => _VICTORY_DAY__OBSERVED_,
    24658_i32 => _CAPITAL_DAY,
    24713_i32 => _CONSTITUTION_DAY,
    24714_i32 => _CONSTITUTION_DAY__OBSERVED_,
    24769_i32 => _REPUBLIC_DAY,
    24770_i32 => _REPUBLIC_DAY__OBSERVED_,
    24821_i32 => _INDEPENDENCE_DAY,
    24837_i32 => _NEW_YEAR_S_DAY,
    24838_i32 => _NEW_YEAR_S_DAY,
    24840_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    24843_i32 => _ORTHODOX_CHRISTMAS,
    24852_i32 => _EID_AL_ADHA__ESTIMATED_,
    24903_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    24916_i32 => _NOWRUZ_HOLIDAY,
    24917_i32 => _NOWRUZ_HOLIDAY,
    24918_i32 => _NOWRUZ_HOLIDAY,
    24919_i32 => _NOWRUZ_HOLIDAY__OBSERVED_,
    24957_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY,
    24959_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY__OBSERVED_,
    24963_i32 => _DEFENDER_OF_THE_FATHERLAND_DAY,
    24965_i32 => _VICTORY_DAY,
    24966_i32 => _VICTORY_DAY__OBSERVED_,
    25023_i32 => _CAPITAL_DAY,
    25078_i32 => _CONSTITUTION_DAY,
    25134_i32 => _REPUBLIC_DAY,
    25186_i32 => _INDEPENDENCE_DAY,
    25202_i32 => _NEW_YEAR_S_DAY,
    25203_i32 => _NEW_YEAR_S_DAY,
    25204_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    25205_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    25206_i32 => _EID_AL_ADHA__ESTIMATED_,
    25208_i32 => _ORTHODOX_CHRISTMAS,
    25268_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    25281_i32 => _NOWRUZ_HOLIDAY,
    25282_i32 => _NOWRUZ_HOLIDAY,
    25283_i32 => _NOWRUZ_HOLIDAY,
    25322_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY,
    25323_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY__OBSERVED_,
    25328_i32 => _DEFENDER_OF_THE_FATHERLAND_DAY,
    25330_i32 => _VICTORY_DAY,
    25331_i32 => _DEFENDER_OF_THE_FATHERLAND_DAY__OBSERVED_,
    25388_i32 => _CAPITAL_DAY,
    25443_i32 => _CONSTITUTION_DAY,
    25499_i32 => _REPUBLIC_DAY,
    25551_i32 => _INDEPENDENCE_DAY,
    25561_i32 => _EID_AL_ADHA__ESTIMATED_,
    25567_i32 => _NEW_YEAR_S_DAY,
    25568_i32 => _NEW_YEAR_S_DAY,
    25569_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    25573_i32 => _ORTHODOX_CHRISTMAS,
    25634_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    25647_i32 => _NOWRUZ_HOLIDAY,
    25648_i32 => _NOWRUZ_HOLIDAY,
    25649_i32 => _NOWRUZ_HOLIDAY,
    25688_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY,
    25694_i32 => _DEFENDER_OF_THE_FATHERLAND_DAY,
    25696_i32 => _VICTORY_DAY,
    25754_i32 => _CAPITAL_DAY,
    25809_i32 => _CONSTITUTION_DAY,
    25865_i32 => _REPUBLIC_DAY,
    25915_i32 => _EID_AL_ADHA__ESTIMATED_,
    25917_i32 => _INDEPENDENCE_DAY,
    25918_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    25933_i32 => _NEW_YEAR_S_DAY,
    25934_i32 => _NEW_YEAR_S_DAY,
    25939_i32 => _ORTHODOX_CHRISTMAS,
    25999_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    26012_i32 => _NOWRUZ_HOLIDAY,
    26013_i32 => _NOWRUZ_HOLIDAY,
    26014_i32 => _NOWRUZ_HOLIDAY,
    26016_i32 => _NOWRUZ_HOLIDAY__OBSERVED_,
    26053_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY,
    26059_i32 => _DEFENDER_OF_THE_FATHERLAND_DAY,
    26061_i32 => _VICTORY_DAY,
    26119_i32 => _CAPITAL_DAY,
    26121_i32 => _CAPITAL_DAY__OBSERVED_,
    26174_i32 => _CONSTITUTION_DAY,
    26230_i32 => _REPUBLIC_DAY,
    26270_i32 => _EID_AL_ADHA__ESTIMATED_,
    26282_i32 => _INDEPENDENCE_DAY,
    26298_i32 => _NEW_YEAR_S_DAY,
    26299_i32 => _NEW_YEAR_S_DAY,
    26304_i32 => _ORTHODOX_CHRISTMAS,
    26364_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    26366_i32 => _INTERNATIONAL_WOMEN_S_DAY__OBSERVED_,
    26377_i32 => _NOWRUZ_HOLIDAY,
    26378_i32 => _NOWRUZ_HOLIDAY,
    26379_i32 => _NOWRUZ_HOLIDAY,
    26380_i32 => _NOWRUZ_HOLIDAY__OBSERVED_,
    26381_i32 => _NOWRUZ_HOLIDAY__OBSERVED_,
    26418_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY,
    26424_i32 => _DEFENDER_OF_THE_FATHERLAND_DAY,
    26426_i32 => _VICTORY_DAY,
    26484_i32 => _CAPITAL_DAY,
    26485_i32 => _CAPITAL_DAY__OBSERVED_,
    26539_i32 => _CONSTITUTION_DAY,
    26541_i32 => _CONSTITUTION_DAY__OBSERVED_,
    26595_i32 => _REPUBLIC_DAY,
    26597_i32 => _REPUBLIC_DAY__OBSERVED_,
    26624_i32 => _EID_AL_ADHA__ESTIMATED_,
    26647_i32 => _INDEPENDENCE_DAY,
    26663_i32 => _NEW_YEAR_S_DAY,
    26664_i32 => _NEW_YEAR_S_DAY,
    26669_i32 => _ORTHODOX_CHRISTMAS,
    26729_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    26730_i32 => _INTERNATIONAL_WOMEN_S_DAY__OBSERVED_,
    26742_i32 => _NOWRUZ_HOLIDAY,
    26743_i32 => _NOWRUZ_HOLIDAY,
    26744_i32 => _NOWRUZ_HOLIDAY,
    26745_i32 => _NOWRUZ_HOLIDAY__OBSERVED_,
    26746_i32 => _NOWRUZ_HOLIDAY__OBSERVED_,
    26783_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY,
    26789_i32 => _DEFENDER_OF_THE_FATHERLAND_DAY,
    26791_i32 => _VICTORY_DAY,
    26793_i32 => _VICTORY_DAY__OBSERVED_,
    26849_i32 => _CAPITAL_DAY,
    26904_i32 => _CONSTITUTION_DAY,
    26905_i32 => _CONSTITUTION_DAY__OBSERVED_,
    26960_i32 => _REPUBLIC_DAY,
    26961_i32 => _REPUBLIC_DAY__OBSERVED_,
    26978_i32 => _EID_AL_ADHA__ESTIMATED_,
    27012_i32 => _INDEPENDENCE_DAY,
    27028_i32 => _NEW_YEAR_S_DAY,
    27029_i32 => _NEW_YEAR_S_DAY,
    27031_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    27034_i32 => _ORTHODOX_CHRISTMAS,
    27095_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    27108_i32 => _NOWRUZ_HOLIDAY,
    27109_i32 => _NOWRUZ_HOLIDAY,
    27110_i32 => _NOWRUZ_HOLIDAY,
    27149_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY,
    27150_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY__OBSERVED_,
    27155_i32 => _DEFENDER_OF_THE_FATHERLAND_DAY,
    27157_i32 => _VICTORY_DAY,
    27158_i32 => _DEFENDER_OF_THE_FATHERLAND_DAY__OBSERVED_,
    27215_i32 => _CAPITAL_DAY,
    27270_i32 => _CONSTITUTION_DAY,
    27326_i32 => _REPUBLIC_DAY,
    27332_i32 => _EID_AL_ADHA__ESTIMATED_,
    27378_i32 => _INDEPENDENCE_DAY,
    27394_i32 => _NEW_YEAR_S_DAY,
    27395_i32 => _NEW_YEAR_S_DAY,
    27396_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    27400_i32 => _ORTHODOX_CHRISTMAS,
    27460_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    27473_i32 => _NOWRUZ_HOLIDAY,
    27474_i32 => _NOWRUZ_HOLIDAY,
    27475_i32 => _NOWRUZ_HOLIDAY,
    27514_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY,
    27520_i32 => _DEFENDER_OF_THE_FATHERLAND_DAY,
    27521_i32 => _DEFENDER_OF_THE_FATHERLAND_DAY__OBSERVED_,
    27522_i32 => _VICTORY_DAY,
    27580_i32 => _CAPITAL_DAY,
    27635_i32 => _CONSTITUTION_DAY,
    27687_i32 => _EID_AL_ADHA__ESTIMATED_,
    27691_i32 => _REPUBLIC_DAY,
    27743_i32 => _INDEPENDENCE_DAY,
    27745_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    27759_i32 => _NEW_YEAR_S_DAY,
    27760_i32 => _NEW_YEAR_S_DAY,
    27765_i32 => _ORTHODOX_CHRISTMAS,
    27825_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    27838_i32 => _NOWRUZ_HOLIDAY,
    27839_i32 => _NOWRUZ_HOLIDAY,
    27840_i32 => _NOWRUZ_HOLIDAY,
    27879_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY,
    27885_i32 => _DEFENDER_OF_THE_FATHERLAND_DAY,
    27887_i32 => _VICTORY_DAY,
    27945_i32 => _CAPITAL_DAY,
    28000_i32 => _CONSTITUTION_DAY,
    28041_i32 => _EID_AL_ADHA__ESTIMATED_,
    28056_i32 => _REPUBLIC_DAY,
    28108_i32 => _INDEPENDENCE_DAY,
    28109_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    28124_i32 => _NEW_YEAR_S_DAY,
    28125_i32 => _NEW_YEAR_S_DAY,
    28130_i32 => _ORTHODOX_CHRISTMAS,
    28190_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    28203_i32 => _NOWRUZ_HOLIDAY,
    28204_i32 => _NOWRUZ_HOLIDAY,
    28205_i32 => _NOWRUZ_HOLIDAY,
    28207_i32 => _NOWRUZ_HOLIDAY__OBSERVED_,
    28244_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY,
    28250_i32 => _DEFENDER_OF_THE_FATHERLAND_DAY,
    28252_i32 => _VICTORY_DAY,
    28310_i32 => _CAPITAL_DAY,
    28312_i32 => _CAPITAL_DAY__OBSERVED_,
    28365_i32 => _CONSTITUTION_DAY,
    28396_i32 => _EID_AL_ADHA__ESTIMATED_,
    28421_i32 => _REPUBLIC_DAY,
    28473_i32 => _INDEPENDENCE_DAY,
    28489_i32 => _NEW_YEAR_S_DAY,
    28490_i32 => _NEW_YEAR_S_DAY,
    28495_i32 => _ORTHODOX_CHRISTMAS,
    28556_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    28557_i32 => _INTERNATIONAL_WOMEN_S_DAY__OBSERVED_,
    28569_i32 => _NOWRUZ_HOLIDAY,
    28570_i32 => _NOWRUZ_HOLIDAY,
    28571_i32 => _NOWRUZ_HOLIDAY,
    28572_i32 => _NOWRUZ_HOLIDAY__OBSERVED_,
    28573_i32 => _NOWRUZ_HOLIDAY__OBSERVED_,
    28610_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY,
    28616_i32 => _DEFENDER_OF_THE_FATHERLAND_DAY,
    28618_i32 => _VICTORY_DAY,
    28620_i32 => _VICTORY_DAY__OBSERVED_,
    28676_i32 => _CAPITAL_DAY,
    28731_i32 => _CONSTITUTION_DAY,
    28732_i32 => _CONSTITUTION_DAY__OBSERVED_,
    28751_i32 => _EID_AL_ADHA__ESTIMATED_,
    28787_i32 => _REPUBLIC_DAY,
    28788_i32 => _REPUBLIC_DAY__OBSERVED_,
    28839_i32 => _INDEPENDENCE_DAY,
    28855_i32 => _NEW_YEAR_S_DAY,
    28856_i32 => _NEW_YEAR_S_DAY,
    28858_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    28861_i32 => _ORTHODOX_CHRISTMAS,
    28921_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    28934_i32 => _NOWRUZ_HOLIDAY,
    28935_i32 => _NOWRUZ_HOLIDAY,
    28936_i32 => _NOWRUZ_HOLIDAY,
    28937_i32 => _NOWRUZ_HOLIDAY__OBSERVED_,
    28975_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY,
    28977_i32 => _KAZAKHSTAN_S_PEOPLE_SOLIDARITY_HOLIDAY__OBSERVED_,
    28981_i32 => _DEFENDER_OF_THE_FATHERLAND_DAY,
    28983_i32 => _VICTORY_DAY,
    28984_i32 => _VICTORY_DAY__OBSERVED_,
    29041_i32 => _CAPITAL_DAY,
    29096_i32 => _CONSTITUTION_DAY,
    29105_i32 => _EID_AL_ADHA__ESTIMATED_,
    29152_i32 => _REPUBLIC_DAY,
    29204_i32 => _INDEPENDENCE_DAY,
    29220_i32 => _NEW_YEAR_S_DAY,
};
