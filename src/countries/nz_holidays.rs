use crate::countries::constants::*;
use phf::phf_map;

pub static NZ_HOLIDAYS: phf::Map<i32, &'static str> = phf_map! {
    10957_i32 => _NEW_YEAR_S_DAY,
    10958_i32 => _DAY_AFTER_NEW_YEAR_S_DAY,
    10959_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    10960_i32 => _DAY_AFTER_NEW_YEAR_S_DAY__OBSERVED_,
    10993_i32 => _WAITANGI_DAY,
    11068_i32 => _GOOD_FRIDAY,
    11071_i32 => _EASTER_MONDAY,
    11072_i32 => _ANZAC_DAY,
    11113_i32 => _QUEEN_S_BIRTHDAY,
    11253_i32 => _LABOUR_DAY,
    11316_i32 => _CHRISTMAS_DAY,
    11317_i32 => _BOXING_DAY,
    11323_i32 => _NEW_YEAR_S_DAY,
    11324_i32 => _DAY_AFTER_NEW_YEAR_S_DAY,
    11359_i32 => _WAITANGI_DAY,
    11425_i32 => _GOOD_FRIDAY,
    11428_i32 => _EASTER_MONDAY,
    11437_i32 => _ANZAC_DAY,
    11477_i32 => _QUEEN_S_BIRTHDAY,
    11617_i32 => _LABOUR_DAY,
    11681_i32 => _CHRISTMAS_DAY,
    11682_i32 => _BOXING_DAY,
    11688_i32 => _NEW_YEAR_S_DAY,
    11689_i32 => _DAY_AFTER_NEW_YEAR_S_DAY,
    11724_i32 => _WAITANGI_DAY,
    11775_i32 => _GOOD_FRIDAY,
    11778_i32 => _EASTER_MONDAY,
    11802_i32 => _ANZAC_DAY,
    11841_i32 => _QUEEN_S_BIRTHDAY,
    11988_i32 => _LABOUR_DAY,
    12046_i32 => _CHRISTMAS_DAY,
    12047_i32 => _BOXING_DAY,
    12053_i32 => _NEW_YEAR_S_DAY,
    12054_i32 => _DAY_AFTER_NEW_YEAR_S_DAY,
    12089_i32 => _WAITANGI_DAY,
    12160_i32 => _GOOD_FRIDAY,
    12163_i32 => _EASTER_MONDAY,
    12167_i32 => _ANZAC_DAY,
    12205_i32 => _QUEEN_S_BIRTHDAY,
    12352_i32 => _LABOUR_DAY,
    12411_i32 => _CHRISTMAS_DAY,
    12412_i32 => _BOXING_DAY,
    12418_i32 => _NEW_YEAR_S_DAY,
    12419_i32 => _DAY_AFTER_NEW_YEAR_S_DAY,
    12454_i32 => _WAITANGI_DAY,
    12517_i32 => _GOOD_FRIDAY,
    12520_i32 => _EASTER_MONDAY,
    12533_i32 => _ANZAC_DAY,
    12576_i32 => _QUEEN_S_BIRTHDAY,
    12716_i32 => _LABOUR_DAY,
    12777_i32 => _CHRISTMAS_DAY,
    12778_i32 => _BOXING_DAY,
    12779_i32 => _CHRISTMAS_DAY__OBSERVED_,
    12780_i32 => _BOXING_DAY__OBSERVED_,
    12784_i32 => _NEW_YEAR_S_DAY,
    12785_i32 => _DAY_AFTER_NEW_YEAR_S_DAY,
    12786_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    12787_i32 => _DAY_AFTER_NEW_YEAR_S_DAY__OBSERVED_,
    12820_i32 => _WAITANGI_DAY,
    12867_i32 => _GOOD_FRIDAY,
    12870_i32 => _EASTER_MONDAY,
    12898_i32 => _ANZAC_DAY,
    12940_i32 => _QUEEN_S_BIRTHDAY,
    13080_i32 => _LABOUR_DAY,
    13142_i32 => _CHRISTMAS_DAY,
    13143_i32 => _BOXING_DAY,
    13144_i32 => _CHRISTMAS_DAY__OBSERVED_,
    13149_i32 => _NEW_YEAR_S_DAY,
    13150_i32 => _DAY_AFTER_NEW_YEAR_S_DAY,
    13151_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    13185_i32 => _WAITANGI_DAY,
    13252_i32 => _GOOD_FRIDAY,
    13255_i32 => _EASTER_MONDAY,
    13263_i32 => _ANZAC_DAY,
    13304_i32 => _QUEEN_S_BIRTHDAY,
    13444_i32 => _LABOUR_DAY,
    13507_i32 => _CHRISTMAS_DAY,
    13508_i32 => _BOXING_DAY,
    13514_i32 => _NEW_YEAR_S_DAY,
    13515_i32 => _DAY_AFTER_NEW_YEAR_S_DAY,
    13550_i32 => _WAITANGI_DAY,
    13609_i32 => _GOOD_FRIDAY,
    13612_i32 => _EASTER_MONDAY,
    13628_i32 => _ANZAC_DAY,
    13668_i32 => _QUEEN_S_BIRTHDAY,
    13808_i32 => _LABOUR_DAY,
    13872_i32 => _CHRISTMAS_DAY,
    13873_i32 => _BOXING_DAY,
    13879_i32 => _NEW_YEAR_S_DAY,
    13880_i32 => _DAY_AFTER_NEW_YEAR_S_DAY,
    13915_i32 => _WAITANGI_DAY,
    13959_i32 => _GOOD_FRIDAY,
    13962_i32 => _EASTER_MONDAY,
    13994_i32 => _ANZAC_DAY,
    14032_i32 => _QUEEN_S_BIRTHDAY,
    14179_i32 => _LABOUR_DAY,
    14238_i32 => _CHRISTMAS_DAY,
    14239_i32 => _BOXING_DAY,
    14245_i32 => _NEW_YEAR_S_DAY,
    14246_i32 => _DAY_AFTER_NEW_YEAR_S_DAY,
    14281_i32 => _WAITANGI_DAY,
    14344_i32 => _GOOD_FRIDAY,
    14347_i32 => _EASTER_MONDAY,
    14359_i32 => _ANZAC_DAY,
    14396_i32 => _QUEEN_S_BIRTHDAY,
    14543_i32 => _LABOUR_DAY,
    14603_i32 => _CHRISTMAS_DAY,
    14604_i32 => _BOXING_DAY,
    14606_i32 => _BOXING_DAY__OBSERVED_,
    14610_i32 => _NEW_YEAR_S_DAY,
    14611_i32 => _DAY_AFTER_NEW_YEAR_S_DAY,
    14613_i32 => _DAY_AFTER_NEW_YEAR_S_DAY__OBSERVED_,
    14646_i32 => _WAITANGI_DAY,
    14701_i32 => _GOOD_FRIDAY,
    14704_i32 => _EASTER_MONDAY,
    14724_i32 => _ANZAC_DAY,
    14767_i32 => _QUEEN_S_BIRTHDAY,
    14907_i32 => _LABOUR_DAY,
    14968_i32 => _CHRISTMAS_DAY,
    14969_i32 => _BOXING_DAY,
    14970_i32 => _CHRISTMAS_DAY__OBSERVED_,
    14971_i32 => _BOXING_DAY__OBSERVED_,
    14975_i32 => _NEW_YEAR_S_DAY,
    14976_i32 => _DAY_AFTER_NEW_YEAR_S_DAY,
    14977_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    14978_i32 => _DAY_AFTER_NEW_YEAR_S_DAY__OBSERVED_,
    15011_i32 => _WAITANGI_DAY,
    15086_i32 => _GOOD_FRIDAY,
    15089_i32 => _ANZAC_DAY__EASTER_MONDAY,
    15131_i32 => _QUEEN_S_BIRTHDAY,
    15271_i32 => _LABOUR_DAY,
    15333_i32 => _CHRISTMAS_DAY,
    15334_i32 => _BOXING_DAY,
    15335_i32 => _CHRISTMAS_DAY__OBSERVED_,
    15340_i32 => _NEW_YEAR_S_DAY,
    15341_i32 => _DAY_AFTER_NEW_YEAR_S_DAY,
    15342_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    15376_i32 => _WAITANGI_DAY,
    15436_i32 => _GOOD_FRIDAY,
    15439_i32 => _EASTER_MONDAY,
    15455_i32 => _ANZAC_DAY,
    15495_i32 => _QUEEN_S_BIRTHDAY,
    15635_i32 => _LABOUR_DAY,
    15699_i32 => _CHRISTMAS_DAY,
    15700_i32 => _BOXING_DAY,
    15706_i32 => _NEW_YEAR_S_DAY,
    15707_i32 => _DAY_AFTER_NEW_YEAR_S_DAY,
    15742_i32 => _WAITANGI_DAY,
    15793_i32 => _GOOD_FRIDAY,
    15796_i32 => _EASTER_MONDAY,
    15820_i32 => _ANZAC_DAY,
    15859_i32 => _QUEEN_S_BIRTHDAY,
    16006_i32 => _LABOUR_DAY,
    16064_i32 => _CHRISTMAS_DAY,
    16065_i32 => _BOXING_DAY,
    16071_i32 => _NEW_YEAR_S_DAY,
    16072_i32 => _DAY_AFTER_NEW_YEAR_S_DAY,
    16107_i32 => _WAITANGI_DAY,
    16178_i32 => _GOOD_FRIDAY,
    16181_i32 => _EASTER_MONDAY,
    16185_i32 => _ANZAC_DAY,
    16223_i32 => _QUEEN_S_BIRTHDAY,
    16370_i32 => _LABOUR_DAY,
    16429_i32 => _CHRISTMAS_DAY,
    16430_i32 => _BOXING_DAY,
    16436_i32 => _NEW_YEAR_S_DAY,
    16437_i32 => _DAY_AFTER_NEW_YEAR_S_DAY,
    16472_i32 => _WAITANGI_DAY,
    16528_i32 => _GOOD_FRIDAY,
    16531_i32 => _EASTER_MONDAY,
    16550_i32 => _ANZAC_DAY,
    16552_i32 => _ANZAC_DAY__OBSERVED_,
    16587_i32 => _QUEEN_S_BIRTHDAY,
    16734_i32 => _LABOUR_DAY,
    16794_i32 => _CHRISTMAS_DAY,
    16795_i32 => _BOXING_DAY,
    16797_i32 => _BOXING_DAY__OBSERVED_,
    16801_i32 => _NEW_YEAR_S_DAY,
    16802_i32 => _DAY_AFTER_NEW_YEAR_S_DAY,
    16804_i32 => _DAY_AFTER_NEW_YEAR_S_DAY__OBSERVED_,
    16837_i32 => _WAITANGI_DAY,
    16839_i32 => _WAITANGI_DAY__OBSERVED_,
    16885_i32 => _GOOD_FRIDAY,
    16888_i32 => _EASTER_MONDAY,
    16916_i32 => _ANZAC_DAY,
    16958_i32 => _QUEEN_S_BIRTHDAY,
    17098_i32 => _LABOUR_DAY,
    17160_i32 => _CHRISTMAS_DAY,
    17161_i32 => _BOXING_DAY,
    17162_i32 => _CHRISTMAS_DAY__OBSERVED_,
    17167_i32 => _NEW_YEAR_S_DAY,
    17168_i32 => _DAY_AFTER_NEW_YEAR_S_DAY,
    17169_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    17203_i32 => _WAITANGI_DAY,
    17270_i32 => _GOOD_FRIDAY,
    17273_i32 => _EASTER_MONDAY,
    17281_i32 => _ANZAC_DAY,
    17322_i32 => _QUEEN_S_BIRTHDAY,
    17462_i32 => _LABOUR_DAY,
    17525_i32 => _CHRISTMAS_DAY,
    17526_i32 => _BOXING_DAY,
    17532_i32 => _NEW_YEAR_S_DAY,
    17533_i32 => _DAY_AFTER_NEW_YEAR_S_DAY,
    17568_i32 => _WAITANGI_DAY,
    17620_i32 => _GOOD_FRIDAY,
    17623_i32 => _EASTER_MONDAY,
    17646_i32 => _ANZAC_DAY,
    17686_i32 => _QUEEN_S_BIRTHDAY,
    17826_i32 => _LABOUR_DAY,
    17890_i32 => _CHRISTMAS_DAY,
    17891_i32 => _BOXING_DAY,
    17897_i32 => _NEW_YEAR_S_DAY,
    17898_i32 => _DAY_AFTER_NEW_YEAR_S_DAY,
    17933_i32 => _WAITANGI_DAY,
    18005_i32 => _GOOD_FRIDAY,
    18008_i32 => _EASTER_MONDAY,
    18011_i32 => _ANZAC_DAY,
    18050_i32 => _QUEEN_S_BIRTHDAY,
    18197_i32 => _LABOUR_DAY,
    18255_i32 => _CHRISTMAS_DAY,
    18256_i32 => _BOXING_DAY,
    18262_i32 => _NEW_YEAR_S_DAY,
    18263_i32 => _DAY_AFTER_NEW_YEAR_S_DAY,
    18298_i32 => _WAITANGI_DAY,
    18362_i32 => _GOOD_FRIDAY,
    18365_i32 => _EASTER_MONDAY,
    18377_i32 => _ANZAC_DAY,
    18379_i32 => _ANZAC_DAY__OBSERVED_,
    18414_i32 => _QUEEN_S_BIRTHDAY,
    18561_i32 => _LABOUR_DAY,
    18621_i32 => _CHRISTMAS_DAY,
    18622_i32 => _BOXING_DAY,
    18624_i32 => _BOXING_DAY__OBSERVED_,
    18628_i32 => _NEW_YEAR_S_DAY,
    18629_i32 => _DAY_AFTER_NEW_YEAR_S_DAY,
    18631_i32 => _DAY_AFTER_NEW_YEAR_S_DAY__OBSERVED_,
    18664_i32 => _WAITANGI_DAY,
    18666_i32 => _WAITANGI_DAY__OBSERVED_,
    18719_i32 => _GOOD_FRIDAY,
    18722_i32 => _EASTER_MONDAY,
    18742_i32 => _ANZAC_DAY,
    18743_i32 => _ANZAC_DAY__OBSERVED_,
    18785_i32 => _QUEEN_S_BIRTHDAY,
    18925_i32 => _LABOUR_DAY,
    18986_i32 => _CHRISTMAS_DAY,
    18987_i32 => _BOXING_DAY,
    18988_i32 => _CHRISTMAS_DAY__OBSERVED_,
    18989_i32 => _BOXING_DAY__OBSERVED_,
    18993_i32 => _NEW_YEAR_S_DAY,
    18994_i32 => _DAY_AFTER_NEW_YEAR_S_DAY,
    18995_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    18996_i32 => _DAY_AFTER_NEW_YEAR_S_DAY__OBSERVED_,
    19029_i32 => _WAITANGI_DAY,
    19030_i32 => _WAITANGI_DAY__OBSERVED_,
    19097_i32 => _GOOD_FRIDAY,
    19100_i32 => _EASTER_MONDAY,
    19107_i32 => _ANZAC_DAY,
    19149_i32 => _QUEEN_S_BIRTHDAY,
    19167_i32 => _MATARIKI,
    19261_i32 => _QUEEN_ELIZABETH_II_MEMORIAL_DAY,
    19289_i32 => _LABOUR_DAY,
    19351_i32 => _CHRISTMAS_DAY,
    19352_i32 => _BOXING_DAY,
    19353_i32 => _CHRISTMAS_DAY__OBSERVED_,
    19358_i32 => _NEW_YEAR_S_DAY,
    19359_i32 => _DAY_AFTER_NEW_YEAR_S_DAY,
    19360_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    19394_i32 => _WAITANGI_DAY,
    19454_i32 => _GOOD_FRIDAY,
    19457_i32 => _EASTER_MONDAY,
    19472_i32 => _ANZAC_DAY,
    19513_i32 => _KING_S_BIRTHDAY,
    19552_i32 => _MATARIKI,
    19653_i32 => _LABOUR_DAY,
    19716_i32 => _CHRISTMAS_DAY,
    19717_i32 => _BOXING_DAY,
    19723_i32 => _NEW_YEAR_S_DAY,
    19724_i32 => _DAY_AFTER_NEW_YEAR_S_DAY,
    19759_i32 => _WAITANGI_DAY,
    19811_i32 => _GOOD_FRIDAY,
    19814_i32 => _EASTER_MONDAY,
    19838_i32 => _ANZAC_DAY,
    19877_i32 => _KING_S_BIRTHDAY,
    19902_i32 => _MATARIKI,
    20024_i32 => _LABOUR_DAY,
    20082_i32 => _CHRISTMAS_DAY,
    20083_i32 => _BOXING_DAY,
    20089_i32 => _NEW_YEAR_S_DAY,
    20090_i32 => _DAY_AFTER_NEW_YEAR_S_DAY,
    20125_i32 => _WAITANGI_DAY,
    20196_i32 => _GOOD_FRIDAY,
    20199_i32 => _EASTER_MONDAY,
    20203_i32 => _ANZAC_DAY,
    20241_i32 => _KING_S_BIRTHDAY,
    20259_i32 => _MATARIKI,
    20388_i32 => _LABOUR_DAY,
    20447_i32 => _CHRISTMAS_DAY,
    20448_i32 => _BOXING_DAY,
    20454_i32 => _NEW_YEAR_S_DAY,
    20455_i32 => _DAY_AFTER_NEW_YEAR_S_DAY,
    20490_i32 => _WAITANGI_DAY,
    20546_i32 => _GOOD_FRIDAY,
    20549_i32 => _EASTER_MONDAY,
    20568_i32 => _ANZAC_DAY,
    20570_i32 => _ANZAC_DAY__OBSERVED_,
    20605_i32 => _KING_S_BIRTHDAY,
    20644_i32 => _MATARIKI,
    20752_i32 => _LABOUR_DAY,
    20812_i32 => _CHRISTMAS_DAY,
    20813_i32 => _BOXING_DAY,
    20815_i32 => _BOXING_DAY__OBSERVED_,
    20819_i32 => _NEW_YEAR_S_DAY,
    20820_i32 => _DAY_AFTER_NEW_YEAR_S_DAY,
    20822_i32 => _DAY_AFTER_NEW_YEAR_S_DAY__OBSERVED_,
    20855_i32 => _WAITANGI_DAY,
    20857_i32 => _WAITANGI_DAY__OBSERVED_,
    20903_i32 => _GOOD_FRIDAY,
    20906_i32 => _EASTER_MONDAY,
    20933_i32 => _ANZAC_DAY,
    20934_i32 => _ANZAC_DAY__OBSERVED_,
    20976_i32 => _KING_S_BIRTHDAY,
    20994_i32 => _MATARIKI,
    21116_i32 => _LABOUR_DAY,
    21177_i32 => _CHRISTMAS_DAY,
    21178_i32 => _BOXING_DAY,
    21179_i32 => _CHRISTMAS_DAY__OBSERVED_,
    21180_i32 => _BOXING_DAY__OBSERVED_,
    21184_i32 => _NEW_YEAR_S_DAY,
    21185_i32 => _DAY_AFTER_NEW_YEAR_S_DAY,
    21186_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    21187_i32 => _DAY_AFTER_NEW_YEAR_S_DAY__OBSERVED_,
    21220_i32 => _WAITANGI_DAY,
    21221_i32 => _WAITANGI_DAY__OBSERVED_,
    21288_i32 => _GOOD_FRIDAY,
    21291_i32 => _EASTER_MONDAY,
    21299_i32 => _ANZAC_DAY,
    21340_i32 => _KING_S_BIRTHDAY,
    21379_i32 => _MATARIKI,
    21480_i32 => _LABOUR_DAY,
    21543_i32 => _CHRISTMAS_DAY,
    21544_i32 => _BOXING_DAY,
    21550_i32 => _NEW_YEAR_S_DAY,
    21551_i32 => _DAY_AFTER_NEW_YEAR_S_DAY,
    21586_i32 => _WAITANGI_DAY,
    21638_i32 => _GOOD_FRIDAY,
    21641_i32 => _EASTER_MONDAY,
    21664_i32 => _ANZAC_DAY,
    21704_i32 => _KING_S_BIRTHDAY,
    21736_i32 => _MATARIKI,
    21844_i32 => _LABOUR_DAY,
    21908_i32 => _CHRISTMAS_DAY,
    21909_i32 => _BOXING_DAY,
    21915_i32 => _NEW_YEAR_S_DAY,
    21916_i32 => _DAY_AFTER_NEW_YEAR_S_DAY,
    21951_i32 => _WAITANGI_DAY,
    22023_i32 => _GOOD_FRIDAY,
    22026_i32 => _EASTER_MONDAY,
    22029_i32 => _ANZAC_DAY,
    22068_i32 => _KING_S_BIRTHDAY,
    22086_i32 => _MATARIKI,
    22215_i32 => _LABOUR_DAY,
    22273_i32 => _CHRISTMAS_DAY,
    22274_i32 => _BOXING_DAY,
    22280_i32 => _NEW_YEAR_S_DAY,
    22281_i32 => _DAY_AFTER_NEW_YEAR_S_DAY,
    22316_i32 => _WAITANGI_DAY,
    22380_i32 => _GOOD_FRIDAY,
    22383_i32 => _EASTER_MONDAY,
    22394_i32 => _ANZAC_DAY,
    22432_i32 => _KING_S_BIRTHDAY,
    22471_i32 => _MATARIKI,
    22579_i32 => _LABOUR_DAY,
    22638_i32 => _CHRISTMAS_DAY,
    22639_i32 => _BOXING_DAY,
    22645_i32 => _NEW_YEAR_S_DAY,
    22646_i32 => _DAY_AFTER_NEW_YEAR_S_DAY,
    22681_i32 => _WAITANGI_DAY,
    22730_i32 => _GOOD_FRIDAY,
    22733_i32 => _EASTER_MONDAY,
    22760_i32 => _ANZAC_DAY,
    22761_i32 => _ANZAC_DAY__OBSERVED_,
    22803_i32 => _KING_S_BIRTHDAY,
    22828_i32 => _MATARIKI,
    22943_i32 => _LABOUR_DAY,
    23004_i32 => _CHRISTMAS_DAY,
    23005_i32 => _BOXING_DAY,
    23006_i32 => _CHRISTMAS_DAY__OBSERVED_,
    23007_i32 => _BOXING_DAY__OBSERVED_,
    23011_i32 => _NEW_YEAR_S_DAY,
    23012_i32 => _DAY_AFTER_NEW_YEAR_S_DAY,
    23013_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    23014_i32 => _DAY_AFTER_NEW_YEAR_S_DAY__OBSERVED_,
    23047_i32 => _WAITANGI_DAY,
    23048_i32 => _WAITANGI_DAY__OBSERVED_,
    23115_i32 => _GOOD_FRIDAY,
    23118_i32 => _EASTER_MONDAY,
    23125_i32 => _ANZAC_DAY,
    23167_i32 => _KING_S_BIRTHDAY,
    23185_i32 => _MATARIKI,
    23307_i32 => _LABOUR_DAY,
    23369_i32 => _CHRISTMAS_DAY,
    23370_i32 => _BOXING_DAY,
    23371_i32 => _CHRISTMAS_DAY__OBSERVED_,
    23376_i32 => _NEW_YEAR_S_DAY,
    23377_i32 => _DAY_AFTER_NEW_YEAR_S_DAY,
    23378_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    23412_i32 => _WAITANGI_DAY,
    23472_i32 => _GOOD_FRIDAY,
    23475_i32 => _EASTER_MONDAY,
    23490_i32 => _ANZAC_DAY,
    23531_i32 => _KING_S_BIRTHDAY,
    23563_i32 => _MATARIKI,
    23671_i32 => _LABOUR_DAY,
    23734_i32 => _CHRISTMAS_DAY,
    23735_i32 => _BOXING_DAY,
    23741_i32 => _NEW_YEAR_S_DAY,
    23742_i32 => _DAY_AFTER_NEW_YEAR_S_DAY,
    23777_i32 => _WAITANGI_DAY,
    23822_i32 => _GOOD_FRIDAY,
    23825_i32 => _EASTER_MONDAY,
    23855_i32 => _ANZAC_DAY,
    23895_i32 => _KING_S_BIRTHDAY,
    23920_i32 => _MATARIKI,
    24035_i32 => _LABOUR_DAY,
    24099_i32 => _CHRISTMAS_DAY,
    24100_i32 => _BOXING_DAY,
    24106_i32 => _NEW_YEAR_S_DAY,
    24107_i32 => _DAY_AFTER_NEW_YEAR_S_DAY,
    24142_i32 => _WAITANGI_DAY,
    24207_i32 => _GOOD_FRIDAY,
    24210_i32 => _EASTER_MONDAY,
    24221_i32 => _ANZAC_DAY,
    24259_i32 => _KING_S_BIRTHDAY,
    24305_i32 => _MATARIKI,
    24406_i32 => _LABOUR_DAY,
    24465_i32 => _CHRISTMAS_DAY,
    24466_i32 => _BOXING_DAY,
    24472_i32 => _NEW_YEAR_S_DAY,
    24473_i32 => _DAY_AFTER_NEW_YEAR_S_DAY,
    24508_i32 => _WAITANGI_DAY,
    24564_i32 => _GOOD_FRIDAY,
    24567_i32 => _EASTER_MONDAY,
    24586_i32 => _ANZAC_DAY,
    24588_i32 => _ANZAC_DAY__OBSERVED_,
    24623_i32 => _KING_S_BIRTHDAY,
    24662_i32 => _MATARIKI,
    24770_i32 => _LABOUR_DAY,
    24830_i32 => _CHRISTMAS_DAY,
    24831_i32 => _BOXING_DAY,
    24833_i32 => _BOXING_DAY__OBSERVED_,
    24837_i32 => _NEW_YEAR_S_DAY,
    24838_i32 => _DAY_AFTER_NEW_YEAR_S_DAY,
    24840_i32 => _DAY_AFTER_NEW_YEAR_S_DAY__OBSERVED_,
    24873_i32 => _WAITANGI_DAY,
    24875_i32 => _WAITANGI_DAY__OBSERVED_,
    24949_i32 => _GOOD_FRIDAY,
    24951_i32 => _ANZAC_DAY,
    24952_i32 => _ANZAC_DAY__OBSERVED___EASTER_MONDAY,
    24994_i32 => _KING_S_BIRTHDAY,
    25012_i32 => _MATARIKI,
    25134_i32 => _LABOUR_DAY,
    25195_i32 => _CHRISTMAS_DAY,
    25196_i32 => _BOXING_DAY,
    25197_i32 => _CHRISTMAS_DAY__OBSERVED_,
    25198_i32 => _BOXING_DAY__OBSERVED_,
    25202_i32 => _NEW_YEAR_S_DAY,
    25203_i32 => _DAY_AFTER_NEW_YEAR_S_DAY,
    25204_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    25205_i32 => _DAY_AFTER_NEW_YEAR_S_DAY__OBSERVED_,
    25238_i32 => _WAITANGI_DAY,
    25239_i32 => _WAITANGI_DAY__OBSERVED_,
    25299_i32 => _GOOD_FRIDAY,
    25302_i32 => _EASTER_MONDAY,
    25316_i32 => _ANZAC_DAY,
    25358_i32 => _KING_S_BIRTHDAY,
    25397_i32 => _MATARIKI,
    25498_i32 => _LABOUR_DAY,
    25560_i32 => _CHRISTMAS_DAY,
    25561_i32 => _BOXING_DAY,
    25562_i32 => _CHRISTMAS_DAY__OBSERVED_,
    25567_i32 => _NEW_YEAR_S_DAY,
    25568_i32 => _DAY_AFTER_NEW_YEAR_S_DAY,
    25569_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    25603_i32 => _WAITANGI_DAY,
    25656_i32 => _GOOD_FRIDAY,
    25659_i32 => _EASTER_MONDAY,
    25682_i32 => _ANZAC_DAY,
    25722_i32 => _KING_S_BIRTHDAY,
    25754_i32 => _MATARIKI,
    25862_i32 => _LABOUR_DAY,
    25926_i32 => _CHRISTMAS_DAY,
    25927_i32 => _BOXING_DAY,
    25933_i32 => _NEW_YEAR_S_DAY,
    25934_i32 => _DAY_AFTER_NEW_YEAR_S_DAY,
    25969_i32 => _WAITANGI_DAY,
    26041_i32 => _GOOD_FRIDAY,
    26044_i32 => _EASTER_MONDAY,
    26047_i32 => _ANZAC_DAY,
    26086_i32 => _KING_S_BIRTHDAY,
    26132_i32 => _MATARIKI,
    26233_i32 => _LABOUR_DAY,
    26291_i32 => _CHRISTMAS_DAY,
    26292_i32 => _BOXING_DAY,
    26298_i32 => _NEW_YEAR_S_DAY,
    26299_i32 => _DAY_AFTER_NEW_YEAR_S_DAY,
    26334_i32 => _WAITANGI_DAY,
    26391_i32 => _GOOD_FRIDAY,
    26394_i32 => _EASTER_MONDAY,
    26412_i32 => _ANZAC_DAY,
    26450_i32 => _KING_S_BIRTHDAY,
    26489_i32 => _MATARIKI,
    26597_i32 => _LABOUR_DAY,
    26656_i32 => _CHRISTMAS_DAY,
    26657_i32 => _BOXING_DAY,
    26663_i32 => _NEW_YEAR_S_DAY,
    26664_i32 => _DAY_AFTER_NEW_YEAR_S_DAY,
    26699_i32 => _WAITANGI_DAY,
    26748_i32 => _GOOD_FRIDAY,
    26751_i32 => _EASTER_MONDAY,
    26777_i32 => _ANZAC_DAY,
    26779_i32 => _ANZAC_DAY__OBSERVED_,
    26814_i32 => _KING_S_BIRTHDAY,
    26846_i32 => _MATARIKI,
    26961_i32 => _LABOUR_DAY,
    27021_i32 => _CHRISTMAS_DAY,
    27022_i32 => _BOXING_DAY,
    27024_i32 => _BOXING_DAY__OBSERVED_,
    27028_i32 => _NEW_YEAR_S_DAY,
    27029_i32 => _DAY_AFTER_NEW_YEAR_S_DAY,
    27031_i32 => _DAY_AFTER_NEW_YEAR_S_DAY__OBSERVED_,
    27064_i32 => _WAITANGI_DAY,
    27066_i32 => _WAITANGI_DAY__OBSERVED_,
    27133_i32 => _GOOD_FRIDAY,
    27136_i32 => _EASTER_MONDAY,
    27143_i32 => _ANZAC_DAY,
    27185_i32 => _KING_S_BIRTHDAY,
    27203_i32 => _MATARIKI,
    27325_i32 => _LABOUR_DAY,
    27387_i32 => _CHRISTMAS_DAY,
    27388_i32 => _BOXING_DAY,
    27389_i32 => _CHRISTMAS_DAY__OBSERVED_,
    27394_i32 => _NEW_YEAR_S_DAY,
    27395_i32 => _DAY_AFTER_NEW_YEAR_S_DAY,
    27396_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    27430_i32 => _WAITANGI_DAY,
    27490_i32 => _GOOD_FRIDAY,
    27493_i32 => _EASTER_MONDAY,
    27508_i32 => _ANZAC_DAY,
    27549_i32 => _KING_S_BIRTHDAY,
    27581_i32 => _MATARIKI,
    27689_i32 => _LABOUR_DAY,
    27752_i32 => _CHRISTMAS_DAY,
    27753_i32 => _BOXING_DAY,
    27759_i32 => _NEW_YEAR_S_DAY,
    27760_i32 => _DAY_AFTER_NEW_YEAR_S_DAY,
    27795_i32 => _WAITANGI_DAY,
    27840_i32 => _GOOD_FRIDAY,
    27843_i32 => _EASTER_MONDAY,
    27873_i32 => _ANZAC_DAY,
    27913_i32 => _KING_S_BIRTHDAY,
    27938_i32 => _MATARIKI,
    28053_i32 => _LABOUR_DAY,
    28117_i32 => _CHRISTMAS_DAY,
    28118_i32 => _BOXING_DAY,
    28124_i32 => _NEW_YEAR_S_DAY,
    28125_i32 => _DAY_AFTER_NEW_YEAR_S_DAY,
    28160_i32 => _WAITANGI_DAY,
    28225_i32 => _GOOD_FRIDAY,
    28228_i32 => _EASTER_MONDAY,
    28238_i32 => _ANZAC_DAY,
    28277_i32 => _KING_S_BIRTHDAY,
    28323_i32 => _MATARIKI,
    28424_i32 => _LABOUR_DAY,
    28482_i32 => _CHRISTMAS_DAY,
    28483_i32 => _BOXING_DAY,
    28489_i32 => _NEW_YEAR_S_DAY,
    28490_i32 => _DAY_AFTER_NEW_YEAR_S_DAY,
    28525_i32 => _WAITANGI_DAY,
    28582_i32 => _GOOD_FRIDAY,
    28585_i32 => _EASTER_MONDAY,
    28604_i32 => _ANZAC_DAY,
    28606_i32 => _ANZAC_DAY__OBSERVED_,
    28641_i32 => _KING_S_BIRTHDAY,
    28673_i32 => _MATARIKI,
    28788_i32 => _LABOUR_DAY,
    28848_i32 => _CHRISTMAS_DAY,
    28849_i32 => _BOXING_DAY,
    28851_i32 => _BOXING_DAY__OBSERVED_,
    28855_i32 => _NEW_YEAR_S_DAY,
    28856_i32 => _DAY_AFTER_NEW_YEAR_S_DAY,
    28858_i32 => _DAY_AFTER_NEW_YEAR_S_DAY__OBSERVED_,
    28891_i32 => _WAITANGI_DAY,
    28893_i32 => _WAITANGI_DAY__OBSERVED_,
    28960_i32 => _GOOD_FRIDAY,
    28963_i32 => _EASTER_MONDAY,
    28969_i32 => _ANZAC_DAY,
    28970_i32 => _ANZAC_DAY__OBSERVED_,
    29012_i32 => _KING_S_BIRTHDAY,
    29030_i32 => _MATARIKI,
    29152_i32 => _LABOUR_DAY,
    29213_i32 => _CHRISTMAS_DAY,
    29214_i32 => _BOXING_DAY,
    29215_i32 => _CHRISTMAS_DAY__OBSERVED_,
    29216_i32 => _BOXING_DAY__OBSERVED_,
    29220_i32 => _NEW_YEAR_S_DAY,
};
