use crate::countries::constants::*;
use phf::phf_map;

pub static NA_HOLIDAYS: phf::Map<i32, &'static str> = phf_map! {
    10957_i32 => _NEW_YEAR_S_DAY,
    10959_i32 => _Y2K_CHANGEOVER,
    11037_i32 => _INDEPENDENCE_DAY,
    11068_i32 => _GOOD_FRIDAY,
    11071_i32 => _EASTER_MONDAY,
    11078_i32 => _WORKERS__DAY,
    11081_i32 => _CASSINGA_DAY,
    11102_i32 => _AFRICA_DAY,
    11109_i32 => _ASCENSION_DAY,
    11195_i32 => _HEROES__DAY,
    11210_i32 => _INTERNATIONAL_HUMAN_RIGHTS_DAY,
    11211_i32 => _INTERNATIONAL_HUMAN_RIGHTS_DAY__OBSERVED_,
    11316_i32 => _CHRISTMAS_DAY,
    11317_i32 => _FAMILY_DAY,
    11323_i32 => _NEW_YEAR_S_DAY,
    11402_i32 => _INDEPENDENCE_DAY,
    11425_i32 => _GOOD_FRIDAY,
    11428_i32 => _EASTER_MONDAY,
    11443_i32 => _WORKERS__DAY,
    11446_i32 => _CASSINGA_DAY,
    11466_i32 => _ASCENSION_DAY,
    11467_i32 => _AFRICA_DAY,
    11560_i32 => _HEROES__DAY,
    11561_i32 => _HEROES__DAY__OBSERVED_,
    11575_i32 => _INTERNATIONAL_HUMAN_RIGHTS_DAY,
    11681_i32 => _CHRISTMAS_DAY,
    11682_i32 => _FAMILY_DAY,
    11688_i32 => _NEW_YEAR_S_DAY,
    11767_i32 => _INDEPENDENCE_DAY,
    11775_i32 => _GOOD_FRIDAY,
    11778_i32 => _EASTER_MONDAY,
    11808_i32 => _WORKERS__DAY,
    11811_i32 => _CASSINGA_DAY,
    11816_i32 => _ASCENSION_DAY,
    11832_i32 => _AFRICA_DAY,
    11925_i32 => _HEROES__DAY,
    11940_i32 => _INTERNATIONAL_HUMAN_RIGHTS_DAY,
    12046_i32 => _CHRISTMAS_DAY,
    12047_i32 => _FAMILY_DAY,
    12053_i32 => _NEW_YEAR_S_DAY,
    12132_i32 => _INDEPENDENCE_DAY,
    12160_i32 => _GOOD_FRIDAY,
    12163_i32 => _EASTER_MONDAY,
    12173_i32 => _WORKERS__DAY,
    12176_i32 => _CASSINGA_DAY,
    12177_i32 => _CASSINGA_DAY__OBSERVED_,
    12197_i32 => _AFRICA_DAY,
    12198_i32 => _AFRICA_DAY__OBSERVED_,
    12201_i32 => _ASCENSION_DAY,
    12290_i32 => _HEROES__DAY,
    12305_i32 => _INTERNATIONAL_HUMAN_RIGHTS_DAY,
    12411_i32 => _CHRISTMAS_DAY,
    12412_i32 => _FAMILY_DAY,
    12418_i32 => _NEW_YEAR_S_DAY,
    12498_i32 => _INDEPENDENCE_DAY,
    12499_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    12517_i32 => _GOOD_FRIDAY,
    12520_i32 => _EASTER_MONDAY,
    12539_i32 => _WORKERS__DAY,
    12542_i32 => _CASSINGA_DAY,
    12558_i32 => _ASCENSION_DAY,
    12563_i32 => _AFRICA_DAY,
    12656_i32 => _HEROES__DAY,
    12671_i32 => _INTERNATIONAL_HUMAN_RIGHTS_DAY,
    12777_i32 => _CHRISTMAS_DAY,
    12778_i32 => _FAMILY_DAY,
    12779_i32 => _FAMILY_DAY__OBSERVED_,
    12784_i32 => _NEW_YEAR_S_DAY,
    12863_i32 => _INDEPENDENCE_DAY,
    12867_i32 => _GOOD_FRIDAY,
    12870_i32 => _EASTER_MONDAY,
    12904_i32 => _WORKERS__DAY,
    12905_i32 => _WORKERS__DAY__OBSERVED_,
    12907_i32 => _CASSINGA_DAY,
    12908_i32 => _ASCENSION_DAY,
    12928_i32 => _AFRICA_DAY,
    13021_i32 => _HEROES__DAY,
    13036_i32 => _DAY_OF_THE_NAMIBIAN_WOMEN_AND_INTERNATIONAL_HUMAN_RIGHTS_DAY,
    13142_i32 => _CHRISTMAS_DAY,
    13143_i32 => _FAMILY_DAY,
    13149_i32 => _NEW_YEAR_S_DAY,
    13150_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    13228_i32 => _INDEPENDENCE_DAY,
    13252_i32 => _GOOD_FRIDAY,
    13255_i32 => _EASTER_MONDAY,
    13269_i32 => _WORKERS__DAY,
    13272_i32 => _CASSINGA_DAY,
    13293_i32 => _AFRICA_DAY__ASCENSION_DAY,
    13386_i32 => _HEROES__DAY,
    13401_i32 => _DAY_OF_THE_NAMIBIAN_WOMEN_AND_INTERNATIONAL_HUMAN_RIGHTS_DAY,
    13402_i32 => _DAY_OF_THE_NAMIBIAN_WOMEN_AND_INTERNATIONAL_HUMAN_RIGHTS_DAY__OBSERVED_,
    13507_i32 => _CHRISTMAS_DAY,
    13508_i32 => _FAMILY_DAY,
    13514_i32 => _NEW_YEAR_S_DAY,
    13593_i32 => _INDEPENDENCE_DAY,
    13609_i32 => _GOOD_FRIDAY,
    13612_i32 => _EASTER_MONDAY,
    13634_i32 => _WORKERS__DAY,
    13637_i32 => _CASSINGA_DAY,
    13650_i32 => _ASCENSION_DAY,
    13658_i32 => _AFRICA_DAY,
    13751_i32 => _HEROES__DAY,
    13752_i32 => _HEROES__DAY__OBSERVED_,
    13766_i32 => _DAY_OF_THE_NAMIBIAN_WOMEN_AND_INTERNATIONAL_HUMAN_RIGHTS_DAY,
    13872_i32 => _CHRISTMAS_DAY,
    13873_i32 => _FAMILY_DAY,
    13879_i32 => _NEW_YEAR_S_DAY,
    13959_i32 => _GOOD_FRIDAY__INDEPENDENCE_DAY,
    13962_i32 => _EASTER_MONDAY,
    14000_i32 => _ASCENSION_DAY__WORKERS__DAY,
    14003_i32 => _CASSINGA_DAY,
    14004_i32 => _CASSINGA_DAY__OBSERVED_,
    14024_i32 => _AFRICA_DAY,
    14025_i32 => _AFRICA_DAY__OBSERVED_,
    14117_i32 => _HEROES__DAY,
    14132_i32 => _DAY_OF_THE_NAMIBIAN_WOMEN_AND_INTERNATIONAL_HUMAN_RIGHTS_DAY,
    14238_i32 => _CHRISTMAS_DAY,
    14239_i32 => _FAMILY_DAY,
    14245_i32 => _NEW_YEAR_S_DAY,
    14324_i32 => _INDEPENDENCE_DAY,
    14344_i32 => _GOOD_FRIDAY,
    14347_i32 => _EASTER_MONDAY,
    14365_i32 => _WORKERS__DAY,
    14368_i32 => _CASSINGA_DAY,
    14385_i32 => _ASCENSION_DAY,
    14389_i32 => _AFRICA_DAY,
    14482_i32 => _HEROES__DAY,
    14497_i32 => _DAY_OF_THE_NAMIBIAN_WOMEN_AND_INTERNATIONAL_HUMAN_RIGHTS_DAY,
    14603_i32 => _CHRISTMAS_DAY,
    14604_i32 => _FAMILY_DAY,
    14610_i32 => _NEW_YEAR_S_DAY,
    14689_i32 => _INDEPENDENCE_DAY,
    14690_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    14701_i32 => _GOOD_FRIDAY,
    14704_i32 => _EASTER_MONDAY,
    14730_i32 => _WORKERS__DAY,
    14733_i32 => _CASSINGA_DAY,
    14742_i32 => _ASCENSION_DAY,
    14754_i32 => _AFRICA_DAY,
    14847_i32 => _HEROES__DAY,
    14862_i32 => _DAY_OF_THE_NAMIBIAN_WOMEN_AND_INTERNATIONAL_HUMAN_RIGHTS_DAY,
    14968_i32 => _CHRISTMAS_DAY,
    14969_i32 => _FAMILY_DAY,
    14970_i32 => _FAMILY_DAY__OBSERVED_,
    14975_i32 => _NEW_YEAR_S_DAY,
    15054_i32 => _INDEPENDENCE_DAY,
    15086_i32 => _GOOD_FRIDAY,
    15089_i32 => _EASTER_MONDAY,
    15095_i32 => _WORKERS__DAY,
    15096_i32 => _WORKERS__DAY__OBSERVED_,
    15098_i32 => _CASSINGA_DAY,
    15119_i32 => _AFRICA_DAY,
    15127_i32 => _ASCENSION_DAY,
    15212_i32 => _HEROES__DAY,
    15227_i32 => _DAY_OF_THE_NAMIBIAN_WOMEN_AND_INTERNATIONAL_HUMAN_RIGHTS_DAY,
    15333_i32 => _CHRISTMAS_DAY,
    15334_i32 => _FAMILY_DAY,
    15340_i32 => _NEW_YEAR_S_DAY,
    15341_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    15420_i32 => _INDEPENDENCE_DAY,
    15436_i32 => _GOOD_FRIDAY,
    15439_i32 => _EASTER_MONDAY,
    15461_i32 => _WORKERS__DAY,
    15464_i32 => _CASSINGA_DAY,
    15477_i32 => _ASCENSION_DAY,
    15485_i32 => _AFRICA_DAY,
    15578_i32 => _HEROES__DAY,
    15579_i32 => _HEROES__DAY__OBSERVED_,
    15593_i32 => _DAY_OF_THE_NAMIBIAN_WOMEN_AND_INTERNATIONAL_HUMAN_RIGHTS_DAY,
    15699_i32 => _CHRISTMAS_DAY,
    15700_i32 => _FAMILY_DAY,
    15706_i32 => _NEW_YEAR_S_DAY,
    15785_i32 => _INDEPENDENCE_DAY,
    15793_i32 => _GOOD_FRIDAY,
    15796_i32 => _EASTER_MONDAY,
    15826_i32 => _WORKERS__DAY,
    15829_i32 => _CASSINGA_DAY,
    15834_i32 => _ASCENSION_DAY,
    15850_i32 => _AFRICA_DAY,
    15943_i32 => _HEROES__DAY,
    15958_i32 => _DAY_OF_THE_NAMIBIAN_WOMEN_AND_INTERNATIONAL_HUMAN_RIGHTS_DAY,
    16064_i32 => _CHRISTMAS_DAY,
    16065_i32 => _FAMILY_DAY,
    16071_i32 => _NEW_YEAR_S_DAY,
    16150_i32 => _INDEPENDENCE_DAY,
    16178_i32 => _GOOD_FRIDAY,
    16181_i32 => _EASTER_MONDAY,
    16191_i32 => _WORKERS__DAY,
    16194_i32 => _CASSINGA_DAY,
    16195_i32 => _CASSINGA_DAY__OBSERVED_,
    16215_i32 => _AFRICA_DAY,
    16216_i32 => _AFRICA_DAY__OBSERVED_,
    16219_i32 => _ASCENSION_DAY,
    16308_i32 => _HEROES__DAY,
    16323_i32 => _DAY_OF_THE_NAMIBIAN_WOMEN_AND_INTERNATIONAL_HUMAN_RIGHTS_DAY,
    16429_i32 => _CHRISTMAS_DAY,
    16430_i32 => _FAMILY_DAY,
    16436_i32 => _NEW_YEAR_S_DAY,
    16515_i32 => _INDEPENDENCE_DAY,
    16528_i32 => _GOOD_FRIDAY,
    16531_i32 => _EASTER_MONDAY,
    16556_i32 => _WORKERS__DAY,
    16559_i32 => _CASSINGA_DAY,
    16569_i32 => _ASCENSION_DAY,
    16580_i32 => _AFRICA_DAY,
    16673_i32 => _HEROES__DAY,
    16688_i32 => _DAY_OF_THE_NAMIBIAN_WOMEN_AND_INTERNATIONAL_HUMAN_RIGHTS_DAY,
    16794_i32 => _CHRISTMAS_DAY,
    16795_i32 => _FAMILY_DAY,
    16801_i32 => _NEW_YEAR_S_DAY,
    16881_i32 => _INDEPENDENCE_DAY,
    16885_i32 => _GOOD_FRIDAY,
    16888_i32 => _EASTER_MONDAY,
    16922_i32 => _WORKERS__DAY,
    16923_i32 => _WORKERS__DAY__OBSERVED_,
    16925_i32 => _CASSINGA_DAY,
    16926_i32 => _ASCENSION_DAY,
    16946_i32 => _AFRICA_DAY,
    17039_i32 => _HEROES__DAY,
    17054_i32 => _DAY_OF_THE_NAMIBIAN_WOMEN_AND_INTERNATIONAL_HUMAN_RIGHTS_DAY,
    17160_i32 => _CHRISTMAS_DAY,
    17161_i32 => _FAMILY_DAY,
    17167_i32 => _NEW_YEAR_S_DAY,
    17168_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    17246_i32 => _INDEPENDENCE_DAY,
    17270_i32 => _GOOD_FRIDAY,
    17273_i32 => _EASTER_MONDAY,
    17287_i32 => _WORKERS__DAY,
    17290_i32 => _CASSINGA_DAY,
    17311_i32 => _AFRICA_DAY__ASCENSION_DAY,
    17404_i32 => _HEROES__DAY,
    17419_i32 => _DAY_OF_THE_NAMIBIAN_WOMEN_AND_INTERNATIONAL_HUMAN_RIGHTS_DAY,
    17420_i32 => _DAY_OF_THE_NAMIBIAN_WOMEN_AND_INTERNATIONAL_HUMAN_RIGHTS_DAY__OBSERVED_,
    17525_i32 => _CHRISTMAS_DAY,
    17526_i32 => _FAMILY_DAY,
    17532_i32 => _NEW_YEAR_S_DAY,
    17611_i32 => _INDEPENDENCE_DAY,
    17620_i32 => _GOOD_FRIDAY,
    17623_i32 => _EASTER_MONDAY,
    17652_i32 => _WORKERS__DAY,
    17655_i32 => _CASSINGA_DAY,
    17661_i32 => _ASCENSION_DAY,
    17676_i32 => _AFRICA_DAY,
    17769_i32 => _HEROES__DAY,
    17770_i32 => _HEROES__DAY__OBSERVED_,
    17784_i32 => _DAY_OF_THE_NAMIBIAN_WOMEN_AND_INTERNATIONAL_HUMAN_RIGHTS_DAY,
    17890_i32 => _CHRISTMAS_DAY,
    17891_i32 => _FAMILY_DAY,
    17897_i32 => _NEW_YEAR_S_DAY,
    17976_i32 => _INDEPENDENCE_DAY,
    18005_i32 => _GOOD_FRIDAY,
    18008_i32 => _EASTER_MONDAY,
    18017_i32 => _WORKERS__DAY,
    18020_i32 => _CASSINGA_DAY,
    18041_i32 => _AFRICA_DAY,
    18046_i32 => _ASCENSION_DAY,
    18134_i32 => _HEROES__DAY,
    18149_i32 => _DAY_OF_THE_NAMIBIAN_WOMEN_AND_INTERNATIONAL_HUMAN_RIGHTS_DAY,
    18255_i32 => _CHRISTMAS_DAY,
    18256_i32 => _FAMILY_DAY,
    18262_i32 => _NEW_YEAR_S_DAY,
    18342_i32 => _INDEPENDENCE_DAY,
    18362_i32 => _GOOD_FRIDAY,
    18365_i32 => _EASTER_MONDAY,
    18383_i32 => _WORKERS__DAY,
    18386_i32 => _CASSINGA_DAY,
    18403_i32 => _ASCENSION_DAY,
    18407_i32 => _AFRICA_DAY,
    18500_i32 => _HEROES__DAY,
    18515_i32 => _DAY_OF_THE_NAMIBIAN_WOMEN_AND_INTERNATIONAL_HUMAN_RIGHTS_DAY,
    18621_i32 => _CHRISTMAS_DAY,
    18622_i32 => _FAMILY_DAY,
    18628_i32 => _NEW_YEAR_S_DAY,
    18707_i32 => _INDEPENDENCE_DAY,
    18708_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    18719_i32 => _GOOD_FRIDAY,
    18722_i32 => _EASTER_MONDAY,
    18748_i32 => _WORKERS__DAY,
    18751_i32 => _CASSINGA_DAY,
    18760_i32 => _ASCENSION_DAY,
    18772_i32 => _AFRICA_DAY,
    18865_i32 => _HEROES__DAY,
    18880_i32 => _DAY_OF_THE_NAMIBIAN_WOMEN_AND_INTERNATIONAL_HUMAN_RIGHTS_DAY,
    18986_i32 => _CHRISTMAS_DAY,
    18987_i32 => _FAMILY_DAY,
    18988_i32 => _FAMILY_DAY__OBSERVED_,
    18993_i32 => _NEW_YEAR_S_DAY,
    19072_i32 => _INDEPENDENCE_DAY,
    19097_i32 => _GOOD_FRIDAY,
    19100_i32 => _EASTER_MONDAY,
    19113_i32 => _WORKERS__DAY,
    19114_i32 => _WORKERS__DAY__OBSERVED_,
    19116_i32 => _CASSINGA_DAY,
    19137_i32 => _AFRICA_DAY,
    19138_i32 => _ASCENSION_DAY,
    19230_i32 => _HEROES__DAY,
    19245_i32 => _DAY_OF_THE_NAMIBIAN_WOMEN_AND_INTERNATIONAL_HUMAN_RIGHTS_DAY,
    19351_i32 => _CHRISTMAS_DAY,
    19352_i32 => _FAMILY_DAY,
    19358_i32 => _NEW_YEAR_S_DAY,
    19359_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    19437_i32 => _INDEPENDENCE_DAY,
    19454_i32 => _GOOD_FRIDAY,
    19457_i32 => _EASTER_MONDAY,
    19478_i32 => _WORKERS__DAY,
    19481_i32 => _CASSINGA_DAY,
    19495_i32 => _ASCENSION_DAY,
    19502_i32 => _AFRICA_DAY,
    19595_i32 => _HEROES__DAY,
    19610_i32 => _DAY_OF_THE_NAMIBIAN_WOMEN_AND_INTERNATIONAL_HUMAN_RIGHTS_DAY,
    19611_i32 => _DAY_OF_THE_NAMIBIAN_WOMEN_AND_INTERNATIONAL_HUMAN_RIGHTS_DAY__OBSERVED_,
    19716_i32 => _CHRISTMAS_DAY,
    19717_i32 => _FAMILY_DAY,
    19723_i32 => _NEW_YEAR_S_DAY,
    19803_i32 => _INDEPENDENCE_DAY,
    19811_i32 => _GOOD_FRIDAY,
    19814_i32 => _EASTER_MONDAY,
    19844_i32 => _WORKERS__DAY,
    19847_i32 => _CASSINGA_DAY,
    19852_i32 => _ASCENSION_DAY,
    19868_i32 => _AFRICA_DAY,
    19961_i32 => _HEROES__DAY,
    19976_i32 => _DAY_OF_THE_NAMIBIAN_WOMEN_AND_INTERNATIONAL_HUMAN_RIGHTS_DAY,
    20082_i32 => _CHRISTMAS_DAY,
    20083_i32 => _FAMILY_DAY,
    20089_i32 => _NEW_YEAR_S_DAY,
    20168_i32 => _INDEPENDENCE_DAY,
    20196_i32 => _GOOD_FRIDAY,
    20199_i32 => _EASTER_MONDAY,
    20209_i32 => _WORKERS__DAY,
    20212_i32 => _CASSINGA_DAY,
    20213_i32 => _CASSINGA_DAY__OBSERVED_,
    20233_i32 => _AFRICA_DAY,
    20234_i32 => _AFRICA_DAY__OBSERVED_,
    20237_i32 => _ASCENSION_DAY,
    20326_i32 => _HEROES__DAY,
    20341_i32 => _DAY_OF_THE_NAMIBIAN_WOMEN_AND_INTERNATIONAL_HUMAN_RIGHTS_DAY,
    20447_i32 => _CHRISTMAS_DAY,
    20448_i32 => _FAMILY_DAY,
    20454_i32 => _NEW_YEAR_S_DAY,
    20533_i32 => _INDEPENDENCE_DAY,
    20546_i32 => _GOOD_FRIDAY,
    20549_i32 => _EASTER_MONDAY,
    20574_i32 => _WORKERS__DAY,
    20577_i32 => _CASSINGA_DAY,
    20587_i32 => _ASCENSION_DAY,
    20598_i32 => _AFRICA_DAY,
    20691_i32 => _HEROES__DAY,
    20706_i32 => _DAY_OF_THE_NAMIBIAN_WOMEN_AND_INTERNATIONAL_HUMAN_RIGHTS_DAY,
    20812_i32 => _CHRISTMAS_DAY,
    20813_i32 => _FAMILY_DAY,
    20819_i32 => _NEW_YEAR_S_DAY,
    20898_i32 => _INDEPENDENCE_DAY,
    20899_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    20903_i32 => _GOOD_FRIDAY,
    20906_i32 => _EASTER_MONDAY,
    20939_i32 => _WORKERS__DAY,
    20942_i32 => _CASSINGA_DAY,
    20944_i32 => _ASCENSION_DAY,
    20963_i32 => _AFRICA_DAY,
    21056_i32 => _HEROES__DAY,
    21071_i32 => _DAY_OF_THE_NAMIBIAN_WOMEN_AND_INTERNATIONAL_HUMAN_RIGHTS_DAY,
    21177_i32 => _CHRISTMAS_DAY,
    21178_i32 => _FAMILY_DAY,
    21179_i32 => _FAMILY_DAY__OBSERVED_,
    21184_i32 => _NEW_YEAR_S_DAY,
    21264_i32 => _INDEPENDENCE_DAY,
    21288_i32 => _GOOD_FRIDAY,
    21291_i32 => _EASTER_MONDAY,
    21305_i32 => _WORKERS__DAY,
    21308_i32 => _CASSINGA_DAY,
    21329_i32 => _AFRICA_DAY__ASCENSION_DAY,
    21422_i32 => _HEROES__DAY,
    21437_i32 => _DAY_OF_THE_NAMIBIAN_WOMEN_AND_INTERNATIONAL_HUMAN_RIGHTS_DAY,
    21438_i32 => _DAY_OF_THE_NAMIBIAN_WOMEN_AND_INTERNATIONAL_HUMAN_RIGHTS_DAY__OBSERVED_,
    21543_i32 => _CHRISTMAS_DAY,
    21544_i32 => _FAMILY_DAY,
    21550_i32 => _NEW_YEAR_S_DAY,
    21629_i32 => _INDEPENDENCE_DAY,
    21638_i32 => _GOOD_FRIDAY,
    21641_i32 => _EASTER_MONDAY,
    21670_i32 => _WORKERS__DAY,
    21673_i32 => _CASSINGA_DAY,
    21679_i32 => _ASCENSION_DAY,
    21694_i32 => _AFRICA_DAY,
    21787_i32 => _HEROES__DAY,
    21788_i32 => _HEROES__DAY__OBSERVED_,
    21802_i32 => _DAY_OF_THE_NAMIBIAN_WOMEN_AND_INTERNATIONAL_HUMAN_RIGHTS_DAY,
    21908_i32 => _CHRISTMAS_DAY,
    21909_i32 => _FAMILY_DAY,
    21915_i32 => _NEW_YEAR_S_DAY,
    21994_i32 => _INDEPENDENCE_DAY,
    22023_i32 => _GOOD_FRIDAY,
    22026_i32 => _EASTER_MONDAY,
    22035_i32 => _WORKERS__DAY,
    22038_i32 => _CASSINGA_DAY,
    22059_i32 => _AFRICA_DAY,
    22064_i32 => _ASCENSION_DAY,
    22152_i32 => _HEROES__DAY,
    22167_i32 => _DAY_OF_THE_NAMIBIAN_WOMEN_AND_INTERNATIONAL_HUMAN_RIGHTS_DAY,
    22273_i32 => _CHRISTMAS_DAY,
    22274_i32 => _FAMILY_DAY,
    22280_i32 => _NEW_YEAR_S_DAY,
    22359_i32 => _INDEPENDENCE_DAY,
    22380_i32 => _GOOD_FRIDAY,
    22383_i32 => _EASTER_MONDAY,
    22400_i32 => _WORKERS__DAY,
    22403_i32 => _CASSINGA_DAY,
    22404_i32 => _CASSINGA_DAY__OBSERVED_,
    22421_i32 => _ASCENSION_DAY,
    22424_i32 => _AFRICA_DAY,
    22425_i32 => _AFRICA_DAY__OBSERVED_,
    22517_i32 => _HEROES__DAY,
    22532_i32 => _DAY_OF_THE_NAMIBIAN_WOMEN_AND_INTERNATIONAL_HUMAN_RIGHTS_DAY,
    22638_i32 => _CHRISTMAS_DAY,
    22639_i32 => _FAMILY_DAY,
    22645_i32 => _NEW_YEAR_S_DAY,
    22725_i32 => _INDEPENDENCE_DAY,
    22726_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    22730_i32 => _GOOD_FRIDAY,
    22733_i32 => _EASTER_MONDAY,
    22766_i32 => _WORKERS__DAY,
    22769_i32 => _CASSINGA_DAY,
    22771_i32 => _ASCENSION_DAY,
    22790_i32 => _AFRICA_DAY,
    22883_i32 => _HEROES__DAY,
    22898_i32 => _DAY_OF_THE_NAMIBIAN_WOMEN_AND_INTERNATIONAL_HUMAN_RIGHTS_DAY,
    23004_i32 => _CHRISTMAS_DAY,
    23005_i32 => _FAMILY_DAY,
    23006_i32 => _FAMILY_DAY__OBSERVED_,
    23011_i32 => _NEW_YEAR_S_DAY,
    23090_i32 => _INDEPENDENCE_DAY,
    23115_i32 => _GOOD_FRIDAY,
    23118_i32 => _EASTER_MONDAY,
    23131_i32 => _WORKERS__DAY,
    23132_i32 => _WORKERS__DAY__OBSERVED_,
    23134_i32 => _CASSINGA_DAY,
    23155_i32 => _AFRICA_DAY,
    23156_i32 => _ASCENSION_DAY,
    23248_i32 => _HEROES__DAY,
    23263_i32 => _DAY_OF_THE_NAMIBIAN_WOMEN_AND_INTERNATIONAL_HUMAN_RIGHTS_DAY,
    23369_i32 => _CHRISTMAS_DAY,
    23370_i32 => _FAMILY_DAY,
    23376_i32 => _NEW_YEAR_S_DAY,
    23377_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    23455_i32 => _INDEPENDENCE_DAY,
    23472_i32 => _GOOD_FRIDAY,
    23475_i32 => _EASTER_MONDAY,
    23496_i32 => _WORKERS__DAY,
    23499_i32 => _CASSINGA_DAY,
    23513_i32 => _ASCENSION_DAY,
    23520_i32 => _AFRICA_DAY,
    23613_i32 => _HEROES__DAY,
    23628_i32 => _DAY_OF_THE_NAMIBIAN_WOMEN_AND_INTERNATIONAL_HUMAN_RIGHTS_DAY,
    23629_i32 => _DAY_OF_THE_NAMIBIAN_WOMEN_AND_INTERNATIONAL_HUMAN_RIGHTS_DAY__OBSERVED_,
    23734_i32 => _CHRISTMAS_DAY,
    23735_i32 => _FAMILY_DAY,
    23741_i32 => _NEW_YEAR_S_DAY,
    23820_i32 => _INDEPENDENCE_DAY,
    23822_i32 => _GOOD_FRIDAY,
    23825_i32 => _EASTER_MONDAY,
    23861_i32 => _WORKERS__DAY,
    23863_i32 => _ASCENSION_DAY,
    23864_i32 => _CASSINGA_DAY,
    23885_i32 => _AFRICA_DAY,
    23978_i32 => _HEROES__DAY,
    23979_i32 => _HEROES__DAY__OBSERVED_,
    23993_i32 => _DAY_OF_THE_NAMIBIAN_WOMEN_AND_INTERNATIONAL_HUMAN_RIGHTS_DAY,
    24099_i32 => _CHRISTMAS_DAY,
    24100_i32 => _FAMILY_DAY,
    24106_i32 => _NEW_YEAR_S_DAY,
    24186_i32 => _INDEPENDENCE_DAY,
    24207_i32 => _GOOD_FRIDAY,
    24210_i32 => _EASTER_MONDAY,
    24227_i32 => _WORKERS__DAY,
    24230_i32 => _CASSINGA_DAY,
    24231_i32 => _CASSINGA_DAY__OBSERVED_,
    24248_i32 => _ASCENSION_DAY,
    24251_i32 => _AFRICA_DAY,
    24252_i32 => _AFRICA_DAY__OBSERVED_,
    24344_i32 => _HEROES__DAY,
    24359_i32 => _DAY_OF_THE_NAMIBIAN_WOMEN_AND_INTERNATIONAL_HUMAN_RIGHTS_DAY,
    24465_i32 => _CHRISTMAS_DAY,
    24466_i32 => _FAMILY_DAY,
    24472_i32 => _NEW_YEAR_S_DAY,
    24551_i32 => _INDEPENDENCE_DAY,
    24564_i32 => _GOOD_FRIDAY,
    24567_i32 => _EASTER_MONDAY,
    24592_i32 => _WORKERS__DAY,
    24595_i32 => _CASSINGA_DAY,
    24605_i32 => _ASCENSION_DAY,
    24616_i32 => _AFRICA_DAY,
    24709_i32 => _HEROES__DAY,
    24724_i32 => _DAY_OF_THE_NAMIBIAN_WOMEN_AND_INTERNATIONAL_HUMAN_RIGHTS_DAY,
    24830_i32 => _CHRISTMAS_DAY,
    24831_i32 => _FAMILY_DAY,
    24837_i32 => _NEW_YEAR_S_DAY,
    24916_i32 => _INDEPENDENCE_DAY,
    24917_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    24949_i32 => _GOOD_FRIDAY,
    24952_i32 => _EASTER_MONDAY,
    24957_i32 => _WORKERS__DAY,
    24960_i32 => _CASSINGA_DAY,
    24981_i32 => _AFRICA_DAY,
    24990_i32 => _ASCENSION_DAY,
    25074_i32 => _HEROES__DAY,
    25089_i32 => _DAY_OF_THE_NAMIBIAN_WOMEN_AND_INTERNATIONAL_HUMAN_RIGHTS_DAY,
    25195_i32 => _CHRISTMAS_DAY,
    25196_i32 => _FAMILY_DAY,
    25197_i32 => _FAMILY_DAY__OBSERVED_,
    25202_i32 => _NEW_YEAR_S_DAY,
    25281_i32 => _INDEPENDENCE_DAY,
    25299_i32 => _GOOD_FRIDAY,
    25302_i32 => _EASTER_MONDAY,
    25322_i32 => _WORKERS__DAY,
    25323_i32 => _WORKERS__DAY__OBSERVED_,
    25325_i32 => _CASSINGA_DAY,
    25340_i32 => _ASCENSION_DAY,
    25346_i32 => _AFRICA_DAY,
    25439_i32 => _HEROES__DAY,
    25454_i32 => _DAY_OF_THE_NAMIBIAN_WOMEN_AND_INTERNATIONAL_HUMAN_RIGHTS_DAY,
    25560_i32 => _CHRISTMAS_DAY,
    25561_i32 => _FAMILY_DAY,
    25567_i32 => _NEW_YEAR_S_DAY,
    25568_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    25647_i32 => _INDEPENDENCE_DAY,
    25656_i32 => _GOOD_FRIDAY,
    25659_i32 => _EASTER_MONDAY,
    25688_i32 => _WORKERS__DAY,
    25691_i32 => _CASSINGA_DAY,
    25697_i32 => _ASCENSION_DAY,
    25712_i32 => _AFRICA_DAY,
    25805_i32 => _HEROES__DAY,
    25806_i32 => _HEROES__DAY__OBSERVED_,
    25820_i32 => _DAY_OF_THE_NAMIBIAN_WOMEN_AND_INTERNATIONAL_HUMAN_RIGHTS_DAY,
    25926_i32 => _CHRISTMAS_DAY,
    25927_i32 => _FAMILY_DAY,
    25933_i32 => _NEW_YEAR_S_DAY,
    26012_i32 => _INDEPENDENCE_DAY,
    26041_i32 => _GOOD_FRIDAY,
    26044_i32 => _EASTER_MONDAY,
    26053_i32 => _WORKERS__DAY,
    26056_i32 => _CASSINGA_DAY,
    26077_i32 => _AFRICA_DAY,
    26082_i32 => _ASCENSION_DAY,
    26170_i32 => _HEROES__DAY,
    26185_i32 => _DAY_OF_THE_NAMIBIAN_WOMEN_AND_INTERNATIONAL_HUMAN_RIGHTS_DAY,
    26291_i32 => _CHRISTMAS_DAY,
    26292_i32 => _FAMILY_DAY,
    26298_i32 => _NEW_YEAR_S_DAY,
    26377_i32 => _INDEPENDENCE_DAY,
    26391_i32 => _GOOD_FRIDAY,
    26394_i32 => _EASTER_MONDAY,
    26418_i32 => _WORKERS__DAY,
    26421_i32 => _CASSINGA_DAY,
    26422_i32 => _CASSINGA_DAY__OBSERVED_,
    26432_i32 => _ASCENSION_DAY,
    26442_i32 => _AFRICA_DAY,
    26443_i32 => _AFRICA_DAY__OBSERVED_,
    26535_i32 => _HEROES__DAY,
    26550_i32 => _DAY_OF_THE_NAMIBIAN_WOMEN_AND_INTERNATIONAL_HUMAN_RIGHTS_DAY,
    26656_i32 => _CHRISTMAS_DAY,
    26657_i32 => _FAMILY_DAY,
    26663_i32 => _NEW_YEAR_S_DAY,
    26742_i32 => _INDEPENDENCE_DAY,
    26748_i32 => _GOOD_FRIDAY,
    26751_i32 => _EASTER_MONDAY,
    26783_i32 => _WORKERS__DAY,
    26786_i32 => _CASSINGA_DAY,
    26789_i32 => _ASCENSION_DAY,
    26807_i32 => _AFRICA_DAY,
    26900_i32 => _HEROES__DAY,
    26915_i32 => _DAY_OF_THE_NAMIBIAN_WOMEN_AND_INTERNATIONAL_HUMAN_RIGHTS_DAY,
    27021_i32 => _CHRISTMAS_DAY,
    27022_i32 => _FAMILY_DAY,
    27028_i32 => _NEW_YEAR_S_DAY,
    27108_i32 => _INDEPENDENCE_DAY,
    27133_i32 => _GOOD_FRIDAY,
    27136_i32 => _EASTER_MONDAY,
    27149_i32 => _WORKERS__DAY,
    27150_i32 => _WORKERS__DAY__OBSERVED_,
    27152_i32 => _CASSINGA_DAY,
    27173_i32 => _AFRICA_DAY,
    27174_i32 => _ASCENSION_DAY,
    27266_i32 => _HEROES__DAY,
    27281_i32 => _DAY_OF_THE_NAMIBIAN_WOMEN_AND_INTERNATIONAL_HUMAN_RIGHTS_DAY,
    27387_i32 => _CHRISTMAS_DAY,
    27388_i32 => _FAMILY_DAY,
    27394_i32 => _NEW_YEAR_S_DAY,
    27395_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    27473_i32 => _INDEPENDENCE_DAY,
    27490_i32 => _GOOD_FRIDAY,
    27493_i32 => _EASTER_MONDAY,
    27514_i32 => _WORKERS__DAY,
    27517_i32 => _CASSINGA_DAY,
    27531_i32 => _ASCENSION_DAY,
    27538_i32 => _AFRICA_DAY,
    27631_i32 => _HEROES__DAY,
    27646_i32 => _DAY_OF_THE_NAMIBIAN_WOMEN_AND_INTERNATIONAL_HUMAN_RIGHTS_DAY,
    27647_i32 => _DAY_OF_THE_NAMIBIAN_WOMEN_AND_INTERNATIONAL_HUMAN_RIGHTS_DAY__OBSERVED_,
    27752_i32 => _CHRISTMAS_DAY,
    27753_i32 => _FAMILY_DAY,
    27759_i32 => _NEW_YEAR_S_DAY,
    27838_i32 => _INDEPENDENCE_DAY,
    27840_i32 => _GOOD_FRIDAY,
    27843_i32 => _EASTER_MONDAY,
    27879_i32 => _WORKERS__DAY,
    27881_i32 => _ASCENSION_DAY,
    27882_i32 => _CASSINGA_DAY,
    27903_i32 => _AFRICA_DAY,
    27996_i32 => _HEROES__DAY,
    27997_i32 => _HEROES__DAY__OBSERVED_,
    28011_i32 => _DAY_OF_THE_NAMIBIAN_WOMEN_AND_INTERNATIONAL_HUMAN_RIGHTS_DAY,
    28117_i32 => _CHRISTMAS_DAY,
    28118_i32 => _FAMILY_DAY,
    28124_i32 => _NEW_YEAR_S_DAY,
    28203_i32 => _INDEPENDENCE_DAY,
    28225_i32 => _GOOD_FRIDAY,
    28228_i32 => _EASTER_MONDAY,
    28244_i32 => _WORKERS__DAY,
    28247_i32 => _CASSINGA_DAY,
    28266_i32 => _ASCENSION_DAY,
    28268_i32 => _AFRICA_DAY,
    28361_i32 => _HEROES__DAY,
    28376_i32 => _DAY_OF_THE_NAMIBIAN_WOMEN_AND_INTERNATIONAL_HUMAN_RIGHTS_DAY,
    28482_i32 => _CHRISTMAS_DAY,
    28483_i32 => _FAMILY_DAY,
    28489_i32 => _NEW_YEAR_S_DAY,
    28569_i32 => _INDEPENDENCE_DAY,
    28582_i32 => _GOOD_FRIDAY,
    28585_i32 => _EASTER_MONDAY,
    28610_i32 => _WORKERS__DAY,
    28613_i32 => _CASSINGA_DAY,
    28623_i32 => _ASCENSION_DAY,
    28634_i32 => _AFRICA_DAY,
    28727_i32 => _HEROES__DAY,
    28742_i32 => _DAY_OF_THE_NAMIBIAN_WOMEN_AND_INTERNATIONAL_HUMAN_RIGHTS_DAY,
    28848_i32 => _CHRISTMAS_DAY,
    28849_i32 => _FAMILY_DAY,
    28855_i32 => _NEW_YEAR_S_DAY,
    28934_i32 => _INDEPENDENCE_DAY,
    28935_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    28960_i32 => _GOOD_FRIDAY,
    28963_i32 => _EASTER_MONDAY,
    28975_i32 => _WORKERS__DAY,
    28978_i32 => _CASSINGA_DAY,
    28999_i32 => _AFRICA_DAY,
    29001_i32 => _ASCENSION_DAY,
    29092_i32 => _HEROES__DAY,
    29107_i32 => _DAY_OF_THE_NAMIBIAN_WOMEN_AND_INTERNATIONAL_HUMAN_RIGHTS_DAY,
    29213_i32 => _CHRISTMAS_DAY,
    29214_i32 => _FAMILY_DAY,
    29215_i32 => _FAMILY_DAY__OBSERVED_,
    29220_i32 => _NEW_YEAR_S_DAY,
};
