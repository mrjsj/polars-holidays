use crate::countries::constants::*;
use phf::phf_map;

pub static SK_HOLIDAYS: phf::Map<i32, &'static str> = phf_map! {
    10957_i32 => _DAY_OF_THE_ESTABLISHMENT_OF_THE_SLOVAK_REPUBLIC,
    10962_i32 => _EPIPHANY__THREE_KINGS__DAY_AND_ORTHODOX_CHRISTMAS_,
    11068_i32 => _GOOD_FRIDAY,
    11071_i32 => _EASTER_MONDAY,
    11078_i32 => _LABOR_DAY,
    11085_i32 => _DAY_OF_VICTORY_OVER_FASCISM,
    11143_i32 => _SAINTS_CYRIL_AND_METHODIUS_DAY,
    11198_i32 => _SLOVAK_NATIONAL_UPRISING_ANNIVERSARY,
    11201_i32 => _CONSTITUTION_DAY,
    11215_i32 => _DAY_OF_OUR_LADY_OF_THE_SEVEN_SORROWS,
    11262_i32 => _ALL_SAINTS__DAY,
    11315_i32 => _CHRISTMAS_EVE,
    11316_i32 => _CHRISTMAS_DAY,
    11317_i32 => _SECOND_DAY_OF_CHRISTMAS,
    11323_i32 => _DAY_OF_THE_ESTABLISHMENT_OF_THE_SLOVAK_REPUBLIC,
    11328_i32 => _EPIPHANY__THREE_KINGS__DAY_AND_ORTHODOX_CHRISTMAS_,
    11425_i32 => _GOOD_FRIDAY,
    11428_i32 => _EASTER_MONDAY,
    11443_i32 => _LABOR_DAY,
    11450_i32 => _DAY_OF_VICTORY_OVER_FASCISM,
    11508_i32 => _SAINTS_CYRIL_AND_METHODIUS_DAY,
    11563_i32 => _SLOVAK_NATIONAL_UPRISING_ANNIVERSARY,
    11566_i32 => _CONSTITUTION_DAY,
    11580_i32 => _DAY_OF_OUR_LADY_OF_THE_SEVEN_SORROWS,
    11627_i32 => _ALL_SAINTS__DAY,
    11643_i32 => _STRUGGLE_FOR_FREEDOM_AND_DEMOCRACY_DAY,
    11680_i32 => _CHRISTMAS_EVE,
    11681_i32 => _CHRISTMAS_DAY,
    11682_i32 => _SECOND_DAY_OF_CHRISTMAS,
    11688_i32 => _DAY_OF_THE_ESTABLISHMENT_OF_THE_SLOVAK_REPUBLIC,
    11693_i32 => _EPIPHANY__THREE_KINGS__DAY_AND_ORTHODOX_CHRISTMAS_,
    11775_i32 => _GOOD_FRIDAY,
    11778_i32 => _EASTER_MONDAY,
    11808_i32 => _LABOR_DAY,
    11815_i32 => _DAY_OF_VICTORY_OVER_FASCISM,
    11873_i32 => _SAINTS_CYRIL_AND_METHODIUS_DAY,
    11928_i32 => _SLOVAK_NATIONAL_UPRISING_ANNIVERSARY,
    11931_i32 => _CONSTITUTION_DAY,
    11945_i32 => _DAY_OF_OUR_LADY_OF_THE_SEVEN_SORROWS,
    11992_i32 => _ALL_SAINTS__DAY,
    12008_i32 => _STRUGGLE_FOR_FREEDOM_AND_DEMOCRACY_DAY,
    12045_i32 => _CHRISTMAS_EVE,
    12046_i32 => _CHRISTMAS_DAY,
    12047_i32 => _SECOND_DAY_OF_CHRISTMAS,
    12053_i32 => _DAY_OF_THE_ESTABLISHMENT_OF_THE_SLOVAK_REPUBLIC,
    12058_i32 => _EPIPHANY__THREE_KINGS__DAY_AND_ORTHODOX_CHRISTMAS_,
    12160_i32 => _GOOD_FRIDAY,
    12163_i32 => _EASTER_MONDAY,
    12173_i32 => _LABOR_DAY,
    12180_i32 => _DAY_OF_VICTORY_OVER_FASCISM,
    12238_i32 => _SAINTS_CYRIL_AND_METHODIUS_DAY,
    12293_i32 => _SLOVAK_NATIONAL_UPRISING_ANNIVERSARY,
    12296_i32 => _CONSTITUTION_DAY,
    12310_i32 => _DAY_OF_OUR_LADY_OF_THE_SEVEN_SORROWS,
    12357_i32 => _ALL_SAINTS__DAY,
    12373_i32 => _STRUGGLE_FOR_FREEDOM_AND_DEMOCRACY_DAY,
    12410_i32 => _CHRISTMAS_EVE,
    12411_i32 => _CHRISTMAS_DAY,
    12412_i32 => _SECOND_DAY_OF_CHRISTMAS,
    12418_i32 => _DAY_OF_THE_ESTABLISHMENT_OF_THE_SLOVAK_REPUBLIC,
    12423_i32 => _EPIPHANY__THREE_KINGS__DAY_AND_ORTHODOX_CHRISTMAS_,
    12517_i32 => _GOOD_FRIDAY,
    12520_i32 => _EASTER_MONDAY,
    12539_i32 => _LABOR_DAY,
    12546_i32 => _DAY_OF_VICTORY_OVER_FASCISM,
    12604_i32 => _SAINTS_CYRIL_AND_METHODIUS_DAY,
    12659_i32 => _SLOVAK_NATIONAL_UPRISING_ANNIVERSARY,
    12662_i32 => _CONSTITUTION_DAY,
    12676_i32 => _DAY_OF_OUR_LADY_OF_THE_SEVEN_SORROWS,
    12723_i32 => _ALL_SAINTS__DAY,
    12739_i32 => _STRUGGLE_FOR_FREEDOM_AND_DEMOCRACY_DAY,
    12776_i32 => _CHRISTMAS_EVE,
    12777_i32 => _CHRISTMAS_DAY,
    12778_i32 => _SECOND_DAY_OF_CHRISTMAS,
    12784_i32 => _DAY_OF_THE_ESTABLISHMENT_OF_THE_SLOVAK_REPUBLIC,
    12789_i32 => _EPIPHANY__THREE_KINGS__DAY_AND_ORTHODOX_CHRISTMAS_,
    12867_i32 => _GOOD_FRIDAY,
    12870_i32 => _EASTER_MONDAY,
    12904_i32 => _LABOR_DAY,
    12911_i32 => _DAY_OF_VICTORY_OVER_FASCISM,
    12969_i32 => _SAINTS_CYRIL_AND_METHODIUS_DAY,
    13024_i32 => _SLOVAK_NATIONAL_UPRISING_ANNIVERSARY,
    13027_i32 => _CONSTITUTION_DAY,
    13041_i32 => _DAY_OF_OUR_LADY_OF_THE_SEVEN_SORROWS,
    13088_i32 => _ALL_SAINTS__DAY,
    13104_i32 => _STRUGGLE_FOR_FREEDOM_AND_DEMOCRACY_DAY,
    13141_i32 => _CHRISTMAS_EVE,
    13142_i32 => _CHRISTMAS_DAY,
    13143_i32 => _SECOND_DAY_OF_CHRISTMAS,
    13149_i32 => _DAY_OF_THE_ESTABLISHMENT_OF_THE_SLOVAK_REPUBLIC,
    13154_i32 => _EPIPHANY__THREE_KINGS__DAY_AND_ORTHODOX_CHRISTMAS_,
    13252_i32 => _GOOD_FRIDAY,
    13255_i32 => _EASTER_MONDAY,
    13269_i32 => _LABOR_DAY,
    13276_i32 => _DAY_OF_VICTORY_OVER_FASCISM,
    13334_i32 => _SAINTS_CYRIL_AND_METHODIUS_DAY,
    13389_i32 => _SLOVAK_NATIONAL_UPRISING_ANNIVERSARY,
    13392_i32 => _CONSTITUTION_DAY,
    13406_i32 => _DAY_OF_OUR_LADY_OF_THE_SEVEN_SORROWS,
    13453_i32 => _ALL_SAINTS__DAY,
    13469_i32 => _STRUGGLE_FOR_FREEDOM_AND_DEMOCRACY_DAY,
    13506_i32 => _CHRISTMAS_EVE,
    13507_i32 => _CHRISTMAS_DAY,
    13508_i32 => _SECOND_DAY_OF_CHRISTMAS,
    13514_i32 => _DAY_OF_THE_ESTABLISHMENT_OF_THE_SLOVAK_REPUBLIC,
    13519_i32 => _EPIPHANY__THREE_KINGS__DAY_AND_ORTHODOX_CHRISTMAS_,
    13609_i32 => _GOOD_FRIDAY,
    13612_i32 => _EASTER_MONDAY,
    13634_i32 => _LABOR_DAY,
    13641_i32 => _DAY_OF_VICTORY_OVER_FASCISM,
    13699_i32 => _SAINTS_CYRIL_AND_METHODIUS_DAY,
    13754_i32 => _SLOVAK_NATIONAL_UPRISING_ANNIVERSARY,
    13757_i32 => _CONSTITUTION_DAY,
    13771_i32 => _DAY_OF_OUR_LADY_OF_THE_SEVEN_SORROWS,
    13818_i32 => _ALL_SAINTS__DAY,
    13834_i32 => _STRUGGLE_FOR_FREEDOM_AND_DEMOCRACY_DAY,
    13871_i32 => _CHRISTMAS_EVE,
    13872_i32 => _CHRISTMAS_DAY,
    13873_i32 => _SECOND_DAY_OF_CHRISTMAS,
    13879_i32 => _DAY_OF_THE_ESTABLISHMENT_OF_THE_SLOVAK_REPUBLIC,
    13884_i32 => _EPIPHANY__THREE_KINGS__DAY_AND_ORTHODOX_CHRISTMAS_,
    13959_i32 => _GOOD_FRIDAY,
    13962_i32 => _EASTER_MONDAY,
    14000_i32 => _LABOR_DAY,
    14007_i32 => _DAY_OF_VICTORY_OVER_FASCISM,
    14065_i32 => _SAINTS_CYRIL_AND_METHODIUS_DAY,
    14120_i32 => _SLOVAK_NATIONAL_UPRISING_ANNIVERSARY,
    14123_i32 => _CONSTITUTION_DAY,
    14137_i32 => _DAY_OF_OUR_LADY_OF_THE_SEVEN_SORROWS,
    14184_i32 => _ALL_SAINTS__DAY,
    14200_i32 => _STRUGGLE_FOR_FREEDOM_AND_DEMOCRACY_DAY,
    14237_i32 => _CHRISTMAS_EVE,
    14238_i32 => _CHRISTMAS_DAY,
    14239_i32 => _SECOND_DAY_OF_CHRISTMAS,
    14245_i32 => _DAY_OF_THE_ESTABLISHMENT_OF_THE_SLOVAK_REPUBLIC,
    14250_i32 => _EPIPHANY__THREE_KINGS__DAY_AND_ORTHODOX_CHRISTMAS_,
    14344_i32 => _GOOD_FRIDAY,
    14347_i32 => _EASTER_MONDAY,
    14365_i32 => _LABOR_DAY,
    14372_i32 => _DAY_OF_VICTORY_OVER_FASCISM,
    14430_i32 => _SAINTS_CYRIL_AND_METHODIUS_DAY,
    14485_i32 => _SLOVAK_NATIONAL_UPRISING_ANNIVERSARY,
    14488_i32 => _CONSTITUTION_DAY,
    14502_i32 => _DAY_OF_OUR_LADY_OF_THE_SEVEN_SORROWS,
    14549_i32 => _ALL_SAINTS__DAY,
    14565_i32 => _STRUGGLE_FOR_FREEDOM_AND_DEMOCRACY_DAY,
    14602_i32 => _CHRISTMAS_EVE,
    14603_i32 => _CHRISTMAS_DAY,
    14604_i32 => _SECOND_DAY_OF_CHRISTMAS,
    14610_i32 => _DAY_OF_THE_ESTABLISHMENT_OF_THE_SLOVAK_REPUBLIC,
    14615_i32 => _EPIPHANY__THREE_KINGS__DAY_AND_ORTHODOX_CHRISTMAS_,
    14701_i32 => _GOOD_FRIDAY,
    14704_i32 => _EASTER_MONDAY,
    14730_i32 => _LABOR_DAY,
    14737_i32 => _DAY_OF_VICTORY_OVER_FASCISM,
    14795_i32 => _SAINTS_CYRIL_AND_METHODIUS_DAY,
    14850_i32 => _SLOVAK_NATIONAL_UPRISING_ANNIVERSARY,
    14853_i32 => _CONSTITUTION_DAY,
    14867_i32 => _DAY_OF_OUR_LADY_OF_THE_SEVEN_SORROWS,
    14914_i32 => _ALL_SAINTS__DAY,
    14930_i32 => _STRUGGLE_FOR_FREEDOM_AND_DEMOCRACY_DAY,
    14967_i32 => _CHRISTMAS_EVE,
    14968_i32 => _CHRISTMAS_DAY,
    14969_i32 => _SECOND_DAY_OF_CHRISTMAS,
    14975_i32 => _DAY_OF_THE_ESTABLISHMENT_OF_THE_SLOVAK_REPUBLIC,
    14980_i32 => _EPIPHANY__THREE_KINGS__DAY_AND_ORTHODOX_CHRISTMAS_,
    15086_i32 => _GOOD_FRIDAY,
    15089_i32 => _EASTER_MONDAY,
    15095_i32 => _LABOR_DAY,
    15102_i32 => _DAY_OF_VICTORY_OVER_FASCISM,
    15160_i32 => _SAINTS_CYRIL_AND_METHODIUS_DAY,
    15215_i32 => _SLOVAK_NATIONAL_UPRISING_ANNIVERSARY,
    15218_i32 => _CONSTITUTION_DAY,
    15232_i32 => _DAY_OF_OUR_LADY_OF_THE_SEVEN_SORROWS,
    15279_i32 => _ALL_SAINTS__DAY,
    15295_i32 => _STRUGGLE_FOR_FREEDOM_AND_DEMOCRACY_DAY,
    15332_i32 => _CHRISTMAS_EVE,
    15333_i32 => _CHRISTMAS_DAY,
    15334_i32 => _SECOND_DAY_OF_CHRISTMAS,
    15340_i32 => _DAY_OF_THE_ESTABLISHMENT_OF_THE_SLOVAK_REPUBLIC,
    15345_i32 => _EPIPHANY__THREE_KINGS__DAY_AND_ORTHODOX_CHRISTMAS_,
    15436_i32 => _GOOD_FRIDAY,
    15439_i32 => _EASTER_MONDAY,
    15461_i32 => _LABOR_DAY,
    15468_i32 => _DAY_OF_VICTORY_OVER_FASCISM,
    15526_i32 => _SAINTS_CYRIL_AND_METHODIUS_DAY,
    15581_i32 => _SLOVAK_NATIONAL_UPRISING_ANNIVERSARY,
    15584_i32 => _CONSTITUTION_DAY,
    15598_i32 => _DAY_OF_OUR_LADY_OF_THE_SEVEN_SORROWS,
    15645_i32 => _ALL_SAINTS__DAY,
    15661_i32 => _STRUGGLE_FOR_FREEDOM_AND_DEMOCRACY_DAY,
    15698_i32 => _CHRISTMAS_EVE,
    15699_i32 => _CHRISTMAS_DAY,
    15700_i32 => _SECOND_DAY_OF_CHRISTMAS,
    15706_i32 => _DAY_OF_THE_ESTABLISHMENT_OF_THE_SLOVAK_REPUBLIC,
    15711_i32 => _EPIPHANY__THREE_KINGS__DAY_AND_ORTHODOX_CHRISTMAS_,
    15793_i32 => _GOOD_FRIDAY,
    15796_i32 => _EASTER_MONDAY,
    15826_i32 => _LABOR_DAY,
    15833_i32 => _DAY_OF_VICTORY_OVER_FASCISM,
    15891_i32 => _SAINTS_CYRIL_AND_METHODIUS_DAY,
    15946_i32 => _SLOVAK_NATIONAL_UPRISING_ANNIVERSARY,
    15949_i32 => _CONSTITUTION_DAY,
    15963_i32 => _DAY_OF_OUR_LADY_OF_THE_SEVEN_SORROWS,
    16010_i32 => _ALL_SAINTS__DAY,
    16026_i32 => _STRUGGLE_FOR_FREEDOM_AND_DEMOCRACY_DAY,
    16063_i32 => _CHRISTMAS_EVE,
    16064_i32 => _CHRISTMAS_DAY,
    16065_i32 => _SECOND_DAY_OF_CHRISTMAS,
    16071_i32 => _DAY_OF_THE_ESTABLISHMENT_OF_THE_SLOVAK_REPUBLIC,
    16076_i32 => _EPIPHANY__THREE_KINGS__DAY_AND_ORTHODOX_CHRISTMAS_,
    16178_i32 => _GOOD_FRIDAY,
    16181_i32 => _EASTER_MONDAY,
    16191_i32 => _LABOR_DAY,
    16198_i32 => _DAY_OF_VICTORY_OVER_FASCISM,
    16256_i32 => _SAINTS_CYRIL_AND_METHODIUS_DAY,
    16311_i32 => _SLOVAK_NATIONAL_UPRISING_ANNIVERSARY,
    16314_i32 => _CONSTITUTION_DAY,
    16328_i32 => _DAY_OF_OUR_LADY_OF_THE_SEVEN_SORROWS,
    16375_i32 => _ALL_SAINTS__DAY,
    16391_i32 => _STRUGGLE_FOR_FREEDOM_AND_DEMOCRACY_DAY,
    16428_i32 => _CHRISTMAS_EVE,
    16429_i32 => _CHRISTMAS_DAY,
    16430_i32 => _SECOND_DAY_OF_CHRISTMAS,
    16436_i32 => _DAY_OF_THE_ESTABLISHMENT_OF_THE_SLOVAK_REPUBLIC,
    16441_i32 => _EPIPHANY__THREE_KINGS__DAY_AND_ORTHODOX_CHRISTMAS_,
    16528_i32 => _GOOD_FRIDAY,
    16531_i32 => _EASTER_MONDAY,
    16556_i32 => _LABOR_DAY,
    16563_i32 => _DAY_OF_VICTORY_OVER_FASCISM,
    16621_i32 => _SAINTS_CYRIL_AND_METHODIUS_DAY,
    16676_i32 => _SLOVAK_NATIONAL_UPRISING_ANNIVERSARY,
    16679_i32 => _CONSTITUTION_DAY,
    16693_i32 => _DAY_OF_OUR_LADY_OF_THE_SEVEN_SORROWS,
    16740_i32 => _ALL_SAINTS__DAY,
    16756_i32 => _STRUGGLE_FOR_FREEDOM_AND_DEMOCRACY_DAY,
    16793_i32 => _CHRISTMAS_EVE,
    16794_i32 => _CHRISTMAS_DAY,
    16795_i32 => _SECOND_DAY_OF_CHRISTMAS,
    16801_i32 => _DAY_OF_THE_ESTABLISHMENT_OF_THE_SLOVAK_REPUBLIC,
    16806_i32 => _EPIPHANY__THREE_KINGS__DAY_AND_ORTHODOX_CHRISTMAS_,
    16885_i32 => _GOOD_FRIDAY,
    16888_i32 => _EASTER_MONDAY,
    16922_i32 => _LABOR_DAY,
    16929_i32 => _DAY_OF_VICTORY_OVER_FASCISM,
    16987_i32 => _SAINTS_CYRIL_AND_METHODIUS_DAY,
    17042_i32 => _SLOVAK_NATIONAL_UPRISING_ANNIVERSARY,
    17045_i32 => _CONSTITUTION_DAY,
    17059_i32 => _DAY_OF_OUR_LADY_OF_THE_SEVEN_SORROWS,
    17106_i32 => _ALL_SAINTS__DAY,
    17122_i32 => _STRUGGLE_FOR_FREEDOM_AND_DEMOCRACY_DAY,
    17159_i32 => _CHRISTMAS_EVE,
    17160_i32 => _CHRISTMAS_DAY,
    17161_i32 => _SECOND_DAY_OF_CHRISTMAS,
    17167_i32 => _DAY_OF_THE_ESTABLISHMENT_OF_THE_SLOVAK_REPUBLIC,
    17172_i32 => _EPIPHANY__THREE_KINGS__DAY_AND_ORTHODOX_CHRISTMAS_,
    17270_i32 => _GOOD_FRIDAY,
    17273_i32 => _EASTER_MONDAY,
    17287_i32 => _LABOR_DAY,
    17294_i32 => _DAY_OF_VICTORY_OVER_FASCISM,
    17352_i32 => _SAINTS_CYRIL_AND_METHODIUS_DAY,
    17407_i32 => _SLOVAK_NATIONAL_UPRISING_ANNIVERSARY,
    17410_i32 => _CONSTITUTION_DAY,
    17424_i32 => _DAY_OF_OUR_LADY_OF_THE_SEVEN_SORROWS,
    17471_i32 => _ALL_SAINTS__DAY,
    17487_i32 => _STRUGGLE_FOR_FREEDOM_AND_DEMOCRACY_DAY,
    17524_i32 => _CHRISTMAS_EVE,
    17525_i32 => _CHRISTMAS_DAY,
    17526_i32 => _SECOND_DAY_OF_CHRISTMAS,
    17532_i32 => _DAY_OF_THE_ESTABLISHMENT_OF_THE_SLOVAK_REPUBLIC,
    17537_i32 => _EPIPHANY__THREE_KINGS__DAY_AND_ORTHODOX_CHRISTMAS_,
    17620_i32 => _GOOD_FRIDAY,
    17623_i32 => _EASTER_MONDAY,
    17652_i32 => _LABOR_DAY,
    17659_i32 => _DAY_OF_VICTORY_OVER_FASCISM,
    17717_i32 => _SAINTS_CYRIL_AND_METHODIUS_DAY,
    17772_i32 => _SLOVAK_NATIONAL_UPRISING_ANNIVERSARY,
    17775_i32 => _CONSTITUTION_DAY,
    17789_i32 => _DAY_OF_OUR_LADY_OF_THE_SEVEN_SORROWS,
    17834_i32 => _100TH_ANNIVERSARY_OF_THE_ADOPTION_OF_THE_DECLARATION_OF_THE_SLOVAK_NATION,
    17836_i32 => _ALL_SAINTS__DAY,
    17852_i32 => _STRUGGLE_FOR_FREEDOM_AND_DEMOCRACY_DAY,
    17889_i32 => _CHRISTMAS_EVE,
    17890_i32 => _CHRISTMAS_DAY,
    17891_i32 => _SECOND_DAY_OF_CHRISTMAS,
    17897_i32 => _DAY_OF_THE_ESTABLISHMENT_OF_THE_SLOVAK_REPUBLIC,
    17902_i32 => _EPIPHANY__THREE_KINGS__DAY_AND_ORTHODOX_CHRISTMAS_,
    18005_i32 => _GOOD_FRIDAY,
    18008_i32 => _EASTER_MONDAY,
    18017_i32 => _LABOR_DAY,
    18024_i32 => _DAY_OF_VICTORY_OVER_FASCISM,
    18082_i32 => _SAINTS_CYRIL_AND_METHODIUS_DAY,
    18137_i32 => _SLOVAK_NATIONAL_UPRISING_ANNIVERSARY,
    18140_i32 => _CONSTITUTION_DAY,
    18154_i32 => _DAY_OF_OUR_LADY_OF_THE_SEVEN_SORROWS,
    18201_i32 => _ALL_SAINTS__DAY,
    18217_i32 => _STRUGGLE_FOR_FREEDOM_AND_DEMOCRACY_DAY,
    18254_i32 => _CHRISTMAS_EVE,
    18255_i32 => _CHRISTMAS_DAY,
    18256_i32 => _SECOND_DAY_OF_CHRISTMAS,
    18262_i32 => _DAY_OF_THE_ESTABLISHMENT_OF_THE_SLOVAK_REPUBLIC,
    18267_i32 => _EPIPHANY__THREE_KINGS__DAY_AND_ORTHODOX_CHRISTMAS_,
    18362_i32 => _GOOD_FRIDAY,
    18365_i32 => _EASTER_MONDAY,
    18383_i32 => _LABOR_DAY,
    18390_i32 => _DAY_OF_VICTORY_OVER_FASCISM,
    18448_i32 => _SAINTS_CYRIL_AND_METHODIUS_DAY,
    18503_i32 => _SLOVAK_NATIONAL_UPRISING_ANNIVERSARY,
    18506_i32 => _CONSTITUTION_DAY,
    18520_i32 => _DAY_OF_OUR_LADY_OF_THE_SEVEN_SORROWS,
    18567_i32 => _ALL_SAINTS__DAY,
    18583_i32 => _STRUGGLE_FOR_FREEDOM_AND_DEMOCRACY_DAY,
    18620_i32 => _CHRISTMAS_EVE,
    18621_i32 => _CHRISTMAS_DAY,
    18622_i32 => _SECOND_DAY_OF_CHRISTMAS,
    18628_i32 => _DAY_OF_THE_ESTABLISHMENT_OF_THE_SLOVAK_REPUBLIC,
    18633_i32 => _EPIPHANY__THREE_KINGS__DAY_AND_ORTHODOX_CHRISTMAS_,
    18719_i32 => _GOOD_FRIDAY,
    18722_i32 => _EASTER_MONDAY,
    18748_i32 => _LABOR_DAY,
    18755_i32 => _DAY_OF_VICTORY_OVER_FASCISM,
    18813_i32 => _SAINTS_CYRIL_AND_METHODIUS_DAY,
    18868_i32 => _SLOVAK_NATIONAL_UPRISING_ANNIVERSARY,
    18871_i32 => _CONSTITUTION_DAY,
    18885_i32 => _DAY_OF_OUR_LADY_OF_THE_SEVEN_SORROWS,
    18932_i32 => _ALL_SAINTS__DAY,
    18948_i32 => _STRUGGLE_FOR_FREEDOM_AND_DEMOCRACY_DAY,
    18985_i32 => _CHRISTMAS_EVE,
    18986_i32 => _CHRISTMAS_DAY,
    18987_i32 => _SECOND_DAY_OF_CHRISTMAS,
    18993_i32 => _DAY_OF_THE_ESTABLISHMENT_OF_THE_SLOVAK_REPUBLIC,
    18998_i32 => _EPIPHANY__THREE_KINGS__DAY_AND_ORTHODOX_CHRISTMAS_,
    19097_i32 => _GOOD_FRIDAY,
    19100_i32 => _EASTER_MONDAY,
    19113_i32 => _LABOR_DAY,
    19120_i32 => _DAY_OF_VICTORY_OVER_FASCISM,
    19178_i32 => _SAINTS_CYRIL_AND_METHODIUS_DAY,
    19233_i32 => _SLOVAK_NATIONAL_UPRISING_ANNIVERSARY,
    19236_i32 => _CONSTITUTION_DAY,
    19250_i32 => _DAY_OF_OUR_LADY_OF_THE_SEVEN_SORROWS,
    19297_i32 => _ALL_SAINTS__DAY,
    19313_i32 => _STRUGGLE_FOR_FREEDOM_AND_DEMOCRACY_DAY,
    19350_i32 => _CHRISTMAS_EVE,
    19351_i32 => _CHRISTMAS_DAY,
    19352_i32 => _SECOND_DAY_OF_CHRISTMAS,
    19358_i32 => _DAY_OF_THE_ESTABLISHMENT_OF_THE_SLOVAK_REPUBLIC,
    19363_i32 => _EPIPHANY__THREE_KINGS__DAY_AND_ORTHODOX_CHRISTMAS_,
    19454_i32 => _GOOD_FRIDAY,
    19457_i32 => _EASTER_MONDAY,
    19478_i32 => _LABOR_DAY,
    19485_i32 => _DAY_OF_VICTORY_OVER_FASCISM,
    19543_i32 => _SAINTS_CYRIL_AND_METHODIUS_DAY,
    19598_i32 => _SLOVAK_NATIONAL_UPRISING_ANNIVERSARY,
    19601_i32 => _CONSTITUTION_DAY,
    19615_i32 => _DAY_OF_OUR_LADY_OF_THE_SEVEN_SORROWS,
    19662_i32 => _ALL_SAINTS__DAY,
    19678_i32 => _STRUGGLE_FOR_FREEDOM_AND_DEMOCRACY_DAY,
    19715_i32 => _CHRISTMAS_EVE,
    19716_i32 => _CHRISTMAS_DAY,
    19717_i32 => _SECOND_DAY_OF_CHRISTMAS,
    19723_i32 => _DAY_OF_THE_ESTABLISHMENT_OF_THE_SLOVAK_REPUBLIC,
    19728_i32 => _EPIPHANY__THREE_KINGS__DAY_AND_ORTHODOX_CHRISTMAS_,
    19811_i32 => _GOOD_FRIDAY,
    19814_i32 => _EASTER_MONDAY,
    19844_i32 => _LABOR_DAY,
    19851_i32 => _DAY_OF_VICTORY_OVER_FASCISM,
    19909_i32 => _SAINTS_CYRIL_AND_METHODIUS_DAY,
    19964_i32 => _SLOVAK_NATIONAL_UPRISING_ANNIVERSARY,
    19967_i32 => _CONSTITUTION_DAY,
    19981_i32 => _DAY_OF_OUR_LADY_OF_THE_SEVEN_SORROWS,
    20028_i32 => _ALL_SAINTS__DAY,
    20044_i32 => _STRUGGLE_FOR_FREEDOM_AND_DEMOCRACY_DAY,
    20081_i32 => _CHRISTMAS_EVE,
    20082_i32 => _CHRISTMAS_DAY,
    20083_i32 => _SECOND_DAY_OF_CHRISTMAS,
    20089_i32 => _DAY_OF_THE_ESTABLISHMENT_OF_THE_SLOVAK_REPUBLIC,
    20094_i32 => _EPIPHANY__THREE_KINGS__DAY_AND_ORTHODOX_CHRISTMAS_,
    20196_i32 => _GOOD_FRIDAY,
    20199_i32 => _EASTER_MONDAY,
    20209_i32 => _LABOR_DAY,
    20216_i32 => _DAY_OF_VICTORY_OVER_FASCISM,
    20274_i32 => _SAINTS_CYRIL_AND_METHODIUS_DAY,
    20329_i32 => _SLOVAK_NATIONAL_UPRISING_ANNIVERSARY,
    20332_i32 => _CONSTITUTION_DAY,
    20346_i32 => _DAY_OF_OUR_LADY_OF_THE_SEVEN_SORROWS,
    20393_i32 => _ALL_SAINTS__DAY,
    20409_i32 => _STRUGGLE_FOR_FREEDOM_AND_DEMOCRACY_DAY,
    20446_i32 => _CHRISTMAS_EVE,
    20447_i32 => _CHRISTMAS_DAY,
    20448_i32 => _SECOND_DAY_OF_CHRISTMAS,
    20454_i32 => _DAY_OF_THE_ESTABLISHMENT_OF_THE_SLOVAK_REPUBLIC,
    20459_i32 => _EPIPHANY__THREE_KINGS__DAY_AND_ORTHODOX_CHRISTMAS_,
    20546_i32 => _GOOD_FRIDAY,
    20549_i32 => _EASTER_MONDAY,
    20574_i32 => _LABOR_DAY,
    20581_i32 => _DAY_OF_VICTORY_OVER_FASCISM,
    20639_i32 => _SAINTS_CYRIL_AND_METHODIUS_DAY,
    20694_i32 => _SLOVAK_NATIONAL_UPRISING_ANNIVERSARY,
    20697_i32 => _CONSTITUTION_DAY,
    20711_i32 => _DAY_OF_OUR_LADY_OF_THE_SEVEN_SORROWS,
    20758_i32 => _ALL_SAINTS__DAY,
    20774_i32 => _STRUGGLE_FOR_FREEDOM_AND_DEMOCRACY_DAY,
    20811_i32 => _CHRISTMAS_EVE,
    20812_i32 => _CHRISTMAS_DAY,
    20813_i32 => _SECOND_DAY_OF_CHRISTMAS,
    20819_i32 => _DAY_OF_THE_ESTABLISHMENT_OF_THE_SLOVAK_REPUBLIC,
    20824_i32 => _EPIPHANY__THREE_KINGS__DAY_AND_ORTHODOX_CHRISTMAS_,
    20903_i32 => _GOOD_FRIDAY,
    20906_i32 => _EASTER_MONDAY,
    20939_i32 => _LABOR_DAY,
    20946_i32 => _DAY_OF_VICTORY_OVER_FASCISM,
    21004_i32 => _SAINTS_CYRIL_AND_METHODIUS_DAY,
    21059_i32 => _SLOVAK_NATIONAL_UPRISING_ANNIVERSARY,
    21062_i32 => _CONSTITUTION_DAY,
    21076_i32 => _DAY_OF_OUR_LADY_OF_THE_SEVEN_SORROWS,
    21123_i32 => _ALL_SAINTS__DAY,
    21139_i32 => _STRUGGLE_FOR_FREEDOM_AND_DEMOCRACY_DAY,
    21176_i32 => _CHRISTMAS_EVE,
    21177_i32 => _CHRISTMAS_DAY,
    21178_i32 => _SECOND_DAY_OF_CHRISTMAS,
    21184_i32 => _DAY_OF_THE_ESTABLISHMENT_OF_THE_SLOVAK_REPUBLIC,
    21189_i32 => _EPIPHANY__THREE_KINGS__DAY_AND_ORTHODOX_CHRISTMAS_,
    21288_i32 => _GOOD_FRIDAY,
    21291_i32 => _EASTER_MONDAY,
    21305_i32 => _LABOR_DAY,
    21312_i32 => _DAY_OF_VICTORY_OVER_FASCISM,
    21370_i32 => _SAINTS_CYRIL_AND_METHODIUS_DAY,
    21425_i32 => _SLOVAK_NATIONAL_UPRISING_ANNIVERSARY,
    21428_i32 => _CONSTITUTION_DAY,
    21442_i32 => _DAY_OF_OUR_LADY_OF_THE_SEVEN_SORROWS,
    21489_i32 => _ALL_SAINTS__DAY,
    21505_i32 => _STRUGGLE_FOR_FREEDOM_AND_DEMOCRACY_DAY,
    21542_i32 => _CHRISTMAS_EVE,
    21543_i32 => _CHRISTMAS_DAY,
    21544_i32 => _SECOND_DAY_OF_CHRISTMAS,
    21550_i32 => _DAY_OF_THE_ESTABLISHMENT_OF_THE_SLOVAK_REPUBLIC,
    21555_i32 => _EPIPHANY__THREE_KINGS__DAY_AND_ORTHODOX_CHRISTMAS_,
    21638_i32 => _GOOD_FRIDAY,
    21641_i32 => _EASTER_MONDAY,
    21670_i32 => _LABOR_DAY,
    21677_i32 => _DAY_OF_VICTORY_OVER_FASCISM,
    21735_i32 => _SAINTS_CYRIL_AND_METHODIUS_DAY,
    21790_i32 => _SLOVAK_NATIONAL_UPRISING_ANNIVERSARY,
    21793_i32 => _CONSTITUTION_DAY,
    21807_i32 => _DAY_OF_OUR_LADY_OF_THE_SEVEN_SORROWS,
    21854_i32 => _ALL_SAINTS__DAY,
    21870_i32 => _STRUGGLE_FOR_FREEDOM_AND_DEMOCRACY_DAY,
    21907_i32 => _CHRISTMAS_EVE,
    21908_i32 => _CHRISTMAS_DAY,
    21909_i32 => _SECOND_DAY_OF_CHRISTMAS,
    21915_i32 => _DAY_OF_THE_ESTABLISHMENT_OF_THE_SLOVAK_REPUBLIC,
    21920_i32 => _EPIPHANY__THREE_KINGS__DAY_AND_ORTHODOX_CHRISTMAS_,
    22023_i32 => _GOOD_FRIDAY,
    22026_i32 => _EASTER_MONDAY,
    22035_i32 => _LABOR_DAY,
    22042_i32 => _DAY_OF_VICTORY_OVER_FASCISM,
    22100_i32 => _SAINTS_CYRIL_AND_METHODIUS_DAY,
    22155_i32 => _SLOVAK_NATIONAL_UPRISING_ANNIVERSARY,
    22158_i32 => _CONSTITUTION_DAY,
    22172_i32 => _DAY_OF_OUR_LADY_OF_THE_SEVEN_SORROWS,
    22219_i32 => _ALL_SAINTS__DAY,
    22235_i32 => _STRUGGLE_FOR_FREEDOM_AND_DEMOCRACY_DAY,
    22272_i32 => _CHRISTMAS_EVE,
    22273_i32 => _CHRISTMAS_DAY,
    22274_i32 => _SECOND_DAY_OF_CHRISTMAS,
    22280_i32 => _DAY_OF_THE_ESTABLISHMENT_OF_THE_SLOVAK_REPUBLIC,
    22285_i32 => _EPIPHANY__THREE_KINGS__DAY_AND_ORTHODOX_CHRISTMAS_,
    22380_i32 => _GOOD_FRIDAY,
    22383_i32 => _EASTER_MONDAY,
    22400_i32 => _LABOR_DAY,
    22407_i32 => _DAY_OF_VICTORY_OVER_FASCISM,
    22465_i32 => _SAINTS_CYRIL_AND_METHODIUS_DAY,
    22520_i32 => _SLOVAK_NATIONAL_UPRISING_ANNIVERSARY,
    22523_i32 => _CONSTITUTION_DAY,
    22537_i32 => _DAY_OF_OUR_LADY_OF_THE_SEVEN_SORROWS,
    22584_i32 => _ALL_SAINTS__DAY,
    22600_i32 => _STRUGGLE_FOR_FREEDOM_AND_DEMOCRACY_DAY,
    22637_i32 => _CHRISTMAS_EVE,
    22638_i32 => _CHRISTMAS_DAY,
    22639_i32 => _SECOND_DAY_OF_CHRISTMAS,
    22645_i32 => _DAY_OF_THE_ESTABLISHMENT_OF_THE_SLOVAK_REPUBLIC,
    22650_i32 => _EPIPHANY__THREE_KINGS__DAY_AND_ORTHODOX_CHRISTMAS_,
    22730_i32 => _GOOD_FRIDAY,
    22733_i32 => _EASTER_MONDAY,
    22766_i32 => _LABOR_DAY,
    22773_i32 => _DAY_OF_VICTORY_OVER_FASCISM,
    22831_i32 => _SAINTS_CYRIL_AND_METHODIUS_DAY,
    22886_i32 => _SLOVAK_NATIONAL_UPRISING_ANNIVERSARY,
    22889_i32 => _CONSTITUTION_DAY,
    22903_i32 => _DAY_OF_OUR_LADY_OF_THE_SEVEN_SORROWS,
    22950_i32 => _ALL_SAINTS__DAY,
    22966_i32 => _STRUGGLE_FOR_FREEDOM_AND_DEMOCRACY_DAY,
    23003_i32 => _CHRISTMAS_EVE,
    23004_i32 => _CHRISTMAS_DAY,
    23005_i32 => _SECOND_DAY_OF_CHRISTMAS,
    23011_i32 => _DAY_OF_THE_ESTABLISHMENT_OF_THE_SLOVAK_REPUBLIC,
    23016_i32 => _EPIPHANY__THREE_KINGS__DAY_AND_ORTHODOX_CHRISTMAS_,
    23115_i32 => _GOOD_FRIDAY,
    23118_i32 => _EASTER_MONDAY,
    23131_i32 => _LABOR_DAY,
    23138_i32 => _DAY_OF_VICTORY_OVER_FASCISM,
    23196_i32 => _SAINTS_CYRIL_AND_METHODIUS_DAY,
    23251_i32 => _SLOVAK_NATIONAL_UPRISING_ANNIVERSARY,
    23254_i32 => _CONSTITUTION_DAY,
    23268_i32 => _DAY_OF_OUR_LADY_OF_THE_SEVEN_SORROWS,
    23315_i32 => _ALL_SAINTS__DAY,
    23331_i32 => _STRUGGLE_FOR_FREEDOM_AND_DEMOCRACY_DAY,
    23368_i32 => _CHRISTMAS_EVE,
    23369_i32 => _CHRISTMAS_DAY,
    23370_i32 => _SECOND_DAY_OF_CHRISTMAS,
    23376_i32 => _DAY_OF_THE_ESTABLISHMENT_OF_THE_SLOVAK_REPUBLIC,
    23381_i32 => _EPIPHANY__THREE_KINGS__DAY_AND_ORTHODOX_CHRISTMAS_,
    23472_i32 => _GOOD_FRIDAY,
    23475_i32 => _EASTER_MONDAY,
    23496_i32 => _LABOR_DAY,
    23503_i32 => _DAY_OF_VICTORY_OVER_FASCISM,
    23561_i32 => _SAINTS_CYRIL_AND_METHODIUS_DAY,
    23616_i32 => _SLOVAK_NATIONAL_UPRISING_ANNIVERSARY,
    23619_i32 => _CONSTITUTION_DAY,
    23633_i32 => _DAY_OF_OUR_LADY_OF_THE_SEVEN_SORROWS,
    23680_i32 => _ALL_SAINTS__DAY,
    23696_i32 => _STRUGGLE_FOR_FREEDOM_AND_DEMOCRACY_DAY,
    23733_i32 => _CHRISTMAS_EVE,
    23734_i32 => _CHRISTMAS_DAY,
    23735_i32 => _SECOND_DAY_OF_CHRISTMAS,
    23741_i32 => _DAY_OF_THE_ESTABLISHMENT_OF_THE_SLOVAK_REPUBLIC,
    23746_i32 => _EPIPHANY__THREE_KINGS__DAY_AND_ORTHODOX_CHRISTMAS_,
    23822_i32 => _GOOD_FRIDAY,
    23825_i32 => _EASTER_MONDAY,
    23861_i32 => _LABOR_DAY,
    23868_i32 => _DAY_OF_VICTORY_OVER_FASCISM,
    23926_i32 => _SAINTS_CYRIL_AND_METHODIUS_DAY,
    23981_i32 => _SLOVAK_NATIONAL_UPRISING_ANNIVERSARY,
    23984_i32 => _CONSTITUTION_DAY,
    23998_i32 => _DAY_OF_OUR_LADY_OF_THE_SEVEN_SORROWS,
    24045_i32 => _ALL_SAINTS__DAY,
    24061_i32 => _STRUGGLE_FOR_FREEDOM_AND_DEMOCRACY_DAY,
    24098_i32 => _CHRISTMAS_EVE,
    24099_i32 => _CHRISTMAS_DAY,
    24100_i32 => _SECOND_DAY_OF_CHRISTMAS,
    24106_i32 => _DAY_OF_THE_ESTABLISHMENT_OF_THE_SLOVAK_REPUBLIC,
    24111_i32 => _EPIPHANY__THREE_KINGS__DAY_AND_ORTHODOX_CHRISTMAS_,
    24207_i32 => _GOOD_FRIDAY,
    24210_i32 => _EASTER_MONDAY,
    24227_i32 => _LABOR_DAY,
    24234_i32 => _DAY_OF_VICTORY_OVER_FASCISM,
    24292_i32 => _SAINTS_CYRIL_AND_METHODIUS_DAY,
    24347_i32 => _SLOVAK_NATIONAL_UPRISING_ANNIVERSARY,
    24350_i32 => _CONSTITUTION_DAY,
    24364_i32 => _DAY_OF_OUR_LADY_OF_THE_SEVEN_SORROWS,
    24411_i32 => _ALL_SAINTS__DAY,
    24427_i32 => _STRUGGLE_FOR_FREEDOM_AND_DEMOCRACY_DAY,
    24464_i32 => _CHRISTMAS_EVE,
    24465_i32 => _CHRISTMAS_DAY,
    24466_i32 => _SECOND_DAY_OF_CHRISTMAS,
    24472_i32 => _DAY_OF_THE_ESTABLISHMENT_OF_THE_SLOVAK_REPUBLIC,
    24477_i32 => _EPIPHANY__THREE_KINGS__DAY_AND_ORTHODOX_CHRISTMAS_,
    24564_i32 => _GOOD_FRIDAY,
    24567_i32 => _EASTER_MONDAY,
    24592_i32 => _LABOR_DAY,
    24599_i32 => _DAY_OF_VICTORY_OVER_FASCISM,
    24657_i32 => _SAINTS_CYRIL_AND_METHODIUS_DAY,
    24712_i32 => _SLOVAK_NATIONAL_UPRISING_ANNIVERSARY,
    24715_i32 => _CONSTITUTION_DAY,
    24729_i32 => _DAY_OF_OUR_LADY_OF_THE_SEVEN_SORROWS,
    24776_i32 => _ALL_SAINTS__DAY,
    24792_i32 => _STRUGGLE_FOR_FREEDOM_AND_DEMOCRACY_DAY,
    24829_i32 => _CHRISTMAS_EVE,
    24830_i32 => _CHRISTMAS_DAY,
    24831_i32 => _SECOND_DAY_OF_CHRISTMAS,
    24837_i32 => _DAY_OF_THE_ESTABLISHMENT_OF_THE_SLOVAK_REPUBLIC,
    24842_i32 => _EPIPHANY__THREE_KINGS__DAY_AND_ORTHODOX_CHRISTMAS_,
    24949_i32 => _GOOD_FRIDAY,
    24952_i32 => _EASTER_MONDAY,
    24957_i32 => _LABOR_DAY,
    24964_i32 => _DAY_OF_VICTORY_OVER_FASCISM,
    25022_i32 => _SAINTS_CYRIL_AND_METHODIUS_DAY,
    25077_i32 => _SLOVAK_NATIONAL_UPRISING_ANNIVERSARY,
    25080_i32 => _CONSTITUTION_DAY,
    25094_i32 => _DAY_OF_OUR_LADY_OF_THE_SEVEN_SORROWS,
    25141_i32 => _ALL_SAINTS__DAY,
    25157_i32 => _STRUGGLE_FOR_FREEDOM_AND_DEMOCRACY_DAY,
    25194_i32 => _CHRISTMAS_EVE,
    25195_i32 => _CHRISTMAS_DAY,
    25196_i32 => _SECOND_DAY_OF_CHRISTMAS,
    25202_i32 => _DAY_OF_THE_ESTABLISHMENT_OF_THE_SLOVAK_REPUBLIC,
    25207_i32 => _EPIPHANY__THREE_KINGS__DAY_AND_ORTHODOX_CHRISTMAS_,
    25299_i32 => _GOOD_FRIDAY,
    25302_i32 => _EASTER_MONDAY,
    25322_i32 => _LABOR_DAY,
    25329_i32 => _DAY_OF_VICTORY_OVER_FASCISM,
    25387_i32 => _SAINTS_CYRIL_AND_METHODIUS_DAY,
    25442_i32 => _SLOVAK_NATIONAL_UPRISING_ANNIVERSARY,
    25445_i32 => _CONSTITUTION_DAY,
    25459_i32 => _DAY_OF_OUR_LADY_OF_THE_SEVEN_SORROWS,
    25506_i32 => _ALL_SAINTS__DAY,
    25522_i32 => _STRUGGLE_FOR_FREEDOM_AND_DEMOCRACY_DAY,
    25559_i32 => _CHRISTMAS_EVE,
    25560_i32 => _CHRISTMAS_DAY,
    25561_i32 => _SECOND_DAY_OF_CHRISTMAS,
    25567_i32 => _DAY_OF_THE_ESTABLISHMENT_OF_THE_SLOVAK_REPUBLIC,
    25572_i32 => _EPIPHANY__THREE_KINGS__DAY_AND_ORTHODOX_CHRISTMAS_,
    25656_i32 => _GOOD_FRIDAY,
    25659_i32 => _EASTER_MONDAY,
    25688_i32 => _LABOR_DAY,
    25695_i32 => _DAY_OF_VICTORY_OVER_FASCISM,
    25753_i32 => _SAINTS_CYRIL_AND_METHODIUS_DAY,
    25808_i32 => _SLOVAK_NATIONAL_UPRISING_ANNIVERSARY,
    25811_i32 => _CONSTITUTION_DAY,
    25825_i32 => _DAY_OF_OUR_LADY_OF_THE_SEVEN_SORROWS,
    25872_i32 => _ALL_SAINTS__DAY,
    25888_i32 => _STRUGGLE_FOR_FREEDOM_AND_DEMOCRACY_DAY,
    25925_i32 => _CHRISTMAS_EVE,
    25926_i32 => _CHRISTMAS_DAY,
    25927_i32 => _SECOND_DAY_OF_CHRISTMAS,
    25933_i32 => _DAY_OF_THE_ESTABLISHMENT_OF_THE_SLOVAK_REPUBLIC,
    25938_i32 => _EPIPHANY__THREE_KINGS__DAY_AND_ORTHODOX_CHRISTMAS_,
    26041_i32 => _GOOD_FRIDAY,
    26044_i32 => _EASTER_MONDAY,
    26053_i32 => _LABOR_DAY,
    26060_i32 => _DAY_OF_VICTORY_OVER_FASCISM,
    26118_i32 => _SAINTS_CYRIL_AND_METHODIUS_DAY,
    26173_i32 => _SLOVAK_NATIONAL_UPRISING_ANNIVERSARY,
    26176_i32 => _CONSTITUTION_DAY,
    26190_i32 => _DAY_OF_OUR_LADY_OF_THE_SEVEN_SORROWS,
    26237_i32 => _ALL_SAINTS__DAY,
    26253_i32 => _STRUGGLE_FOR_FREEDOM_AND_DEMOCRACY_DAY,
    26290_i32 => _CHRISTMAS_EVE,
    26291_i32 => _CHRISTMAS_DAY,
    26292_i32 => _SECOND_DAY_OF_CHRISTMAS,
    26298_i32 => _DAY_OF_THE_ESTABLISHMENT_OF_THE_SLOVAK_REPUBLIC,
    26303_i32 => _EPIPHANY__THREE_KINGS__DAY_AND_ORTHODOX_CHRISTMAS_,
    26391_i32 => _GOOD_FRIDAY,
    26394_i32 => _EASTER_MONDAY,
    26418_i32 => _LABOR_DAY,
    26425_i32 => _DAY_OF_VICTORY_OVER_FASCISM,
    26483_i32 => _SAINTS_CYRIL_AND_METHODIUS_DAY,
    26538_i32 => _SLOVAK_NATIONAL_UPRISING_ANNIVERSARY,
    26541_i32 => _CONSTITUTION_DAY,
    26555_i32 => _DAY_OF_OUR_LADY_OF_THE_SEVEN_SORROWS,
    26602_i32 => _ALL_SAINTS__DAY,
    26618_i32 => _STRUGGLE_FOR_FREEDOM_AND_DEMOCRACY_DAY,
    26655_i32 => _CHRISTMAS_EVE,
    26656_i32 => _CHRISTMAS_DAY,
    26657_i32 => _SECOND_DAY_OF_CHRISTMAS,
    26663_i32 => _DAY_OF_THE_ESTABLISHMENT_OF_THE_SLOVAK_REPUBLIC,
    26668_i32 => _EPIPHANY__THREE_KINGS__DAY_AND_ORTHODOX_CHRISTMAS_,
    26748_i32 => _GOOD_FRIDAY,
    26751_i32 => _EASTER_MONDAY,
    26783_i32 => _LABOR_DAY,
    26790_i32 => _DAY_OF_VICTORY_OVER_FASCISM,
    26848_i32 => _SAINTS_CYRIL_AND_METHODIUS_DAY,
    26903_i32 => _SLOVAK_NATIONAL_UPRISING_ANNIVERSARY,
    26906_i32 => _CONSTITUTION_DAY,
    26920_i32 => _DAY_OF_OUR_LADY_OF_THE_SEVEN_SORROWS,
    26967_i32 => _ALL_SAINTS__DAY,
    26983_i32 => _STRUGGLE_FOR_FREEDOM_AND_DEMOCRACY_DAY,
    27020_i32 => _CHRISTMAS_EVE,
    27021_i32 => _CHRISTMAS_DAY,
    27022_i32 => _SECOND_DAY_OF_CHRISTMAS,
    27028_i32 => _DAY_OF_THE_ESTABLISHMENT_OF_THE_SLOVAK_REPUBLIC,
    27033_i32 => _EPIPHANY__THREE_KINGS__DAY_AND_ORTHODOX_CHRISTMAS_,
    27133_i32 => _GOOD_FRIDAY,
    27136_i32 => _EASTER_MONDAY,
    27149_i32 => _LABOR_DAY,
    27156_i32 => _DAY_OF_VICTORY_OVER_FASCISM,
    27214_i32 => _SAINTS_CYRIL_AND_METHODIUS_DAY,
    27269_i32 => _SLOVAK_NATIONAL_UPRISING_ANNIVERSARY,
    27272_i32 => _CONSTITUTION_DAY,
    27286_i32 => _DAY_OF_OUR_LADY_OF_THE_SEVEN_SORROWS,
    27333_i32 => _ALL_SAINTS__DAY,
    27349_i32 => _STRUGGLE_FOR_FREEDOM_AND_DEMOCRACY_DAY,
    27386_i32 => _CHRISTMAS_EVE,
    27387_i32 => _CHRISTMAS_DAY,
    27388_i32 => _SECOND_DAY_OF_CHRISTMAS,
    27394_i32 => _DAY_OF_THE_ESTABLISHMENT_OF_THE_SLOVAK_REPUBLIC,
    27399_i32 => _EPIPHANY__THREE_KINGS__DAY_AND_ORTHODOX_CHRISTMAS_,
    27490_i32 => _GOOD_FRIDAY,
    27493_i32 => _EASTER_MONDAY,
    27514_i32 => _LABOR_DAY,
    27521_i32 => _DAY_OF_VICTORY_OVER_FASCISM,
    27579_i32 => _SAINTS_CYRIL_AND_METHODIUS_DAY,
    27634_i32 => _SLOVAK_NATIONAL_UPRISING_ANNIVERSARY,
    27637_i32 => _CONSTITUTION_DAY,
    27651_i32 => _DAY_OF_OUR_LADY_OF_THE_SEVEN_SORROWS,
    27698_i32 => _ALL_SAINTS__DAY,
    27714_i32 => _STRUGGLE_FOR_FREEDOM_AND_DEMOCRACY_DAY,
    27751_i32 => _CHRISTMAS_EVE,
    27752_i32 => _CHRISTMAS_DAY,
    27753_i32 => _SECOND_DAY_OF_CHRISTMAS,
    27759_i32 => _DAY_OF_THE_ESTABLISHMENT_OF_THE_SLOVAK_REPUBLIC,
    27764_i32 => _EPIPHANY__THREE_KINGS__DAY_AND_ORTHODOX_CHRISTMAS_,
    27840_i32 => _GOOD_FRIDAY,
    27843_i32 => _EASTER_MONDAY,
    27879_i32 => _LABOR_DAY,
    27886_i32 => _DAY_OF_VICTORY_OVER_FASCISM,
    27944_i32 => _SAINTS_CYRIL_AND_METHODIUS_DAY,
    27999_i32 => _SLOVAK_NATIONAL_UPRISING_ANNIVERSARY,
    28002_i32 => _CONSTITUTION_DAY,
    28016_i32 => _DAY_OF_OUR_LADY_OF_THE_SEVEN_SORROWS,
    28063_i32 => _ALL_SAINTS__DAY,
    28079_i32 => _STRUGGLE_FOR_FREEDOM_AND_DEMOCRACY_DAY,
    28116_i32 => _CHRISTMAS_EVE,
    28117_i32 => _CHRISTMAS_DAY,
    28118_i32 => _SECOND_DAY_OF_CHRISTMAS,
    28124_i32 => _DAY_OF_THE_ESTABLISHMENT_OF_THE_SLOVAK_REPUBLIC,
    28129_i32 => _EPIPHANY__THREE_KINGS__DAY_AND_ORTHODOX_CHRISTMAS_,
    28225_i32 => _GOOD_FRIDAY,
    28228_i32 => _EASTER_MONDAY,
    28244_i32 => _LABOR_DAY,
    28251_i32 => _DAY_OF_VICTORY_OVER_FASCISM,
    28309_i32 => _SAINTS_CYRIL_AND_METHODIUS_DAY,
    28364_i32 => _SLOVAK_NATIONAL_UPRISING_ANNIVERSARY,
    28367_i32 => _CONSTITUTION_DAY,
    28381_i32 => _DAY_OF_OUR_LADY_OF_THE_SEVEN_SORROWS,
    28428_i32 => _ALL_SAINTS__DAY,
    28444_i32 => _STRUGGLE_FOR_FREEDOM_AND_DEMOCRACY_DAY,
    28481_i32 => _CHRISTMAS_EVE,
    28482_i32 => _CHRISTMAS_DAY,
    28483_i32 => _SECOND_DAY_OF_CHRISTMAS,
    28489_i32 => _DAY_OF_THE_ESTABLISHMENT_OF_THE_SLOVAK_REPUBLIC,
    28494_i32 => _EPIPHANY__THREE_KINGS__DAY_AND_ORTHODOX_CHRISTMAS_,
    28582_i32 => _GOOD_FRIDAY,
    28585_i32 => _EASTER_MONDAY,
    28610_i32 => _LABOR_DAY,
    28617_i32 => _DAY_OF_VICTORY_OVER_FASCISM,
    28675_i32 => _SAINTS_CYRIL_AND_METHODIUS_DAY,
    28730_i32 => _SLOVAK_NATIONAL_UPRISING_ANNIVERSARY,
    28733_i32 => _CONSTITUTION_DAY,
    28747_i32 => _DAY_OF_OUR_LADY_OF_THE_SEVEN_SORROWS,
    28794_i32 => _ALL_SAINTS__DAY,
    28810_i32 => _STRUGGLE_FOR_FREEDOM_AND_DEMOCRACY_DAY,
    28847_i32 => _CHRISTMAS_EVE,
    28848_i32 => _CHRISTMAS_DAY,
    28849_i32 => _SECOND_DAY_OF_CHRISTMAS,
    28855_i32 => _DAY_OF_THE_ESTABLISHMENT_OF_THE_SLOVAK_REPUBLIC,
    28860_i32 => _EPIPHANY__THREE_KINGS__DAY_AND_ORTHODOX_CHRISTMAS_,
    28960_i32 => _GOOD_FRIDAY,
    28963_i32 => _EASTER_MONDAY,
    28975_i32 => _LABOR_DAY,
    28982_i32 => _DAY_OF_VICTORY_OVER_FASCISM,
    29040_i32 => _SAINTS_CYRIL_AND_METHODIUS_DAY,
    29095_i32 => _SLOVAK_NATIONAL_UPRISING_ANNIVERSARY,
    29098_i32 => _CONSTITUTION_DAY,
    29112_i32 => _DAY_OF_OUR_LADY_OF_THE_SEVEN_SORROWS,
    29159_i32 => _ALL_SAINTS__DAY,
    29175_i32 => _STRUGGLE_FOR_FREEDOM_AND_DEMOCRACY_DAY,
    29212_i32 => _CHRISTMAS_EVE,
    29213_i32 => _CHRISTMAS_DAY,
    29214_i32 => _SECOND_DAY_OF_CHRISTMAS,
    29220_i32 => _DAY_OF_THE_ESTABLISHMENT_OF_THE_SLOVAK_REPUBLIC,
};
