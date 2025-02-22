use crate::countries::constants::*;
use phf::phf_map;

pub static CM_HOLIDAYS: phf::Map<i32, &'static str> = phf_map! {
    10957_i32 => _NEW_YEAR_S_DAY,
    10964_i32 => _EID_AL_FITR__ESTIMATED_,
    10998_i32 => _YOUTH_DAY,
    11032_i32 => _EID_AL_ADHA__ESTIMATED_,
    11068_i32 => _GOOD_FRIDAY,
    11078_i32 => _LABOUR_DAY,
    11097_i32 => _NATIONAL_DAY,
    11109_i32 => _ASCENSION_DAY,
    11122_i32 => _MAWLID__ESTIMATED_,
    11184_i32 => _ASSUMPTION_DAY,
    11316_i32 => _CHRISTMAS_DAY,
    11318_i32 => _EID_AL_FITR__ESTIMATED_,
    11323_i32 => _NEW_YEAR_S_DAY,
    11364_i32 => _YOUTH_DAY,
    11365_i32 => _YOUTH_DAY__OBSERVED_,
    11387_i32 => _EID_AL_ADHA,
    11425_i32 => _GOOD_FRIDAY,
    11443_i32 => _LABOUR_DAY,
    11462_i32 => _NATIONAL_DAY,
    11463_i32 => _NATIONAL_DAY__OBSERVED_,
    11466_i32 => _ASCENSION_DAY,
    11477_i32 => _MAWLID,
    11549_i32 => _ASSUMPTION_DAY,
    11673_i32 => _EID_AL_FITR,
    11681_i32 => _CHRISTMAS_DAY,
    11688_i32 => _NEW_YEAR_S_DAY,
    11729_i32 => _YOUTH_DAY,
    11741_i32 => _EID_AL_ADHA,
    11775_i32 => _GOOD_FRIDAY,
    11808_i32 => _LABOUR_DAY,
    11816_i32 => _ASCENSION_DAY,
    11827_i32 => _NATIONAL_DAY,
    11831_i32 => _MAWLID,
    11914_i32 => _ASSUMPTION_DAY,
    12027_i32 => _EID_AL_FITR,
    12046_i32 => _CHRISTMAS_DAY,
    12053_i32 => _NEW_YEAR_S_DAY,
    12094_i32 => _YOUTH_DAY,
    12095_i32 => _EID_AL_ADHA,
    12160_i32 => _GOOD_FRIDAY,
    12173_i32 => _LABOUR_DAY,
    12186_i32 => _MAWLID,
    12192_i32 => _NATIONAL_DAY,
    12201_i32 => _ASCENSION_DAY,
    12279_i32 => _ASSUMPTION_DAY,
    12382_i32 => _EID_AL_FITR,
    12411_i32 => _CHRISTMAS_DAY,
    12418_i32 => _NEW_YEAR_S_DAY,
    12450_i32 => _EID_AL_ADHA,
    12459_i32 => _YOUTH_DAY,
    12517_i32 => _GOOD_FRIDAY,
    12539_i32 => _LABOUR_DAY,
    12540_i32 => _MAWLID,
    12541_i32 => _MAWLID__OBSERVED_,
    12558_i32 => _ASCENSION_DAY__NATIONAL_DAY,
    12645_i32 => _ASSUMPTION_DAY,
    12646_i32 => _ASSUMPTION_DAY__OBSERVED_,
    12736_i32 => _EID_AL_FITR,
    12737_i32 => _EID_AL_FITR__OBSERVED_,
    12777_i32 => _CHRISTMAS_DAY,
    12784_i32 => _NEW_YEAR_S_DAY,
    12804_i32 => _EID_AL_ADHA,
    12825_i32 => _YOUTH_DAY,
    12867_i32 => _GOOD_FRIDAY,
    12894_i32 => _MAWLID,
    12904_i32 => _LABOUR_DAY,
    12905_i32 => _LABOUR_DAY__OBSERVED_,
    12908_i32 => _ASCENSION_DAY,
    12923_i32 => _NATIONAL_DAY,
    13010_i32 => _ASSUMPTION_DAY,
    13091_i32 => _EID_AL_FITR,
    13142_i32 => _CHRISTMAS_DAY,
    13143_i32 => _CHRISTMAS_DAY__OBSERVED_,
    13149_i32 => _NEW_YEAR_S_DAY,
    13150_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    13158_i32 => _EID_AL_ADHA,
    13190_i32 => _YOUTH_DAY,
    13249_i32 => _MAWLID,
    13252_i32 => _GOOD_FRIDAY,
    13269_i32 => _LABOUR_DAY,
    13288_i32 => _NATIONAL_DAY,
    13293_i32 => _ASCENSION_DAY,
    13375_i32 => _ASSUMPTION_DAY,
    13445_i32 => _EID_AL_FITR,
    13507_i32 => _CHRISTMAS_DAY,
    13513_i32 => _EID_AL_ADHA,
    13514_i32 => _NEW_YEAR_S_DAY,
    13515_i32 => _EID_AL_ADHA__OBSERVED_,
    13555_i32 => _YOUTH_DAY,
    13556_i32 => _YOUTH_DAY__OBSERVED_,
    13603_i32 => _MAWLID,
    13609_i32 => _GOOD_FRIDAY,
    13634_i32 => _LABOUR_DAY,
    13650_i32 => _ASCENSION_DAY,
    13653_i32 => _NATIONAL_DAY,
    13654_i32 => _NATIONAL_DAY__OBSERVED_,
    13740_i32 => _ASSUMPTION_DAY,
    13799_i32 => _EID_AL_FITR,
    13867_i32 => _EID_AL_ADHA,
    13872_i32 => _CHRISTMAS_DAY,
    13879_i32 => _NEW_YEAR_S_DAY,
    13920_i32 => _YOUTH_DAY,
    13958_i32 => _MAWLID,
    13959_i32 => _GOOD_FRIDAY,
    14000_i32 => _ASCENSION_DAY__LABOUR_DAY,
    14019_i32 => _NATIONAL_DAY,
    14106_i32 => _ASSUMPTION_DAY,
    14154_i32 => _EID_AL_FITR,
    14222_i32 => _EID_AL_ADHA,
    14238_i32 => _CHRISTMAS_DAY,
    14245_i32 => _NEW_YEAR_S_DAY,
    14286_i32 => _YOUTH_DAY,
    14312_i32 => _MAWLID,
    14344_i32 => _GOOD_FRIDAY,
    14365_i32 => _LABOUR_DAY,
    14384_i32 => _NATIONAL_DAY,
    14385_i32 => _ASCENSION_DAY,
    14471_i32 => _ASSUMPTION_DAY,
    14508_i32 => _EID_AL_FITR,
    14576_i32 => _EID_AL_ADHA,
    14603_i32 => _CHRISTMAS_DAY,
    14610_i32 => _NEW_YEAR_S_DAY,
    14651_i32 => _YOUTH_DAY,
    14666_i32 => _MAWLID,
    14701_i32 => _GOOD_FRIDAY,
    14730_i32 => _LABOUR_DAY,
    14742_i32 => _ASCENSION_DAY,
    14749_i32 => _NATIONAL_DAY,
    14836_i32 => _ASSUMPTION_DAY,
    14837_i32 => _ASSUMPTION_DAY__OBSERVED_,
    14862_i32 => _EID_AL_FITR,
    14930_i32 => _EID_AL_ADHA,
    14968_i32 => _CHRISTMAS_DAY,
    14975_i32 => _NEW_YEAR_S_DAY,
    15016_i32 => _YOUTH_DAY,
    15021_i32 => _MAWLID,
    15086_i32 => _GOOD_FRIDAY,
    15095_i32 => _LABOUR_DAY,
    15096_i32 => _LABOUR_DAY__OBSERVED_,
    15114_i32 => _NATIONAL_DAY,
    15127_i32 => _ASCENSION_DAY,
    15201_i32 => _ASSUMPTION_DAY,
    15217_i32 => _EID_AL_FITR,
    15285_i32 => _EID_AL_ADHA,
    15333_i32 => _CHRISTMAS_DAY,
    15334_i32 => _CHRISTMAS_DAY__OBSERVED_,
    15340_i32 => _NEW_YEAR_S_DAY,
    15341_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    15375_i32 => _MAWLID,
    15376_i32 => _MAWLID__OBSERVED_,
    15381_i32 => _YOUTH_DAY,
    15436_i32 => _GOOD_FRIDAY,
    15461_i32 => _LABOUR_DAY,
    15477_i32 => _ASCENSION_DAY,
    15480_i32 => _NATIONAL_DAY,
    15481_i32 => _NATIONAL_DAY__OBSERVED_,
    15567_i32 => _ASSUMPTION_DAY,
    15571_i32 => _EID_AL_FITR,
    15572_i32 => _EID_AL_FITR__OBSERVED_,
    15639_i32 => _EID_AL_ADHA,
    15699_i32 => _CHRISTMAS_DAY,
    15706_i32 => _NEW_YEAR_S_DAY,
    15729_i32 => _MAWLID,
    15747_i32 => _YOUTH_DAY,
    15793_i32 => _GOOD_FRIDAY,
    15826_i32 => _LABOUR_DAY,
    15834_i32 => _ASCENSION_DAY,
    15845_i32 => _NATIONAL_DAY,
    15925_i32 => _EID_AL_FITR,
    15932_i32 => _ASSUMPTION_DAY,
    15993_i32 => _EID_AL_ADHA,
    16064_i32 => _CHRISTMAS_DAY,
    16071_i32 => _NEW_YEAR_S_DAY,
    16084_i32 => _MAWLID,
    16112_i32 => _YOUTH_DAY,
    16178_i32 => _GOOD_FRIDAY,
    16191_i32 => _LABOUR_DAY,
    16210_i32 => _NATIONAL_DAY,
    16219_i32 => _ASCENSION_DAY,
    16279_i32 => _EID_AL_FITR,
    16297_i32 => _ASSUMPTION_DAY,
    16348_i32 => _EID_AL_ADHA,
    16349_i32 => _EID_AL_ADHA__OBSERVED_,
    16429_i32 => _CHRISTMAS_DAY,
    16436_i32 => _NEW_YEAR_S_DAY,
    16438_i32 => _MAWLID,
    16477_i32 => _YOUTH_DAY,
    16528_i32 => _GOOD_FRIDAY,
    16556_i32 => _LABOUR_DAY,
    16569_i32 => _ASCENSION_DAY,
    16575_i32 => _NATIONAL_DAY,
    16634_i32 => _EID_AL_FITR,
    16662_i32 => _ASSUMPTION_DAY,
    16702_i32 => _EID_AL_ADHA,
    16793_i32 => _MAWLID,
    16794_i32 => _CHRISTMAS_DAY,
    16801_i32 => _NEW_YEAR_S_DAY,
    16842_i32 => _YOUTH_DAY,
    16885_i32 => _GOOD_FRIDAY,
    16922_i32 => _LABOUR_DAY,
    16923_i32 => _LABOUR_DAY__OBSERVED_,
    16926_i32 => _ASCENSION_DAY,
    16941_i32 => _NATIONAL_DAY,
    16989_i32 => _EID_AL_FITR,
    17028_i32 => _ASSUMPTION_DAY,
    17057_i32 => _EID_AL_ADHA,
    17147_i32 => _MAWLID,
    17160_i32 => _CHRISTMAS_DAY,
    17161_i32 => _CHRISTMAS_DAY__OBSERVED_,
    17167_i32 => _NEW_YEAR_S_DAY,
    17168_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    17208_i32 => _YOUTH_DAY,
    17270_i32 => _GOOD_FRIDAY,
    17287_i32 => _LABOUR_DAY,
    17306_i32 => _NATIONAL_DAY,
    17311_i32 => _ASCENSION_DAY,
    17343_i32 => _EID_AL_FITR,
    17393_i32 => _ASSUMPTION_DAY,
    17411_i32 => _EID_AL_ADHA,
    17501_i32 => _MAWLID,
    17525_i32 => _CHRISTMAS_DAY,
    17532_i32 => _NEW_YEAR_S_DAY,
    17573_i32 => _YOUTH_DAY,
    17574_i32 => _YOUTH_DAY__OBSERVED_,
    17620_i32 => _GOOD_FRIDAY,
    17652_i32 => _LABOUR_DAY,
    17661_i32 => _ASCENSION_DAY,
    17671_i32 => _NATIONAL_DAY,
    17672_i32 => _NATIONAL_DAY__OBSERVED_,
    17697_i32 => _EID_AL_FITR,
    17758_i32 => _ASSUMPTION_DAY,
    17764_i32 => _EID_AL_ADHA,
    17856_i32 => _MAWLID,
    17890_i32 => _CHRISTMAS_DAY,
    17897_i32 => _NEW_YEAR_S_DAY,
    17938_i32 => _YOUTH_DAY,
    18005_i32 => _GOOD_FRIDAY,
    18017_i32 => _LABOUR_DAY,
    18036_i32 => _NATIONAL_DAY,
    18046_i32 => _ASCENSION_DAY,
    18051_i32 => _EID_AL_FITR,
    18119_i32 => _EID_AL_ADHA,
    18120_i32 => _EID_AL_ADHA__OBSERVED_,
    18123_i32 => _ASSUMPTION_DAY,
    18210_i32 => _MAWLID,
    18211_i32 => _MAWLID__OBSERVED_,
    18255_i32 => _CHRISTMAS_DAY,
    18262_i32 => _NEW_YEAR_S_DAY,
    18303_i32 => _YOUTH_DAY,
    18362_i32 => _GOOD_FRIDAY,
    18383_i32 => _LABOUR_DAY,
    18402_i32 => _NATIONAL_DAY,
    18403_i32 => _ASCENSION_DAY,
    18406_i32 => _EID_AL_FITR,
    18407_i32 => _EID_AL_FITR__OBSERVED_,
    18474_i32 => _EID_AL_ADHA,
    18489_i32 => _ASSUMPTION_DAY,
    18564_i32 => _MAWLID,
    18621_i32 => _CHRISTMAS_DAY,
    18628_i32 => _NEW_YEAR_S_DAY,
    18669_i32 => _YOUTH_DAY,
    18719_i32 => _GOOD_FRIDAY,
    18748_i32 => _LABOUR_DAY,
    18760_i32 => _ASCENSION_DAY__EID_AL_FITR,
    18761_i32 => _PUBLIC_HOLIDAY,
    18767_i32 => _NATIONAL_DAY,
    18827_i32 => _PUBLIC_HOLIDAY,
    18828_i32 => _EID_AL_ADHA,
    18854_i32 => _ASSUMPTION_DAY,
    18855_i32 => _ASSUMPTION_DAY__OBSERVED_,
    18919_i32 => _MAWLID,
    18986_i32 => _CHRISTMAS_DAY,
    18993_i32 => _NEW_YEAR_S_DAY,
    19034_i32 => _YOUTH_DAY,
    19097_i32 => _GOOD_FRIDAY,
    19113_i32 => _LABOUR_DAY,
    19114_i32 => _EID_AL_FITR,
    19115_i32 => _LABOUR_DAY__OBSERVED_,
    19132_i32 => _NATIONAL_DAY,
    19138_i32 => _ASCENSION_DAY,
    19182_i32 => _EID_AL_ADHA,
    19219_i32 => _ASSUMPTION_DAY,
    19273_i32 => _MAWLID,
    19351_i32 => _CHRISTMAS_DAY,
    19352_i32 => _CHRISTMAS_DAY__OBSERVED_,
    19358_i32 => _NEW_YEAR_S_DAY,
    19359_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    19399_i32 => _YOUTH_DAY,
    19454_i32 => _GOOD_FRIDAY,
    19468_i32 => _EID_AL_FITR,
    19478_i32 => _LABOUR_DAY,
    19495_i32 => _ASCENSION_DAY,
    19497_i32 => _NATIONAL_DAY,
    19536_i32 => _EID_AL_ADHA,
    19584_i32 => _ASSUMPTION_DAY,
    19627_i32 => _MAWLID__ESTIMATED_,
    19716_i32 => _CHRISTMAS_DAY,
    19723_i32 => _NEW_YEAR_S_DAY,
    19764_i32 => _YOUTH_DAY,
    19765_i32 => _YOUTH_DAY__OBSERVED_,
    19811_i32 => _GOOD_FRIDAY,
    19823_i32 => _EID_AL_FITR,
    19844_i32 => _LABOUR_DAY,
    19852_i32 => _ASCENSION_DAY,
    19863_i32 => _NATIONAL_DAY,
    19890_i32 => _EID_AL_ADHA__ESTIMATED_,
    19891_i32 => _EID_AL_ADHA__ESTIMATED___OBSERVED_,
    19950_i32 => _ASSUMPTION_DAY,
    19981_i32 => _MAWLID__ESTIMATED_,
    19982_i32 => _MAWLID__ESTIMATED___OBSERVED_,
    20082_i32 => _CHRISTMAS_DAY,
    20089_i32 => _NEW_YEAR_S_DAY,
    20130_i32 => _YOUTH_DAY,
    20177_i32 => _EID_AL_FITR__ESTIMATED_,
    20178_i32 => _EID_AL_FITR__ESTIMATED___OBSERVED_,
    20196_i32 => _GOOD_FRIDAY,
    20209_i32 => _LABOUR_DAY,
    20228_i32 => _NATIONAL_DAY,
    20237_i32 => _ASCENSION_DAY,
    20245_i32 => _EID_AL_ADHA__ESTIMATED_,
    20315_i32 => _ASSUMPTION_DAY,
    20335_i32 => _MAWLID__ESTIMATED_,
    20447_i32 => _CHRISTMAS_DAY,
    20454_i32 => _NEW_YEAR_S_DAY,
    20495_i32 => _YOUTH_DAY,
    20532_i32 => _EID_AL_FITR__ESTIMATED_,
    20546_i32 => _GOOD_FRIDAY,
    20574_i32 => _LABOUR_DAY,
    20587_i32 => _ASCENSION_DAY,
    20593_i32 => _NATIONAL_DAY,
    20600_i32 => _EID_AL_ADHA__ESTIMATED_,
    20680_i32 => _ASSUMPTION_DAY,
    20690_i32 => _MAWLID__ESTIMATED_,
    20812_i32 => _CHRISTMAS_DAY,
    20819_i32 => _NEW_YEAR_S_DAY,
    20860_i32 => _YOUTH_DAY,
    20886_i32 => _EID_AL_FITR__ESTIMATED_,
    20903_i32 => _GOOD_FRIDAY,
    20939_i32 => _LABOUR_DAY,
    20944_i32 => _ASCENSION_DAY,
    20954_i32 => _EID_AL_ADHA__ESTIMATED_,
    20955_i32 => _EID_AL_ADHA__ESTIMATED___OBSERVED_,
    20958_i32 => _NATIONAL_DAY,
    21044_i32 => _MAWLID__ESTIMATED_,
    21045_i32 => _ASSUMPTION_DAY,
    21046_i32 => _ASSUMPTION_DAY__OBSERVED_,
    21177_i32 => _CHRISTMAS_DAY,
    21184_i32 => _NEW_YEAR_S_DAY,
    21225_i32 => _YOUTH_DAY,
    21240_i32 => _EID_AL_FITR__ESTIMATED_,
    21288_i32 => _GOOD_FRIDAY,
    21305_i32 => _LABOUR_DAY,
    21309_i32 => _EID_AL_ADHA__ESTIMATED_,
    21324_i32 => _NATIONAL_DAY,
    21329_i32 => _ASCENSION_DAY,
    21399_i32 => _MAWLID__ESTIMATED_,
    21411_i32 => _ASSUMPTION_DAY,
    21543_i32 => _CHRISTMAS_DAY,
    21550_i32 => _NEW_YEAR_S_DAY,
    21591_i32 => _YOUTH_DAY,
    21592_i32 => _YOUTH_DAY__OBSERVED_,
    21594_i32 => _EID_AL_FITR__ESTIMATED_,
    21638_i32 => _GOOD_FRIDAY,
    21663_i32 => _EID_AL_ADHA__ESTIMATED_,
    21670_i32 => _LABOUR_DAY,
    21679_i32 => _ASCENSION_DAY,
    21689_i32 => _NATIONAL_DAY,
    21690_i32 => _NATIONAL_DAY__OBSERVED_,
    21754_i32 => _MAWLID__ESTIMATED_,
    21776_i32 => _ASSUMPTION_DAY,
    21908_i32 => _CHRISTMAS_DAY,
    21915_i32 => _NEW_YEAR_S_DAY,
    21949_i32 => _EID_AL_FITR__ESTIMATED_,
    21956_i32 => _YOUTH_DAY,
    22017_i32 => _EID_AL_ADHA__ESTIMATED_,
    22023_i32 => _GOOD_FRIDAY,
    22035_i32 => _LABOUR_DAY,
    22054_i32 => _NATIONAL_DAY,
    22064_i32 => _ASCENSION_DAY,
    22108_i32 => _MAWLID__ESTIMATED_,
    22141_i32 => _ASSUMPTION_DAY,
    22273_i32 => _CHRISTMAS_DAY,
    22280_i32 => _NEW_YEAR_S_DAY,
    22303_i32 => _EID_AL_FITR__ESTIMATED_,
    22321_i32 => _YOUTH_DAY,
    22371_i32 => _EID_AL_ADHA__ESTIMATED_,
    22380_i32 => _GOOD_FRIDAY,
    22400_i32 => _LABOUR_DAY,
    22419_i32 => _NATIONAL_DAY,
    22421_i32 => _ASCENSION_DAY,
    22462_i32 => _MAWLID__ESTIMATED_,
    22506_i32 => _ASSUMPTION_DAY,
    22638_i32 => _CHRISTMAS_DAY,
    22645_i32 => _NEW_YEAR_S_DAY,
    22658_i32 => _EID_AL_FITR__ESTIMATED_,
    22686_i32 => _YOUTH_DAY,
    22726_i32 => _EID_AL_ADHA__ESTIMATED_,
    22730_i32 => _GOOD_FRIDAY,
    22766_i32 => _LABOUR_DAY,
    22771_i32 => _ASCENSION_DAY,
    22785_i32 => _NATIONAL_DAY,
    22816_i32 => _MAWLID__ESTIMATED_,
    22817_i32 => _MAWLID__ESTIMATED___OBSERVED_,
    22872_i32 => _ASSUMPTION_DAY,
    22873_i32 => _ASSUMPTION_DAY__OBSERVED_,
    23004_i32 => _CHRISTMAS_DAY,
    23011_i32 => _NEW_YEAR_S_DAY,
    23012_i32 => _EID_AL_FITR__ESTIMATED_,
    23013_i32 => _EID_AL_FITR__ESTIMATED___OBSERVED_,
    23052_i32 => _YOUTH_DAY,
    23080_i32 => _EID_AL_ADHA__ESTIMATED_,
    23115_i32 => _GOOD_FRIDAY,
    23131_i32 => _LABOUR_DAY,
    23132_i32 => _LABOUR_DAY__OBSERVED_,
    23150_i32 => _NATIONAL_DAY,
    23156_i32 => _ASCENSION_DAY,
    23170_i32 => _MAWLID__ESTIMATED_,
    23237_i32 => _ASSUMPTION_DAY,
    23367_i32 => _EID_AL_FITR__ESTIMATED_,
    23369_i32 => _CHRISTMAS_DAY,
    23370_i32 => _CHRISTMAS_DAY__OBSERVED_,
    23376_i32 => _NEW_YEAR_S_DAY,
    23377_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    23417_i32 => _YOUTH_DAY,
    23435_i32 => _EID_AL_ADHA__ESTIMATED_,
    23472_i32 => _GOOD_FRIDAY,
    23496_i32 => _LABOUR_DAY,
    23513_i32 => _ASCENSION_DAY,
    23515_i32 => _NATIONAL_DAY,
    23525_i32 => _MAWLID__ESTIMATED_,
    23602_i32 => _ASSUMPTION_DAY,
    23721_i32 => _EID_AL_FITR__ESTIMATED_,
    23734_i32 => _CHRISTMAS_DAY,
    23741_i32 => _NEW_YEAR_S_DAY,
    23782_i32 => _YOUTH_DAY,
    23783_i32 => _YOUTH_DAY__OBSERVED_,
    23789_i32 => _EID_AL_ADHA__ESTIMATED_,
    23790_i32 => _EID_AL_ADHA__ESTIMATED___OBSERVED_,
    23822_i32 => _GOOD_FRIDAY,
    23861_i32 => _LABOUR_DAY,
    23863_i32 => _ASCENSION_DAY,
    23880_i32 => _MAWLID__ESTIMATED___NATIONAL_DAY,
    23881_i32 => _MAWLID__ESTIMATED___OBSERVED___NATIONAL_DAY__OBSERVED_,
    23967_i32 => _ASSUMPTION_DAY,
    24075_i32 => _EID_AL_FITR__ESTIMATED_,
    24099_i32 => _CHRISTMAS_DAY,
    24106_i32 => _NEW_YEAR_S_DAY,
    24143_i32 => _EID_AL_ADHA__ESTIMATED_,
    24147_i32 => _YOUTH_DAY,
    24207_i32 => _GOOD_FRIDAY,
    24227_i32 => _LABOUR_DAY,
    24234_i32 => _MAWLID__ESTIMATED_,
    24246_i32 => _NATIONAL_DAY,
    24248_i32 => _ASCENSION_DAY,
    24333_i32 => _ASSUMPTION_DAY,
    24429_i32 => _EID_AL_FITR__ESTIMATED_,
    24465_i32 => _CHRISTMAS_DAY,
    24472_i32 => _NEW_YEAR_S_DAY,
    24497_i32 => _EID_AL_ADHA__ESTIMATED_,
    24513_i32 => _YOUTH_DAY,
    24564_i32 => _GOOD_FRIDAY,
    24589_i32 => _MAWLID__ESTIMATED_,
    24592_i32 => _LABOUR_DAY,
    24605_i32 => _ASCENSION_DAY,
    24611_i32 => _NATIONAL_DAY,
    24698_i32 => _ASSUMPTION_DAY,
    24783_i32 => _EID_AL_FITR__ESTIMATED_,
    24784_i32 => _EID_AL_FITR__ESTIMATED___OBSERVED_,
    24830_i32 => _CHRISTMAS_DAY,
    24837_i32 => _NEW_YEAR_S_DAY,
    24852_i32 => _EID_AL_ADHA__ESTIMATED_,
    24878_i32 => _YOUTH_DAY,
    24943_i32 => _MAWLID__ESTIMATED_,
    24949_i32 => _GOOD_FRIDAY,
    24957_i32 => _LABOUR_DAY,
    24976_i32 => _NATIONAL_DAY,
    24990_i32 => _ASCENSION_DAY,
    25063_i32 => _ASSUMPTION_DAY,
    25064_i32 => _ASSUMPTION_DAY__OBSERVED_,
    25138_i32 => _EID_AL_FITR__ESTIMATED_,
    25195_i32 => _CHRISTMAS_DAY,
    25202_i32 => _NEW_YEAR_S_DAY,
    25206_i32 => _EID_AL_ADHA__ESTIMATED_,
    25243_i32 => _YOUTH_DAY,
    25297_i32 => _MAWLID__ESTIMATED_,
    25299_i32 => _GOOD_FRIDAY,
    25322_i32 => _LABOUR_DAY,
    25323_i32 => _LABOUR_DAY__OBSERVED_,
    25340_i32 => _ASCENSION_DAY,
    25341_i32 => _NATIONAL_DAY,
    25428_i32 => _ASSUMPTION_DAY,
    25493_i32 => _EID_AL_FITR__ESTIMATED_,
    25560_i32 => _CHRISTMAS_DAY,
    25561_i32 => _EID_AL_ADHA__ESTIMATED_,
    25562_i32 => _CHRISTMAS_DAY__OBSERVED_,
    25567_i32 => _NEW_YEAR_S_DAY,
    25568_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    25608_i32 => _YOUTH_DAY,
    25651_i32 => _MAWLID__ESTIMATED_,
    25652_i32 => _MAWLID__ESTIMATED___OBSERVED_,
    25656_i32 => _GOOD_FRIDAY,
    25688_i32 => _LABOUR_DAY,
    25697_i32 => _ASCENSION_DAY,
    25707_i32 => _NATIONAL_DAY,
    25708_i32 => _NATIONAL_DAY__OBSERVED_,
    25794_i32 => _ASSUMPTION_DAY,
    25847_i32 => _EID_AL_FITR__ESTIMATED_,
    25848_i32 => _EID_AL_FITR__ESTIMATED___OBSERVED_,
    25915_i32 => _EID_AL_ADHA__ESTIMATED_,
    25926_i32 => _CHRISTMAS_DAY,
    25933_i32 => _NEW_YEAR_S_DAY,
    25974_i32 => _YOUTH_DAY,
    26006_i32 => _MAWLID__ESTIMATED_,
    26041_i32 => _GOOD_FRIDAY,
    26053_i32 => _LABOUR_DAY,
    26072_i32 => _NATIONAL_DAY,
    26082_i32 => _ASCENSION_DAY,
    26159_i32 => _ASSUMPTION_DAY,
    26201_i32 => _EID_AL_FITR__ESTIMATED_,
    26270_i32 => _EID_AL_ADHA__ESTIMATED_,
    26291_i32 => _CHRISTMAS_DAY,
    26298_i32 => _NEW_YEAR_S_DAY,
    26339_i32 => _YOUTH_DAY,
    26360_i32 => _MAWLID__ESTIMATED_,
    26391_i32 => _GOOD_FRIDAY,
    26418_i32 => _LABOUR_DAY,
    26432_i32 => _ASCENSION_DAY,
    26437_i32 => _NATIONAL_DAY,
    26524_i32 => _ASSUMPTION_DAY,
    26555_i32 => _EID_AL_FITR__ESTIMATED_,
    26624_i32 => _EID_AL_ADHA__ESTIMATED_,
    26625_i32 => _EID_AL_ADHA__ESTIMATED___OBSERVED_,
    26656_i32 => _CHRISTMAS_DAY,
    26663_i32 => _NEW_YEAR_S_DAY,
    26704_i32 => _YOUTH_DAY,
    26715_i32 => _MAWLID__ESTIMATED_,
    26716_i32 => _MAWLID__ESTIMATED___OBSERVED_,
    26748_i32 => _GOOD_FRIDAY,
    26783_i32 => _LABOUR_DAY,
    26789_i32 => _ASCENSION_DAY,
    26802_i32 => _NATIONAL_DAY,
    26889_i32 => _ASSUMPTION_DAY,
    26909_i32 => _EID_AL_FITR__ESTIMATED_,
    26978_i32 => _EID_AL_ADHA__ESTIMATED_,
    27021_i32 => _CHRISTMAS_DAY,
    27028_i32 => _NEW_YEAR_S_DAY,
    27069_i32 => _MAWLID__ESTIMATED___YOUTH_DAY,
    27133_i32 => _GOOD_FRIDAY,
    27149_i32 => _LABOUR_DAY,
    27150_i32 => _LABOUR_DAY__OBSERVED_,
    27168_i32 => _NATIONAL_DAY,
    27174_i32 => _ASCENSION_DAY,
    27255_i32 => _ASSUMPTION_DAY,
    27264_i32 => _EID_AL_FITR__ESTIMATED_,
    27332_i32 => _EID_AL_ADHA__ESTIMATED_,
    27387_i32 => _CHRISTMAS_DAY,
    27388_i32 => _CHRISTMAS_DAY__OBSERVED_,
    27394_i32 => _NEW_YEAR_S_DAY,
    27395_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    27423_i32 => _MAWLID__ESTIMATED_,
    27435_i32 => _YOUTH_DAY,
    27490_i32 => _GOOD_FRIDAY,
    27514_i32 => _LABOUR_DAY,
    27531_i32 => _ASCENSION_DAY,
    27533_i32 => _NATIONAL_DAY,
    27619_i32 => _EID_AL_FITR__ESTIMATED_,
    27620_i32 => _ASSUMPTION_DAY,
    27687_i32 => _EID_AL_ADHA__ESTIMATED_,
    27752_i32 => _CHRISTMAS_DAY,
    27759_i32 => _NEW_YEAR_S_DAY,
    27777_i32 => _MAWLID__ESTIMATED_,
    27800_i32 => _YOUTH_DAY,
    27801_i32 => _YOUTH_DAY__OBSERVED_,
    27840_i32 => _GOOD_FRIDAY,
    27879_i32 => _LABOUR_DAY,
    27881_i32 => _ASCENSION_DAY,
    27898_i32 => _NATIONAL_DAY,
    27899_i32 => _NATIONAL_DAY__OBSERVED_,
    27973_i32 => _EID_AL_FITR__ESTIMATED_,
    27985_i32 => _ASSUMPTION_DAY,
    28041_i32 => _EID_AL_ADHA__ESTIMATED_,
    28117_i32 => _CHRISTMAS_DAY,
    28124_i32 => _NEW_YEAR_S_DAY,
    28131_i32 => _MAWLID__ESTIMATED_,
    28165_i32 => _YOUTH_DAY,
    28225_i32 => _GOOD_FRIDAY,
    28244_i32 => _LABOUR_DAY,
    28263_i32 => _NATIONAL_DAY,
    28266_i32 => _ASCENSION_DAY,
    28328_i32 => _EID_AL_FITR__ESTIMATED_,
    28350_i32 => _ASSUMPTION_DAY,
    28396_i32 => _EID_AL_ADHA__ESTIMATED_,
    28482_i32 => _CHRISTMAS_DAY,
    28486_i32 => _MAWLID__ESTIMATED_,
    28487_i32 => _MAWLID__ESTIMATED___OBSERVED_,
    28489_i32 => _NEW_YEAR_S_DAY,
    28530_i32 => _YOUTH_DAY,
    28582_i32 => _GOOD_FRIDAY,
    28610_i32 => _LABOUR_DAY,
    28623_i32 => _ASCENSION_DAY,
    28629_i32 => _NATIONAL_DAY,
    28682_i32 => _EID_AL_FITR__ESTIMATED_,
    28683_i32 => _EID_AL_FITR__ESTIMATED___OBSERVED_,
    28716_i32 => _ASSUMPTION_DAY,
    28751_i32 => _EID_AL_ADHA__ESTIMATED_,
    28841_i32 => _MAWLID__ESTIMATED_,
    28848_i32 => _CHRISTMAS_DAY,
    28855_i32 => _NEW_YEAR_S_DAY,
    28896_i32 => _YOUTH_DAY,
    28960_i32 => _GOOD_FRIDAY,
    28975_i32 => _LABOUR_DAY,
    28994_i32 => _NATIONAL_DAY,
    29001_i32 => _ASCENSION_DAY,
    29036_i32 => _EID_AL_FITR__ESTIMATED_,
    29081_i32 => _ASSUMPTION_DAY,
    29082_i32 => _ASSUMPTION_DAY__OBSERVED_,
    29105_i32 => _EID_AL_ADHA__ESTIMATED_,
    29195_i32 => _MAWLID__ESTIMATED_,
    29213_i32 => _CHRISTMAS_DAY,
    29220_i32 => _NEW_YEAR_S_DAY,
};
