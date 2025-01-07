use crate::countries::constants::*;
use phf::phf_map;

pub static BF_HOLIDAYS: phf::Map<i32, &'static str> = phf_map! {
    10957_i32 => _NEW_YEAR_S_DAY,
    10959_i32 => _REVOLUTION_DAY,
    10964_i32 => _EID_AL_FITR__ESTIMATED_,
    11024_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    11032_i32 => _EID_AL_ADHA__ESTIMATED_,
    11071_i32 => _EASTER_MONDAY,
    11078_i32 => _LABOUR_DAY,
    11109_i32 => _ASCENSION_DAY,
    11122_i32 => _MAWLID__ESTIMATED_,
    11174_i32 => _INDEPENDENCE_DAY,
    11184_i32 => _ASSUMPTION_DAY,
    11262_i32 => _ALL_SAINTS__DAY,
    11302_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    11316_i32 => _CHRISTMAS_DAY,
    11318_i32 => _EID_AL_FITR__ESTIMATED_,
    11323_i32 => _NEW_YEAR_S_DAY,
    11325_i32 => _REVOLUTION_DAY,
    11386_i32 => _EID_AL_ADHA__ESTIMATED_,
    11389_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    11428_i32 => _EASTER_MONDAY,
    11443_i32 => _LABOUR_DAY,
    11466_i32 => _ASCENSION_DAY,
    11477_i32 => _MAWLID__ESTIMATED_,
    11539_i32 => _INDEPENDENCE_DAY,
    11540_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    11549_i32 => _ASSUMPTION_DAY,
    11627_i32 => _ALL_SAINTS__DAY,
    11667_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    11672_i32 => _EID_AL_FITR__ESTIMATED_,
    11681_i32 => _CHRISTMAS_DAY,
    11688_i32 => _NEW_YEAR_S_DAY,
    11690_i32 => _REVOLUTION_DAY,
    11740_i32 => _EID_AL_ADHA__ESTIMATED_,
    11754_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    11778_i32 => _EASTER_MONDAY,
    11808_i32 => _LABOUR_DAY,
    11816_i32 => _ASCENSION_DAY,
    11831_i32 => _MAWLID__ESTIMATED_,
    11904_i32 => _INDEPENDENCE_DAY,
    11914_i32 => _ASSUMPTION_DAY,
    11992_i32 => _ALL_SAINTS__DAY,
    12026_i32 => _EID_AL_FITR__ESTIMATED_,
    12032_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    12046_i32 => _CHRISTMAS_DAY,
    12053_i32 => _NEW_YEAR_S_DAY,
    12055_i32 => _REVOLUTION_DAY,
    12094_i32 => _EID_AL_ADHA__ESTIMATED_,
    12119_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    12163_i32 => _EASTER_MONDAY,
    12173_i32 => _LABOUR_DAY,
    12185_i32 => _MAWLID__ESTIMATED_,
    12201_i32 => _ASCENSION_DAY,
    12269_i32 => _INDEPENDENCE_DAY,
    12279_i32 => _ASSUMPTION_DAY,
    12357_i32 => _ALL_SAINTS__DAY,
    12381_i32 => _EID_AL_FITR__ESTIMATED_,
    12397_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    12411_i32 => _CHRISTMAS_DAY,
    12418_i32 => _NEW_YEAR_S_DAY,
    12420_i32 => _REVOLUTION_DAY,
    12449_i32 => _EID_AL_ADHA__ESTIMATED_,
    12485_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    12520_i32 => _EASTER_MONDAY,
    12539_i32 => _LABOUR_DAY__MAWLID__ESTIMATED_,
    12558_i32 => _ASCENSION_DAY,
    12635_i32 => _INDEPENDENCE_DAY,
    12645_i32 => _ASSUMPTION_DAY,
    12646_i32 => _ASSUMPTION_DAY__OBSERVED_,
    12723_i32 => _ALL_SAINTS__DAY,
    12736_i32 => _EID_AL_FITR__ESTIMATED_,
    12763_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    12777_i32 => _CHRISTMAS_DAY,
    12784_i32 => _NEW_YEAR_S_DAY,
    12786_i32 => _REVOLUTION_DAY,
    12804_i32 => _EID_AL_ADHA__ESTIMATED_,
    12850_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    12870_i32 => _EASTER_MONDAY,
    12894_i32 => _MAWLID__ESTIMATED_,
    12904_i32 => _LABOUR_DAY,
    12905_i32 => _LABOUR_DAY__OBSERVED_,
    12908_i32 => _ASCENSION_DAY,
    13000_i32 => _INDEPENDENCE_DAY,
    13010_i32 => _ASSUMPTION_DAY,
    13088_i32 => _ALL_SAINTS__DAY,
    13090_i32 => _EID_AL_FITR__ESTIMATED_,
    13128_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    13129_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY__OBSERVED_,
    13142_i32 => _CHRISTMAS_DAY,
    13143_i32 => _CHRISTMAS_DAY__OBSERVED_,
    13149_i32 => _NEW_YEAR_S_DAY,
    13150_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    13151_i32 => _REVOLUTION_DAY,
    13158_i32 => _EID_AL_ADHA__ESTIMATED_,
    13215_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    13248_i32 => _MAWLID__ESTIMATED_,
    13255_i32 => _EASTER_MONDAY,
    13269_i32 => _LABOUR_DAY,
    13293_i32 => _ASCENSION_DAY,
    13365_i32 => _INDEPENDENCE_DAY,
    13375_i32 => _ASSUMPTION_DAY,
    13444_i32 => _EID_AL_FITR__ESTIMATED_,
    13453_i32 => _ALL_SAINTS__DAY,
    13493_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    13507_i32 => _CHRISTMAS_DAY,
    13513_i32 => _EID_AL_ADHA__ESTIMATED_,
    13514_i32 => _NEW_YEAR_S_DAY,
    13516_i32 => _REVOLUTION_DAY,
    13580_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    13603_i32 => _MAWLID__ESTIMATED_,
    13612_i32 => _EASTER_MONDAY,
    13634_i32 => _LABOUR_DAY,
    13650_i32 => _ASCENSION_DAY,
    13730_i32 => _INDEPENDENCE_DAY,
    13731_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    13740_i32 => _ASSUMPTION_DAY,
    13799_i32 => _EID_AL_FITR__ESTIMATED_,
    13818_i32 => _ALL_SAINTS__DAY,
    13858_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    13867_i32 => _EID_AL_ADHA__ESTIMATED_,
    13872_i32 => _CHRISTMAS_DAY,
    13879_i32 => _NEW_YEAR_S_DAY,
    13881_i32 => _REVOLUTION_DAY,
    13946_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    13958_i32 => _MAWLID__ESTIMATED_,
    13962_i32 => _EASTER_MONDAY,
    14000_i32 => _ASCENSION_DAY__LABOUR_DAY,
    14096_i32 => _INDEPENDENCE_DAY,
    14106_i32 => _ASSUMPTION_DAY,
    14153_i32 => _EID_AL_FITR__ESTIMATED_,
    14184_i32 => _ALL_SAINTS__DAY,
    14221_i32 => _EID_AL_ADHA__ESTIMATED_,
    14224_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    14238_i32 => _CHRISTMAS_DAY,
    14245_i32 => _NEW_YEAR_S_DAY,
    14247_i32 => _REVOLUTION_DAY,
    14311_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    14312_i32 => _INTERNATIONAL_WOMEN_S_DAY__OBSERVED___MAWLID__ESTIMATED_,
    14347_i32 => _EASTER_MONDAY,
    14365_i32 => _LABOUR_DAY,
    14385_i32 => _ASCENSION_DAY,
    14461_i32 => _INDEPENDENCE_DAY,
    14471_i32 => _ASSUMPTION_DAY,
    14507_i32 => _EID_AL_FITR__ESTIMATED_,
    14549_i32 => _ALL_SAINTS__DAY,
    14550_i32 => _ALL_SAINTS__DAY__OBSERVED_,
    14575_i32 => _EID_AL_ADHA__ESTIMATED_,
    14589_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    14603_i32 => _CHRISTMAS_DAY,
    14610_i32 => _NEW_YEAR_S_DAY,
    14612_i32 => _REVOLUTION_DAY,
    14613_i32 => _REVOLUTION_DAY__OBSERVED_,
    14666_i32 => _MAWLID__ESTIMATED_,
    14676_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    14704_i32 => _EASTER_MONDAY,
    14730_i32 => _LABOUR_DAY,
    14742_i32 => _ASCENSION_DAY,
    14826_i32 => _INDEPENDENCE_DAY,
    14836_i32 => _ASSUMPTION_DAY,
    14837_i32 => _ASSUMPTION_DAY__OBSERVED_,
    14862_i32 => _EID_AL_FITR__ESTIMATED_,
    14914_i32 => _ALL_SAINTS__DAY,
    14929_i32 => _EID_AL_ADHA__ESTIMATED_,
    14954_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    14968_i32 => _CHRISTMAS_DAY,
    14975_i32 => _NEW_YEAR_S_DAY,
    14977_i32 => _REVOLUTION_DAY,
    15020_i32 => _MAWLID__ESTIMATED_,
    15041_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    15089_i32 => _EASTER_MONDAY,
    15095_i32 => _LABOUR_DAY,
    15096_i32 => _LABOUR_DAY__OBSERVED_,
    15127_i32 => _ASCENSION_DAY,
    15191_i32 => _INDEPENDENCE_DAY,
    15201_i32 => _ASSUMPTION_DAY,
    15216_i32 => _EID_AL_FITR__ESTIMATED_,
    15279_i32 => _ALL_SAINTS__DAY,
    15284_i32 => _EID_AL_ADHA__ESTIMATED_,
    15319_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    15320_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY__OBSERVED_,
    15333_i32 => _CHRISTMAS_DAY,
    15334_i32 => _CHRISTMAS_DAY__OBSERVED_,
    15340_i32 => _NEW_YEAR_S_DAY,
    15341_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    15342_i32 => _REVOLUTION_DAY,
    15374_i32 => _MAWLID__ESTIMATED_,
    15407_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    15439_i32 => _EASTER_MONDAY,
    15461_i32 => _LABOUR_DAY,
    15477_i32 => _ASCENSION_DAY,
    15557_i32 => _INDEPENDENCE_DAY,
    15558_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    15567_i32 => _ASSUMPTION_DAY,
    15571_i32 => _EID_AL_FITR__ESTIMATED_,
    15639_i32 => _EID_AL_ADHA__ESTIMATED_,
    15645_i32 => _ALL_SAINTS__DAY,
    15685_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    15699_i32 => _CHRISTMAS_DAY,
    15706_i32 => _NEW_YEAR_S_DAY,
    15708_i32 => _REVOLUTION_DAY,
    15729_i32 => _MAWLID__ESTIMATED_,
    15772_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    15796_i32 => _EASTER_MONDAY,
    15826_i32 => _LABOUR_DAY,
    15834_i32 => _ASCENSION_DAY,
    15922_i32 => _INDEPENDENCE_DAY,
    15925_i32 => _EID_AL_FITR__ESTIMATED_,
    15932_i32 => _ASSUMPTION_DAY,
    15993_i32 => _EID_AL_ADHA__ESTIMATED_,
    16010_i32 => _ALL_SAINTS__DAY,
    16050_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    16064_i32 => _CHRISTMAS_DAY,
    16071_i32 => _NEW_YEAR_S_DAY,
    16073_i32 => _REVOLUTION_DAY,
    16084_i32 => _MAWLID,
    16137_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    16181_i32 => _EASTER_MONDAY,
    16191_i32 => _LABOUR_DAY,
    16219_i32 => _ASCENSION_DAY,
    16280_i32 => _EID_AL_FITR,
    16287_i32 => _INDEPENDENCE_DAY,
    16297_i32 => _ASSUMPTION_DAY,
    16348_i32 => _EID_AL_ADHA,
    16375_i32 => _ALL_SAINTS__DAY,
    16415_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    16429_i32 => _CHRISTMAS_DAY,
    16436_i32 => _NEW_YEAR_S_DAY,
    16438_i32 => _MAWLID__REVOLUTION_DAY,
    16502_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    16503_i32 => _INTERNATIONAL_WOMEN_S_DAY__OBSERVED_,
    16531_i32 => _EASTER_MONDAY,
    16556_i32 => _LABOUR_DAY,
    16569_i32 => _ASCENSION_DAY,
    16634_i32 => _EID_AL_FITR,
    16652_i32 => _INDEPENDENCE_DAY,
    16662_i32 => _ASSUMPTION_DAY,
    16702_i32 => _EID_AL_ADHA,
    16740_i32 => _ALL_SAINTS__DAY,
    16741_i32 => _ALL_SAINTS__DAY__OBSERVED_,
    16780_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    16793_i32 => _MAWLID,
    16794_i32 => _CHRISTMAS_DAY,
    16801_i32 => _NEW_YEAR_S_DAY,
    16803_i32 => _REVOLUTION_DAY,
    16804_i32 => _REVOLUTION_DAY__OBSERVED_,
    16868_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    16888_i32 => _EASTER_MONDAY,
    16922_i32 => _LABOUR_DAY,
    16923_i32 => _LABOUR_DAY__OBSERVED_,
    16926_i32 => _ASCENSION_DAY,
    16989_i32 => _EID_AL_FITR,
    17018_i32 => _INDEPENDENCE_DAY,
    17028_i32 => _ASSUMPTION_DAY,
    17057_i32 => _EID_AL_ADHA,
    17105_i32 => _MARTYRS__DAY,
    17106_i32 => _ALL_SAINTS__DAY,
    17146_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    17147_i32 => _MAWLID__PROCLAMATION_OF_INDEPENDENCE_DAY__OBSERVED_,
    17160_i32 => _CHRISTMAS_DAY,
    17161_i32 => _CHRISTMAS_DAY__OBSERVED_,
    17167_i32 => _NEW_YEAR_S_DAY,
    17168_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    17169_i32 => _REVOLUTION_DAY,
    17233_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    17273_i32 => _EASTER_MONDAY,
    17287_i32 => _LABOUR_DAY,
    17311_i32 => _ASCENSION_DAY,
    17343_i32 => _EID_AL_FITR,
    17383_i32 => _INDEPENDENCE_DAY,
    17393_i32 => _ASSUMPTION_DAY,
    17411_i32 => _EID_AL_ADHA,
    17470_i32 => _MARTYRS__DAY,
    17471_i32 => _ALL_SAINTS__DAY,
    17501_i32 => _MAWLID,
    17511_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    17525_i32 => _CHRISTMAS_DAY,
    17532_i32 => _NEW_YEAR_S_DAY,
    17534_i32 => _REVOLUTION_DAY,
    17598_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    17623_i32 => _EASTER_MONDAY,
    17652_i32 => _LABOUR_DAY,
    17661_i32 => _ASCENSION_DAY,
    17697_i32 => _EID_AL_FITR,
    17748_i32 => _INDEPENDENCE_DAY,
    17749_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    17758_i32 => _ASSUMPTION_DAY,
    17764_i32 => _EID_AL_ADHA,
    17835_i32 => _MARTYRS__DAY,
    17836_i32 => _ALL_SAINTS__DAY,
    17856_i32 => _MAWLID,
    17876_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    17890_i32 => _CHRISTMAS_DAY,
    17897_i32 => _NEW_YEAR_S_DAY,
    17899_i32 => _REVOLUTION_DAY,
    17963_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    18008_i32 => _EASTER_MONDAY,
    18017_i32 => _LABOUR_DAY,
    18046_i32 => _ASCENSION_DAY,
    18051_i32 => _EID_AL_FITR,
    18113_i32 => _INDEPENDENCE_DAY,
    18119_i32 => _EID_AL_ADHA,
    18123_i32 => _ASSUMPTION_DAY,
    18200_i32 => _MARTYRS__DAY,
    18201_i32 => _ALL_SAINTS__DAY,
    18210_i32 => _MAWLID,
    18241_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    18255_i32 => _CHRISTMAS_DAY,
    18262_i32 => _NEW_YEAR_S_DAY,
    18264_i32 => _REVOLUTION_DAY,
    18329_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    18330_i32 => _INTERNATIONAL_WOMEN_S_DAY__OBSERVED_,
    18365_i32 => _EASTER_MONDAY,
    18383_i32 => _LABOUR_DAY,
    18403_i32 => _ASCENSION_DAY,
    18406_i32 => _EID_AL_FITR,
    18474_i32 => _EID_AL_ADHA,
    18479_i32 => _INDEPENDENCE_DAY,
    18489_i32 => _ASSUMPTION_DAY,
    18564_i32 => _MAWLID,
    18566_i32 => _MARTYRS__DAY,
    18567_i32 => _ALL_SAINTS__DAY,
    18568_i32 => _ALL_SAINTS__DAY__OBSERVED_,
    18607_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    18621_i32 => _CHRISTMAS_DAY,
    18628_i32 => _NEW_YEAR_S_DAY,
    18630_i32 => _REVOLUTION_DAY,
    18631_i32 => _REVOLUTION_DAY__OBSERVED_,
    18694_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    18722_i32 => _EASTER_MONDAY,
    18748_i32 => _LABOUR_DAY,
    18760_i32 => _ASCENSION_DAY__EID_AL_FITR,
    18828_i32 => _EID_AL_ADHA,
    18844_i32 => _INDEPENDENCE_DAY,
    18854_i32 => _ASSUMPTION_DAY,
    18855_i32 => _ASSUMPTION_DAY__OBSERVED_,
    18919_i32 => _MAWLID,
    18931_i32 => _MARTYRS__DAY,
    18932_i32 => _ALL_SAINTS__DAY__MARTYRS__DAY__OBSERVED_,
    18972_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    18986_i32 => _CHRISTMAS_DAY,
    18993_i32 => _NEW_YEAR_S_DAY,
    18995_i32 => _REVOLUTION_DAY,
    19059_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    19100_i32 => _EASTER_MONDAY,
    19113_i32 => _LABOUR_DAY,
    19114_i32 => _EID_AL_FITR__LABOUR_DAY__OBSERVED_,
    19138_i32 => _ASCENSION_DAY,
    19182_i32 => _EID_AL_ADHA,
    19209_i32 => _INDEPENDENCE_DAY,
    19219_i32 => _ASSUMPTION_DAY,
    19274_i32 => _MAWLID,
    19296_i32 => _MARTYRS__DAY,
    19297_i32 => _ALL_SAINTS__DAY,
    19337_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    19338_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY__OBSERVED_,
    19351_i32 => _CHRISTMAS_DAY,
    19352_i32 => _CHRISTMAS_DAY__OBSERVED_,
    19358_i32 => _NEW_YEAR_S_DAY,
    19359_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    19360_i32 => _REVOLUTION_DAY,
    19424_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    19457_i32 => _EASTER_MONDAY,
    19468_i32 => _EID_AL_FITR,
    19478_i32 => _LABOUR_DAY,
    19495_i32 => _ASCENSION_DAY,
    19536_i32 => _EID_AL_ADHA,
    19574_i32 => _INDEPENDENCE_DAY,
    19584_i32 => _ASSUMPTION_DAY,
    19627_i32 => _MAWLID__ESTIMATED_,
    19661_i32 => _MARTYRS__DAY,
    19662_i32 => _ALL_SAINTS__DAY,
    19702_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    19716_i32 => _CHRISTMAS_DAY,
    19723_i32 => _NEW_YEAR_S_DAY,
    19725_i32 => _REVOLUTION_DAY,
    19790_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    19814_i32 => _EASTER_MONDAY,
    19823_i32 => _EID_AL_FITR,
    19844_i32 => _LABOUR_DAY,
    19852_i32 => _ASCENSION_DAY,
    19890_i32 => _EID_AL_ADHA__ESTIMATED_,
    19940_i32 => _INDEPENDENCE_DAY,
    19950_i32 => _ASSUMPTION_DAY,
    19981_i32 => _MAWLID__ESTIMATED_,
    20027_i32 => _MARTYRS__DAY,
    20028_i32 => _ALL_SAINTS__DAY,
    20068_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    20082_i32 => _CHRISTMAS_DAY,
    20089_i32 => _NEW_YEAR_S_DAY,
    20091_i32 => _REVOLUTION_DAY,
    20155_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    20177_i32 => _EID_AL_FITR__ESTIMATED_,
    20199_i32 => _EASTER_MONDAY,
    20209_i32 => _LABOUR_DAY,
    20237_i32 => _ASCENSION_DAY,
    20245_i32 => _EID_AL_ADHA__ESTIMATED_,
    20305_i32 => _INDEPENDENCE_DAY,
    20315_i32 => _ASSUMPTION_DAY,
    20335_i32 => _MAWLID__ESTIMATED_,
    20392_i32 => _MARTYRS__DAY,
    20393_i32 => _ALL_SAINTS__DAY,
    20433_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    20447_i32 => _CHRISTMAS_DAY,
    20454_i32 => _NEW_YEAR_S_DAY,
    20456_i32 => _REVOLUTION_DAY,
    20520_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    20521_i32 => _INTERNATIONAL_WOMEN_S_DAY__OBSERVED_,
    20532_i32 => _EID_AL_FITR__ESTIMATED_,
    20549_i32 => _EASTER_MONDAY,
    20574_i32 => _LABOUR_DAY,
    20587_i32 => _ASCENSION_DAY,
    20600_i32 => _EID_AL_ADHA__ESTIMATED_,
    20670_i32 => _INDEPENDENCE_DAY,
    20680_i32 => _ASSUMPTION_DAY,
    20690_i32 => _MAWLID__ESTIMATED_,
    20757_i32 => _MARTYRS__DAY,
    20758_i32 => _ALL_SAINTS__DAY,
    20759_i32 => _ALL_SAINTS__DAY__OBSERVED_,
    20798_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    20812_i32 => _CHRISTMAS_DAY,
    20819_i32 => _NEW_YEAR_S_DAY,
    20821_i32 => _REVOLUTION_DAY,
    20822_i32 => _REVOLUTION_DAY__OBSERVED_,
    20885_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    20886_i32 => _EID_AL_FITR__ESTIMATED_,
    20906_i32 => _EASTER_MONDAY,
    20939_i32 => _LABOUR_DAY,
    20944_i32 => _ASCENSION_DAY,
    20954_i32 => _EID_AL_ADHA__ESTIMATED_,
    21035_i32 => _INDEPENDENCE_DAY,
    21044_i32 => _MAWLID__ESTIMATED_,
    21045_i32 => _ASSUMPTION_DAY,
    21046_i32 => _ASSUMPTION_DAY__OBSERVED_,
    21122_i32 => _MARTYRS__DAY,
    21123_i32 => _ALL_SAINTS__DAY__MARTYRS__DAY__OBSERVED_,
    21163_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    21177_i32 => _CHRISTMAS_DAY,
    21184_i32 => _NEW_YEAR_S_DAY,
    21186_i32 => _REVOLUTION_DAY,
    21240_i32 => _EID_AL_FITR__ESTIMATED_,
    21251_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    21291_i32 => _EASTER_MONDAY,
    21305_i32 => _LABOUR_DAY,
    21309_i32 => _EID_AL_ADHA__ESTIMATED_,
    21329_i32 => _ASCENSION_DAY,
    21399_i32 => _MAWLID__ESTIMATED_,
    21401_i32 => _INDEPENDENCE_DAY,
    21411_i32 => _ASSUMPTION_DAY,
    21488_i32 => _MARTYRS__DAY,
    21489_i32 => _ALL_SAINTS__DAY,
    21529_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    21543_i32 => _CHRISTMAS_DAY,
    21550_i32 => _NEW_YEAR_S_DAY,
    21552_i32 => _REVOLUTION_DAY,
    21594_i32 => _EID_AL_FITR__ESTIMATED_,
    21616_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    21641_i32 => _EASTER_MONDAY,
    21663_i32 => _EID_AL_ADHA__ESTIMATED_,
    21670_i32 => _LABOUR_DAY,
    21679_i32 => _ASCENSION_DAY,
    21754_i32 => _MAWLID__ESTIMATED_,
    21766_i32 => _INDEPENDENCE_DAY,
    21767_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    21776_i32 => _ASSUMPTION_DAY,
    21853_i32 => _MARTYRS__DAY,
    21854_i32 => _ALL_SAINTS__DAY,
    21894_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    21908_i32 => _CHRISTMAS_DAY,
    21915_i32 => _NEW_YEAR_S_DAY,
    21917_i32 => _REVOLUTION_DAY,
    21949_i32 => _EID_AL_FITR__ESTIMATED_,
    21981_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    22017_i32 => _EID_AL_ADHA__ESTIMATED_,
    22026_i32 => _EASTER_MONDAY,
    22035_i32 => _LABOUR_DAY,
    22064_i32 => _ASCENSION_DAY,
    22108_i32 => _MAWLID__ESTIMATED_,
    22131_i32 => _INDEPENDENCE_DAY,
    22141_i32 => _ASSUMPTION_DAY,
    22218_i32 => _MARTYRS__DAY,
    22219_i32 => _ALL_SAINTS__DAY,
    22259_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    22273_i32 => _CHRISTMAS_DAY,
    22280_i32 => _NEW_YEAR_S_DAY,
    22282_i32 => _REVOLUTION_DAY,
    22303_i32 => _EID_AL_FITR__ESTIMATED_,
    22346_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    22371_i32 => _EID_AL_ADHA__ESTIMATED_,
    22383_i32 => _EASTER_MONDAY,
    22400_i32 => _LABOUR_DAY,
    22421_i32 => _ASCENSION_DAY,
    22462_i32 => _MAWLID__ESTIMATED_,
    22496_i32 => _INDEPENDENCE_DAY,
    22506_i32 => _ASSUMPTION_DAY,
    22583_i32 => _MARTYRS__DAY,
    22584_i32 => _ALL_SAINTS__DAY,
    22624_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    22638_i32 => _CHRISTMAS_DAY,
    22645_i32 => _NEW_YEAR_S_DAY,
    22647_i32 => _REVOLUTION_DAY,
    22658_i32 => _EID_AL_FITR__ESTIMATED_,
    22712_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    22726_i32 => _EID_AL_ADHA__ESTIMATED_,
    22733_i32 => _EASTER_MONDAY,
    22766_i32 => _LABOUR_DAY,
    22771_i32 => _ASCENSION_DAY,
    22816_i32 => _MAWLID__ESTIMATED_,
    22862_i32 => _INDEPENDENCE_DAY,
    22872_i32 => _ASSUMPTION_DAY,
    22873_i32 => _ASSUMPTION_DAY__OBSERVED_,
    22949_i32 => _MARTYRS__DAY,
    22950_i32 => _ALL_SAINTS__DAY__MARTYRS__DAY__OBSERVED_,
    22990_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    23004_i32 => _CHRISTMAS_DAY,
    23011_i32 => _NEW_YEAR_S_DAY,
    23012_i32 => _EID_AL_FITR__ESTIMATED_,
    23013_i32 => _REVOLUTION_DAY,
    23077_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    23080_i32 => _EID_AL_ADHA__ESTIMATED_,
    23118_i32 => _EASTER_MONDAY,
    23131_i32 => _LABOUR_DAY,
    23132_i32 => _LABOUR_DAY__OBSERVED_,
    23156_i32 => _ASCENSION_DAY,
    23170_i32 => _MAWLID__ESTIMATED_,
    23227_i32 => _INDEPENDENCE_DAY,
    23237_i32 => _ASSUMPTION_DAY,
    23314_i32 => _MARTYRS__DAY,
    23315_i32 => _ALL_SAINTS__DAY,
    23355_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    23356_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY__OBSERVED_,
    23367_i32 => _EID_AL_FITR__ESTIMATED_,
    23369_i32 => _CHRISTMAS_DAY,
    23370_i32 => _CHRISTMAS_DAY__OBSERVED_,
    23376_i32 => _NEW_YEAR_S_DAY,
    23377_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    23378_i32 => _REVOLUTION_DAY,
    23435_i32 => _EID_AL_ADHA__ESTIMATED_,
    23442_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    23475_i32 => _EASTER_MONDAY,
    23496_i32 => _LABOUR_DAY,
    23513_i32 => _ASCENSION_DAY,
    23525_i32 => _MAWLID__ESTIMATED_,
    23592_i32 => _INDEPENDENCE_DAY,
    23602_i32 => _ASSUMPTION_DAY,
    23679_i32 => _MARTYRS__DAY,
    23680_i32 => _ALL_SAINTS__DAY,
    23720_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    23721_i32 => _EID_AL_FITR__ESTIMATED_,
    23734_i32 => _CHRISTMAS_DAY,
    23741_i32 => _NEW_YEAR_S_DAY,
    23743_i32 => _REVOLUTION_DAY,
    23789_i32 => _EID_AL_ADHA__ESTIMATED_,
    23807_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    23825_i32 => _EASTER_MONDAY,
    23861_i32 => _LABOUR_DAY,
    23863_i32 => _ASCENSION_DAY,
    23880_i32 => _MAWLID__ESTIMATED_,
    23957_i32 => _INDEPENDENCE_DAY,
    23958_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    23967_i32 => _ASSUMPTION_DAY,
    24044_i32 => _MARTYRS__DAY,
    24045_i32 => _ALL_SAINTS__DAY,
    24075_i32 => _EID_AL_FITR__ESTIMATED_,
    24085_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    24099_i32 => _CHRISTMAS_DAY,
    24106_i32 => _NEW_YEAR_S_DAY,
    24108_i32 => _REVOLUTION_DAY,
    24143_i32 => _EID_AL_ADHA__ESTIMATED_,
    24173_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    24210_i32 => _EASTER_MONDAY,
    24227_i32 => _LABOUR_DAY,
    24234_i32 => _MAWLID__ESTIMATED_,
    24248_i32 => _ASCENSION_DAY,
    24323_i32 => _INDEPENDENCE_DAY,
    24333_i32 => _ASSUMPTION_DAY,
    24410_i32 => _MARTYRS__DAY,
    24411_i32 => _ALL_SAINTS__DAY,
    24429_i32 => _EID_AL_FITR__ESTIMATED_,
    24451_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    24465_i32 => _CHRISTMAS_DAY,
    24472_i32 => _NEW_YEAR_S_DAY,
    24474_i32 => _REVOLUTION_DAY,
    24497_i32 => _EID_AL_ADHA__ESTIMATED_,
    24538_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    24539_i32 => _INTERNATIONAL_WOMEN_S_DAY__OBSERVED_,
    24567_i32 => _EASTER_MONDAY,
    24589_i32 => _MAWLID__ESTIMATED_,
    24592_i32 => _LABOUR_DAY,
    24605_i32 => _ASCENSION_DAY,
    24688_i32 => _INDEPENDENCE_DAY,
    24698_i32 => _ASSUMPTION_DAY,
    24775_i32 => _MARTYRS__DAY,
    24776_i32 => _ALL_SAINTS__DAY,
    24777_i32 => _ALL_SAINTS__DAY__OBSERVED_,
    24783_i32 => _EID_AL_FITR__ESTIMATED_,
    24816_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    24830_i32 => _CHRISTMAS_DAY,
    24837_i32 => _NEW_YEAR_S_DAY,
    24839_i32 => _REVOLUTION_DAY,
    24840_i32 => _REVOLUTION_DAY__OBSERVED_,
    24852_i32 => _EID_AL_ADHA__ESTIMATED_,
    24903_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    24943_i32 => _MAWLID__ESTIMATED_,
    24952_i32 => _EASTER_MONDAY,
    24957_i32 => _LABOUR_DAY,
    24990_i32 => _ASCENSION_DAY,
    25053_i32 => _INDEPENDENCE_DAY,
    25063_i32 => _ASSUMPTION_DAY,
    25064_i32 => _ASSUMPTION_DAY__OBSERVED_,
    25138_i32 => _EID_AL_FITR__ESTIMATED_,
    25140_i32 => _MARTYRS__DAY,
    25141_i32 => _ALL_SAINTS__DAY__MARTYRS__DAY__OBSERVED_,
    25181_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    25195_i32 => _CHRISTMAS_DAY,
    25202_i32 => _NEW_YEAR_S_DAY,
    25204_i32 => _REVOLUTION_DAY,
    25206_i32 => _EID_AL_ADHA__ESTIMATED_,
    25268_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    25297_i32 => _MAWLID__ESTIMATED_,
    25302_i32 => _EASTER_MONDAY,
    25322_i32 => _LABOUR_DAY,
    25323_i32 => _LABOUR_DAY__OBSERVED_,
    25340_i32 => _ASCENSION_DAY,
    25418_i32 => _INDEPENDENCE_DAY,
    25428_i32 => _ASSUMPTION_DAY,
    25493_i32 => _EID_AL_FITR__ESTIMATED_,
    25505_i32 => _MARTYRS__DAY,
    25506_i32 => _ALL_SAINTS__DAY,
    25546_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    25547_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY__OBSERVED_,
    25560_i32 => _CHRISTMAS_DAY,
    25561_i32 => _CHRISTMAS_DAY__OBSERVED___EID_AL_ADHA__ESTIMATED_,
    25567_i32 => _NEW_YEAR_S_DAY,
    25568_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    25569_i32 => _REVOLUTION_DAY,
    25634_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    25651_i32 => _MAWLID__ESTIMATED_,
    25659_i32 => _EASTER_MONDAY,
    25688_i32 => _LABOUR_DAY,
    25697_i32 => _ASCENSION_DAY,
    25784_i32 => _INDEPENDENCE_DAY,
    25785_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    25794_i32 => _ASSUMPTION_DAY,
    25847_i32 => _EID_AL_FITR__ESTIMATED_,
    25871_i32 => _MARTYRS__DAY,
    25872_i32 => _ALL_SAINTS__DAY,
    25912_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    25915_i32 => _EID_AL_ADHA__ESTIMATED_,
    25926_i32 => _CHRISTMAS_DAY,
    25933_i32 => _NEW_YEAR_S_DAY,
    25935_i32 => _REVOLUTION_DAY,
    25999_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    26006_i32 => _MAWLID__ESTIMATED_,
    26044_i32 => _EASTER_MONDAY,
    26053_i32 => _LABOUR_DAY,
    26082_i32 => _ASCENSION_DAY,
    26149_i32 => _INDEPENDENCE_DAY,
    26159_i32 => _ASSUMPTION_DAY,
    26201_i32 => _EID_AL_FITR__ESTIMATED_,
    26236_i32 => _MARTYRS__DAY,
    26237_i32 => _ALL_SAINTS__DAY,
    26270_i32 => _EID_AL_ADHA__ESTIMATED_,
    26277_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    26291_i32 => _CHRISTMAS_DAY,
    26298_i32 => _NEW_YEAR_S_DAY,
    26300_i32 => _REVOLUTION_DAY,
    26360_i32 => _MAWLID__ESTIMATED_,
    26364_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    26394_i32 => _EASTER_MONDAY,
    26418_i32 => _LABOUR_DAY,
    26432_i32 => _ASCENSION_DAY,
    26514_i32 => _INDEPENDENCE_DAY,
    26524_i32 => _ASSUMPTION_DAY,
    26555_i32 => _EID_AL_FITR__ESTIMATED_,
    26601_i32 => _MARTYRS__DAY,
    26602_i32 => _ALL_SAINTS__DAY,
    26624_i32 => _EID_AL_ADHA__ESTIMATED_,
    26642_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    26656_i32 => _CHRISTMAS_DAY,
    26663_i32 => _NEW_YEAR_S_DAY,
    26665_i32 => _REVOLUTION_DAY,
    26715_i32 => _MAWLID__ESTIMATED_,
    26729_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    26730_i32 => _INTERNATIONAL_WOMEN_S_DAY__OBSERVED_,
    26751_i32 => _EASTER_MONDAY,
    26783_i32 => _LABOUR_DAY,
    26789_i32 => _ASCENSION_DAY,
    26879_i32 => _INDEPENDENCE_DAY,
    26889_i32 => _ASSUMPTION_DAY,
    26909_i32 => _EID_AL_FITR__ESTIMATED_,
    26966_i32 => _MARTYRS__DAY,
    26967_i32 => _ALL_SAINTS__DAY,
    26968_i32 => _ALL_SAINTS__DAY__OBSERVED_,
    26978_i32 => _EID_AL_ADHA__ESTIMATED_,
    27007_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    27021_i32 => _CHRISTMAS_DAY,
    27028_i32 => _NEW_YEAR_S_DAY,
    27030_i32 => _REVOLUTION_DAY,
    27031_i32 => _REVOLUTION_DAY__OBSERVED_,
    27069_i32 => _MAWLID__ESTIMATED_,
    27095_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    27136_i32 => _EASTER_MONDAY,
    27149_i32 => _LABOUR_DAY,
    27150_i32 => _LABOUR_DAY__OBSERVED_,
    27174_i32 => _ASCENSION_DAY,
    27245_i32 => _INDEPENDENCE_DAY,
    27255_i32 => _ASSUMPTION_DAY,
    27264_i32 => _EID_AL_FITR__ESTIMATED_,
    27332_i32 => _EID_AL_ADHA__ESTIMATED___MARTYRS__DAY,
    27333_i32 => _ALL_SAINTS__DAY,
    27373_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    27374_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY__OBSERVED_,
    27387_i32 => _CHRISTMAS_DAY,
    27388_i32 => _CHRISTMAS_DAY__OBSERVED_,
    27394_i32 => _NEW_YEAR_S_DAY,
    27395_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    27396_i32 => _REVOLUTION_DAY,
    27423_i32 => _MAWLID__ESTIMATED_,
    27460_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    27493_i32 => _EASTER_MONDAY,
    27514_i32 => _LABOUR_DAY,
    27531_i32 => _ASCENSION_DAY,
    27610_i32 => _INDEPENDENCE_DAY,
    27619_i32 => _EID_AL_FITR__ESTIMATED_,
    27620_i32 => _ASSUMPTION_DAY,
    27687_i32 => _EID_AL_ADHA__ESTIMATED_,
    27697_i32 => _MARTYRS__DAY,
    27698_i32 => _ALL_SAINTS__DAY,
    27738_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    27752_i32 => _CHRISTMAS_DAY,
    27759_i32 => _NEW_YEAR_S_DAY,
    27761_i32 => _REVOLUTION_DAY,
    27777_i32 => _MAWLID__ESTIMATED_,
    27825_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    27843_i32 => _EASTER_MONDAY,
    27879_i32 => _LABOUR_DAY,
    27881_i32 => _ASCENSION_DAY,
    27973_i32 => _EID_AL_FITR__ESTIMATED_,
    27975_i32 => _INDEPENDENCE_DAY,
    27976_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    27985_i32 => _ASSUMPTION_DAY,
    28041_i32 => _EID_AL_ADHA__ESTIMATED_,
    28062_i32 => _MARTYRS__DAY,
    28063_i32 => _ALL_SAINTS__DAY,
    28103_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    28117_i32 => _CHRISTMAS_DAY,
    28124_i32 => _NEW_YEAR_S_DAY,
    28126_i32 => _REVOLUTION_DAY,
    28131_i32 => _MAWLID__ESTIMATED_,
    28190_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    28228_i32 => _EASTER_MONDAY,
    28244_i32 => _LABOUR_DAY,
    28266_i32 => _ASCENSION_DAY,
    28328_i32 => _EID_AL_FITR__ESTIMATED_,
    28340_i32 => _INDEPENDENCE_DAY,
    28350_i32 => _ASSUMPTION_DAY,
    28396_i32 => _EID_AL_ADHA__ESTIMATED_,
    28427_i32 => _MARTYRS__DAY,
    28428_i32 => _ALL_SAINTS__DAY,
    28468_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    28482_i32 => _CHRISTMAS_DAY,
    28486_i32 => _MAWLID__ESTIMATED_,
    28489_i32 => _NEW_YEAR_S_DAY,
    28491_i32 => _REVOLUTION_DAY,
    28556_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    28557_i32 => _INTERNATIONAL_WOMEN_S_DAY__OBSERVED_,
    28585_i32 => _EASTER_MONDAY,
    28610_i32 => _LABOUR_DAY,
    28623_i32 => _ASCENSION_DAY,
    28682_i32 => _EID_AL_FITR__ESTIMATED_,
    28706_i32 => _INDEPENDENCE_DAY,
    28716_i32 => _ASSUMPTION_DAY,
    28751_i32 => _EID_AL_ADHA__ESTIMATED_,
    28793_i32 => _MARTYRS__DAY,
    28794_i32 => _ALL_SAINTS__DAY,
    28795_i32 => _ALL_SAINTS__DAY__OBSERVED_,
    28834_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    28841_i32 => _MAWLID__ESTIMATED_,
    28848_i32 => _CHRISTMAS_DAY,
    28855_i32 => _NEW_YEAR_S_DAY,
    28857_i32 => _REVOLUTION_DAY,
    28858_i32 => _REVOLUTION_DAY__OBSERVED_,
    28921_i32 => _INTERNATIONAL_WOMEN_S_DAY,
    28963_i32 => _EASTER_MONDAY,
    28975_i32 => _LABOUR_DAY,
    29001_i32 => _ASCENSION_DAY,
    29036_i32 => _EID_AL_FITR__ESTIMATED_,
    29071_i32 => _INDEPENDENCE_DAY,
    29081_i32 => _ASSUMPTION_DAY,
    29082_i32 => _ASSUMPTION_DAY__OBSERVED_,
    29105_i32 => _EID_AL_ADHA__ESTIMATED_,
    29158_i32 => _MARTYRS__DAY,
    29159_i32 => _ALL_SAINTS__DAY__MARTYRS__DAY__OBSERVED_,
    29195_i32 => _MAWLID__ESTIMATED_,
    29199_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    29213_i32 => _CHRISTMAS_DAY,
    29220_i32 => _NEW_YEAR_S_DAY,
};
