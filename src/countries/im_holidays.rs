use crate::countries::constants::*;
use phf::phf_map;

pub static IM_HOLIDAYS: phf::Map<i32, &'static str> = phf_map! {
    10957_i32 => _NEW_YEAR_S_DAY,
    10959_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    11068_i32 => _GOOD_FRIDAY,
    11071_i32 => _EASTER_MONDAY,
    11078_i32 => _MAY_DAY,
    11106_i32 => _SPRING_BANK_HOLIDAY,
    11110_i32 => _TT_BANK_HOLIDAY,
    11143_i32 => _TYNWALD_DAY,
    11197_i32 => _LATE_SUMMER_BANK_HOLIDAY,
    11316_i32 => _CHRISTMAS_DAY,
    11317_i32 => _BOXING_DAY,
    11323_i32 => _NEW_YEAR_S_DAY,
    11425_i32 => _GOOD_FRIDAY,
    11428_i32 => _EASTER_MONDAY,
    11449_i32 => _MAY_DAY,
    11470_i32 => _SPRING_BANK_HOLIDAY,
    11474_i32 => _TT_BANK_HOLIDAY,
    11508_i32 => _TYNWALD_DAY,
    11561_i32 => _LATE_SUMMER_BANK_HOLIDAY,
    11681_i32 => _CHRISTMAS_DAY,
    11682_i32 => _BOXING_DAY,
    11688_i32 => _NEW_YEAR_S_DAY,
    11775_i32 => _GOOD_FRIDAY,
    11778_i32 => _EASTER_MONDAY,
    11813_i32 => _MAY_DAY,
    11841_i32 => _GOLDEN_JUBILEE_OF_ELIZABETH_II,
    11842_i32 => _SPRING_BANK_HOLIDAY,
    11845_i32 => _TT_BANK_HOLIDAY,
    11873_i32 => _TYNWALD_DAY,
    11925_i32 => _LATE_SUMMER_BANK_HOLIDAY,
    12046_i32 => _CHRISTMAS_DAY,
    12047_i32 => _BOXING_DAY,
    12053_i32 => _NEW_YEAR_S_DAY,
    12160_i32 => _GOOD_FRIDAY,
    12163_i32 => _EASTER_MONDAY,
    12177_i32 => _MAY_DAY,
    12198_i32 => _SPRING_BANK_HOLIDAY,
    12209_i32 => _TT_BANK_HOLIDAY,
    12240_i32 => _TYNWALD_DAY,
    12289_i32 => _LATE_SUMMER_BANK_HOLIDAY,
    12411_i32 => _CHRISTMAS_DAY,
    12412_i32 => _BOXING_DAY,
    12418_i32 => _NEW_YEAR_S_DAY,
    12517_i32 => _GOOD_FRIDAY,
    12520_i32 => _EASTER_MONDAY,
    12541_i32 => _MAY_DAY,
    12569_i32 => _SPRING_BANK_HOLIDAY,
    12573_i32 => _TT_BANK_HOLIDAY,
    12604_i32 => _TYNWALD_DAY,
    12660_i32 => _LATE_SUMMER_BANK_HOLIDAY,
    12777_i32 => _CHRISTMAS_DAY,
    12778_i32 => _BOXING_DAY,
    12779_i32 => _CHRISTMAS_DAY__OBSERVED_,
    12780_i32 => _BOXING_DAY__OBSERVED_,
    12784_i32 => _NEW_YEAR_S_DAY,
    12786_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    12867_i32 => _GOOD_FRIDAY,
    12870_i32 => _EASTER_MONDAY,
    12905_i32 => _MAY_DAY,
    12933_i32 => _SPRING_BANK_HOLIDAY,
    12937_i32 => _TT_BANK_HOLIDAY,
    12969_i32 => _TYNWALD_DAY,
    13024_i32 => _LATE_SUMMER_BANK_HOLIDAY,
    13142_i32 => _CHRISTMAS_DAY,
    13143_i32 => _BOXING_DAY,
    13144_i32 => _CHRISTMAS_DAY__OBSERVED_,
    13149_i32 => _NEW_YEAR_S_DAY,
    13150_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    13252_i32 => _GOOD_FRIDAY,
    13255_i32 => _EASTER_MONDAY,
    13269_i32 => _MAY_DAY,
    13297_i32 => _SPRING_BANK_HOLIDAY,
    13301_i32 => _TT_BANK_HOLIDAY,
    13334_i32 => _TYNWALD_DAY,
    13388_i32 => _LATE_SUMMER_BANK_HOLIDAY,
    13507_i32 => _CHRISTMAS_DAY,
    13508_i32 => _BOXING_DAY,
    13514_i32 => _NEW_YEAR_S_DAY,
    13609_i32 => _GOOD_FRIDAY,
    13612_i32 => _EASTER_MONDAY,
    13640_i32 => _MAY_DAY,
    13661_i32 => _SPRING_BANK_HOLIDAY,
    13665_i32 => _TT_BANK_HOLIDAY,
    13699_i32 => _TYNWALD_DAY,
    13752_i32 => _LATE_SUMMER_BANK_HOLIDAY,
    13872_i32 => _CHRISTMAS_DAY,
    13873_i32 => _BOXING_DAY,
    13879_i32 => _NEW_YEAR_S_DAY,
    13959_i32 => _GOOD_FRIDAY,
    13962_i32 => _EASTER_MONDAY,
    14004_i32 => _MAY_DAY,
    14025_i32 => _SPRING_BANK_HOLIDAY,
    14036_i32 => _TT_BANK_HOLIDAY,
    14067_i32 => _TYNWALD_DAY,
    14116_i32 => _LATE_SUMMER_BANK_HOLIDAY,
    14238_i32 => _CHRISTMAS_DAY,
    14239_i32 => _BOXING_DAY,
    14245_i32 => _NEW_YEAR_S_DAY,
    14344_i32 => _GOOD_FRIDAY,
    14347_i32 => _EASTER_MONDAY,
    14368_i32 => _MAY_DAY,
    14389_i32 => _SPRING_BANK_HOLIDAY,
    14400_i32 => _TT_BANK_HOLIDAY,
    14431_i32 => _TYNWALD_DAY,
    14487_i32 => _LATE_SUMMER_BANK_HOLIDAY,
    14603_i32 => _CHRISTMAS_DAY,
    14604_i32 => _BOXING_DAY,
    14606_i32 => _BOXING_DAY__OBSERVED_,
    14610_i32 => _NEW_YEAR_S_DAY,
    14701_i32 => _GOOD_FRIDAY,
    14704_i32 => _EASTER_MONDAY,
    14732_i32 => _MAY_DAY,
    14760_i32 => _SPRING_BANK_HOLIDAY,
    14764_i32 => _TT_BANK_HOLIDAY,
    14795_i32 => _TYNWALD_DAY,
    14851_i32 => _LATE_SUMMER_BANK_HOLIDAY,
    14968_i32 => _CHRISTMAS_DAY,
    14969_i32 => _BOXING_DAY,
    14970_i32 => _CHRISTMAS_DAY__OBSERVED_,
    14971_i32 => _BOXING_DAY__OBSERVED_,
    14975_i32 => _NEW_YEAR_S_DAY,
    14977_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    15086_i32 => _GOOD_FRIDAY,
    15089_i32 => _EASTER_MONDAY,
    15093_i32 => _WEDDING_OF_WILLIAM_AND_CATHERINE,
    15096_i32 => _MAY_DAY,
    15124_i32 => _SPRING_BANK_HOLIDAY,
    15128_i32 => _TT_BANK_HOLIDAY,
    15160_i32 => _TYNWALD_DAY,
    15215_i32 => _LATE_SUMMER_BANK_HOLIDAY,
    15333_i32 => _CHRISTMAS_DAY,
    15334_i32 => _BOXING_DAY,
    15335_i32 => _CHRISTMAS_DAY__OBSERVED_,
    15340_i32 => _NEW_YEAR_S_DAY,
    15341_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    15436_i32 => _GOOD_FRIDAY,
    15439_i32 => _EASTER_MONDAY,
    15467_i32 => _MAY_DAY,
    15492_i32 => _TT_BANK_HOLIDAY,
    15495_i32 => _SPRING_BANK_HOLIDAY,
    15496_i32 => _DIAMOND_JUBILEE_OF_ELIZABETH_II,
    15526_i32 => _TYNWALD_DAY,
    15579_i32 => _LATE_SUMMER_BANK_HOLIDAY,
    15699_i32 => _CHRISTMAS_DAY,
    15700_i32 => _BOXING_DAY,
    15706_i32 => _NEW_YEAR_S_DAY,
    15793_i32 => _GOOD_FRIDAY,
    15796_i32 => _EASTER_MONDAY,
    15831_i32 => _MAY_DAY,
    15852_i32 => _SPRING_BANK_HOLIDAY,
    15863_i32 => _TT_BANK_HOLIDAY,
    15891_i32 => _TYNWALD_DAY,
    15943_i32 => _LATE_SUMMER_BANK_HOLIDAY,
    16064_i32 => _CHRISTMAS_DAY,
    16065_i32 => _BOXING_DAY,
    16071_i32 => _NEW_YEAR_S_DAY,
    16178_i32 => _GOOD_FRIDAY,
    16181_i32 => _EASTER_MONDAY,
    16195_i32 => _MAY_DAY,
    16216_i32 => _SPRING_BANK_HOLIDAY,
    16227_i32 => _TT_BANK_HOLIDAY,
    16258_i32 => _TYNWALD_DAY,
    16307_i32 => _LATE_SUMMER_BANK_HOLIDAY,
    16429_i32 => _CHRISTMAS_DAY,
    16430_i32 => _BOXING_DAY,
    16436_i32 => _NEW_YEAR_S_DAY,
    16528_i32 => _GOOD_FRIDAY,
    16531_i32 => _EASTER_MONDAY,
    16559_i32 => _MAY_DAY,
    16580_i32 => _SPRING_BANK_HOLIDAY,
    16591_i32 => _TT_BANK_HOLIDAY,
    16622_i32 => _TYNWALD_DAY,
    16678_i32 => _LATE_SUMMER_BANK_HOLIDAY,
    16794_i32 => _CHRISTMAS_DAY,
    16795_i32 => _BOXING_DAY,
    16797_i32 => _BOXING_DAY__OBSERVED_,
    16801_i32 => _NEW_YEAR_S_DAY,
    16885_i32 => _GOOD_FRIDAY,
    16888_i32 => _EASTER_MONDAY,
    16923_i32 => _MAY_DAY,
    16951_i32 => _SPRING_BANK_HOLIDAY,
    16955_i32 => _TT_BANK_HOLIDAY,
    16987_i32 => _TYNWALD_DAY,
    17042_i32 => _LATE_SUMMER_BANK_HOLIDAY,
    17160_i32 => _CHRISTMAS_DAY,
    17161_i32 => _BOXING_DAY,
    17162_i32 => _CHRISTMAS_DAY__OBSERVED_,
    17167_i32 => _NEW_YEAR_S_DAY,
    17168_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    17270_i32 => _GOOD_FRIDAY,
    17273_i32 => _EASTER_MONDAY,
    17287_i32 => _MAY_DAY,
    17315_i32 => _SPRING_BANK_HOLIDAY,
    17319_i32 => _TT_BANK_HOLIDAY,
    17352_i32 => _TYNWALD_DAY,
    17406_i32 => _LATE_SUMMER_BANK_HOLIDAY,
    17525_i32 => _CHRISTMAS_DAY,
    17526_i32 => _BOXING_DAY,
    17532_i32 => _NEW_YEAR_S_DAY,
    17620_i32 => _GOOD_FRIDAY,
    17623_i32 => _EASTER_MONDAY,
    17658_i32 => _MAY_DAY,
    17679_i32 => _SPRING_BANK_HOLIDAY,
    17683_i32 => _TT_BANK_HOLIDAY,
    17717_i32 => _TYNWALD_DAY,
    17770_i32 => _LATE_SUMMER_BANK_HOLIDAY,
    17890_i32 => _CHRISTMAS_DAY,
    17891_i32 => _BOXING_DAY,
    17897_i32 => _NEW_YEAR_S_DAY,
    18005_i32 => _GOOD_FRIDAY,
    18008_i32 => _EASTER_MONDAY,
    18022_i32 => _MAY_DAY,
    18043_i32 => _SPRING_BANK_HOLIDAY,
    18054_i32 => _TT_BANK_HOLIDAY,
    18082_i32 => _TYNWALD_DAY,
    18134_i32 => _LATE_SUMMER_BANK_HOLIDAY,
    18255_i32 => _CHRISTMAS_DAY,
    18256_i32 => _BOXING_DAY,
    18262_i32 => _NEW_YEAR_S_DAY,
    18362_i32 => _GOOD_FRIDAY,
    18365_i32 => _EASTER_MONDAY,
    18390_i32 => _MAY_DAY,
    18407_i32 => _SPRING_BANK_HOLIDAY,
    18418_i32 => _TT_BANK_HOLIDAY,
    18449_i32 => _TYNWALD_DAY,
    18505_i32 => _LATE_SUMMER_BANK_HOLIDAY,
    18621_i32 => _CHRISTMAS_DAY,
    18622_i32 => _BOXING_DAY,
    18624_i32 => _BOXING_DAY__OBSERVED_,
    18628_i32 => _NEW_YEAR_S_DAY,
    18719_i32 => _GOOD_FRIDAY,
    18722_i32 => _EASTER_MONDAY,
    18750_i32 => _MAY_DAY,
    18778_i32 => _SPRING_BANK_HOLIDAY,
    18782_i32 => _TT_BANK_HOLIDAY,
    18813_i32 => _TYNWALD_DAY,
    18869_i32 => _LATE_SUMMER_BANK_HOLIDAY,
    18986_i32 => _CHRISTMAS_DAY,
    18987_i32 => _BOXING_DAY,
    18988_i32 => _CHRISTMAS_DAY__OBSERVED_,
    18989_i32 => _BOXING_DAY__OBSERVED_,
    18993_i32 => _NEW_YEAR_S_DAY,
    18995_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    19097_i32 => _GOOD_FRIDAY,
    19100_i32 => _EASTER_MONDAY,
    19114_i32 => _MAY_DAY,
    19145_i32 => _SPRING_BANK_HOLIDAY,
    19146_i32 => _PLATINUM_JUBILEE_OF_ELIZABETH_II__TT_BANK_HOLIDAY,
    19178_i32 => _TYNWALD_DAY,
    19233_i32 => _LATE_SUMMER_BANK_HOLIDAY,
    19254_i32 => _STATE_FUNERAL_OF_QUEEN_ELIZABETH_II,
    19351_i32 => _CHRISTMAS_DAY,
    19352_i32 => _BOXING_DAY,
    19353_i32 => _CHRISTMAS_DAY__OBSERVED_,
    19358_i32 => _NEW_YEAR_S_DAY,
    19359_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    19454_i32 => _GOOD_FRIDAY,
    19457_i32 => _EASTER_MONDAY,
    19478_i32 => _MAY_DAY,
    19485_i32 => _CORONATION_OF_CHARLES_III,
    19506_i32 => _SPRING_BANK_HOLIDAY,
    19510_i32 => _TT_BANK_HOLIDAY,
    19543_i32 => _TYNWALD_DAY,
    19597_i32 => _LATE_SUMMER_BANK_HOLIDAY,
    19716_i32 => _CHRISTMAS_DAY,
    19717_i32 => _BOXING_DAY,
    19723_i32 => _NEW_YEAR_S_DAY,
    19811_i32 => _GOOD_FRIDAY,
    19814_i32 => _EASTER_MONDAY,
    19849_i32 => _MAY_DAY,
    19870_i32 => _SPRING_BANK_HOLIDAY,
    19881_i32 => _TT_BANK_HOLIDAY,
    19909_i32 => _TYNWALD_DAY,
    19961_i32 => _LATE_SUMMER_BANK_HOLIDAY,
    20082_i32 => _CHRISTMAS_DAY,
    20083_i32 => _BOXING_DAY,
    20089_i32 => _NEW_YEAR_S_DAY,
    20196_i32 => _GOOD_FRIDAY,
    20199_i32 => _EASTER_MONDAY,
    20213_i32 => _MAY_DAY,
    20234_i32 => _SPRING_BANK_HOLIDAY,
    20245_i32 => _TT_BANK_HOLIDAY,
    20276_i32 => _TYNWALD_DAY,
    20325_i32 => _LATE_SUMMER_BANK_HOLIDAY,
    20447_i32 => _CHRISTMAS_DAY,
    20448_i32 => _BOXING_DAY,
    20454_i32 => _NEW_YEAR_S_DAY,
    20546_i32 => _GOOD_FRIDAY,
    20549_i32 => _EASTER_MONDAY,
    20577_i32 => _MAY_DAY,
    20598_i32 => _SPRING_BANK_HOLIDAY,
    20609_i32 => _TT_BANK_HOLIDAY,
    20640_i32 => _TYNWALD_DAY,
    20696_i32 => _LATE_SUMMER_BANK_HOLIDAY,
    20812_i32 => _CHRISTMAS_DAY,
    20813_i32 => _BOXING_DAY,
    20815_i32 => _BOXING_DAY__OBSERVED_,
    20819_i32 => _NEW_YEAR_S_DAY,
    20903_i32 => _GOOD_FRIDAY,
    20906_i32 => _EASTER_MONDAY,
    20941_i32 => _MAY_DAY,
    20969_i32 => _SPRING_BANK_HOLIDAY,
    20973_i32 => _TT_BANK_HOLIDAY,
    21004_i32 => _TYNWALD_DAY,
    21060_i32 => _LATE_SUMMER_BANK_HOLIDAY,
    21177_i32 => _CHRISTMAS_DAY,
    21178_i32 => _BOXING_DAY,
    21179_i32 => _CHRISTMAS_DAY__OBSERVED_,
    21180_i32 => _BOXING_DAY__OBSERVED_,
    21184_i32 => _NEW_YEAR_S_DAY,
    21186_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    21288_i32 => _GOOD_FRIDAY,
    21291_i32 => _EASTER_MONDAY,
    21305_i32 => _MAY_DAY,
    21333_i32 => _SPRING_BANK_HOLIDAY,
    21337_i32 => _TT_BANK_HOLIDAY,
    21370_i32 => _TYNWALD_DAY,
    21424_i32 => _LATE_SUMMER_BANK_HOLIDAY,
    21543_i32 => _CHRISTMAS_DAY,
    21544_i32 => _BOXING_DAY,
    21550_i32 => _NEW_YEAR_S_DAY,
    21638_i32 => _GOOD_FRIDAY,
    21641_i32 => _EASTER_MONDAY,
    21676_i32 => _MAY_DAY,
    21697_i32 => _SPRING_BANK_HOLIDAY,
    21701_i32 => _TT_BANK_HOLIDAY,
    21735_i32 => _TYNWALD_DAY,
    21788_i32 => _LATE_SUMMER_BANK_HOLIDAY,
    21908_i32 => _CHRISTMAS_DAY,
    21909_i32 => _BOXING_DAY,
    21915_i32 => _NEW_YEAR_S_DAY,
    22023_i32 => _GOOD_FRIDAY,
    22026_i32 => _EASTER_MONDAY,
    22040_i32 => _MAY_DAY,
    22061_i32 => _SPRING_BANK_HOLIDAY,
    22072_i32 => _TT_BANK_HOLIDAY,
    22100_i32 => _TYNWALD_DAY,
    22152_i32 => _LATE_SUMMER_BANK_HOLIDAY,
    22273_i32 => _CHRISTMAS_DAY,
    22274_i32 => _BOXING_DAY,
    22280_i32 => _NEW_YEAR_S_DAY,
    22380_i32 => _GOOD_FRIDAY,
    22383_i32 => _EASTER_MONDAY,
    22404_i32 => _MAY_DAY,
    22425_i32 => _SPRING_BANK_HOLIDAY,
    22436_i32 => _TT_BANK_HOLIDAY,
    22467_i32 => _TYNWALD_DAY,
    22516_i32 => _LATE_SUMMER_BANK_HOLIDAY,
    22638_i32 => _CHRISTMAS_DAY,
    22639_i32 => _BOXING_DAY,
    22645_i32 => _NEW_YEAR_S_DAY,
    22730_i32 => _GOOD_FRIDAY,
    22733_i32 => _EASTER_MONDAY,
    22768_i32 => _MAY_DAY,
    22796_i32 => _SPRING_BANK_HOLIDAY,
    22800_i32 => _TT_BANK_HOLIDAY,
    22831_i32 => _TYNWALD_DAY,
    22887_i32 => _LATE_SUMMER_BANK_HOLIDAY,
    23004_i32 => _CHRISTMAS_DAY,
    23005_i32 => _BOXING_DAY,
    23006_i32 => _CHRISTMAS_DAY__OBSERVED_,
    23007_i32 => _BOXING_DAY__OBSERVED_,
    23011_i32 => _NEW_YEAR_S_DAY,
    23013_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    23115_i32 => _GOOD_FRIDAY,
    23118_i32 => _EASTER_MONDAY,
    23132_i32 => _MAY_DAY,
    23160_i32 => _SPRING_BANK_HOLIDAY,
    23164_i32 => _TT_BANK_HOLIDAY,
    23196_i32 => _TYNWALD_DAY,
    23251_i32 => _LATE_SUMMER_BANK_HOLIDAY,
    23369_i32 => _CHRISTMAS_DAY,
    23370_i32 => _BOXING_DAY,
    23371_i32 => _CHRISTMAS_DAY__OBSERVED_,
    23376_i32 => _NEW_YEAR_S_DAY,
    23377_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    23472_i32 => _GOOD_FRIDAY,
    23475_i32 => _EASTER_MONDAY,
    23496_i32 => _MAY_DAY,
    23524_i32 => _SPRING_BANK_HOLIDAY,
    23528_i32 => _TT_BANK_HOLIDAY,
    23561_i32 => _TYNWALD_DAY,
    23615_i32 => _LATE_SUMMER_BANK_HOLIDAY,
    23734_i32 => _CHRISTMAS_DAY,
    23735_i32 => _BOXING_DAY,
    23741_i32 => _NEW_YEAR_S_DAY,
    23822_i32 => _GOOD_FRIDAY,
    23825_i32 => _EASTER_MONDAY,
    23867_i32 => _MAY_DAY,
    23888_i32 => _SPRING_BANK_HOLIDAY,
    23892_i32 => _TT_BANK_HOLIDAY,
    23926_i32 => _TYNWALD_DAY,
    23979_i32 => _LATE_SUMMER_BANK_HOLIDAY,
    24099_i32 => _CHRISTMAS_DAY,
    24100_i32 => _BOXING_DAY,
    24106_i32 => _NEW_YEAR_S_DAY,
    24207_i32 => _GOOD_FRIDAY,
    24210_i32 => _EASTER_MONDAY,
    24231_i32 => _MAY_DAY,
    24252_i32 => _SPRING_BANK_HOLIDAY,
    24263_i32 => _TT_BANK_HOLIDAY,
    24294_i32 => _TYNWALD_DAY,
    24343_i32 => _LATE_SUMMER_BANK_HOLIDAY,
    24465_i32 => _CHRISTMAS_DAY,
    24466_i32 => _BOXING_DAY,
    24472_i32 => _NEW_YEAR_S_DAY,
    24564_i32 => _GOOD_FRIDAY,
    24567_i32 => _EASTER_MONDAY,
    24595_i32 => _MAY_DAY,
    24616_i32 => _SPRING_BANK_HOLIDAY,
    24627_i32 => _TT_BANK_HOLIDAY,
    24658_i32 => _TYNWALD_DAY,
    24714_i32 => _LATE_SUMMER_BANK_HOLIDAY,
    24830_i32 => _CHRISTMAS_DAY,
    24831_i32 => _BOXING_DAY,
    24833_i32 => _BOXING_DAY__OBSERVED_,
    24837_i32 => _NEW_YEAR_S_DAY,
    24949_i32 => _GOOD_FRIDAY,
    24952_i32 => _EASTER_MONDAY,
    24959_i32 => _MAY_DAY,
    24987_i32 => _SPRING_BANK_HOLIDAY,
    24991_i32 => _TT_BANK_HOLIDAY,
    25022_i32 => _TYNWALD_DAY,
    25078_i32 => _LATE_SUMMER_BANK_HOLIDAY,
    25195_i32 => _CHRISTMAS_DAY,
    25196_i32 => _BOXING_DAY,
    25197_i32 => _CHRISTMAS_DAY__OBSERVED_,
    25198_i32 => _BOXING_DAY__OBSERVED_,
    25202_i32 => _NEW_YEAR_S_DAY,
    25204_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    25299_i32 => _GOOD_FRIDAY,
    25302_i32 => _EASTER_MONDAY,
    25323_i32 => _MAY_DAY,
    25351_i32 => _SPRING_BANK_HOLIDAY,
    25355_i32 => _TT_BANK_HOLIDAY,
    25387_i32 => _TYNWALD_DAY,
    25442_i32 => _LATE_SUMMER_BANK_HOLIDAY,
    25560_i32 => _CHRISTMAS_DAY,
    25561_i32 => _BOXING_DAY,
    25562_i32 => _CHRISTMAS_DAY__OBSERVED_,
    25567_i32 => _NEW_YEAR_S_DAY,
    25568_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    25656_i32 => _GOOD_FRIDAY,
    25659_i32 => _EASTER_MONDAY,
    25694_i32 => _MAY_DAY,
    25715_i32 => _SPRING_BANK_HOLIDAY,
    25719_i32 => _TT_BANK_HOLIDAY,
    25753_i32 => _TYNWALD_DAY,
    25806_i32 => _LATE_SUMMER_BANK_HOLIDAY,
    25926_i32 => _CHRISTMAS_DAY,
    25927_i32 => _BOXING_DAY,
    25933_i32 => _NEW_YEAR_S_DAY,
    26041_i32 => _GOOD_FRIDAY,
    26044_i32 => _EASTER_MONDAY,
    26058_i32 => _MAY_DAY,
    26079_i32 => _SPRING_BANK_HOLIDAY,
    26090_i32 => _TT_BANK_HOLIDAY,
    26118_i32 => _TYNWALD_DAY,
    26170_i32 => _LATE_SUMMER_BANK_HOLIDAY,
    26291_i32 => _CHRISTMAS_DAY,
    26292_i32 => _BOXING_DAY,
    26298_i32 => _NEW_YEAR_S_DAY,
    26391_i32 => _GOOD_FRIDAY,
    26394_i32 => _EASTER_MONDAY,
    26422_i32 => _MAY_DAY,
    26443_i32 => _SPRING_BANK_HOLIDAY,
    26454_i32 => _TT_BANK_HOLIDAY,
    26485_i32 => _TYNWALD_DAY,
    26534_i32 => _LATE_SUMMER_BANK_HOLIDAY,
    26656_i32 => _CHRISTMAS_DAY,
    26657_i32 => _BOXING_DAY,
    26663_i32 => _NEW_YEAR_S_DAY,
    26748_i32 => _GOOD_FRIDAY,
    26751_i32 => _EASTER_MONDAY,
    26786_i32 => _MAY_DAY,
    26807_i32 => _SPRING_BANK_HOLIDAY,
    26818_i32 => _TT_BANK_HOLIDAY,
    26849_i32 => _TYNWALD_DAY,
    26905_i32 => _LATE_SUMMER_BANK_HOLIDAY,
    27021_i32 => _CHRISTMAS_DAY,
    27022_i32 => _BOXING_DAY,
    27024_i32 => _BOXING_DAY__OBSERVED_,
    27028_i32 => _NEW_YEAR_S_DAY,
    27133_i32 => _GOOD_FRIDAY,
    27136_i32 => _EASTER_MONDAY,
    27150_i32 => _MAY_DAY,
    27178_i32 => _SPRING_BANK_HOLIDAY,
    27182_i32 => _TT_BANK_HOLIDAY,
    27214_i32 => _TYNWALD_DAY,
    27269_i32 => _LATE_SUMMER_BANK_HOLIDAY,
    27387_i32 => _CHRISTMAS_DAY,
    27388_i32 => _BOXING_DAY,
    27389_i32 => _CHRISTMAS_DAY__OBSERVED_,
    27394_i32 => _NEW_YEAR_S_DAY,
    27395_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    27490_i32 => _GOOD_FRIDAY,
    27493_i32 => _EASTER_MONDAY,
    27514_i32 => _MAY_DAY,
    27542_i32 => _SPRING_BANK_HOLIDAY,
    27546_i32 => _TT_BANK_HOLIDAY,
    27579_i32 => _TYNWALD_DAY,
    27633_i32 => _LATE_SUMMER_BANK_HOLIDAY,
    27752_i32 => _CHRISTMAS_DAY,
    27753_i32 => _BOXING_DAY,
    27759_i32 => _NEW_YEAR_S_DAY,
    27840_i32 => _GOOD_FRIDAY,
    27843_i32 => _EASTER_MONDAY,
    27885_i32 => _MAY_DAY,
    27906_i32 => _SPRING_BANK_HOLIDAY,
    27910_i32 => _TT_BANK_HOLIDAY,
    27944_i32 => _TYNWALD_DAY,
    27997_i32 => _LATE_SUMMER_BANK_HOLIDAY,
    28117_i32 => _CHRISTMAS_DAY,
    28118_i32 => _BOXING_DAY,
    28124_i32 => _NEW_YEAR_S_DAY,
    28225_i32 => _GOOD_FRIDAY,
    28228_i32 => _EASTER_MONDAY,
    28249_i32 => _MAY_DAY,
    28270_i32 => _SPRING_BANK_HOLIDAY,
    28281_i32 => _TT_BANK_HOLIDAY,
    28309_i32 => _TYNWALD_DAY,
    28361_i32 => _LATE_SUMMER_BANK_HOLIDAY,
    28482_i32 => _CHRISTMAS_DAY,
    28483_i32 => _BOXING_DAY,
    28489_i32 => _NEW_YEAR_S_DAY,
    28582_i32 => _GOOD_FRIDAY,
    28585_i32 => _EASTER_MONDAY,
    28613_i32 => _MAY_DAY,
    28634_i32 => _SPRING_BANK_HOLIDAY,
    28645_i32 => _TT_BANK_HOLIDAY,
    28676_i32 => _TYNWALD_DAY,
    28732_i32 => _LATE_SUMMER_BANK_HOLIDAY,
    28848_i32 => _CHRISTMAS_DAY,
    28849_i32 => _BOXING_DAY,
    28851_i32 => _BOXING_DAY__OBSERVED_,
    28855_i32 => _NEW_YEAR_S_DAY,
    28960_i32 => _GOOD_FRIDAY,
    28963_i32 => _EASTER_MONDAY,
    28977_i32 => _MAY_DAY,
    29005_i32 => _SPRING_BANK_HOLIDAY,
    29009_i32 => _TT_BANK_HOLIDAY,
    29040_i32 => _TYNWALD_DAY,
    29096_i32 => _LATE_SUMMER_BANK_HOLIDAY,
    29213_i32 => _CHRISTMAS_DAY,
    29214_i32 => _BOXING_DAY,
    29215_i32 => _CHRISTMAS_DAY__OBSERVED_,
    29216_i32 => _BOXING_DAY__OBSERVED_,
    29220_i32 => _NEW_YEAR_S_DAY,
};
