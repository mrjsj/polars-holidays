use crate::countries::constants::*;
use phf::phf_map;

pub static LU_HOLIDAYS: phf::Map<i32, &'static str> = phf_map! {
    10957_i32 => _NEW_YEAR_S_DAY,
    11071_i32 => _EASTER_MONDAY,
    11078_i32 => _LABOR_DAY,
    11109_i32 => _ASCENSION_DAY,
    11120_i32 => _WHIT_MONDAY,
    11131_i32 => _NATIONAL_DAY,
    11184_i32 => _ASSUMPTION_DAY,
    11262_i32 => _ALL_SAINTS__DAY,
    11316_i32 => _CHRISTMAS_DAY,
    11317_i32 => _SAINT_STEPHEN_S_DAY,
    11323_i32 => _NEW_YEAR_S_DAY,
    11428_i32 => _EASTER_MONDAY,
    11443_i32 => _LABOR_DAY,
    11466_i32 => _ASCENSION_DAY,
    11477_i32 => _WHIT_MONDAY,
    11496_i32 => _NATIONAL_DAY,
    11549_i32 => _ASSUMPTION_DAY,
    11627_i32 => _ALL_SAINTS__DAY,
    11681_i32 => _CHRISTMAS_DAY,
    11682_i32 => _SAINT_STEPHEN_S_DAY,
    11688_i32 => _NEW_YEAR_S_DAY,
    11778_i32 => _EASTER_MONDAY,
    11808_i32 => _LABOR_DAY,
    11816_i32 => _ASCENSION_DAY,
    11827_i32 => _WHIT_MONDAY,
    11861_i32 => _NATIONAL_DAY,
    11914_i32 => _ASSUMPTION_DAY,
    11992_i32 => _ALL_SAINTS__DAY,
    12046_i32 => _CHRISTMAS_DAY,
    12047_i32 => _SAINT_STEPHEN_S_DAY,
    12053_i32 => _NEW_YEAR_S_DAY,
    12163_i32 => _EASTER_MONDAY,
    12173_i32 => _LABOR_DAY,
    12201_i32 => _ASCENSION_DAY,
    12212_i32 => _WHIT_MONDAY,
    12226_i32 => _NATIONAL_DAY,
    12279_i32 => _ASSUMPTION_DAY,
    12357_i32 => _ALL_SAINTS__DAY,
    12411_i32 => _CHRISTMAS_DAY,
    12412_i32 => _SAINT_STEPHEN_S_DAY,
    12418_i32 => _NEW_YEAR_S_DAY,
    12520_i32 => _EASTER_MONDAY,
    12539_i32 => _LABOR_DAY,
    12558_i32 => _ASCENSION_DAY,
    12569_i32 => _WHIT_MONDAY,
    12592_i32 => _NATIONAL_DAY,
    12645_i32 => _ASSUMPTION_DAY,
    12723_i32 => _ALL_SAINTS__DAY,
    12777_i32 => _CHRISTMAS_DAY,
    12778_i32 => _SAINT_STEPHEN_S_DAY,
    12784_i32 => _NEW_YEAR_S_DAY,
    12870_i32 => _EASTER_MONDAY,
    12904_i32 => _LABOR_DAY,
    12908_i32 => _ASCENSION_DAY,
    12919_i32 => _WHIT_MONDAY,
    12957_i32 => _NATIONAL_DAY,
    13010_i32 => _ASSUMPTION_DAY,
    13088_i32 => _ALL_SAINTS__DAY,
    13142_i32 => _CHRISTMAS_DAY,
    13143_i32 => _SAINT_STEPHEN_S_DAY,
    13149_i32 => _NEW_YEAR_S_DAY,
    13255_i32 => _EASTER_MONDAY,
    13269_i32 => _LABOR_DAY,
    13293_i32 => _ASCENSION_DAY,
    13304_i32 => _WHIT_MONDAY,
    13322_i32 => _NATIONAL_DAY,
    13375_i32 => _ASSUMPTION_DAY,
    13453_i32 => _ALL_SAINTS__DAY,
    13507_i32 => _CHRISTMAS_DAY,
    13508_i32 => _SAINT_STEPHEN_S_DAY,
    13514_i32 => _NEW_YEAR_S_DAY,
    13612_i32 => _EASTER_MONDAY,
    13634_i32 => _LABOR_DAY,
    13650_i32 => _ASCENSION_DAY,
    13661_i32 => _WHIT_MONDAY,
    13687_i32 => _NATIONAL_DAY,
    13740_i32 => _ASSUMPTION_DAY,
    13818_i32 => _ALL_SAINTS__DAY,
    13872_i32 => _CHRISTMAS_DAY,
    13873_i32 => _SAINT_STEPHEN_S_DAY,
    13879_i32 => _NEW_YEAR_S_DAY,
    13962_i32 => _EASTER_MONDAY,
    14000_i32 => _ASCENSION_DAY__LABOR_DAY,
    14011_i32 => _WHIT_MONDAY,
    14053_i32 => _NATIONAL_DAY,
    14106_i32 => _ASSUMPTION_DAY,
    14184_i32 => _ALL_SAINTS__DAY,
    14238_i32 => _CHRISTMAS_DAY,
    14239_i32 => _SAINT_STEPHEN_S_DAY,
    14245_i32 => _NEW_YEAR_S_DAY,
    14347_i32 => _EASTER_MONDAY,
    14365_i32 => _LABOR_DAY,
    14385_i32 => _ASCENSION_DAY,
    14396_i32 => _WHIT_MONDAY,
    14418_i32 => _NATIONAL_DAY,
    14471_i32 => _ASSUMPTION_DAY,
    14549_i32 => _ALL_SAINTS__DAY,
    14603_i32 => _CHRISTMAS_DAY,
    14604_i32 => _SAINT_STEPHEN_S_DAY,
    14610_i32 => _NEW_YEAR_S_DAY,
    14704_i32 => _EASTER_MONDAY,
    14730_i32 => _LABOR_DAY,
    14742_i32 => _ASCENSION_DAY,
    14753_i32 => _WHIT_MONDAY,
    14783_i32 => _NATIONAL_DAY,
    14836_i32 => _ASSUMPTION_DAY,
    14914_i32 => _ALL_SAINTS__DAY,
    14968_i32 => _CHRISTMAS_DAY,
    14969_i32 => _SAINT_STEPHEN_S_DAY,
    14975_i32 => _NEW_YEAR_S_DAY,
    15089_i32 => _EASTER_MONDAY,
    15095_i32 => _LABOR_DAY,
    15127_i32 => _ASCENSION_DAY,
    15138_i32 => _WHIT_MONDAY,
    15148_i32 => _NATIONAL_DAY,
    15201_i32 => _ASSUMPTION_DAY,
    15279_i32 => _ALL_SAINTS__DAY,
    15333_i32 => _CHRISTMAS_DAY,
    15334_i32 => _SAINT_STEPHEN_S_DAY,
    15340_i32 => _NEW_YEAR_S_DAY,
    15439_i32 => _EASTER_MONDAY,
    15461_i32 => _LABOR_DAY,
    15477_i32 => _ASCENSION_DAY,
    15488_i32 => _WHIT_MONDAY,
    15514_i32 => _NATIONAL_DAY,
    15567_i32 => _ASSUMPTION_DAY,
    15645_i32 => _ALL_SAINTS__DAY,
    15699_i32 => _CHRISTMAS_DAY,
    15700_i32 => _SAINT_STEPHEN_S_DAY,
    15706_i32 => _NEW_YEAR_S_DAY,
    15796_i32 => _EASTER_MONDAY,
    15826_i32 => _LABOR_DAY,
    15834_i32 => _ASCENSION_DAY,
    15845_i32 => _WHIT_MONDAY,
    15879_i32 => _NATIONAL_DAY,
    15932_i32 => _ASSUMPTION_DAY,
    16010_i32 => _ALL_SAINTS__DAY,
    16064_i32 => _CHRISTMAS_DAY,
    16065_i32 => _SAINT_STEPHEN_S_DAY,
    16071_i32 => _NEW_YEAR_S_DAY,
    16181_i32 => _EASTER_MONDAY,
    16191_i32 => _LABOR_DAY,
    16219_i32 => _ASCENSION_DAY,
    16230_i32 => _WHIT_MONDAY,
    16244_i32 => _NATIONAL_DAY,
    16297_i32 => _ASSUMPTION_DAY,
    16375_i32 => _ALL_SAINTS__DAY,
    16429_i32 => _CHRISTMAS_DAY,
    16430_i32 => _SAINT_STEPHEN_S_DAY,
    16436_i32 => _NEW_YEAR_S_DAY,
    16531_i32 => _EASTER_MONDAY,
    16556_i32 => _LABOR_DAY,
    16569_i32 => _ASCENSION_DAY,
    16580_i32 => _WHIT_MONDAY,
    16609_i32 => _NATIONAL_DAY,
    16662_i32 => _ASSUMPTION_DAY,
    16740_i32 => _ALL_SAINTS__DAY,
    16794_i32 => _CHRISTMAS_DAY,
    16795_i32 => _SAINT_STEPHEN_S_DAY,
    16801_i32 => _NEW_YEAR_S_DAY,
    16888_i32 => _EASTER_MONDAY,
    16922_i32 => _LABOR_DAY,
    16926_i32 => _ASCENSION_DAY,
    16937_i32 => _WHIT_MONDAY,
    16975_i32 => _NATIONAL_DAY,
    17028_i32 => _ASSUMPTION_DAY,
    17106_i32 => _ALL_SAINTS__DAY,
    17160_i32 => _CHRISTMAS_DAY,
    17161_i32 => _SAINT_STEPHEN_S_DAY,
    17167_i32 => _NEW_YEAR_S_DAY,
    17273_i32 => _EASTER_MONDAY,
    17287_i32 => _LABOR_DAY,
    17311_i32 => _ASCENSION_DAY,
    17322_i32 => _WHIT_MONDAY,
    17340_i32 => _NATIONAL_DAY,
    17393_i32 => _ASSUMPTION_DAY,
    17471_i32 => _ALL_SAINTS__DAY,
    17525_i32 => _CHRISTMAS_DAY,
    17526_i32 => _SAINT_STEPHEN_S_DAY,
    17532_i32 => _NEW_YEAR_S_DAY,
    17623_i32 => _EASTER_MONDAY,
    17652_i32 => _LABOR_DAY,
    17661_i32 => _ASCENSION_DAY,
    17672_i32 => _WHIT_MONDAY,
    17705_i32 => _NATIONAL_DAY,
    17758_i32 => _ASSUMPTION_DAY,
    17836_i32 => _ALL_SAINTS__DAY,
    17890_i32 => _CHRISTMAS_DAY,
    17891_i32 => _SAINT_STEPHEN_S_DAY,
    17897_i32 => _NEW_YEAR_S_DAY,
    18008_i32 => _EASTER_MONDAY,
    18017_i32 => _LABOR_DAY,
    18025_i32 => _EUROPE_DAY,
    18046_i32 => _ASCENSION_DAY,
    18057_i32 => _WHIT_MONDAY,
    18070_i32 => _NATIONAL_DAY,
    18123_i32 => _ASSUMPTION_DAY,
    18201_i32 => _ALL_SAINTS__DAY,
    18255_i32 => _CHRISTMAS_DAY,
    18256_i32 => _SAINT_STEPHEN_S_DAY,
    18262_i32 => _NEW_YEAR_S_DAY,
    18365_i32 => _EASTER_MONDAY,
    18383_i32 => _LABOR_DAY,
    18391_i32 => _EUROPE_DAY,
    18403_i32 => _ASCENSION_DAY,
    18414_i32 => _WHIT_MONDAY,
    18436_i32 => _NATIONAL_DAY,
    18489_i32 => _ASSUMPTION_DAY,
    18567_i32 => _ALL_SAINTS__DAY,
    18621_i32 => _CHRISTMAS_DAY,
    18622_i32 => _SAINT_STEPHEN_S_DAY,
    18628_i32 => _NEW_YEAR_S_DAY,
    18722_i32 => _EASTER_MONDAY,
    18748_i32 => _LABOR_DAY,
    18756_i32 => _EUROPE_DAY,
    18760_i32 => _ASCENSION_DAY,
    18771_i32 => _WHIT_MONDAY,
    18801_i32 => _NATIONAL_DAY,
    18854_i32 => _ASSUMPTION_DAY,
    18932_i32 => _ALL_SAINTS__DAY,
    18986_i32 => _CHRISTMAS_DAY,
    18987_i32 => _SAINT_STEPHEN_S_DAY,
    18993_i32 => _NEW_YEAR_S_DAY,
    19100_i32 => _EASTER_MONDAY,
    19113_i32 => _LABOR_DAY,
    19121_i32 => _EUROPE_DAY,
    19138_i32 => _ASCENSION_DAY,
    19149_i32 => _WHIT_MONDAY,
    19166_i32 => _NATIONAL_DAY,
    19219_i32 => _ASSUMPTION_DAY,
    19297_i32 => _ALL_SAINTS__DAY,
    19351_i32 => _CHRISTMAS_DAY,
    19352_i32 => _SAINT_STEPHEN_S_DAY,
    19358_i32 => _NEW_YEAR_S_DAY,
    19457_i32 => _EASTER_MONDAY,
    19478_i32 => _LABOR_DAY,
    19486_i32 => _EUROPE_DAY,
    19495_i32 => _ASCENSION_DAY,
    19506_i32 => _WHIT_MONDAY,
    19531_i32 => _NATIONAL_DAY,
    19584_i32 => _ASSUMPTION_DAY,
    19662_i32 => _ALL_SAINTS__DAY,
    19716_i32 => _CHRISTMAS_DAY,
    19717_i32 => _SAINT_STEPHEN_S_DAY,
    19723_i32 => _NEW_YEAR_S_DAY,
    19814_i32 => _EASTER_MONDAY,
    19844_i32 => _LABOR_DAY,
    19852_i32 => _ASCENSION_DAY__EUROPE_DAY,
    19863_i32 => _WHIT_MONDAY,
    19897_i32 => _NATIONAL_DAY,
    19950_i32 => _ASSUMPTION_DAY,
    20028_i32 => _ALL_SAINTS__DAY,
    20082_i32 => _CHRISTMAS_DAY,
    20083_i32 => _SAINT_STEPHEN_S_DAY,
    20089_i32 => _NEW_YEAR_S_DAY,
    20199_i32 => _EASTER_MONDAY,
    20209_i32 => _LABOR_DAY,
    20217_i32 => _EUROPE_DAY,
    20237_i32 => _ASCENSION_DAY,
    20248_i32 => _WHIT_MONDAY,
    20262_i32 => _NATIONAL_DAY,
    20315_i32 => _ASSUMPTION_DAY,
    20393_i32 => _ALL_SAINTS__DAY,
    20447_i32 => _CHRISTMAS_DAY,
    20448_i32 => _SAINT_STEPHEN_S_DAY,
    20454_i32 => _NEW_YEAR_S_DAY,
    20549_i32 => _EASTER_MONDAY,
    20574_i32 => _LABOR_DAY,
    20582_i32 => _EUROPE_DAY,
    20587_i32 => _ASCENSION_DAY,
    20598_i32 => _WHIT_MONDAY,
    20627_i32 => _NATIONAL_DAY,
    20680_i32 => _ASSUMPTION_DAY,
    20758_i32 => _ALL_SAINTS__DAY,
    20812_i32 => _CHRISTMAS_DAY,
    20813_i32 => _SAINT_STEPHEN_S_DAY,
    20819_i32 => _NEW_YEAR_S_DAY,
    20906_i32 => _EASTER_MONDAY,
    20939_i32 => _LABOR_DAY,
    20944_i32 => _ASCENSION_DAY,
    20947_i32 => _EUROPE_DAY,
    20955_i32 => _WHIT_MONDAY,
    20992_i32 => _NATIONAL_DAY,
    21045_i32 => _ASSUMPTION_DAY,
    21123_i32 => _ALL_SAINTS__DAY,
    21177_i32 => _CHRISTMAS_DAY,
    21178_i32 => _SAINT_STEPHEN_S_DAY,
    21184_i32 => _NEW_YEAR_S_DAY,
    21291_i32 => _EASTER_MONDAY,
    21305_i32 => _LABOR_DAY,
    21313_i32 => _EUROPE_DAY,
    21329_i32 => _ASCENSION_DAY,
    21340_i32 => _WHIT_MONDAY,
    21358_i32 => _NATIONAL_DAY,
    21411_i32 => _ASSUMPTION_DAY,
    21489_i32 => _ALL_SAINTS__DAY,
    21543_i32 => _CHRISTMAS_DAY,
    21544_i32 => _SAINT_STEPHEN_S_DAY,
    21550_i32 => _NEW_YEAR_S_DAY,
    21641_i32 => _EASTER_MONDAY,
    21670_i32 => _LABOR_DAY,
    21678_i32 => _EUROPE_DAY,
    21679_i32 => _ASCENSION_DAY,
    21690_i32 => _WHIT_MONDAY,
    21723_i32 => _NATIONAL_DAY,
    21776_i32 => _ASSUMPTION_DAY,
    21854_i32 => _ALL_SAINTS__DAY,
    21908_i32 => _CHRISTMAS_DAY,
    21909_i32 => _SAINT_STEPHEN_S_DAY,
    21915_i32 => _NEW_YEAR_S_DAY,
    22026_i32 => _EASTER_MONDAY,
    22035_i32 => _LABOR_DAY,
    22043_i32 => _EUROPE_DAY,
    22064_i32 => _ASCENSION_DAY,
    22075_i32 => _WHIT_MONDAY,
    22088_i32 => _NATIONAL_DAY,
    22141_i32 => _ASSUMPTION_DAY,
    22219_i32 => _ALL_SAINTS__DAY,
    22273_i32 => _CHRISTMAS_DAY,
    22274_i32 => _SAINT_STEPHEN_S_DAY,
    22280_i32 => _NEW_YEAR_S_DAY,
    22383_i32 => _EASTER_MONDAY,
    22400_i32 => _LABOR_DAY,
    22408_i32 => _EUROPE_DAY,
    22421_i32 => _ASCENSION_DAY,
    22432_i32 => _WHIT_MONDAY,
    22453_i32 => _NATIONAL_DAY,
    22506_i32 => _ASSUMPTION_DAY,
    22584_i32 => _ALL_SAINTS__DAY,
    22638_i32 => _CHRISTMAS_DAY,
    22639_i32 => _SAINT_STEPHEN_S_DAY,
    22645_i32 => _NEW_YEAR_S_DAY,
    22733_i32 => _EASTER_MONDAY,
    22766_i32 => _LABOR_DAY,
    22771_i32 => _ASCENSION_DAY,
    22774_i32 => _EUROPE_DAY,
    22782_i32 => _WHIT_MONDAY,
    22819_i32 => _NATIONAL_DAY,
    22872_i32 => _ASSUMPTION_DAY,
    22950_i32 => _ALL_SAINTS__DAY,
    23004_i32 => _CHRISTMAS_DAY,
    23005_i32 => _SAINT_STEPHEN_S_DAY,
    23011_i32 => _NEW_YEAR_S_DAY,
    23118_i32 => _EASTER_MONDAY,
    23131_i32 => _LABOR_DAY,
    23139_i32 => _EUROPE_DAY,
    23156_i32 => _ASCENSION_DAY,
    23167_i32 => _WHIT_MONDAY,
    23184_i32 => _NATIONAL_DAY,
    23237_i32 => _ASSUMPTION_DAY,
    23315_i32 => _ALL_SAINTS__DAY,
    23369_i32 => _CHRISTMAS_DAY,
    23370_i32 => _SAINT_STEPHEN_S_DAY,
    23376_i32 => _NEW_YEAR_S_DAY,
    23475_i32 => _EASTER_MONDAY,
    23496_i32 => _LABOR_DAY,
    23504_i32 => _EUROPE_DAY,
    23513_i32 => _ASCENSION_DAY,
    23524_i32 => _WHIT_MONDAY,
    23549_i32 => _NATIONAL_DAY,
    23602_i32 => _ASSUMPTION_DAY,
    23680_i32 => _ALL_SAINTS__DAY,
    23734_i32 => _CHRISTMAS_DAY,
    23735_i32 => _SAINT_STEPHEN_S_DAY,
    23741_i32 => _NEW_YEAR_S_DAY,
    23825_i32 => _EASTER_MONDAY,
    23861_i32 => _LABOR_DAY,
    23863_i32 => _ASCENSION_DAY,
    23869_i32 => _EUROPE_DAY,
    23874_i32 => _WHIT_MONDAY,
    23914_i32 => _NATIONAL_DAY,
    23967_i32 => _ASSUMPTION_DAY,
    24045_i32 => _ALL_SAINTS__DAY,
    24099_i32 => _CHRISTMAS_DAY,
    24100_i32 => _SAINT_STEPHEN_S_DAY,
    24106_i32 => _NEW_YEAR_S_DAY,
    24210_i32 => _EASTER_MONDAY,
    24227_i32 => _LABOR_DAY,
    24235_i32 => _EUROPE_DAY,
    24248_i32 => _ASCENSION_DAY,
    24259_i32 => _WHIT_MONDAY,
    24280_i32 => _NATIONAL_DAY,
    24333_i32 => _ASSUMPTION_DAY,
    24411_i32 => _ALL_SAINTS__DAY,
    24465_i32 => _CHRISTMAS_DAY,
    24466_i32 => _SAINT_STEPHEN_S_DAY,
    24472_i32 => _NEW_YEAR_S_DAY,
    24567_i32 => _EASTER_MONDAY,
    24592_i32 => _LABOR_DAY,
    24600_i32 => _EUROPE_DAY,
    24605_i32 => _ASCENSION_DAY,
    24616_i32 => _WHIT_MONDAY,
    24645_i32 => _NATIONAL_DAY,
    24698_i32 => _ASSUMPTION_DAY,
    24776_i32 => _ALL_SAINTS__DAY,
    24830_i32 => _CHRISTMAS_DAY,
    24831_i32 => _SAINT_STEPHEN_S_DAY,
    24837_i32 => _NEW_YEAR_S_DAY,
    24952_i32 => _EASTER_MONDAY,
    24957_i32 => _LABOR_DAY,
    24965_i32 => _EUROPE_DAY,
    24990_i32 => _ASCENSION_DAY,
    25001_i32 => _WHIT_MONDAY,
    25010_i32 => _NATIONAL_DAY,
    25063_i32 => _ASSUMPTION_DAY,
    25141_i32 => _ALL_SAINTS__DAY,
    25195_i32 => _CHRISTMAS_DAY,
    25196_i32 => _SAINT_STEPHEN_S_DAY,
    25202_i32 => _NEW_YEAR_S_DAY,
    25302_i32 => _EASTER_MONDAY,
    25322_i32 => _LABOR_DAY,
    25330_i32 => _EUROPE_DAY,
    25340_i32 => _ASCENSION_DAY,
    25351_i32 => _WHIT_MONDAY,
    25375_i32 => _NATIONAL_DAY,
    25428_i32 => _ASSUMPTION_DAY,
    25506_i32 => _ALL_SAINTS__DAY,
    25560_i32 => _CHRISTMAS_DAY,
    25561_i32 => _SAINT_STEPHEN_S_DAY,
    25567_i32 => _NEW_YEAR_S_DAY,
    25659_i32 => _EASTER_MONDAY,
    25688_i32 => _LABOR_DAY,
    25696_i32 => _EUROPE_DAY,
    25697_i32 => _ASCENSION_DAY,
    25708_i32 => _WHIT_MONDAY,
    25741_i32 => _NATIONAL_DAY,
    25794_i32 => _ASSUMPTION_DAY,
    25872_i32 => _ALL_SAINTS__DAY,
    25926_i32 => _CHRISTMAS_DAY,
    25927_i32 => _SAINT_STEPHEN_S_DAY,
    25933_i32 => _NEW_YEAR_S_DAY,
    26044_i32 => _EASTER_MONDAY,
    26053_i32 => _LABOR_DAY,
    26061_i32 => _EUROPE_DAY,
    26082_i32 => _ASCENSION_DAY,
    26093_i32 => _WHIT_MONDAY,
    26106_i32 => _NATIONAL_DAY,
    26159_i32 => _ASSUMPTION_DAY,
    26237_i32 => _ALL_SAINTS__DAY,
    26291_i32 => _CHRISTMAS_DAY,
    26292_i32 => _SAINT_STEPHEN_S_DAY,
    26298_i32 => _NEW_YEAR_S_DAY,
    26394_i32 => _EASTER_MONDAY,
    26418_i32 => _LABOR_DAY,
    26426_i32 => _EUROPE_DAY,
    26432_i32 => _ASCENSION_DAY,
    26443_i32 => _WHIT_MONDAY,
    26471_i32 => _NATIONAL_DAY,
    26524_i32 => _ASSUMPTION_DAY,
    26602_i32 => _ALL_SAINTS__DAY,
    26656_i32 => _CHRISTMAS_DAY,
    26657_i32 => _SAINT_STEPHEN_S_DAY,
    26663_i32 => _NEW_YEAR_S_DAY,
    26751_i32 => _EASTER_MONDAY,
    26783_i32 => _LABOR_DAY,
    26789_i32 => _ASCENSION_DAY,
    26791_i32 => _EUROPE_DAY,
    26800_i32 => _WHIT_MONDAY,
    26836_i32 => _NATIONAL_DAY,
    26889_i32 => _ASSUMPTION_DAY,
    26967_i32 => _ALL_SAINTS__DAY,
    27021_i32 => _CHRISTMAS_DAY,
    27022_i32 => _SAINT_STEPHEN_S_DAY,
    27028_i32 => _NEW_YEAR_S_DAY,
    27136_i32 => _EASTER_MONDAY,
    27149_i32 => _LABOR_DAY,
    27157_i32 => _EUROPE_DAY,
    27174_i32 => _ASCENSION_DAY,
    27185_i32 => _WHIT_MONDAY,
    27202_i32 => _NATIONAL_DAY,
    27255_i32 => _ASSUMPTION_DAY,
    27333_i32 => _ALL_SAINTS__DAY,
    27387_i32 => _CHRISTMAS_DAY,
    27388_i32 => _SAINT_STEPHEN_S_DAY,
    27394_i32 => _NEW_YEAR_S_DAY,
    27493_i32 => _EASTER_MONDAY,
    27514_i32 => _LABOR_DAY,
    27522_i32 => _EUROPE_DAY,
    27531_i32 => _ASCENSION_DAY,
    27542_i32 => _WHIT_MONDAY,
    27567_i32 => _NATIONAL_DAY,
    27620_i32 => _ASSUMPTION_DAY,
    27698_i32 => _ALL_SAINTS__DAY,
    27752_i32 => _CHRISTMAS_DAY,
    27753_i32 => _SAINT_STEPHEN_S_DAY,
    27759_i32 => _NEW_YEAR_S_DAY,
    27843_i32 => _EASTER_MONDAY,
    27879_i32 => _LABOR_DAY,
    27881_i32 => _ASCENSION_DAY,
    27887_i32 => _EUROPE_DAY,
    27892_i32 => _WHIT_MONDAY,
    27932_i32 => _NATIONAL_DAY,
    27985_i32 => _ASSUMPTION_DAY,
    28063_i32 => _ALL_SAINTS__DAY,
    28117_i32 => _CHRISTMAS_DAY,
    28118_i32 => _SAINT_STEPHEN_S_DAY,
    28124_i32 => _NEW_YEAR_S_DAY,
    28228_i32 => _EASTER_MONDAY,
    28244_i32 => _LABOR_DAY,
    28252_i32 => _EUROPE_DAY,
    28266_i32 => _ASCENSION_DAY,
    28277_i32 => _WHIT_MONDAY,
    28297_i32 => _NATIONAL_DAY,
    28350_i32 => _ASSUMPTION_DAY,
    28428_i32 => _ALL_SAINTS__DAY,
    28482_i32 => _CHRISTMAS_DAY,
    28483_i32 => _SAINT_STEPHEN_S_DAY,
    28489_i32 => _NEW_YEAR_S_DAY,
    28585_i32 => _EASTER_MONDAY,
    28610_i32 => _LABOR_DAY,
    28618_i32 => _EUROPE_DAY,
    28623_i32 => _ASCENSION_DAY,
    28634_i32 => _WHIT_MONDAY,
    28663_i32 => _NATIONAL_DAY,
    28716_i32 => _ASSUMPTION_DAY,
    28794_i32 => _ALL_SAINTS__DAY,
    28848_i32 => _CHRISTMAS_DAY,
    28849_i32 => _SAINT_STEPHEN_S_DAY,
    28855_i32 => _NEW_YEAR_S_DAY,
    28963_i32 => _EASTER_MONDAY,
    28975_i32 => _LABOR_DAY,
    28983_i32 => _EUROPE_DAY,
    29001_i32 => _ASCENSION_DAY,
    29012_i32 => _WHIT_MONDAY,
    29028_i32 => _NATIONAL_DAY,
    29081_i32 => _ASSUMPTION_DAY,
    29159_i32 => _ALL_SAINTS__DAY,
    29213_i32 => _CHRISTMAS_DAY,
    29214_i32 => _SAINT_STEPHEN_S_DAY,
    29220_i32 => _NEW_YEAR_S_DAY,
};
