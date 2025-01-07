use crate::countries::constants::*;
use phf::phf_map;

pub static CY_HOLIDAYS: phf::Map<i32, &'static str> = phf_map! {
    10957_i32 => _NEW_YEAR_S_DAY,
    10962_i32 => _EPIPHANY,
    11029_i32 => _GREEN_MONDAY,
    11041_i32 => _GREEK_INDEPENDENCE_DAY,
    11048_i32 => _CYPRUS_NATIONAL_DAY,
    11075_i32 => _GOOD_FRIDAY,
    11077_i32 => _EASTER_SUNDAY,
    11078_i32 => _EASTER_MONDAY__LABOR_DAY,
    11127_i32 => _WHIT_MONDAY,
    11184_i32 => _ASSUMPTION_DAY,
    11231_i32 => _CYPRUS_INDEPENDENCE_DAY,
    11258_i32 => _GREEK_NATIONAL_DAY,
    11316_i32 => _CHRISTMAS_DAY,
    11317_i32 => _DAY_AFTER_CHRISTMAS,
    11323_i32 => _NEW_YEAR_S_DAY,
    11328_i32 => _EPIPHANY,
    11379_i32 => _GREEN_MONDAY,
    11406_i32 => _GREEK_INDEPENDENCE_DAY,
    11413_i32 => _CYPRUS_NATIONAL_DAY,
    11425_i32 => _GOOD_FRIDAY,
    11427_i32 => _EASTER_SUNDAY,
    11428_i32 => _EASTER_MONDAY,
    11443_i32 => _LABOR_DAY,
    11477_i32 => _WHIT_MONDAY,
    11549_i32 => _ASSUMPTION_DAY,
    11596_i32 => _CYPRUS_INDEPENDENCE_DAY,
    11623_i32 => _GREEK_NATIONAL_DAY,
    11681_i32 => _CHRISTMAS_DAY,
    11682_i32 => _DAY_AFTER_CHRISTMAS,
    11688_i32 => _NEW_YEAR_S_DAY,
    11693_i32 => _EPIPHANY,
    11764_i32 => _GREEN_MONDAY,
    11771_i32 => _GREEK_INDEPENDENCE_DAY,
    11778_i32 => _CYPRUS_NATIONAL_DAY,
    11808_i32 => _LABOR_DAY,
    11810_i32 => _GOOD_FRIDAY,
    11812_i32 => _EASTER_SUNDAY,
    11813_i32 => _EASTER_MONDAY,
    11862_i32 => _WHIT_MONDAY,
    11914_i32 => _ASSUMPTION_DAY,
    11961_i32 => _CYPRUS_INDEPENDENCE_DAY,
    11988_i32 => _GREEK_NATIONAL_DAY,
    12046_i32 => _CHRISTMAS_DAY,
    12047_i32 => _DAY_AFTER_CHRISTMAS,
    12053_i32 => _NEW_YEAR_S_DAY,
    12058_i32 => _EPIPHANY,
    12121_i32 => _GREEN_MONDAY,
    12136_i32 => _GREEK_INDEPENDENCE_DAY,
    12143_i32 => _CYPRUS_NATIONAL_DAY,
    12167_i32 => _GOOD_FRIDAY,
    12169_i32 => _EASTER_SUNDAY,
    12170_i32 => _EASTER_MONDAY,
    12173_i32 => _LABOR_DAY,
    12219_i32 => _WHIT_MONDAY,
    12279_i32 => _ASSUMPTION_DAY,
    12326_i32 => _CYPRUS_INDEPENDENCE_DAY,
    12353_i32 => _GREEK_NATIONAL_DAY,
    12411_i32 => _CHRISTMAS_DAY,
    12412_i32 => _DAY_AFTER_CHRISTMAS,
    12418_i32 => _NEW_YEAR_S_DAY,
    12423_i32 => _EPIPHANY,
    12471_i32 => _GREEN_MONDAY,
    12502_i32 => _GREEK_INDEPENDENCE_DAY,
    12509_i32 => _CYPRUS_NATIONAL_DAY,
    12517_i32 => _GOOD_FRIDAY,
    12519_i32 => _EASTER_SUNDAY,
    12520_i32 => _EASTER_MONDAY,
    12539_i32 => _LABOR_DAY,
    12569_i32 => _WHIT_MONDAY,
    12645_i32 => _ASSUMPTION_DAY,
    12692_i32 => _CYPRUS_INDEPENDENCE_DAY,
    12719_i32 => _GREEK_NATIONAL_DAY,
    12777_i32 => _CHRISTMAS_DAY,
    12778_i32 => _DAY_AFTER_CHRISTMAS,
    12784_i32 => _NEW_YEAR_S_DAY,
    12789_i32 => _EPIPHANY,
    12856_i32 => _GREEN_MONDAY,
    12867_i32 => _GREEK_INDEPENDENCE_DAY,
    12874_i32 => _CYPRUS_NATIONAL_DAY,
    12902_i32 => _GOOD_FRIDAY,
    12904_i32 => _EASTER_SUNDAY__LABOR_DAY,
    12905_i32 => _EASTER_MONDAY,
    12954_i32 => _WHIT_MONDAY,
    13010_i32 => _ASSUMPTION_DAY,
    13057_i32 => _CYPRUS_INDEPENDENCE_DAY,
    13084_i32 => _GREEK_NATIONAL_DAY,
    13142_i32 => _CHRISTMAS_DAY,
    13143_i32 => _DAY_AFTER_CHRISTMAS,
    13149_i32 => _NEW_YEAR_S_DAY,
    13154_i32 => _EPIPHANY,
    13213_i32 => _GREEN_MONDAY,
    13232_i32 => _GREEK_INDEPENDENCE_DAY,
    13239_i32 => _CYPRUS_NATIONAL_DAY,
    13259_i32 => _GOOD_FRIDAY,
    13261_i32 => _EASTER_SUNDAY,
    13262_i32 => _EASTER_MONDAY,
    13269_i32 => _LABOR_DAY,
    13311_i32 => _WHIT_MONDAY,
    13375_i32 => _ASSUMPTION_DAY,
    13422_i32 => _CYPRUS_INDEPENDENCE_DAY,
    13449_i32 => _GREEK_NATIONAL_DAY,
    13507_i32 => _CHRISTMAS_DAY,
    13508_i32 => _DAY_AFTER_CHRISTMAS,
    13514_i32 => _NEW_YEAR_S_DAY,
    13519_i32 => _EPIPHANY,
    13563_i32 => _GREEN_MONDAY,
    13597_i32 => _GREEK_INDEPENDENCE_DAY,
    13604_i32 => _CYPRUS_NATIONAL_DAY,
    13609_i32 => _GOOD_FRIDAY,
    13611_i32 => _EASTER_SUNDAY,
    13612_i32 => _EASTER_MONDAY,
    13634_i32 => _LABOR_DAY,
    13661_i32 => _WHIT_MONDAY,
    13740_i32 => _ASSUMPTION_DAY,
    13787_i32 => _CYPRUS_INDEPENDENCE_DAY,
    13814_i32 => _GREEK_NATIONAL_DAY,
    13872_i32 => _CHRISTMAS_DAY,
    13873_i32 => _DAY_AFTER_CHRISTMAS,
    13879_i32 => _NEW_YEAR_S_DAY,
    13884_i32 => _EPIPHANY,
    13948_i32 => _GREEN_MONDAY,
    13963_i32 => _GREEK_INDEPENDENCE_DAY,
    13970_i32 => _CYPRUS_NATIONAL_DAY,
    13994_i32 => _GOOD_FRIDAY,
    13996_i32 => _EASTER_SUNDAY,
    13997_i32 => _EASTER_MONDAY,
    14000_i32 => _LABOR_DAY,
    14046_i32 => _WHIT_MONDAY,
    14106_i32 => _ASSUMPTION_DAY,
    14153_i32 => _CYPRUS_INDEPENDENCE_DAY,
    14180_i32 => _GREEK_NATIONAL_DAY,
    14238_i32 => _CHRISTMAS_DAY,
    14239_i32 => _DAY_AFTER_CHRISTMAS,
    14245_i32 => _NEW_YEAR_S_DAY,
    14250_i32 => _EPIPHANY,
    14305_i32 => _GREEN_MONDAY,
    14328_i32 => _GREEK_INDEPENDENCE_DAY,
    14335_i32 => _CYPRUS_NATIONAL_DAY,
    14351_i32 => _GOOD_FRIDAY,
    14353_i32 => _EASTER_SUNDAY,
    14354_i32 => _EASTER_MONDAY,
    14365_i32 => _LABOR_DAY,
    14403_i32 => _WHIT_MONDAY,
    14471_i32 => _ASSUMPTION_DAY,
    14518_i32 => _CYPRUS_INDEPENDENCE_DAY,
    14545_i32 => _GREEK_NATIONAL_DAY,
    14603_i32 => _CHRISTMAS_DAY,
    14604_i32 => _DAY_AFTER_CHRISTMAS,
    14610_i32 => _NEW_YEAR_S_DAY,
    14615_i32 => _EPIPHANY,
    14655_i32 => _GREEN_MONDAY,
    14693_i32 => _GREEK_INDEPENDENCE_DAY,
    14700_i32 => _CYPRUS_NATIONAL_DAY,
    14701_i32 => _GOOD_FRIDAY,
    14703_i32 => _EASTER_SUNDAY,
    14704_i32 => _EASTER_MONDAY,
    14730_i32 => _LABOR_DAY,
    14753_i32 => _WHIT_MONDAY,
    14836_i32 => _ASSUMPTION_DAY,
    14883_i32 => _CYPRUS_INDEPENDENCE_DAY,
    14910_i32 => _GREEK_NATIONAL_DAY,
    14968_i32 => _CHRISTMAS_DAY,
    14969_i32 => _DAY_AFTER_CHRISTMAS,
    14975_i32 => _NEW_YEAR_S_DAY,
    14980_i32 => _EPIPHANY,
    15040_i32 => _GREEN_MONDAY,
    15058_i32 => _GREEK_INDEPENDENCE_DAY,
    15065_i32 => _CYPRUS_NATIONAL_DAY,
    15086_i32 => _GOOD_FRIDAY,
    15088_i32 => _EASTER_SUNDAY,
    15089_i32 => _EASTER_MONDAY,
    15095_i32 => _LABOR_DAY,
    15138_i32 => _WHIT_MONDAY,
    15201_i32 => _ASSUMPTION_DAY,
    15248_i32 => _CYPRUS_INDEPENDENCE_DAY,
    15275_i32 => _GREEK_NATIONAL_DAY,
    15333_i32 => _CHRISTMAS_DAY,
    15334_i32 => _DAY_AFTER_CHRISTMAS,
    15340_i32 => _NEW_YEAR_S_DAY,
    15345_i32 => _EPIPHANY,
    15397_i32 => _GREEN_MONDAY,
    15424_i32 => _GREEK_INDEPENDENCE_DAY,
    15431_i32 => _CYPRUS_NATIONAL_DAY,
    15443_i32 => _GOOD_FRIDAY,
    15445_i32 => _EASTER_SUNDAY,
    15446_i32 => _EASTER_MONDAY,
    15461_i32 => _LABOR_DAY,
    15495_i32 => _WHIT_MONDAY,
    15567_i32 => _ASSUMPTION_DAY,
    15614_i32 => _CYPRUS_INDEPENDENCE_DAY,
    15641_i32 => _GREEK_NATIONAL_DAY,
    15699_i32 => _CHRISTMAS_DAY,
    15700_i32 => _DAY_AFTER_CHRISTMAS,
    15706_i32 => _NEW_YEAR_S_DAY,
    15711_i32 => _EPIPHANY,
    15782_i32 => _GREEN_MONDAY,
    15789_i32 => _GREEK_INDEPENDENCE_DAY,
    15796_i32 => _CYPRUS_NATIONAL_DAY,
    15826_i32 => _LABOR_DAY,
    15828_i32 => _GOOD_FRIDAY,
    15830_i32 => _EASTER_SUNDAY,
    15831_i32 => _EASTER_MONDAY,
    15880_i32 => _WHIT_MONDAY,
    15932_i32 => _ASSUMPTION_DAY,
    15979_i32 => _CYPRUS_INDEPENDENCE_DAY,
    16006_i32 => _GREEK_NATIONAL_DAY,
    16064_i32 => _CHRISTMAS_DAY,
    16065_i32 => _DAY_AFTER_CHRISTMAS,
    16071_i32 => _NEW_YEAR_S_DAY,
    16076_i32 => _EPIPHANY,
    16132_i32 => _GREEN_MONDAY,
    16154_i32 => _GREEK_INDEPENDENCE_DAY,
    16161_i32 => _CYPRUS_NATIONAL_DAY,
    16178_i32 => _GOOD_FRIDAY,
    16180_i32 => _EASTER_SUNDAY,
    16181_i32 => _EASTER_MONDAY,
    16191_i32 => _LABOR_DAY,
    16230_i32 => _WHIT_MONDAY,
    16297_i32 => _ASSUMPTION_DAY,
    16344_i32 => _CYPRUS_INDEPENDENCE_DAY,
    16371_i32 => _GREEK_NATIONAL_DAY,
    16429_i32 => _CHRISTMAS_DAY,
    16430_i32 => _DAY_AFTER_CHRISTMAS,
    16436_i32 => _NEW_YEAR_S_DAY,
    16441_i32 => _EPIPHANY,
    16489_i32 => _GREEN_MONDAY,
    16519_i32 => _GREEK_INDEPENDENCE_DAY,
    16526_i32 => _CYPRUS_NATIONAL_DAY,
    16535_i32 => _GOOD_FRIDAY,
    16537_i32 => _EASTER_SUNDAY,
    16538_i32 => _EASTER_MONDAY,
    16556_i32 => _LABOR_DAY,
    16587_i32 => _WHIT_MONDAY,
    16662_i32 => _ASSUMPTION_DAY,
    16709_i32 => _CYPRUS_INDEPENDENCE_DAY,
    16736_i32 => _GREEK_NATIONAL_DAY,
    16794_i32 => _CHRISTMAS_DAY,
    16795_i32 => _DAY_AFTER_CHRISTMAS,
    16801_i32 => _NEW_YEAR_S_DAY,
    16806_i32 => _EPIPHANY,
    16874_i32 => _GREEN_MONDAY,
    16885_i32 => _GREEK_INDEPENDENCE_DAY,
    16892_i32 => _CYPRUS_NATIONAL_DAY,
    16920_i32 => _GOOD_FRIDAY,
    16922_i32 => _EASTER_SUNDAY__LABOR_DAY,
    16923_i32 => _EASTER_MONDAY,
    16972_i32 => _WHIT_MONDAY,
    17028_i32 => _ASSUMPTION_DAY,
    17075_i32 => _CYPRUS_INDEPENDENCE_DAY,
    17102_i32 => _GREEK_NATIONAL_DAY,
    17160_i32 => _CHRISTMAS_DAY,
    17161_i32 => _DAY_AFTER_CHRISTMAS,
    17167_i32 => _NEW_YEAR_S_DAY,
    17172_i32 => _EPIPHANY,
    17224_i32 => _GREEN_MONDAY,
    17250_i32 => _GREEK_INDEPENDENCE_DAY,
    17257_i32 => _CYPRUS_NATIONAL_DAY,
    17270_i32 => _GOOD_FRIDAY,
    17272_i32 => _EASTER_SUNDAY,
    17273_i32 => _EASTER_MONDAY,
    17287_i32 => _LABOR_DAY,
    17322_i32 => _WHIT_MONDAY,
    17393_i32 => _ASSUMPTION_DAY,
    17440_i32 => _CYPRUS_INDEPENDENCE_DAY,
    17467_i32 => _GREEK_NATIONAL_DAY,
    17525_i32 => _CHRISTMAS_DAY,
    17526_i32 => _DAY_AFTER_CHRISTMAS,
    17532_i32 => _NEW_YEAR_S_DAY,
    17537_i32 => _EPIPHANY,
    17581_i32 => _GREEN_MONDAY,
    17615_i32 => _GREEK_INDEPENDENCE_DAY,
    17622_i32 => _CYPRUS_NATIONAL_DAY,
    17627_i32 => _GOOD_FRIDAY,
    17629_i32 => _EASTER_SUNDAY,
    17630_i32 => _EASTER_MONDAY,
    17652_i32 => _LABOR_DAY,
    17679_i32 => _WHIT_MONDAY,
    17758_i32 => _ASSUMPTION_DAY,
    17805_i32 => _CYPRUS_INDEPENDENCE_DAY,
    17832_i32 => _GREEK_NATIONAL_DAY,
    17890_i32 => _CHRISTMAS_DAY,
    17891_i32 => _DAY_AFTER_CHRISTMAS,
    17897_i32 => _NEW_YEAR_S_DAY,
    17902_i32 => _EPIPHANY,
    17966_i32 => _GREEN_MONDAY,
    17980_i32 => _GREEK_INDEPENDENCE_DAY,
    17987_i32 => _CYPRUS_NATIONAL_DAY,
    18012_i32 => _GOOD_FRIDAY,
    18014_i32 => _EASTER_SUNDAY,
    18015_i32 => _EASTER_MONDAY,
    18017_i32 => _LABOR_DAY,
    18064_i32 => _WHIT_MONDAY,
    18123_i32 => _ASSUMPTION_DAY,
    18170_i32 => _CYPRUS_INDEPENDENCE_DAY,
    18197_i32 => _GREEK_NATIONAL_DAY,
    18255_i32 => _CHRISTMAS_DAY,
    18256_i32 => _DAY_AFTER_CHRISTMAS,
    18262_i32 => _NEW_YEAR_S_DAY,
    18267_i32 => _EPIPHANY,
    18323_i32 => _GREEN_MONDAY,
    18346_i32 => _GREEK_INDEPENDENCE_DAY,
    18353_i32 => _CYPRUS_NATIONAL_DAY,
    18369_i32 => _GOOD_FRIDAY,
    18371_i32 => _EASTER_SUNDAY,
    18372_i32 => _EASTER_MONDAY,
    18383_i32 => _LABOR_DAY,
    18421_i32 => _WHIT_MONDAY,
    18489_i32 => _ASSUMPTION_DAY,
    18536_i32 => _CYPRUS_INDEPENDENCE_DAY,
    18563_i32 => _GREEK_NATIONAL_DAY,
    18621_i32 => _CHRISTMAS_DAY,
    18622_i32 => _DAY_AFTER_CHRISTMAS,
    18628_i32 => _NEW_YEAR_S_DAY,
    18633_i32 => _EPIPHANY,
    18701_i32 => _GREEN_MONDAY,
    18711_i32 => _GREEK_INDEPENDENCE_DAY,
    18718_i32 => _CYPRUS_NATIONAL_DAY,
    18747_i32 => _GOOD_FRIDAY,
    18748_i32 => _LABOR_DAY,
    18749_i32 => _EASTER_SUNDAY,
    18750_i32 => _EASTER_MONDAY,
    18799_i32 => _WHIT_MONDAY,
    18854_i32 => _ASSUMPTION_DAY,
    18901_i32 => _CYPRUS_INDEPENDENCE_DAY,
    18928_i32 => _GREEK_NATIONAL_DAY,
    18986_i32 => _CHRISTMAS_DAY,
    18987_i32 => _DAY_AFTER_CHRISTMAS,
    18993_i32 => _NEW_YEAR_S_DAY,
    18998_i32 => _EPIPHANY,
    19058_i32 => _GREEN_MONDAY,
    19076_i32 => _GREEK_INDEPENDENCE_DAY,
    19083_i32 => _CYPRUS_NATIONAL_DAY,
    19104_i32 => _GOOD_FRIDAY,
    19106_i32 => _EASTER_SUNDAY,
    19107_i32 => _EASTER_MONDAY,
    19113_i32 => _LABOR_DAY,
    19156_i32 => _WHIT_MONDAY,
    19219_i32 => _ASSUMPTION_DAY,
    19266_i32 => _CYPRUS_INDEPENDENCE_DAY,
    19293_i32 => _GREEK_NATIONAL_DAY,
    19351_i32 => _CHRISTMAS_DAY,
    19352_i32 => _DAY_AFTER_CHRISTMAS,
    19358_i32 => _NEW_YEAR_S_DAY,
    19363_i32 => _EPIPHANY,
    19415_i32 => _GREEN_MONDAY,
    19441_i32 => _GREEK_INDEPENDENCE_DAY,
    19448_i32 => _CYPRUS_NATIONAL_DAY,
    19461_i32 => _GOOD_FRIDAY,
    19463_i32 => _EASTER_SUNDAY,
    19464_i32 => _EASTER_MONDAY,
    19478_i32 => _LABOR_DAY,
    19513_i32 => _WHIT_MONDAY,
    19584_i32 => _ASSUMPTION_DAY,
    19631_i32 => _CYPRUS_INDEPENDENCE_DAY,
    19658_i32 => _GREEK_NATIONAL_DAY,
    19716_i32 => _CHRISTMAS_DAY,
    19717_i32 => _DAY_AFTER_CHRISTMAS,
    19723_i32 => _NEW_YEAR_S_DAY,
    19728_i32 => _EPIPHANY,
    19800_i32 => _GREEN_MONDAY,
    19807_i32 => _GREEK_INDEPENDENCE_DAY,
    19814_i32 => _CYPRUS_NATIONAL_DAY,
    19844_i32 => _LABOR_DAY,
    19846_i32 => _GOOD_FRIDAY,
    19848_i32 => _EASTER_SUNDAY,
    19849_i32 => _EASTER_MONDAY,
    19898_i32 => _WHIT_MONDAY,
    19950_i32 => _ASSUMPTION_DAY,
    19997_i32 => _CYPRUS_INDEPENDENCE_DAY,
    20024_i32 => _GREEK_NATIONAL_DAY,
    20082_i32 => _CHRISTMAS_DAY,
    20083_i32 => _DAY_AFTER_CHRISTMAS,
    20089_i32 => _NEW_YEAR_S_DAY,
    20094_i32 => _EPIPHANY,
    20150_i32 => _GREEN_MONDAY,
    20172_i32 => _GREEK_INDEPENDENCE_DAY,
    20179_i32 => _CYPRUS_NATIONAL_DAY,
    20196_i32 => _GOOD_FRIDAY,
    20198_i32 => _EASTER_SUNDAY,
    20199_i32 => _EASTER_MONDAY,
    20209_i32 => _LABOR_DAY,
    20248_i32 => _WHIT_MONDAY,
    20315_i32 => _ASSUMPTION_DAY,
    20362_i32 => _CYPRUS_INDEPENDENCE_DAY,
    20389_i32 => _GREEK_NATIONAL_DAY,
    20447_i32 => _CHRISTMAS_DAY,
    20448_i32 => _DAY_AFTER_CHRISTMAS,
    20454_i32 => _NEW_YEAR_S_DAY,
    20459_i32 => _EPIPHANY,
    20507_i32 => _GREEN_MONDAY,
    20537_i32 => _GREEK_INDEPENDENCE_DAY,
    20544_i32 => _CYPRUS_NATIONAL_DAY,
    20553_i32 => _GOOD_FRIDAY,
    20555_i32 => _EASTER_SUNDAY,
    20556_i32 => _EASTER_MONDAY,
    20574_i32 => _LABOR_DAY,
    20605_i32 => _WHIT_MONDAY,
    20680_i32 => _ASSUMPTION_DAY,
    20727_i32 => _CYPRUS_INDEPENDENCE_DAY,
    20754_i32 => _GREEK_NATIONAL_DAY,
    20812_i32 => _CHRISTMAS_DAY,
    20813_i32 => _DAY_AFTER_CHRISTMAS,
    20819_i32 => _NEW_YEAR_S_DAY,
    20824_i32 => _EPIPHANY,
    20892_i32 => _GREEN_MONDAY,
    20902_i32 => _GREEK_INDEPENDENCE_DAY,
    20909_i32 => _CYPRUS_NATIONAL_DAY,
    20938_i32 => _GOOD_FRIDAY,
    20939_i32 => _LABOR_DAY,
    20940_i32 => _EASTER_SUNDAY,
    20941_i32 => _EASTER_MONDAY,
    20990_i32 => _WHIT_MONDAY,
    21045_i32 => _ASSUMPTION_DAY,
    21092_i32 => _CYPRUS_INDEPENDENCE_DAY,
    21119_i32 => _GREEK_NATIONAL_DAY,
    21177_i32 => _CHRISTMAS_DAY,
    21178_i32 => _DAY_AFTER_CHRISTMAS,
    21184_i32 => _NEW_YEAR_S_DAY,
    21189_i32 => _EPIPHANY,
    21242_i32 => _GREEN_MONDAY,
    21268_i32 => _GREEK_INDEPENDENCE_DAY,
    21275_i32 => _CYPRUS_NATIONAL_DAY,
    21288_i32 => _GOOD_FRIDAY,
    21290_i32 => _EASTER_SUNDAY,
    21291_i32 => _EASTER_MONDAY,
    21305_i32 => _LABOR_DAY,
    21340_i32 => _WHIT_MONDAY,
    21411_i32 => _ASSUMPTION_DAY,
    21458_i32 => _CYPRUS_INDEPENDENCE_DAY,
    21485_i32 => _GREEK_NATIONAL_DAY,
    21543_i32 => _CHRISTMAS_DAY,
    21544_i32 => _DAY_AFTER_CHRISTMAS,
    21550_i32 => _NEW_YEAR_S_DAY,
    21555_i32 => _EPIPHANY,
    21599_i32 => _GREEN_MONDAY,
    21633_i32 => _GREEK_INDEPENDENCE_DAY,
    21640_i32 => _CYPRUS_NATIONAL_DAY,
    21645_i32 => _GOOD_FRIDAY,
    21647_i32 => _EASTER_SUNDAY,
    21648_i32 => _EASTER_MONDAY,
    21670_i32 => _LABOR_DAY,
    21697_i32 => _WHIT_MONDAY,
    21776_i32 => _ASSUMPTION_DAY,
    21823_i32 => _CYPRUS_INDEPENDENCE_DAY,
    21850_i32 => _GREEK_NATIONAL_DAY,
    21908_i32 => _CHRISTMAS_DAY,
    21909_i32 => _DAY_AFTER_CHRISTMAS,
    21915_i32 => _NEW_YEAR_S_DAY,
    21920_i32 => _EPIPHANY,
    21984_i32 => _GREEN_MONDAY,
    21998_i32 => _GREEK_INDEPENDENCE_DAY,
    22005_i32 => _CYPRUS_NATIONAL_DAY,
    22030_i32 => _GOOD_FRIDAY,
    22032_i32 => _EASTER_SUNDAY,
    22033_i32 => _EASTER_MONDAY,
    22035_i32 => _LABOR_DAY,
    22082_i32 => _WHIT_MONDAY,
    22141_i32 => _ASSUMPTION_DAY,
    22188_i32 => _CYPRUS_INDEPENDENCE_DAY,
    22215_i32 => _GREEK_NATIONAL_DAY,
    22273_i32 => _CHRISTMAS_DAY,
    22274_i32 => _DAY_AFTER_CHRISTMAS,
    22280_i32 => _NEW_YEAR_S_DAY,
    22285_i32 => _EPIPHANY,
    22334_i32 => _GREEN_MONDAY,
    22363_i32 => _GREEK_INDEPENDENCE_DAY,
    22370_i32 => _CYPRUS_NATIONAL_DAY,
    22380_i32 => _GOOD_FRIDAY,
    22382_i32 => _EASTER_SUNDAY,
    22383_i32 => _EASTER_MONDAY,
    22400_i32 => _LABOR_DAY,
    22432_i32 => _WHIT_MONDAY,
    22506_i32 => _ASSUMPTION_DAY,
    22553_i32 => _CYPRUS_INDEPENDENCE_DAY,
    22580_i32 => _GREEK_NATIONAL_DAY,
    22638_i32 => _CHRISTMAS_DAY,
    22639_i32 => _DAY_AFTER_CHRISTMAS,
    22645_i32 => _NEW_YEAR_S_DAY,
    22650_i32 => _EPIPHANY,
    22719_i32 => _GREEN_MONDAY,
    22729_i32 => _GREEK_INDEPENDENCE_DAY,
    22736_i32 => _CYPRUS_NATIONAL_DAY,
    22765_i32 => _GOOD_FRIDAY,
    22766_i32 => _LABOR_DAY,
    22767_i32 => _EASTER_SUNDAY,
    22768_i32 => _EASTER_MONDAY,
    22817_i32 => _WHIT_MONDAY,
    22872_i32 => _ASSUMPTION_DAY,
    22919_i32 => _CYPRUS_INDEPENDENCE_DAY,
    22946_i32 => _GREEK_NATIONAL_DAY,
    23004_i32 => _CHRISTMAS_DAY,
    23005_i32 => _DAY_AFTER_CHRISTMAS,
    23011_i32 => _NEW_YEAR_S_DAY,
    23016_i32 => _EPIPHANY,
    23076_i32 => _GREEN_MONDAY,
    23094_i32 => _GREEK_INDEPENDENCE_DAY,
    23101_i32 => _CYPRUS_NATIONAL_DAY,
    23122_i32 => _GOOD_FRIDAY,
    23124_i32 => _EASTER_SUNDAY,
    23125_i32 => _EASTER_MONDAY,
    23131_i32 => _LABOR_DAY,
    23174_i32 => _WHIT_MONDAY,
    23237_i32 => _ASSUMPTION_DAY,
    23284_i32 => _CYPRUS_INDEPENDENCE_DAY,
    23311_i32 => _GREEK_NATIONAL_DAY,
    23369_i32 => _CHRISTMAS_DAY,
    23370_i32 => _DAY_AFTER_CHRISTMAS,
    23376_i32 => _NEW_YEAR_S_DAY,
    23381_i32 => _EPIPHANY,
    23426_i32 => _GREEN_MONDAY,
    23459_i32 => _GREEK_INDEPENDENCE_DAY,
    23466_i32 => _CYPRUS_NATIONAL_DAY,
    23472_i32 => _GOOD_FRIDAY,
    23474_i32 => _EASTER_SUNDAY,
    23475_i32 => _EASTER_MONDAY,
    23496_i32 => _LABOR_DAY,
    23524_i32 => _WHIT_MONDAY,
    23602_i32 => _ASSUMPTION_DAY,
    23649_i32 => _CYPRUS_INDEPENDENCE_DAY,
    23676_i32 => _GREEK_NATIONAL_DAY,
    23734_i32 => _CHRISTMAS_DAY,
    23735_i32 => _DAY_AFTER_CHRISTMAS,
    23741_i32 => _NEW_YEAR_S_DAY,
    23746_i32 => _EPIPHANY,
    23811_i32 => _GREEN_MONDAY,
    23824_i32 => _GREEK_INDEPENDENCE_DAY,
    23831_i32 => _CYPRUS_NATIONAL_DAY,
    23857_i32 => _GOOD_FRIDAY,
    23859_i32 => _EASTER_SUNDAY,
    23860_i32 => _EASTER_MONDAY,
    23861_i32 => _LABOR_DAY,
    23909_i32 => _WHIT_MONDAY,
    23967_i32 => _ASSUMPTION_DAY,
    24014_i32 => _CYPRUS_INDEPENDENCE_DAY,
    24041_i32 => _GREEK_NATIONAL_DAY,
    24099_i32 => _CHRISTMAS_DAY,
    24100_i32 => _DAY_AFTER_CHRISTMAS,
    24106_i32 => _NEW_YEAR_S_DAY,
    24111_i32 => _EPIPHANY,
    24168_i32 => _GREEN_MONDAY,
    24190_i32 => _GREEK_INDEPENDENCE_DAY,
    24197_i32 => _CYPRUS_NATIONAL_DAY,
    24214_i32 => _GOOD_FRIDAY,
    24216_i32 => _EASTER_SUNDAY,
    24217_i32 => _EASTER_MONDAY,
    24227_i32 => _LABOR_DAY,
    24266_i32 => _WHIT_MONDAY,
    24333_i32 => _ASSUMPTION_DAY,
    24380_i32 => _CYPRUS_INDEPENDENCE_DAY,
    24407_i32 => _GREEK_NATIONAL_DAY,
    24465_i32 => _CHRISTMAS_DAY,
    24466_i32 => _DAY_AFTER_CHRISTMAS,
    24472_i32 => _NEW_YEAR_S_DAY,
    24477_i32 => _EPIPHANY,
    24518_i32 => _GREEN_MONDAY,
    24555_i32 => _GREEK_INDEPENDENCE_DAY,
    24562_i32 => _CYPRUS_NATIONAL_DAY,
    24564_i32 => _GOOD_FRIDAY,
    24566_i32 => _EASTER_SUNDAY,
    24567_i32 => _EASTER_MONDAY,
    24592_i32 => _LABOR_DAY,
    24616_i32 => _WHIT_MONDAY,
    24698_i32 => _ASSUMPTION_DAY,
    24745_i32 => _CYPRUS_INDEPENDENCE_DAY,
    24772_i32 => _GREEK_NATIONAL_DAY,
    24830_i32 => _CHRISTMAS_DAY,
    24831_i32 => _DAY_AFTER_CHRISTMAS,
    24837_i32 => _NEW_YEAR_S_DAY,
    24842_i32 => _EPIPHANY,
    24903_i32 => _GREEN_MONDAY,
    24920_i32 => _GREEK_INDEPENDENCE_DAY,
    24927_i32 => _CYPRUS_NATIONAL_DAY,
    24949_i32 => _GOOD_FRIDAY,
    24951_i32 => _EASTER_SUNDAY,
    24952_i32 => _EASTER_MONDAY,
    24957_i32 => _LABOR_DAY,
    25001_i32 => _WHIT_MONDAY,
    25063_i32 => _ASSUMPTION_DAY,
    25110_i32 => _CYPRUS_INDEPENDENCE_DAY,
    25137_i32 => _GREEK_NATIONAL_DAY,
    25195_i32 => _CHRISTMAS_DAY,
    25196_i32 => _DAY_AFTER_CHRISTMAS,
    25202_i32 => _NEW_YEAR_S_DAY,
    25207_i32 => _EPIPHANY,
    25260_i32 => _GREEN_MONDAY,
    25285_i32 => _GREEK_INDEPENDENCE_DAY,
    25292_i32 => _CYPRUS_NATIONAL_DAY,
    25306_i32 => _GOOD_FRIDAY,
    25308_i32 => _EASTER_SUNDAY,
    25309_i32 => _EASTER_MONDAY,
    25322_i32 => _LABOR_DAY,
    25358_i32 => _WHIT_MONDAY,
    25428_i32 => _ASSUMPTION_DAY,
    25475_i32 => _CYPRUS_INDEPENDENCE_DAY,
    25502_i32 => _GREEK_NATIONAL_DAY,
    25560_i32 => _CHRISTMAS_DAY,
    25561_i32 => _DAY_AFTER_CHRISTMAS,
    25567_i32 => _NEW_YEAR_S_DAY,
    25572_i32 => _EPIPHANY,
    25645_i32 => _GREEN_MONDAY,
    25651_i32 => _GREEK_INDEPENDENCE_DAY,
    25658_i32 => _CYPRUS_NATIONAL_DAY,
    25688_i32 => _LABOR_DAY,
    25691_i32 => _GOOD_FRIDAY,
    25693_i32 => _EASTER_SUNDAY,
    25694_i32 => _EASTER_MONDAY,
    25743_i32 => _WHIT_MONDAY,
    25794_i32 => _ASSUMPTION_DAY,
    25841_i32 => _CYPRUS_INDEPENDENCE_DAY,
    25868_i32 => _GREEK_NATIONAL_DAY,
    25926_i32 => _CHRISTMAS_DAY,
    25927_i32 => _DAY_AFTER_CHRISTMAS,
    25933_i32 => _NEW_YEAR_S_DAY,
    25938_i32 => _EPIPHANY,
    25995_i32 => _GREEN_MONDAY,
    26016_i32 => _GREEK_INDEPENDENCE_DAY,
    26023_i32 => _CYPRUS_NATIONAL_DAY,
    26041_i32 => _GOOD_FRIDAY,
    26043_i32 => _EASTER_SUNDAY,
    26044_i32 => _EASTER_MONDAY,
    26053_i32 => _LABOR_DAY,
    26093_i32 => _WHIT_MONDAY,
    26159_i32 => _ASSUMPTION_DAY,
    26206_i32 => _CYPRUS_INDEPENDENCE_DAY,
    26233_i32 => _GREEK_NATIONAL_DAY,
    26291_i32 => _CHRISTMAS_DAY,
    26292_i32 => _DAY_AFTER_CHRISTMAS,
    26298_i32 => _NEW_YEAR_S_DAY,
    26303_i32 => _EPIPHANY,
    26352_i32 => _GREEN_MONDAY,
    26381_i32 => _GREEK_INDEPENDENCE_DAY,
    26388_i32 => _CYPRUS_NATIONAL_DAY,
    26398_i32 => _GOOD_FRIDAY,
    26400_i32 => _EASTER_SUNDAY,
    26401_i32 => _EASTER_MONDAY,
    26418_i32 => _LABOR_DAY,
    26450_i32 => _WHIT_MONDAY,
    26524_i32 => _ASSUMPTION_DAY,
    26571_i32 => _CYPRUS_INDEPENDENCE_DAY,
    26598_i32 => _GREEK_NATIONAL_DAY,
    26656_i32 => _CHRISTMAS_DAY,
    26657_i32 => _DAY_AFTER_CHRISTMAS,
    26663_i32 => _NEW_YEAR_S_DAY,
    26668_i32 => _EPIPHANY,
    26737_i32 => _GREEN_MONDAY,
    26746_i32 => _GREEK_INDEPENDENCE_DAY,
    26753_i32 => _CYPRUS_NATIONAL_DAY,
    26783_i32 => _GOOD_FRIDAY__LABOR_DAY,
    26785_i32 => _EASTER_SUNDAY,
    26786_i32 => _EASTER_MONDAY,
    26835_i32 => _WHIT_MONDAY,
    26889_i32 => _ASSUMPTION_DAY,
    26936_i32 => _CYPRUS_INDEPENDENCE_DAY,
    26963_i32 => _GREEK_NATIONAL_DAY,
    27021_i32 => _CHRISTMAS_DAY,
    27022_i32 => _DAY_AFTER_CHRISTMAS,
    27028_i32 => _NEW_YEAR_S_DAY,
    27033_i32 => _EPIPHANY,
    27094_i32 => _GREEN_MONDAY,
    27112_i32 => _GREEK_INDEPENDENCE_DAY,
    27119_i32 => _CYPRUS_NATIONAL_DAY,
    27140_i32 => _GOOD_FRIDAY,
    27142_i32 => _EASTER_SUNDAY,
    27143_i32 => _EASTER_MONDAY,
    27149_i32 => _LABOR_DAY,
    27192_i32 => _WHIT_MONDAY,
    27255_i32 => _ASSUMPTION_DAY,
    27302_i32 => _CYPRUS_INDEPENDENCE_DAY,
    27329_i32 => _GREEK_NATIONAL_DAY,
    27387_i32 => _CHRISTMAS_DAY,
    27388_i32 => _DAY_AFTER_CHRISTMAS,
    27394_i32 => _NEW_YEAR_S_DAY,
    27399_i32 => _EPIPHANY,
    27444_i32 => _GREEN_MONDAY,
    27477_i32 => _GREEK_INDEPENDENCE_DAY,
    27484_i32 => _CYPRUS_NATIONAL_DAY,
    27490_i32 => _GOOD_FRIDAY,
    27492_i32 => _EASTER_SUNDAY,
    27493_i32 => _EASTER_MONDAY,
    27514_i32 => _LABOR_DAY,
    27542_i32 => _WHIT_MONDAY,
    27620_i32 => _ASSUMPTION_DAY,
    27667_i32 => _CYPRUS_INDEPENDENCE_DAY,
    27694_i32 => _GREEK_NATIONAL_DAY,
    27752_i32 => _CHRISTMAS_DAY,
    27753_i32 => _DAY_AFTER_CHRISTMAS,
    27759_i32 => _NEW_YEAR_S_DAY,
    27764_i32 => _EPIPHANY,
    27829_i32 => _GREEN_MONDAY,
    27842_i32 => _GREEK_INDEPENDENCE_DAY,
    27849_i32 => _CYPRUS_NATIONAL_DAY,
    27875_i32 => _GOOD_FRIDAY,
    27877_i32 => _EASTER_SUNDAY,
    27878_i32 => _EASTER_MONDAY,
    27879_i32 => _LABOR_DAY,
    27927_i32 => _WHIT_MONDAY,
    27985_i32 => _ASSUMPTION_DAY,
    28032_i32 => _CYPRUS_INDEPENDENCE_DAY,
    28059_i32 => _GREEK_NATIONAL_DAY,
    28117_i32 => _CHRISTMAS_DAY,
    28118_i32 => _DAY_AFTER_CHRISTMAS,
    28124_i32 => _NEW_YEAR_S_DAY,
    28129_i32 => _EPIPHANY,
    28186_i32 => _GREEN_MONDAY,
    28207_i32 => _GREEK_INDEPENDENCE_DAY,
    28214_i32 => _CYPRUS_NATIONAL_DAY,
    28232_i32 => _GOOD_FRIDAY,
    28234_i32 => _EASTER_SUNDAY,
    28235_i32 => _EASTER_MONDAY,
    28244_i32 => _LABOR_DAY,
    28284_i32 => _WHIT_MONDAY,
    28350_i32 => _ASSUMPTION_DAY,
    28397_i32 => _CYPRUS_INDEPENDENCE_DAY,
    28424_i32 => _GREEK_NATIONAL_DAY,
    28482_i32 => _CHRISTMAS_DAY,
    28483_i32 => _DAY_AFTER_CHRISTMAS,
    28489_i32 => _NEW_YEAR_S_DAY,
    28494_i32 => _EPIPHANY,
    28536_i32 => _GREEN_MONDAY,
    28573_i32 => _GREEK_INDEPENDENCE_DAY,
    28580_i32 => _CYPRUS_NATIONAL_DAY,
    28582_i32 => _GOOD_FRIDAY,
    28584_i32 => _EASTER_SUNDAY,
    28585_i32 => _EASTER_MONDAY,
    28610_i32 => _LABOR_DAY,
    28634_i32 => _WHIT_MONDAY,
    28716_i32 => _ASSUMPTION_DAY,
    28763_i32 => _CYPRUS_INDEPENDENCE_DAY,
    28790_i32 => _GREEK_NATIONAL_DAY,
    28848_i32 => _CHRISTMAS_DAY,
    28849_i32 => _DAY_AFTER_CHRISTMAS,
    28855_i32 => _NEW_YEAR_S_DAY,
    28860_i32 => _EPIPHANY,
    28921_i32 => _GREEN_MONDAY,
    28938_i32 => _GREEK_INDEPENDENCE_DAY,
    28945_i32 => _CYPRUS_NATIONAL_DAY,
    28967_i32 => _GOOD_FRIDAY,
    28969_i32 => _EASTER_SUNDAY,
    28970_i32 => _EASTER_MONDAY,
    28975_i32 => _LABOR_DAY,
    29019_i32 => _WHIT_MONDAY,
    29081_i32 => _ASSUMPTION_DAY,
    29128_i32 => _CYPRUS_INDEPENDENCE_DAY,
    29155_i32 => _GREEK_NATIONAL_DAY,
    29213_i32 => _CHRISTMAS_DAY,
    29214_i32 => _DAY_AFTER_CHRISTMAS,
    29220_i32 => _NEW_YEAR_S_DAY,
};
