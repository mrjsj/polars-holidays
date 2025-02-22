use crate::countries::constants::*;
use phf::phf_map;

pub static PG_HOLIDAYS: phf::Map<i32, &'static str> = phf_map! {
    10957_i32 => _NEW_YEAR_S_DAY,
    11068_i32 => _GOOD_FRIDAY,
    11069_i32 => _EASTER_SATURDAY,
    11070_i32 => _EASTER_SUNDAY,
    11071_i32 => _EASTER_MONDAY,
    11120_i32 => _QUEEN_S_BIRTHDAY,
    11161_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY,
    11162_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY__OBSERVED_,
    11216_i32 => _INDEPENDENCE_DAY,
    11316_i32 => _CHRISTMAS_DAY,
    11317_i32 => _BOXING_DAY,
    11323_i32 => _NEW_YEAR_S_DAY,
    11425_i32 => _GOOD_FRIDAY,
    11426_i32 => _EASTER_SATURDAY,
    11427_i32 => _EASTER_SUNDAY,
    11428_i32 => _EASTER_MONDAY,
    11484_i32 => _QUEEN_S_BIRTHDAY,
    11526_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY,
    11581_i32 => _INDEPENDENCE_DAY,
    11582_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    11681_i32 => _CHRISTMAS_DAY,
    11682_i32 => _BOXING_DAY,
    11688_i32 => _NEW_YEAR_S_DAY,
    11775_i32 => _GOOD_FRIDAY,
    11776_i32 => _EASTER_SATURDAY,
    11777_i32 => _EASTER_SUNDAY,
    11778_i32 => _EASTER_MONDAY,
    11848_i32 => _QUEEN_S_BIRTHDAY,
    11891_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY,
    11946_i32 => _INDEPENDENCE_DAY,
    12046_i32 => _CHRISTMAS_DAY,
    12047_i32 => _BOXING_DAY,
    12053_i32 => _NEW_YEAR_S_DAY,
    12160_i32 => _GOOD_FRIDAY,
    12161_i32 => _EASTER_SATURDAY,
    12162_i32 => _EASTER_SUNDAY,
    12163_i32 => _EASTER_MONDAY,
    12212_i32 => _QUEEN_S_BIRTHDAY,
    12256_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY,
    12311_i32 => _INDEPENDENCE_DAY,
    12411_i32 => _CHRISTMAS_DAY,
    12412_i32 => _BOXING_DAY,
    12418_i32 => _NEW_YEAR_S_DAY,
    12517_i32 => _GOOD_FRIDAY,
    12518_i32 => _EASTER_SATURDAY,
    12519_i32 => _EASTER_SUNDAY,
    12520_i32 => _EASTER_MONDAY,
    12583_i32 => _QUEEN_S_BIRTHDAY,
    12622_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY,
    12677_i32 => _INDEPENDENCE_DAY,
    12777_i32 => _CHRISTMAS_DAY,
    12778_i32 => _BOXING_DAY,
    12779_i32 => _BOXING_DAY__OBSERVED_,
    12784_i32 => _NEW_YEAR_S_DAY,
    12867_i32 => _GOOD_FRIDAY,
    12868_i32 => _EASTER_SATURDAY,
    12869_i32 => _EASTER_SUNDAY,
    12870_i32 => _EASTER_MONDAY,
    12947_i32 => _QUEEN_S_BIRTHDAY,
    12987_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY,
    13042_i32 => _INDEPENDENCE_DAY,
    13142_i32 => _CHRISTMAS_DAY,
    13143_i32 => _BOXING_DAY,
    13144_i32 => _CHRISTMAS_DAY__OBSERVED_,
    13149_i32 => _NEW_YEAR_S_DAY,
    13150_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    13252_i32 => _GOOD_FRIDAY,
    13253_i32 => _EASTER_SATURDAY,
    13254_i32 => _EASTER_SUNDAY,
    13255_i32 => _EASTER_MONDAY,
    13311_i32 => _QUEEN_S_BIRTHDAY,
    13352_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY,
    13353_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY__OBSERVED_,
    13407_i32 => _INDEPENDENCE_DAY,
    13507_i32 => _CHRISTMAS_DAY,
    13508_i32 => _BOXING_DAY,
    13514_i32 => _NEW_YEAR_S_DAY,
    13609_i32 => _GOOD_FRIDAY,
    13610_i32 => _EASTER_SATURDAY,
    13611_i32 => _EASTER_SUNDAY,
    13612_i32 => _EASTER_MONDAY,
    13675_i32 => _QUEEN_S_BIRTHDAY,
    13717_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY,
    13772_i32 => _INDEPENDENCE_DAY,
    13773_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    13872_i32 => _CHRISTMAS_DAY,
    13873_i32 => _BOXING_DAY,
    13879_i32 => _NEW_YEAR_S_DAY,
    13959_i32 => _GOOD_FRIDAY,
    13960_i32 => _EASTER_SATURDAY,
    13961_i32 => _EASTER_SUNDAY,
    13962_i32 => _EASTER_MONDAY,
    14039_i32 => _QUEEN_S_BIRTHDAY,
    14083_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY,
    14138_i32 => _INDEPENDENCE_DAY,
    14238_i32 => _CHRISTMAS_DAY,
    14239_i32 => _BOXING_DAY,
    14245_i32 => _NEW_YEAR_S_DAY,
    14344_i32 => _GOOD_FRIDAY,
    14345_i32 => _EASTER_SATURDAY,
    14346_i32 => _EASTER_SUNDAY,
    14347_i32 => _EASTER_MONDAY,
    14403_i32 => _QUEEN_S_BIRTHDAY,
    14448_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY,
    14503_i32 => _INDEPENDENCE_DAY,
    14603_i32 => _CHRISTMAS_DAY,
    14604_i32 => _BOXING_DAY,
    14610_i32 => _NEW_YEAR_S_DAY,
    14701_i32 => _GOOD_FRIDAY,
    14702_i32 => _EASTER_SATURDAY,
    14703_i32 => _EASTER_SUNDAY,
    14704_i32 => _EASTER_MONDAY,
    14774_i32 => _QUEEN_S_BIRTHDAY,
    14813_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY,
    14868_i32 => _INDEPENDENCE_DAY,
    14968_i32 => _CHRISTMAS_DAY,
    14969_i32 => _BOXING_DAY,
    14970_i32 => _BOXING_DAY__OBSERVED_,
    14975_i32 => _NEW_YEAR_S_DAY,
    15086_i32 => _GOOD_FRIDAY,
    15087_i32 => _EASTER_SATURDAY,
    15088_i32 => _EASTER_SUNDAY,
    15089_i32 => _EASTER_MONDAY,
    15138_i32 => _QUEEN_S_BIRTHDAY,
    15178_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY,
    15212_i32 => _NATIONAL_REPENTANCE_DAY,
    15233_i32 => _INDEPENDENCE_DAY,
    15333_i32 => _CHRISTMAS_DAY,
    15334_i32 => _BOXING_DAY,
    15335_i32 => _CHRISTMAS_DAY__OBSERVED_,
    15340_i32 => _NEW_YEAR_S_DAY,
    15341_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    15436_i32 => _GOOD_FRIDAY,
    15437_i32 => _EASTER_SATURDAY,
    15438_i32 => _EASTER_SUNDAY,
    15439_i32 => _EASTER_MONDAY,
    15502_i32 => _QUEEN_S_BIRTHDAY,
    15544_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY,
    15578_i32 => _NATIONAL_REPENTANCE_DAY,
    15579_i32 => _NATIONAL_REPENTANCE_DAY__OBSERVED_,
    15599_i32 => _INDEPENDENCE_DAY,
    15600_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    15699_i32 => _CHRISTMAS_DAY,
    15700_i32 => _BOXING_DAY,
    15706_i32 => _NEW_YEAR_S_DAY,
    15793_i32 => _GOOD_FRIDAY,
    15794_i32 => _EASTER_SATURDAY,
    15795_i32 => _EASTER_SUNDAY,
    15796_i32 => _EASTER_MONDAY,
    15866_i32 => _QUEEN_S_BIRTHDAY,
    15909_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY,
    15943_i32 => _NATIONAL_REPENTANCE_DAY,
    15964_i32 => _INDEPENDENCE_DAY,
    16064_i32 => _CHRISTMAS_DAY,
    16065_i32 => _BOXING_DAY,
    16071_i32 => _NEW_YEAR_S_DAY,
    16178_i32 => _GOOD_FRIDAY,
    16179_i32 => _EASTER_SATURDAY,
    16180_i32 => _EASTER_SUNDAY,
    16181_i32 => _EASTER_MONDAY,
    16230_i32 => _QUEEN_S_BIRTHDAY,
    16274_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY,
    16308_i32 => _NATIONAL_REPENTANCE_DAY,
    16329_i32 => _INDEPENDENCE_DAY,
    16429_i32 => _CHRISTMAS_DAY,
    16430_i32 => _BOXING_DAY,
    16436_i32 => _NEW_YEAR_S_DAY,
    16528_i32 => _GOOD_FRIDAY,
    16529_i32 => _EASTER_SATURDAY,
    16530_i32 => _EASTER_SUNDAY,
    16531_i32 => _EASTER_MONDAY,
    16594_i32 => _QUEEN_S_BIRTHDAY,
    16639_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY,
    16673_i32 => _NATIONAL_REPENTANCE_DAY,
    16694_i32 => _INDEPENDENCE_DAY,
    16794_i32 => _CHRISTMAS_DAY,
    16795_i32 => _BOXING_DAY,
    16801_i32 => _NEW_YEAR_S_DAY,
    16885_i32 => _GOOD_FRIDAY,
    16886_i32 => _EASTER_SATURDAY,
    16887_i32 => _EASTER_SUNDAY,
    16888_i32 => _EASTER_MONDAY,
    16965_i32 => _QUEEN_S_BIRTHDAY,
    17005_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY,
    17039_i32 => _NATIONAL_REPENTANCE_DAY,
    17060_i32 => _INDEPENDENCE_DAY,
    17160_i32 => _CHRISTMAS_DAY,
    17161_i32 => _BOXING_DAY,
    17162_i32 => _CHRISTMAS_DAY__OBSERVED_,
    17167_i32 => _NEW_YEAR_S_DAY,
    17168_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    17270_i32 => _GOOD_FRIDAY,
    17271_i32 => _EASTER_SATURDAY,
    17272_i32 => _EASTER_SUNDAY,
    17273_i32 => _EASTER_MONDAY,
    17329_i32 => _QUEEN_S_BIRTHDAY,
    17370_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY,
    17371_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY__OBSERVED_,
    17404_i32 => _NATIONAL_REPENTANCE_DAY,
    17425_i32 => _INDEPENDENCE_DAY,
    17525_i32 => _CHRISTMAS_DAY,
    17526_i32 => _BOXING_DAY,
    17532_i32 => _NEW_YEAR_S_DAY,
    17620_i32 => _GOOD_FRIDAY,
    17621_i32 => _EASTER_SATURDAY,
    17622_i32 => _EASTER_SUNDAY,
    17623_i32 => _EASTER_MONDAY,
    17693_i32 => _QUEEN_S_BIRTHDAY,
    17735_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY,
    17769_i32 => _NATIONAL_REPENTANCE_DAY,
    17770_i32 => _NATIONAL_REPENTANCE_DAY__OBSERVED_,
    17790_i32 => _INDEPENDENCE_DAY,
    17791_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    17851_i32 => _APEC_LEADERS__SUMMIT_PUBLIC_HOLIDAY,
    17890_i32 => _CHRISTMAS_DAY,
    17891_i32 => _BOXING_DAY,
    17897_i32 => _NEW_YEAR_S_DAY,
    18005_i32 => _GOOD_FRIDAY,
    18006_i32 => _EASTER_SATURDAY,
    18007_i32 => _EASTER_SUNDAY,
    18008_i32 => _EASTER_MONDAY,
    18057_i32 => _QUEEN_S_BIRTHDAY,
    18100_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY,
    18134_i32 => _NATIONAL_REPENTANCE_DAY,
    18155_i32 => _INDEPENDENCE_DAY,
    18255_i32 => _CHRISTMAS_DAY,
    18256_i32 => _BOXING_DAY,
    18262_i32 => _NEW_YEAR_S_DAY,
    18362_i32 => _GOOD_FRIDAY,
    18363_i32 => _EASTER_SATURDAY,
    18364_i32 => _EASTER_SUNDAY,
    18365_i32 => _EASTER_MONDAY,
    18421_i32 => _QUEEN_S_BIRTHDAY,
    18466_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY,
    18500_i32 => _NATIONAL_REPENTANCE_DAY,
    18521_i32 => _INDEPENDENCE_DAY,
    18621_i32 => _CHRISTMAS_DAY,
    18622_i32 => _BOXING_DAY,
    18628_i32 => _NEW_YEAR_S_DAY,
    18635_i32 => _STATE_FUNERAL_OF_SIR_MEKERE_MORAUTA,
    18687_i32 => _NATIONAL_DAY_OF_MOURNING_FOR_SIR_MICHAEL_SOMARE,
    18698_i32 => _NATIONAL_DAY_OF_MOURNING_FOR_SIR_MICHAEL_SOMARE,
    18719_i32 => _GOOD_FRIDAY,
    18720_i32 => _EASTER_SATURDAY,
    18721_i32 => _EASTER_SUNDAY,
    18722_i32 => _EASTER_MONDAY,
    18792_i32 => _QUEEN_S_BIRTHDAY,
    18831_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY,
    18865_i32 => _NATIONAL_REPENTANCE_DAY,
    18886_i32 => _INDEPENDENCE_DAY,
    18986_i32 => _CHRISTMAS_DAY,
    18987_i32 => _BOXING_DAY,
    18988_i32 => _BOXING_DAY__OBSERVED_,
    18993_i32 => _NEW_YEAR_S_DAY,
    19049_i32 => _GRAND_CHIEF_SIR_MICHAEL_SOMARE_REMEMBRANCE_DAY,
    19051_i32 => _GRAND_CHIEF_SIR_MICHAEL_SOMARE_REMEMBRANCE_DAY__OBSERVED_,
    19097_i32 => _GOOD_FRIDAY,
    19098_i32 => _EASTER_SATURDAY,
    19099_i32 => _EASTER_SUNDAY,
    19100_i32 => _EASTER_MONDAY,
    19156_i32 => _QUEEN_S_BIRTHDAY,
    19196_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY,
    19230_i32 => _NATIONAL_REPENTANCE_DAY,
    19251_i32 => _INDEPENDENCE_DAY,
    19254_i32 => _STATE_FUNERAL_OF_QUEEN_ELIZABETH_II,
    19351_i32 => _CHRISTMAS_DAY,
    19352_i32 => _BOXING_DAY,
    19353_i32 => _CHRISTMAS_DAY__OBSERVED_,
    19358_i32 => _NEW_YEAR_S_DAY,
    19359_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    19412_i32 => _GRAND_CHIEF_SIR_MICHAEL_SOMARE_REMEMBRANCE_DAY__OBSERVED_,
    19414_i32 => _GRAND_CHIEF_SIR_MICHAEL_SOMARE_REMEMBRANCE_DAY,
    19454_i32 => _GOOD_FRIDAY,
    19455_i32 => _EASTER_SATURDAY,
    19456_i32 => _EASTER_SUNDAY,
    19457_i32 => _EASTER_MONDAY,
    19465_i32 => _STATE_FUNERAL_OF_SIR_RABBIE_NAMALIU,
    19524_i32 => _KING_S_BIRTHDAY__OBSERVED_,
    19525_i32 => _KING_S_BIRTHDAY,
    19561_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY,
    19562_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY__OBSERVED_,
    19595_i32 => _NATIONAL_REPENTANCE_DAY,
    19615_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    19616_i32 => _INDEPENDENCE_DAY,
    19716_i32 => _CHRISTMAS_DAY,
    19717_i32 => _BOXING_DAY,
    19723_i32 => _NEW_YEAR_S_DAY,
    19779_i32 => _GRAND_CHIEF_SIR_MICHAEL_SOMARE_REMEMBRANCE_DAY,
    19811_i32 => _GOOD_FRIDAY,
    19813_i32 => _EASTER_SUNDAY,
    19814_i32 => _EASTER_MONDAY,
    19891_i32 => _KING_S_BIRTHDAY,
    19927_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY,
    19961_i32 => _NATIONAL_REPENTANCE_DAY,
    19982_i32 => _INDEPENDENCE_DAY,
    20082_i32 => _CHRISTMAS_DAY,
    20083_i32 => _BOXING_DAY,
    20089_i32 => _NEW_YEAR_S_DAY,
    20145_i32 => _GRAND_CHIEF_SIR_MICHAEL_SOMARE_REMEMBRANCE_DAY,
    20196_i32 => _GOOD_FRIDAY,
    20198_i32 => _EASTER_SUNDAY,
    20199_i32 => _EASTER_MONDAY,
    20256_i32 => _KING_S_BIRTHDAY,
    20292_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY,
    20326_i32 => _NATIONAL_REPENTANCE_DAY,
    20347_i32 => _INDEPENDENCE_DAY,
    20447_i32 => _CHRISTMAS_DAY,
    20448_i32 => _BOXING_DAY,
    20454_i32 => _NEW_YEAR_S_DAY,
    20510_i32 => _GRAND_CHIEF_SIR_MICHAEL_SOMARE_REMEMBRANCE_DAY,
    20546_i32 => _GOOD_FRIDAY,
    20548_i32 => _EASTER_SUNDAY,
    20549_i32 => _EASTER_MONDAY,
    20621_i32 => _KING_S_BIRTHDAY,
    20657_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY,
    20691_i32 => _NATIONAL_REPENTANCE_DAY,
    20712_i32 => _INDEPENDENCE_DAY,
    20812_i32 => _CHRISTMAS_DAY,
    20813_i32 => _BOXING_DAY,
    20819_i32 => _NEW_YEAR_S_DAY,
    20875_i32 => _GRAND_CHIEF_SIR_MICHAEL_SOMARE_REMEMBRANCE_DAY,
    20903_i32 => _GOOD_FRIDAY,
    20905_i32 => _EASTER_SUNDAY,
    20906_i32 => _EASTER_MONDAY,
    20986_i32 => _KING_S_BIRTHDAY,
    21022_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY,
    21056_i32 => _NATIONAL_REPENTANCE_DAY,
    21077_i32 => _INDEPENDENCE_DAY,
    21177_i32 => _CHRISTMAS_DAY,
    21178_i32 => _BOXING_DAY,
    21179_i32 => _BOXING_DAY__OBSERVED_,
    21184_i32 => _NEW_YEAR_S_DAY,
    21240_i32 => _GRAND_CHIEF_SIR_MICHAEL_SOMARE_REMEMBRANCE_DAY,
    21288_i32 => _GOOD_FRIDAY,
    21290_i32 => _EASTER_SUNDAY,
    21291_i32 => _EASTER_MONDAY,
    21352_i32 => _KING_S_BIRTHDAY,
    21388_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY,
    21389_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY__OBSERVED_,
    21422_i32 => _NATIONAL_REPENTANCE_DAY,
    21443_i32 => _INDEPENDENCE_DAY,
    21543_i32 => _CHRISTMAS_DAY,
    21544_i32 => _BOXING_DAY,
    21550_i32 => _NEW_YEAR_S_DAY,
    21606_i32 => _GRAND_CHIEF_SIR_MICHAEL_SOMARE_REMEMBRANCE_DAY,
    21638_i32 => _GOOD_FRIDAY,
    21640_i32 => _EASTER_SUNDAY,
    21641_i32 => _EASTER_MONDAY,
    21717_i32 => _KING_S_BIRTHDAY,
    21753_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY,
    21787_i32 => _NATIONAL_REPENTANCE_DAY,
    21788_i32 => _NATIONAL_REPENTANCE_DAY__OBSERVED_,
    21808_i32 => _INDEPENDENCE_DAY,
    21809_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    21908_i32 => _CHRISTMAS_DAY,
    21909_i32 => _BOXING_DAY,
    21915_i32 => _NEW_YEAR_S_DAY,
    21971_i32 => _GRAND_CHIEF_SIR_MICHAEL_SOMARE_REMEMBRANCE_DAY,
    22023_i32 => _GOOD_FRIDAY,
    22025_i32 => _EASTER_SUNDAY,
    22026_i32 => _EASTER_MONDAY,
    22082_i32 => _KING_S_BIRTHDAY,
    22118_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY,
    22152_i32 => _NATIONAL_REPENTANCE_DAY,
    22173_i32 => _INDEPENDENCE_DAY,
    22273_i32 => _CHRISTMAS_DAY,
    22274_i32 => _BOXING_DAY,
    22280_i32 => _NEW_YEAR_S_DAY,
    22336_i32 => _GRAND_CHIEF_SIR_MICHAEL_SOMARE_REMEMBRANCE_DAY,
    22380_i32 => _GOOD_FRIDAY,
    22382_i32 => _EASTER_SUNDAY,
    22383_i32 => _EASTER_MONDAY,
    22447_i32 => _KING_S_BIRTHDAY,
    22483_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY,
    22517_i32 => _NATIONAL_REPENTANCE_DAY,
    22538_i32 => _INDEPENDENCE_DAY,
    22638_i32 => _CHRISTMAS_DAY,
    22639_i32 => _BOXING_DAY,
    22645_i32 => _NEW_YEAR_S_DAY,
    22701_i32 => _GRAND_CHIEF_SIR_MICHAEL_SOMARE_REMEMBRANCE_DAY,
    22730_i32 => _GOOD_FRIDAY,
    22732_i32 => _EASTER_SUNDAY,
    22733_i32 => _EASTER_MONDAY,
    22813_i32 => _KING_S_BIRTHDAY,
    22849_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY,
    22883_i32 => _NATIONAL_REPENTANCE_DAY,
    22904_i32 => _INDEPENDENCE_DAY,
    23004_i32 => _CHRISTMAS_DAY,
    23005_i32 => _BOXING_DAY,
    23006_i32 => _BOXING_DAY__OBSERVED_,
    23011_i32 => _NEW_YEAR_S_DAY,
    23067_i32 => _GRAND_CHIEF_SIR_MICHAEL_SOMARE_REMEMBRANCE_DAY,
    23115_i32 => _GOOD_FRIDAY,
    23117_i32 => _EASTER_SUNDAY,
    23118_i32 => _EASTER_MONDAY,
    23178_i32 => _KING_S_BIRTHDAY,
    23214_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY,
    23248_i32 => _NATIONAL_REPENTANCE_DAY,
    23269_i32 => _INDEPENDENCE_DAY,
    23369_i32 => _CHRISTMAS_DAY,
    23370_i32 => _BOXING_DAY,
    23371_i32 => _CHRISTMAS_DAY__OBSERVED_,
    23376_i32 => _NEW_YEAR_S_DAY,
    23377_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    23432_i32 => _GRAND_CHIEF_SIR_MICHAEL_SOMARE_REMEMBRANCE_DAY,
    23472_i32 => _GOOD_FRIDAY,
    23474_i32 => _EASTER_SUNDAY,
    23475_i32 => _EASTER_MONDAY,
    23543_i32 => _KING_S_BIRTHDAY,
    23579_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY,
    23580_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY__OBSERVED_,
    23613_i32 => _NATIONAL_REPENTANCE_DAY,
    23634_i32 => _INDEPENDENCE_DAY,
    23734_i32 => _CHRISTMAS_DAY,
    23735_i32 => _BOXING_DAY,
    23741_i32 => _NEW_YEAR_S_DAY,
    23797_i32 => _GRAND_CHIEF_SIR_MICHAEL_SOMARE_REMEMBRANCE_DAY,
    23822_i32 => _GOOD_FRIDAY,
    23824_i32 => _EASTER_SUNDAY,
    23825_i32 => _EASTER_MONDAY,
    23908_i32 => _KING_S_BIRTHDAY,
    23944_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY,
    23978_i32 => _NATIONAL_REPENTANCE_DAY,
    23979_i32 => _NATIONAL_REPENTANCE_DAY__OBSERVED_,
    23999_i32 => _INDEPENDENCE_DAY,
    24000_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    24099_i32 => _CHRISTMAS_DAY,
    24100_i32 => _BOXING_DAY,
    24106_i32 => _NEW_YEAR_S_DAY,
    24162_i32 => _GRAND_CHIEF_SIR_MICHAEL_SOMARE_REMEMBRANCE_DAY,
    24207_i32 => _GOOD_FRIDAY,
    24209_i32 => _EASTER_SUNDAY,
    24210_i32 => _EASTER_MONDAY,
    24274_i32 => _KING_S_BIRTHDAY,
    24310_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY,
    24344_i32 => _NATIONAL_REPENTANCE_DAY,
    24365_i32 => _INDEPENDENCE_DAY,
    24465_i32 => _CHRISTMAS_DAY,
    24466_i32 => _BOXING_DAY,
    24472_i32 => _NEW_YEAR_S_DAY,
    24528_i32 => _GRAND_CHIEF_SIR_MICHAEL_SOMARE_REMEMBRANCE_DAY,
    24564_i32 => _GOOD_FRIDAY,
    24566_i32 => _EASTER_SUNDAY,
    24567_i32 => _EASTER_MONDAY,
    24639_i32 => _KING_S_BIRTHDAY,
    24675_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY,
    24709_i32 => _NATIONAL_REPENTANCE_DAY,
    24730_i32 => _INDEPENDENCE_DAY,
    24830_i32 => _CHRISTMAS_DAY,
    24831_i32 => _BOXING_DAY,
    24837_i32 => _NEW_YEAR_S_DAY,
    24893_i32 => _GRAND_CHIEF_SIR_MICHAEL_SOMARE_REMEMBRANCE_DAY,
    24949_i32 => _GOOD_FRIDAY,
    24951_i32 => _EASTER_SUNDAY,
    24952_i32 => _EASTER_MONDAY,
    25004_i32 => _KING_S_BIRTHDAY,
    25040_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY,
    25074_i32 => _NATIONAL_REPENTANCE_DAY,
    25095_i32 => _INDEPENDENCE_DAY,
    25195_i32 => _CHRISTMAS_DAY,
    25196_i32 => _BOXING_DAY,
    25197_i32 => _BOXING_DAY__OBSERVED_,
    25202_i32 => _NEW_YEAR_S_DAY,
    25258_i32 => _GRAND_CHIEF_SIR_MICHAEL_SOMARE_REMEMBRANCE_DAY,
    25299_i32 => _GOOD_FRIDAY,
    25301_i32 => _EASTER_SUNDAY,
    25302_i32 => _EASTER_MONDAY,
    25369_i32 => _KING_S_BIRTHDAY,
    25405_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY,
    25439_i32 => _NATIONAL_REPENTANCE_DAY,
    25460_i32 => _INDEPENDENCE_DAY,
    25560_i32 => _CHRISTMAS_DAY,
    25561_i32 => _BOXING_DAY,
    25562_i32 => _CHRISTMAS_DAY__OBSERVED_,
    25567_i32 => _NEW_YEAR_S_DAY,
    25568_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    25623_i32 => _GRAND_CHIEF_SIR_MICHAEL_SOMARE_REMEMBRANCE_DAY,
    25656_i32 => _GOOD_FRIDAY,
    25658_i32 => _EASTER_SUNDAY,
    25659_i32 => _EASTER_MONDAY,
    25735_i32 => _KING_S_BIRTHDAY,
    25771_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY,
    25805_i32 => _NATIONAL_REPENTANCE_DAY,
    25806_i32 => _NATIONAL_REPENTANCE_DAY__OBSERVED_,
    25826_i32 => _INDEPENDENCE_DAY,
    25827_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    25926_i32 => _CHRISTMAS_DAY,
    25927_i32 => _BOXING_DAY,
    25933_i32 => _NEW_YEAR_S_DAY,
    25989_i32 => _GRAND_CHIEF_SIR_MICHAEL_SOMARE_REMEMBRANCE_DAY,
    26041_i32 => _GOOD_FRIDAY,
    26043_i32 => _EASTER_SUNDAY,
    26044_i32 => _EASTER_MONDAY,
    26100_i32 => _KING_S_BIRTHDAY,
    26136_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY,
    26170_i32 => _NATIONAL_REPENTANCE_DAY,
    26191_i32 => _INDEPENDENCE_DAY,
    26291_i32 => _CHRISTMAS_DAY,
    26292_i32 => _BOXING_DAY,
    26298_i32 => _NEW_YEAR_S_DAY,
    26354_i32 => _GRAND_CHIEF_SIR_MICHAEL_SOMARE_REMEMBRANCE_DAY,
    26391_i32 => _GOOD_FRIDAY,
    26393_i32 => _EASTER_SUNDAY,
    26394_i32 => _EASTER_MONDAY,
    26465_i32 => _KING_S_BIRTHDAY,
    26501_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY,
    26535_i32 => _NATIONAL_REPENTANCE_DAY,
    26556_i32 => _INDEPENDENCE_DAY,
    26656_i32 => _CHRISTMAS_DAY,
    26657_i32 => _BOXING_DAY,
    26663_i32 => _NEW_YEAR_S_DAY,
    26719_i32 => _GRAND_CHIEF_SIR_MICHAEL_SOMARE_REMEMBRANCE_DAY,
    26748_i32 => _GOOD_FRIDAY,
    26750_i32 => _EASTER_SUNDAY,
    26751_i32 => _EASTER_MONDAY,
    26830_i32 => _KING_S_BIRTHDAY,
    26866_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY,
    26900_i32 => _NATIONAL_REPENTANCE_DAY,
    26921_i32 => _INDEPENDENCE_DAY,
    27021_i32 => _CHRISTMAS_DAY,
    27022_i32 => _BOXING_DAY,
    27028_i32 => _NEW_YEAR_S_DAY,
    27084_i32 => _GRAND_CHIEF_SIR_MICHAEL_SOMARE_REMEMBRANCE_DAY,
    27133_i32 => _GOOD_FRIDAY,
    27135_i32 => _EASTER_SUNDAY,
    27136_i32 => _EASTER_MONDAY,
    27196_i32 => _KING_S_BIRTHDAY,
    27232_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY,
    27266_i32 => _NATIONAL_REPENTANCE_DAY,
    27287_i32 => _INDEPENDENCE_DAY,
    27387_i32 => _CHRISTMAS_DAY,
    27388_i32 => _BOXING_DAY,
    27389_i32 => _CHRISTMAS_DAY__OBSERVED_,
    27394_i32 => _NEW_YEAR_S_DAY,
    27395_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    27450_i32 => _GRAND_CHIEF_SIR_MICHAEL_SOMARE_REMEMBRANCE_DAY,
    27490_i32 => _GOOD_FRIDAY,
    27492_i32 => _EASTER_SUNDAY,
    27493_i32 => _EASTER_MONDAY,
    27561_i32 => _KING_S_BIRTHDAY,
    27597_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY,
    27598_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY__OBSERVED_,
    27631_i32 => _NATIONAL_REPENTANCE_DAY,
    27652_i32 => _INDEPENDENCE_DAY,
    27752_i32 => _CHRISTMAS_DAY,
    27753_i32 => _BOXING_DAY,
    27759_i32 => _NEW_YEAR_S_DAY,
    27815_i32 => _GRAND_CHIEF_SIR_MICHAEL_SOMARE_REMEMBRANCE_DAY,
    27840_i32 => _GOOD_FRIDAY,
    27842_i32 => _EASTER_SUNDAY,
    27843_i32 => _EASTER_MONDAY,
    27926_i32 => _KING_S_BIRTHDAY,
    27962_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY,
    27996_i32 => _NATIONAL_REPENTANCE_DAY,
    27997_i32 => _NATIONAL_REPENTANCE_DAY__OBSERVED_,
    28017_i32 => _INDEPENDENCE_DAY,
    28018_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    28117_i32 => _CHRISTMAS_DAY,
    28118_i32 => _BOXING_DAY,
    28124_i32 => _NEW_YEAR_S_DAY,
    28180_i32 => _GRAND_CHIEF_SIR_MICHAEL_SOMARE_REMEMBRANCE_DAY,
    28225_i32 => _GOOD_FRIDAY,
    28227_i32 => _EASTER_SUNDAY,
    28228_i32 => _EASTER_MONDAY,
    28291_i32 => _KING_S_BIRTHDAY,
    28327_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY,
    28361_i32 => _NATIONAL_REPENTANCE_DAY,
    28382_i32 => _INDEPENDENCE_DAY,
    28482_i32 => _CHRISTMAS_DAY,
    28483_i32 => _BOXING_DAY,
    28489_i32 => _NEW_YEAR_S_DAY,
    28545_i32 => _GRAND_CHIEF_SIR_MICHAEL_SOMARE_REMEMBRANCE_DAY,
    28582_i32 => _GOOD_FRIDAY,
    28584_i32 => _EASTER_SUNDAY,
    28585_i32 => _EASTER_MONDAY,
    28657_i32 => _KING_S_BIRTHDAY,
    28693_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY,
    28727_i32 => _NATIONAL_REPENTANCE_DAY,
    28748_i32 => _INDEPENDENCE_DAY,
    28848_i32 => _CHRISTMAS_DAY,
    28849_i32 => _BOXING_DAY,
    28855_i32 => _NEW_YEAR_S_DAY,
    28911_i32 => _GRAND_CHIEF_SIR_MICHAEL_SOMARE_REMEMBRANCE_DAY,
    28960_i32 => _GOOD_FRIDAY,
    28962_i32 => _EASTER_SUNDAY,
    28963_i32 => _EASTER_MONDAY,
    29022_i32 => _KING_S_BIRTHDAY,
    29058_i32 => _PAPUA_NEW_GUINEA_REMEMBRANCE_DAY,
    29092_i32 => _NATIONAL_REPENTANCE_DAY,
    29113_i32 => _INDEPENDENCE_DAY,
    29213_i32 => _CHRISTMAS_DAY,
    29214_i32 => _BOXING_DAY,
    29215_i32 => _BOXING_DAY__OBSERVED_,
    29220_i32 => _NEW_YEAR_S_DAY,
};
