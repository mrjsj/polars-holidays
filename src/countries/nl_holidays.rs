use crate::countries::constants::*;
use phf::phf_map;

pub static NL_HOLIDAYS: phf::Map<i32, &'static str> = phf_map! {
    10957_i32 => _NEW_YEAR_S_DAY,
    11070_i32 => _EASTER_SUNDAY,
    11071_i32 => _EASTER_MONDAY,
    11076_i32 => _QUEEN_S_DAY,
    11082_i32 => _LIBERATION_DAY,
    11109_i32 => _ASCENSION_DAY,
    11119_i32 => _WHIT_SUNDAY,
    11120_i32 => _WHIT_MONDAY,
    11316_i32 => _CHRISTMAS_DAY,
    11317_i32 => _SECOND_DAY_OF_CHRISTMAS,
    11323_i32 => _NEW_YEAR_S_DAY,
    11427_i32 => _EASTER_SUNDAY,
    11428_i32 => _EASTER_MONDAY,
    11442_i32 => _QUEEN_S_DAY,
    11466_i32 => _ASCENSION_DAY,
    11476_i32 => _WHIT_SUNDAY,
    11477_i32 => _WHIT_MONDAY,
    11681_i32 => _CHRISTMAS_DAY,
    11682_i32 => _SECOND_DAY_OF_CHRISTMAS,
    11688_i32 => _NEW_YEAR_S_DAY,
    11777_i32 => _EASTER_SUNDAY,
    11778_i32 => _EASTER_MONDAY,
    11807_i32 => _QUEEN_S_DAY,
    11816_i32 => _ASCENSION_DAY,
    11826_i32 => _WHIT_SUNDAY,
    11827_i32 => _WHIT_MONDAY,
    12046_i32 => _CHRISTMAS_DAY,
    12047_i32 => _SECOND_DAY_OF_CHRISTMAS,
    12053_i32 => _NEW_YEAR_S_DAY,
    12162_i32 => _EASTER_SUNDAY,
    12163_i32 => _EASTER_MONDAY,
    12172_i32 => _QUEEN_S_DAY,
    12201_i32 => _ASCENSION_DAY,
    12211_i32 => _WHIT_SUNDAY,
    12212_i32 => _WHIT_MONDAY,
    12411_i32 => _CHRISTMAS_DAY,
    12412_i32 => _SECOND_DAY_OF_CHRISTMAS,
    12418_i32 => _NEW_YEAR_S_DAY,
    12519_i32 => _EASTER_SUNDAY,
    12520_i32 => _EASTER_MONDAY,
    12538_i32 => _QUEEN_S_DAY,
    12558_i32 => _ASCENSION_DAY,
    12568_i32 => _WHIT_SUNDAY,
    12569_i32 => _WHIT_MONDAY,
    12777_i32 => _CHRISTMAS_DAY,
    12778_i32 => _SECOND_DAY_OF_CHRISTMAS,
    12784_i32 => _NEW_YEAR_S_DAY,
    12869_i32 => _EASTER_SUNDAY,
    12870_i32 => _EASTER_MONDAY,
    12903_i32 => _QUEEN_S_DAY,
    12908_i32 => _ASCENSION_DAY__LIBERATION_DAY,
    12918_i32 => _WHIT_SUNDAY,
    12919_i32 => _WHIT_MONDAY,
    13142_i32 => _CHRISTMAS_DAY,
    13143_i32 => _SECOND_DAY_OF_CHRISTMAS,
    13149_i32 => _NEW_YEAR_S_DAY,
    13254_i32 => _EASTER_SUNDAY,
    13255_i32 => _EASTER_MONDAY,
    13267_i32 => _QUEEN_S_DAY,
    13293_i32 => _ASCENSION_DAY,
    13303_i32 => _WHIT_SUNDAY,
    13304_i32 => _WHIT_MONDAY,
    13507_i32 => _CHRISTMAS_DAY,
    13508_i32 => _SECOND_DAY_OF_CHRISTMAS,
    13514_i32 => _NEW_YEAR_S_DAY,
    13611_i32 => _EASTER_SUNDAY,
    13612_i32 => _EASTER_MONDAY,
    13633_i32 => _QUEEN_S_DAY,
    13650_i32 => _ASCENSION_DAY,
    13660_i32 => _WHIT_SUNDAY,
    13661_i32 => _WHIT_MONDAY,
    13872_i32 => _CHRISTMAS_DAY,
    13873_i32 => _SECOND_DAY_OF_CHRISTMAS,
    13879_i32 => _NEW_YEAR_S_DAY,
    13961_i32 => _EASTER_SUNDAY,
    13962_i32 => _EASTER_MONDAY,
    13999_i32 => _QUEEN_S_DAY,
    14000_i32 => _ASCENSION_DAY,
    14010_i32 => _WHIT_SUNDAY,
    14011_i32 => _WHIT_MONDAY,
    14238_i32 => _CHRISTMAS_DAY,
    14239_i32 => _SECOND_DAY_OF_CHRISTMAS,
    14245_i32 => _NEW_YEAR_S_DAY,
    14346_i32 => _EASTER_SUNDAY,
    14347_i32 => _EASTER_MONDAY,
    14364_i32 => _QUEEN_S_DAY,
    14385_i32 => _ASCENSION_DAY,
    14395_i32 => _WHIT_SUNDAY,
    14396_i32 => _WHIT_MONDAY,
    14603_i32 => _CHRISTMAS_DAY,
    14604_i32 => _SECOND_DAY_OF_CHRISTMAS,
    14610_i32 => _NEW_YEAR_S_DAY,
    14703_i32 => _EASTER_SUNDAY,
    14704_i32 => _EASTER_MONDAY,
    14729_i32 => _QUEEN_S_DAY,
    14734_i32 => _LIBERATION_DAY,
    14742_i32 => _ASCENSION_DAY,
    14752_i32 => _WHIT_SUNDAY,
    14753_i32 => _WHIT_MONDAY,
    14968_i32 => _CHRISTMAS_DAY,
    14969_i32 => _SECOND_DAY_OF_CHRISTMAS,
    14975_i32 => _NEW_YEAR_S_DAY,
    15088_i32 => _EASTER_SUNDAY,
    15089_i32 => _EASTER_MONDAY,
    15094_i32 => _QUEEN_S_DAY,
    15127_i32 => _ASCENSION_DAY,
    15137_i32 => _WHIT_SUNDAY,
    15138_i32 => _WHIT_MONDAY,
    15333_i32 => _CHRISTMAS_DAY,
    15334_i32 => _SECOND_DAY_OF_CHRISTMAS,
    15340_i32 => _NEW_YEAR_S_DAY,
    15438_i32 => _EASTER_SUNDAY,
    15439_i32 => _EASTER_MONDAY,
    15460_i32 => _QUEEN_S_DAY,
    15477_i32 => _ASCENSION_DAY,
    15487_i32 => _WHIT_SUNDAY,
    15488_i32 => _WHIT_MONDAY,
    15699_i32 => _CHRISTMAS_DAY,
    15700_i32 => _SECOND_DAY_OF_CHRISTMAS,
    15706_i32 => _NEW_YEAR_S_DAY,
    15795_i32 => _EASTER_SUNDAY,
    15796_i32 => _EASTER_MONDAY,
    15825_i32 => _QUEEN_S_DAY,
    15834_i32 => _ASCENSION_DAY,
    15844_i32 => _WHIT_SUNDAY,
    15845_i32 => _WHIT_MONDAY,
    16064_i32 => _CHRISTMAS_DAY,
    16065_i32 => _SECOND_DAY_OF_CHRISTMAS,
    16071_i32 => _NEW_YEAR_S_DAY,
    16180_i32 => _EASTER_SUNDAY,
    16181_i32 => _EASTER_MONDAY,
    16186_i32 => _KING_S_DAY,
    16219_i32 => _ASCENSION_DAY,
    16229_i32 => _WHIT_SUNDAY,
    16230_i32 => _WHIT_MONDAY,
    16429_i32 => _CHRISTMAS_DAY,
    16430_i32 => _SECOND_DAY_OF_CHRISTMAS,
    16436_i32 => _NEW_YEAR_S_DAY,
    16530_i32 => _EASTER_SUNDAY,
    16531_i32 => _EASTER_MONDAY,
    16552_i32 => _KING_S_DAY,
    16560_i32 => _LIBERATION_DAY,
    16569_i32 => _ASCENSION_DAY,
    16579_i32 => _WHIT_SUNDAY,
    16580_i32 => _WHIT_MONDAY,
    16794_i32 => _CHRISTMAS_DAY,
    16795_i32 => _SECOND_DAY_OF_CHRISTMAS,
    16801_i32 => _NEW_YEAR_S_DAY,
    16887_i32 => _EASTER_SUNDAY,
    16888_i32 => _EASTER_MONDAY,
    16918_i32 => _KING_S_DAY,
    16926_i32 => _ASCENSION_DAY,
    16936_i32 => _WHIT_SUNDAY,
    16937_i32 => _WHIT_MONDAY,
    17160_i32 => _CHRISTMAS_DAY,
    17161_i32 => _SECOND_DAY_OF_CHRISTMAS,
    17167_i32 => _NEW_YEAR_S_DAY,
    17272_i32 => _EASTER_SUNDAY,
    17273_i32 => _EASTER_MONDAY,
    17283_i32 => _KING_S_DAY,
    17311_i32 => _ASCENSION_DAY,
    17321_i32 => _WHIT_SUNDAY,
    17322_i32 => _WHIT_MONDAY,
    17525_i32 => _CHRISTMAS_DAY,
    17526_i32 => _SECOND_DAY_OF_CHRISTMAS,
    17532_i32 => _NEW_YEAR_S_DAY,
    17622_i32 => _EASTER_SUNDAY,
    17623_i32 => _EASTER_MONDAY,
    17648_i32 => _KING_S_DAY,
    17661_i32 => _ASCENSION_DAY,
    17671_i32 => _WHIT_SUNDAY,
    17672_i32 => _WHIT_MONDAY,
    17890_i32 => _CHRISTMAS_DAY,
    17891_i32 => _SECOND_DAY_OF_CHRISTMAS,
    17897_i32 => _NEW_YEAR_S_DAY,
    18007_i32 => _EASTER_SUNDAY,
    18008_i32 => _EASTER_MONDAY,
    18013_i32 => _KING_S_DAY,
    18046_i32 => _ASCENSION_DAY,
    18056_i32 => _WHIT_SUNDAY,
    18057_i32 => _WHIT_MONDAY,
    18255_i32 => _CHRISTMAS_DAY,
    18256_i32 => _SECOND_DAY_OF_CHRISTMAS,
    18262_i32 => _NEW_YEAR_S_DAY,
    18364_i32 => _EASTER_SUNDAY,
    18365_i32 => _EASTER_MONDAY,
    18379_i32 => _KING_S_DAY,
    18387_i32 => _LIBERATION_DAY,
    18403_i32 => _ASCENSION_DAY,
    18413_i32 => _WHIT_SUNDAY,
    18414_i32 => _WHIT_MONDAY,
    18621_i32 => _CHRISTMAS_DAY,
    18622_i32 => _SECOND_DAY_OF_CHRISTMAS,
    18628_i32 => _NEW_YEAR_S_DAY,
    18721_i32 => _EASTER_SUNDAY,
    18722_i32 => _EASTER_MONDAY,
    18744_i32 => _KING_S_DAY,
    18760_i32 => _ASCENSION_DAY,
    18770_i32 => _WHIT_SUNDAY,
    18771_i32 => _WHIT_MONDAY,
    18986_i32 => _CHRISTMAS_DAY,
    18987_i32 => _SECOND_DAY_OF_CHRISTMAS,
    18993_i32 => _NEW_YEAR_S_DAY,
    19099_i32 => _EASTER_SUNDAY,
    19100_i32 => _EASTER_MONDAY,
    19109_i32 => _KING_S_DAY,
    19138_i32 => _ASCENSION_DAY,
    19148_i32 => _WHIT_SUNDAY,
    19149_i32 => _WHIT_MONDAY,
    19351_i32 => _CHRISTMAS_DAY,
    19352_i32 => _SECOND_DAY_OF_CHRISTMAS,
    19358_i32 => _NEW_YEAR_S_DAY,
    19456_i32 => _EASTER_SUNDAY,
    19457_i32 => _EASTER_MONDAY,
    19474_i32 => _KING_S_DAY,
    19495_i32 => _ASCENSION_DAY,
    19505_i32 => _WHIT_SUNDAY,
    19506_i32 => _WHIT_MONDAY,
    19716_i32 => _CHRISTMAS_DAY,
    19717_i32 => _SECOND_DAY_OF_CHRISTMAS,
    19723_i32 => _NEW_YEAR_S_DAY,
    19813_i32 => _EASTER_SUNDAY,
    19814_i32 => _EASTER_MONDAY,
    19840_i32 => _KING_S_DAY,
    19852_i32 => _ASCENSION_DAY,
    19862_i32 => _WHIT_SUNDAY,
    19863_i32 => _WHIT_MONDAY,
    20082_i32 => _CHRISTMAS_DAY,
    20083_i32 => _SECOND_DAY_OF_CHRISTMAS,
    20089_i32 => _NEW_YEAR_S_DAY,
    20198_i32 => _EASTER_SUNDAY,
    20199_i32 => _EASTER_MONDAY,
    20204_i32 => _KING_S_DAY,
    20213_i32 => _LIBERATION_DAY,
    20237_i32 => _ASCENSION_DAY,
    20247_i32 => _WHIT_SUNDAY,
    20248_i32 => _WHIT_MONDAY,
    20447_i32 => _CHRISTMAS_DAY,
    20448_i32 => _SECOND_DAY_OF_CHRISTMAS,
    20454_i32 => _NEW_YEAR_S_DAY,
    20548_i32 => _EASTER_SUNDAY,
    20549_i32 => _EASTER_MONDAY,
    20570_i32 => _KING_S_DAY,
    20587_i32 => _ASCENSION_DAY,
    20597_i32 => _WHIT_SUNDAY,
    20598_i32 => _WHIT_MONDAY,
    20812_i32 => _CHRISTMAS_DAY,
    20813_i32 => _SECOND_DAY_OF_CHRISTMAS,
    20819_i32 => _NEW_YEAR_S_DAY,
    20905_i32 => _EASTER_SUNDAY,
    20906_i32 => _EASTER_MONDAY,
    20935_i32 => _KING_S_DAY,
    20944_i32 => _ASCENSION_DAY,
    20954_i32 => _WHIT_SUNDAY,
    20955_i32 => _WHIT_MONDAY,
    21177_i32 => _CHRISTMAS_DAY,
    21178_i32 => _SECOND_DAY_OF_CHRISTMAS,
    21184_i32 => _NEW_YEAR_S_DAY,
    21290_i32 => _EASTER_SUNDAY,
    21291_i32 => _EASTER_MONDAY,
    21301_i32 => _KING_S_DAY,
    21329_i32 => _ASCENSION_DAY,
    21339_i32 => _WHIT_SUNDAY,
    21340_i32 => _WHIT_MONDAY,
    21543_i32 => _CHRISTMAS_DAY,
    21544_i32 => _SECOND_DAY_OF_CHRISTMAS,
    21550_i32 => _NEW_YEAR_S_DAY,
    21640_i32 => _EASTER_SUNDAY,
    21641_i32 => _EASTER_MONDAY,
    21666_i32 => _KING_S_DAY,
    21679_i32 => _ASCENSION_DAY,
    21689_i32 => _WHIT_SUNDAY,
    21690_i32 => _WHIT_MONDAY,
    21908_i32 => _CHRISTMAS_DAY,
    21909_i32 => _SECOND_DAY_OF_CHRISTMAS,
    21915_i32 => _NEW_YEAR_S_DAY,
    22025_i32 => _EASTER_SUNDAY,
    22026_i32 => _EASTER_MONDAY,
    22031_i32 => _KING_S_DAY,
    22039_i32 => _LIBERATION_DAY,
    22064_i32 => _ASCENSION_DAY,
    22074_i32 => _WHIT_SUNDAY,
    22075_i32 => _WHIT_MONDAY,
    22273_i32 => _CHRISTMAS_DAY,
    22274_i32 => _SECOND_DAY_OF_CHRISTMAS,
    22280_i32 => _NEW_YEAR_S_DAY,
    22382_i32 => _EASTER_SUNDAY,
    22383_i32 => _EASTER_MONDAY,
    22395_i32 => _KING_S_DAY,
    22421_i32 => _ASCENSION_DAY,
    22431_i32 => _WHIT_SUNDAY,
    22432_i32 => _WHIT_MONDAY,
    22638_i32 => _CHRISTMAS_DAY,
    22639_i32 => _SECOND_DAY_OF_CHRISTMAS,
    22645_i32 => _NEW_YEAR_S_DAY,
    22732_i32 => _EASTER_SUNDAY,
    22733_i32 => _EASTER_MONDAY,
    22762_i32 => _KING_S_DAY,
    22771_i32 => _ASCENSION_DAY,
    22781_i32 => _WHIT_SUNDAY,
    22782_i32 => _WHIT_MONDAY,
    23004_i32 => _CHRISTMAS_DAY,
    23005_i32 => _SECOND_DAY_OF_CHRISTMAS,
    23011_i32 => _NEW_YEAR_S_DAY,
    23117_i32 => _EASTER_SUNDAY,
    23118_i32 => _EASTER_MONDAY,
    23127_i32 => _KING_S_DAY,
    23156_i32 => _ASCENSION_DAY,
    23166_i32 => _WHIT_SUNDAY,
    23167_i32 => _WHIT_MONDAY,
    23369_i32 => _CHRISTMAS_DAY,
    23370_i32 => _SECOND_DAY_OF_CHRISTMAS,
    23376_i32 => _NEW_YEAR_S_DAY,
    23474_i32 => _EASTER_SUNDAY,
    23475_i32 => _EASTER_MONDAY,
    23492_i32 => _KING_S_DAY,
    23513_i32 => _ASCENSION_DAY,
    23523_i32 => _WHIT_SUNDAY,
    23524_i32 => _WHIT_MONDAY,
    23734_i32 => _CHRISTMAS_DAY,
    23735_i32 => _SECOND_DAY_OF_CHRISTMAS,
    23741_i32 => _NEW_YEAR_S_DAY,
    23824_i32 => _EASTER_SUNDAY,
    23825_i32 => _EASTER_MONDAY,
    23857_i32 => _KING_S_DAY,
    23863_i32 => _ASCENSION_DAY,
    23865_i32 => _LIBERATION_DAY,
    23873_i32 => _WHIT_SUNDAY,
    23874_i32 => _WHIT_MONDAY,
    24099_i32 => _CHRISTMAS_DAY,
    24100_i32 => _SECOND_DAY_OF_CHRISTMAS,
    24106_i32 => _NEW_YEAR_S_DAY,
    24209_i32 => _EASTER_SUNDAY,
    24210_i32 => _EASTER_MONDAY,
    24222_i32 => _KING_S_DAY,
    24248_i32 => _ASCENSION_DAY,
    24258_i32 => _WHIT_SUNDAY,
    24259_i32 => _WHIT_MONDAY,
    24465_i32 => _CHRISTMAS_DAY,
    24466_i32 => _SECOND_DAY_OF_CHRISTMAS,
    24472_i32 => _NEW_YEAR_S_DAY,
    24566_i32 => _EASTER_SUNDAY,
    24567_i32 => _EASTER_MONDAY,
    24588_i32 => _KING_S_DAY,
    24605_i32 => _ASCENSION_DAY,
    24615_i32 => _WHIT_SUNDAY,
    24616_i32 => _WHIT_MONDAY,
    24830_i32 => _CHRISTMAS_DAY,
    24831_i32 => _SECOND_DAY_OF_CHRISTMAS,
    24837_i32 => _NEW_YEAR_S_DAY,
    24951_i32 => _EASTER_SUNDAY,
    24952_i32 => _EASTER_MONDAY,
    24953_i32 => _KING_S_DAY,
    24990_i32 => _ASCENSION_DAY,
    25000_i32 => _WHIT_SUNDAY,
    25001_i32 => _WHIT_MONDAY,
    25195_i32 => _CHRISTMAS_DAY,
    25196_i32 => _SECOND_DAY_OF_CHRISTMAS,
    25202_i32 => _NEW_YEAR_S_DAY,
    25301_i32 => _EASTER_SUNDAY,
    25302_i32 => _EASTER_MONDAY,
    25318_i32 => _KING_S_DAY,
    25340_i32 => _ASCENSION_DAY,
    25350_i32 => _WHIT_SUNDAY,
    25351_i32 => _WHIT_MONDAY,
    25560_i32 => _CHRISTMAS_DAY,
    25561_i32 => _SECOND_DAY_OF_CHRISTMAS,
    25567_i32 => _NEW_YEAR_S_DAY,
    25658_i32 => _EASTER_SUNDAY,
    25659_i32 => _EASTER_MONDAY,
    25684_i32 => _KING_S_DAY,
    25692_i32 => _LIBERATION_DAY,
    25697_i32 => _ASCENSION_DAY,
    25707_i32 => _WHIT_SUNDAY,
    25708_i32 => _WHIT_MONDAY,
    25926_i32 => _CHRISTMAS_DAY,
    25927_i32 => _SECOND_DAY_OF_CHRISTMAS,
    25933_i32 => _NEW_YEAR_S_DAY,
    26043_i32 => _EASTER_SUNDAY,
    26044_i32 => _EASTER_MONDAY,
    26049_i32 => _KING_S_DAY,
    26082_i32 => _ASCENSION_DAY,
    26092_i32 => _WHIT_SUNDAY,
    26093_i32 => _WHIT_MONDAY,
    26291_i32 => _CHRISTMAS_DAY,
    26292_i32 => _SECOND_DAY_OF_CHRISTMAS,
    26298_i32 => _NEW_YEAR_S_DAY,
    26393_i32 => _EASTER_SUNDAY,
    26394_i32 => _EASTER_MONDAY,
    26413_i32 => _KING_S_DAY,
    26432_i32 => _ASCENSION_DAY,
    26442_i32 => _WHIT_SUNDAY,
    26443_i32 => _WHIT_MONDAY,
    26656_i32 => _CHRISTMAS_DAY,
    26657_i32 => _SECOND_DAY_OF_CHRISTMAS,
    26663_i32 => _NEW_YEAR_S_DAY,
    26750_i32 => _EASTER_SUNDAY,
    26751_i32 => _EASTER_MONDAY,
    26779_i32 => _KING_S_DAY,
    26789_i32 => _ASCENSION_DAY,
    26799_i32 => _WHIT_SUNDAY,
    26800_i32 => _WHIT_MONDAY,
    27021_i32 => _CHRISTMAS_DAY,
    27022_i32 => _SECOND_DAY_OF_CHRISTMAS,
    27028_i32 => _NEW_YEAR_S_DAY,
    27135_i32 => _EASTER_SUNDAY,
    27136_i32 => _EASTER_MONDAY,
    27145_i32 => _KING_S_DAY,
    27174_i32 => _ASCENSION_DAY,
    27184_i32 => _WHIT_SUNDAY,
    27185_i32 => _WHIT_MONDAY,
    27387_i32 => _CHRISTMAS_DAY,
    27388_i32 => _SECOND_DAY_OF_CHRISTMAS,
    27394_i32 => _NEW_YEAR_S_DAY,
    27492_i32 => _EASTER_SUNDAY,
    27493_i32 => _EASTER_MONDAY,
    27510_i32 => _KING_S_DAY,
    27518_i32 => _LIBERATION_DAY,
    27531_i32 => _ASCENSION_DAY,
    27541_i32 => _WHIT_SUNDAY,
    27542_i32 => _WHIT_MONDAY,
    27752_i32 => _CHRISTMAS_DAY,
    27753_i32 => _SECOND_DAY_OF_CHRISTMAS,
    27759_i32 => _NEW_YEAR_S_DAY,
    27842_i32 => _EASTER_SUNDAY,
    27843_i32 => _EASTER_MONDAY,
    27875_i32 => _KING_S_DAY,
    27881_i32 => _ASCENSION_DAY,
    27891_i32 => _WHIT_SUNDAY,
    27892_i32 => _WHIT_MONDAY,
    28117_i32 => _CHRISTMAS_DAY,
    28118_i32 => _SECOND_DAY_OF_CHRISTMAS,
    28124_i32 => _NEW_YEAR_S_DAY,
    28227_i32 => _EASTER_SUNDAY,
    28228_i32 => _EASTER_MONDAY,
    28240_i32 => _KING_S_DAY,
    28266_i32 => _ASCENSION_DAY,
    28276_i32 => _WHIT_SUNDAY,
    28277_i32 => _WHIT_MONDAY,
    28482_i32 => _CHRISTMAS_DAY,
    28483_i32 => _SECOND_DAY_OF_CHRISTMAS,
    28489_i32 => _NEW_YEAR_S_DAY,
    28584_i32 => _EASTER_SUNDAY,
    28585_i32 => _EASTER_MONDAY,
    28606_i32 => _KING_S_DAY,
    28623_i32 => _ASCENSION_DAY,
    28633_i32 => _WHIT_SUNDAY,
    28634_i32 => _WHIT_MONDAY,
    28848_i32 => _CHRISTMAS_DAY,
    28849_i32 => _SECOND_DAY_OF_CHRISTMAS,
    28855_i32 => _NEW_YEAR_S_DAY,
    28962_i32 => _EASTER_SUNDAY,
    28963_i32 => _EASTER_MONDAY,
    28971_i32 => _KING_S_DAY,
    29001_i32 => _ASCENSION_DAY,
    29011_i32 => _WHIT_SUNDAY,
    29012_i32 => _WHIT_MONDAY,
    29213_i32 => _CHRISTMAS_DAY,
    29214_i32 => _SECOND_DAY_OF_CHRISTMAS,
    29220_i32 => _NEW_YEAR_S_DAY,
};
