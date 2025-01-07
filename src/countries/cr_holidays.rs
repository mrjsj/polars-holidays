use crate::countries::constants::*;
use phf::phf_map;

pub static CR_HOLIDAYS: phf::Map<i32, &'static str> = phf_map! {
    10957_i32 => _NEW_YEAR_S_DAY,
    11058_i32 => _JUAN_SANTAMAR_A_DAY,
    11067_i32 => _MAUNDY_THURSDAY,
    11068_i32 => _GOOD_FRIDAY,
    11078_i32 => _INTERNATIONAL_LABOR_DAY,
    11163_i32 => _ANNEXATION_OF_THE_PARTY_OF_NICOYA_TO_COSTA_RICA,
    11184_i32 => _MOTHER_S_DAY,
    11215_i32 => _INDEPENDENCE_DAY,
    11246_i32 => _CULTURES_DAY__OBSERVED_,
    11316_i32 => _CHRISTMAS_DAY,
    11323_i32 => _NEW_YEAR_S_DAY,
    11423_i32 => _JUAN_SANTAMAR_A_DAY,
    11424_i32 => _MAUNDY_THURSDAY,
    11425_i32 => _GOOD_FRIDAY,
    11443_i32 => _INTERNATIONAL_LABOR_DAY,
    11528_i32 => _ANNEXATION_OF_THE_PARTY_OF_NICOYA_TO_COSTA_RICA,
    11549_i32 => _MOTHER_S_DAY,
    11580_i32 => _INDEPENDENCE_DAY,
    11610_i32 => _CULTURES_DAY__OBSERVED_,
    11681_i32 => _CHRISTMAS_DAY,
    11688_i32 => _NEW_YEAR_S_DAY,
    11774_i32 => _MAUNDY_THURSDAY,
    11775_i32 => _GOOD_FRIDAY,
    11788_i32 => _JUAN_SANTAMAR_A_DAY,
    11808_i32 => _INTERNATIONAL_LABOR_DAY,
    11893_i32 => _ANNEXATION_OF_THE_PARTY_OF_NICOYA_TO_COSTA_RICA,
    11914_i32 => _MOTHER_S_DAY,
    11945_i32 => _INDEPENDENCE_DAY,
    11972_i32 => _CULTURES_DAY,
    12046_i32 => _CHRISTMAS_DAY,
    12053_i32 => _NEW_YEAR_S_DAY,
    12153_i32 => _JUAN_SANTAMAR_A_DAY,
    12159_i32 => _MAUNDY_THURSDAY,
    12160_i32 => _GOOD_FRIDAY,
    12173_i32 => _INTERNATIONAL_LABOR_DAY,
    12258_i32 => _ANNEXATION_OF_THE_PARTY_OF_NICOYA_TO_COSTA_RICA,
    12279_i32 => _MOTHER_S_DAY,
    12310_i32 => _INDEPENDENCE_DAY,
    12337_i32 => _CULTURES_DAY,
    12411_i32 => _CHRISTMAS_DAY,
    12418_i32 => _NEW_YEAR_S_DAY,
    12516_i32 => _MAUNDY_THURSDAY,
    12517_i32 => _GOOD_FRIDAY,
    12519_i32 => _JUAN_SANTAMAR_A_DAY,
    12539_i32 => _INTERNATIONAL_LABOR_DAY,
    12624_i32 => _ANNEXATION_OF_THE_PARTY_OF_NICOYA_TO_COSTA_RICA,
    12645_i32 => _MOTHER_S_DAY,
    12676_i32 => _INDEPENDENCE_DAY,
    12709_i32 => _CULTURES_DAY__OBSERVED_,
    12777_i32 => _CHRISTMAS_DAY,
    12784_i32 => _NEW_YEAR_S_DAY,
    12866_i32 => _MAUNDY_THURSDAY,
    12867_i32 => _GOOD_FRIDAY,
    12884_i32 => _JUAN_SANTAMAR_A_DAY,
    12904_i32 => _INTERNATIONAL_LABOR_DAY,
    12989_i32 => _ANNEXATION_OF_THE_PARTY_OF_NICOYA_TO_COSTA_RICA,
    13010_i32 => _MOTHER_S_DAY,
    13041_i32 => _INDEPENDENCE_DAY,
    13073_i32 => _CULTURES_DAY__OBSERVED_,
    13142_i32 => _CHRISTMAS_DAY,
    13149_i32 => _NEW_YEAR_S_DAY,
    13251_i32 => _MAUNDY_THURSDAY,
    13252_i32 => _GOOD_FRIDAY,
    13255_i32 => _JUAN_SANTAMAR_A_DAY__OBSERVED_,
    13269_i32 => _INTERNATIONAL_LABOR_DAY,
    13360_i32 => _ANNEXATION_OF_THE_PARTY_OF_NICOYA_TO_COSTA_RICA__OBSERVED_,
    13381_i32 => _MOTHER_S_DAY__OBSERVED_,
    13406_i32 => _INDEPENDENCE_DAY,
    13437_i32 => _CULTURES_DAY__OBSERVED_,
    13507_i32 => _CHRISTMAS_DAY,
    13514_i32 => _NEW_YEAR_S_DAY,
    13608_i32 => _MAUNDY_THURSDAY,
    13609_i32 => _GOOD_FRIDAY,
    13619_i32 => _JUAN_SANTAMAR_A_DAY__OBSERVED_,
    13634_i32 => _INTERNATIONAL_LABOR_DAY,
    13724_i32 => _ANNEXATION_OF_THE_PARTY_OF_NICOYA_TO_COSTA_RICA__OBSERVED_,
    13745_i32 => _MOTHER_S_DAY__OBSERVED_,
    13771_i32 => _INDEPENDENCE_DAY,
    13801_i32 => _CULTURES_DAY__OBSERVED_,
    13872_i32 => _CHRISTMAS_DAY,
    13879_i32 => _NEW_YEAR_S_DAY,
    13958_i32 => _MAUNDY_THURSDAY,
    13959_i32 => _GOOD_FRIDAY,
    13983_i32 => _JUAN_SANTAMAR_A_DAY__OBSERVED_,
    14000_i32 => _INTERNATIONAL_LABOR_DAY,
    14088_i32 => _ANNEXATION_OF_THE_PARTY_OF_NICOYA_TO_COSTA_RICA__OBSERVED_,
    14106_i32 => _MOTHER_S_DAY,
    14137_i32 => _INDEPENDENCE_DAY,
    14164_i32 => _CULTURES_DAY,
    14238_i32 => _CHRISTMAS_DAY,
    14245_i32 => _NEW_YEAR_S_DAY,
    14343_i32 => _MAUNDY_THURSDAY,
    14344_i32 => _GOOD_FRIDAY,
    14345_i32 => _JUAN_SANTAMAR_A_DAY,
    14365_i32 => _INTERNATIONAL_LABOR_DAY,
    14450_i32 => _ANNEXATION_OF_THE_PARTY_OF_NICOYA_TO_COSTA_RICA,
    14471_i32 => _MOTHER_S_DAY,
    14502_i32 => _INDEPENDENCE_DAY,
    14529_i32 => _CULTURES_DAY,
    14603_i32 => _CHRISTMAS_DAY,
    14610_i32 => _NEW_YEAR_S_DAY,
    14700_i32 => _MAUNDY_THURSDAY,
    14701_i32 => _GOOD_FRIDAY,
    14710_i32 => _JUAN_SANTAMAR_A_DAY,
    14730_i32 => _INTERNATIONAL_LABOR_DAY,
    14815_i32 => _ANNEXATION_OF_THE_PARTY_OF_NICOYA_TO_COSTA_RICA,
    14836_i32 => _MOTHER_S_DAY,
    14867_i32 => _INDEPENDENCE_DAY,
    14900_i32 => _CULTURES_DAY__OBSERVED_,
    14968_i32 => _CHRISTMAS_DAY,
    14975_i32 => _NEW_YEAR_S_DAY,
    15075_i32 => _JUAN_SANTAMAR_A_DAY,
    15085_i32 => _MAUNDY_THURSDAY,
    15086_i32 => _GOOD_FRIDAY,
    15095_i32 => _INTERNATIONAL_LABOR_DAY,
    15180_i32 => _ANNEXATION_OF_THE_PARTY_OF_NICOYA_TO_COSTA_RICA,
    15201_i32 => _MOTHER_S_DAY,
    15232_i32 => _INDEPENDENCE_DAY,
    15264_i32 => _CULTURES_DAY__OBSERVED_,
    15333_i32 => _CHRISTMAS_DAY,
    15340_i32 => _NEW_YEAR_S_DAY,
    15435_i32 => _MAUNDY_THURSDAY,
    15436_i32 => _GOOD_FRIDAY,
    15441_i32 => _JUAN_SANTAMAR_A_DAY,
    15461_i32 => _INTERNATIONAL_LABOR_DAY,
    15546_i32 => _ANNEXATION_OF_THE_PARTY_OF_NICOYA_TO_COSTA_RICA,
    15567_i32 => _MOTHER_S_DAY,
    15598_i32 => _INDEPENDENCE_DAY,
    15628_i32 => _CULTURES_DAY__OBSERVED_,
    15699_i32 => _CHRISTMAS_DAY,
    15706_i32 => _NEW_YEAR_S_DAY,
    15792_i32 => _MAUNDY_THURSDAY,
    15793_i32 => _GOOD_FRIDAY,
    15806_i32 => _JUAN_SANTAMAR_A_DAY,
    15826_i32 => _INTERNATIONAL_LABOR_DAY,
    15911_i32 => _ANNEXATION_OF_THE_PARTY_OF_NICOYA_TO_COSTA_RICA,
    15932_i32 => _MOTHER_S_DAY,
    15963_i32 => _INDEPENDENCE_DAY,
    15990_i32 => _CULTURES_DAY,
    16064_i32 => _CHRISTMAS_DAY,
    16071_i32 => _NEW_YEAR_S_DAY,
    16171_i32 => _JUAN_SANTAMAR_A_DAY,
    16177_i32 => _MAUNDY_THURSDAY,
    16178_i32 => _GOOD_FRIDAY,
    16191_i32 => _INTERNATIONAL_LABOR_DAY,
    16276_i32 => _ANNEXATION_OF_THE_PARTY_OF_NICOYA_TO_COSTA_RICA,
    16297_i32 => _MOTHER_S_DAY,
    16328_i32 => _INDEPENDENCE_DAY,
    16355_i32 => _CULTURES_DAY,
    16429_i32 => _CHRISTMAS_DAY,
    16436_i32 => _NEW_YEAR_S_DAY,
    16527_i32 => _MAUNDY_THURSDAY,
    16528_i32 => _GOOD_FRIDAY,
    16536_i32 => _JUAN_SANTAMAR_A_DAY,
    16556_i32 => _INTERNATIONAL_LABOR_DAY,
    16641_i32 => _ANNEXATION_OF_THE_PARTY_OF_NICOYA_TO_COSTA_RICA,
    16662_i32 => _MOTHER_S_DAY,
    16693_i32 => _INDEPENDENCE_DAY,
    16720_i32 => _CULTURES_DAY,
    16794_i32 => _CHRISTMAS_DAY,
    16801_i32 => _NEW_YEAR_S_DAY,
    16884_i32 => _MAUNDY_THURSDAY,
    16885_i32 => _GOOD_FRIDAY,
    16902_i32 => _JUAN_SANTAMAR_A_DAY,
    16922_i32 => _INTERNATIONAL_LABOR_DAY,
    17007_i32 => _ANNEXATION_OF_THE_PARTY_OF_NICOYA_TO_COSTA_RICA,
    17028_i32 => _MOTHER_S_DAY,
    17059_i32 => _INDEPENDENCE_DAY,
    17091_i32 => _CULTURES_DAY__OBSERVED_,
    17160_i32 => _CHRISTMAS_DAY,
    17167_i32 => _NEW_YEAR_S_DAY,
    17267_i32 => _JUAN_SANTAMAR_A_DAY,
    17269_i32 => _MAUNDY_THURSDAY,
    17270_i32 => _GOOD_FRIDAY,
    17287_i32 => _INTERNATIONAL_LABOR_DAY,
    17372_i32 => _ANNEXATION_OF_THE_PARTY_OF_NICOYA_TO_COSTA_RICA,
    17393_i32 => _MOTHER_S_DAY,
    17424_i32 => _INDEPENDENCE_DAY,
    17455_i32 => _CULTURES_DAY__OBSERVED_,
    17525_i32 => _CHRISTMAS_DAY,
    17532_i32 => _NEW_YEAR_S_DAY,
    17619_i32 => _MAUNDY_THURSDAY,
    17620_i32 => _GOOD_FRIDAY,
    17632_i32 => _JUAN_SANTAMAR_A_DAY,
    17652_i32 => _INTERNATIONAL_LABOR_DAY,
    17737_i32 => _ANNEXATION_OF_THE_PARTY_OF_NICOYA_TO_COSTA_RICA,
    17758_i32 => _MOTHER_S_DAY,
    17789_i32 => _INDEPENDENCE_DAY,
    17819_i32 => _CULTURES_DAY__OBSERVED_,
    17890_i32 => _CHRISTMAS_DAY,
    17897_i32 => _NEW_YEAR_S_DAY,
    17997_i32 => _JUAN_SANTAMAR_A_DAY,
    18004_i32 => _MAUNDY_THURSDAY,
    18005_i32 => _GOOD_FRIDAY,
    18017_i32 => _INTERNATIONAL_LABOR_DAY,
    18102_i32 => _ANNEXATION_OF_THE_PARTY_OF_NICOYA_TO_COSTA_RICA,
    18123_i32 => _MOTHER_S_DAY,
    18154_i32 => _INDEPENDENCE_DAY,
    18181_i32 => _CULTURES_DAY,
    18255_i32 => _CHRISTMAS_DAY,
    18262_i32 => _NEW_YEAR_S_DAY,
    18361_i32 => _MAUNDY_THURSDAY,
    18362_i32 => _GOOD_FRIDAY,
    18363_i32 => _JUAN_SANTAMAR_A_DAY,
    18383_i32 => _INTERNATIONAL_LABOR_DAY,
    18470_i32 => _ANNEXATION_OF_THE_PARTY_OF_NICOYA_TO_COSTA_RICA__OBSERVED_,
    18491_i32 => _MOTHER_S_DAY__OBSERVED_,
    18519_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    18621_i32 => _CHRISTMAS_DAY,
    18628_i32 => _NEW_YEAR_S_DAY,
    18718_i32 => _MAUNDY_THURSDAY,
    18719_i32 => _GOOD_FRIDAY,
    18728_i32 => _JUAN_SANTAMAR_A_DAY,
    18750_i32 => _INTERNATIONAL_LABOR_DAY__OBSERVED_,
    18834_i32 => _ANNEXATION_OF_THE_PARTY_OF_NICOYA_TO_COSTA_RICA__OBSERVED_,
    18854_i32 => _MOTHER_S_DAY,
    18883_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    18986_i32 => _CHRISTMAS_DAY,
    18993_i32 => _NEW_YEAR_S_DAY,
    19093_i32 => _JUAN_SANTAMAR_A_DAY,
    19096_i32 => _MAUNDY_THURSDAY,
    19097_i32 => _GOOD_FRIDAY,
    19113_i32 => _INTERNATIONAL_LABOR_DAY,
    19198_i32 => _ANNEXATION_OF_THE_PARTY_OF_NICOYA_TO_COSTA_RICA,
    19219_i32 => _MOTHER_S_DAY,
    19254_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    19351_i32 => _CHRISTMAS_DAY,
    19358_i32 => _NEW_YEAR_S_DAY,
    19453_i32 => _MAUNDY_THURSDAY,
    19454_i32 => _GOOD_FRIDAY,
    19457_i32 => _JUAN_SANTAMAR_A_DAY__OBSERVED_,
    19478_i32 => _INTERNATIONAL_LABOR_DAY,
    19562_i32 => _ANNEXATION_OF_THE_PARTY_OF_NICOYA_TO_COSTA_RICA__OBSERVED_,
    19583_i32 => _MOTHER_S_DAY__OBSERVED_,
    19615_i32 => _INDEPENDENCE_DAY,
    19716_i32 => _CHRISTMAS_DAY,
    19723_i32 => _NEW_YEAR_S_DAY,
    19810_i32 => _MAUNDY_THURSDAY,
    19811_i32 => _GOOD_FRIDAY,
    19828_i32 => _JUAN_SANTAMAR_A_DAY__OBSERVED_,
    19844_i32 => _INTERNATIONAL_LABOR_DAY,
    19933_i32 => _ANNEXATION_OF_THE_PARTY_OF_NICOYA_TO_COSTA_RICA__OBSERVED_,
    19954_i32 => _MOTHER_S_DAY__OBSERVED_,
    19981_i32 => _INDEPENDENCE_DAY,
    20082_i32 => _CHRISTMAS_DAY,
    20089_i32 => _NEW_YEAR_S_DAY,
    20189_i32 => _JUAN_SANTAMAR_A_DAY,
    20195_i32 => _MAUNDY_THURSDAY,
    20196_i32 => _GOOD_FRIDAY,
    20209_i32 => _INTERNATIONAL_LABOR_DAY,
    20294_i32 => _ANNEXATION_OF_THE_PARTY_OF_NICOYA_TO_COSTA_RICA,
    20315_i32 => _MOTHER_S_DAY,
    20346_i32 => _INDEPENDENCE_DAY,
    20447_i32 => _CHRISTMAS_DAY,
    20454_i32 => _NEW_YEAR_S_DAY,
    20545_i32 => _MAUNDY_THURSDAY,
    20546_i32 => _GOOD_FRIDAY,
    20554_i32 => _JUAN_SANTAMAR_A_DAY,
    20574_i32 => _INTERNATIONAL_LABOR_DAY,
    20659_i32 => _ANNEXATION_OF_THE_PARTY_OF_NICOYA_TO_COSTA_RICA,
    20680_i32 => _MOTHER_S_DAY,
    20711_i32 => _INDEPENDENCE_DAY,
    20812_i32 => _CHRISTMAS_DAY,
    20819_i32 => _NEW_YEAR_S_DAY,
    20902_i32 => _MAUNDY_THURSDAY,
    20903_i32 => _GOOD_FRIDAY,
    20919_i32 => _JUAN_SANTAMAR_A_DAY,
    20939_i32 => _INTERNATIONAL_LABOR_DAY,
    21024_i32 => _ANNEXATION_OF_THE_PARTY_OF_NICOYA_TO_COSTA_RICA,
    21045_i32 => _MOTHER_S_DAY,
    21076_i32 => _INDEPENDENCE_DAY,
    21177_i32 => _CHRISTMAS_DAY,
    21184_i32 => _NEW_YEAR_S_DAY,
    21285_i32 => _JUAN_SANTAMAR_A_DAY,
    21287_i32 => _MAUNDY_THURSDAY,
    21288_i32 => _GOOD_FRIDAY,
    21305_i32 => _INTERNATIONAL_LABOR_DAY,
    21390_i32 => _ANNEXATION_OF_THE_PARTY_OF_NICOYA_TO_COSTA_RICA,
    21411_i32 => _MOTHER_S_DAY,
    21442_i32 => _INDEPENDENCE_DAY,
    21543_i32 => _CHRISTMAS_DAY,
    21550_i32 => _NEW_YEAR_S_DAY,
    21637_i32 => _MAUNDY_THURSDAY,
    21638_i32 => _GOOD_FRIDAY,
    21650_i32 => _JUAN_SANTAMAR_A_DAY,
    21670_i32 => _INTERNATIONAL_LABOR_DAY,
    21755_i32 => _ANNEXATION_OF_THE_PARTY_OF_NICOYA_TO_COSTA_RICA,
    21776_i32 => _MOTHER_S_DAY,
    21807_i32 => _INDEPENDENCE_DAY,
    21908_i32 => _CHRISTMAS_DAY,
    21915_i32 => _NEW_YEAR_S_DAY,
    22015_i32 => _JUAN_SANTAMAR_A_DAY,
    22022_i32 => _MAUNDY_THURSDAY,
    22023_i32 => _GOOD_FRIDAY,
    22035_i32 => _INTERNATIONAL_LABOR_DAY,
    22120_i32 => _ANNEXATION_OF_THE_PARTY_OF_NICOYA_TO_COSTA_RICA,
    22141_i32 => _MOTHER_S_DAY,
    22172_i32 => _INDEPENDENCE_DAY,
    22273_i32 => _CHRISTMAS_DAY,
    22280_i32 => _NEW_YEAR_S_DAY,
    22379_i32 => _MAUNDY_THURSDAY,
    22380_i32 => _GOOD_FRIDAY__JUAN_SANTAMAR_A_DAY,
    22400_i32 => _INTERNATIONAL_LABOR_DAY,
    22485_i32 => _ANNEXATION_OF_THE_PARTY_OF_NICOYA_TO_COSTA_RICA,
    22506_i32 => _MOTHER_S_DAY,
    22537_i32 => _INDEPENDENCE_DAY,
    22638_i32 => _CHRISTMAS_DAY,
    22645_i32 => _NEW_YEAR_S_DAY,
    22729_i32 => _MAUNDY_THURSDAY,
    22730_i32 => _GOOD_FRIDAY,
    22746_i32 => _JUAN_SANTAMAR_A_DAY,
    22766_i32 => _INTERNATIONAL_LABOR_DAY,
    22851_i32 => _ANNEXATION_OF_THE_PARTY_OF_NICOYA_TO_COSTA_RICA,
    22872_i32 => _MOTHER_S_DAY,
    22903_i32 => _INDEPENDENCE_DAY,
    23004_i32 => _CHRISTMAS_DAY,
    23011_i32 => _NEW_YEAR_S_DAY,
    23111_i32 => _JUAN_SANTAMAR_A_DAY,
    23114_i32 => _MAUNDY_THURSDAY,
    23115_i32 => _GOOD_FRIDAY,
    23131_i32 => _INTERNATIONAL_LABOR_DAY,
    23216_i32 => _ANNEXATION_OF_THE_PARTY_OF_NICOYA_TO_COSTA_RICA,
    23237_i32 => _MOTHER_S_DAY,
    23268_i32 => _INDEPENDENCE_DAY,
    23369_i32 => _CHRISTMAS_DAY,
    23376_i32 => _NEW_YEAR_S_DAY,
    23471_i32 => _MAUNDY_THURSDAY,
    23472_i32 => _GOOD_FRIDAY,
    23476_i32 => _JUAN_SANTAMAR_A_DAY,
    23496_i32 => _INTERNATIONAL_LABOR_DAY,
    23581_i32 => _ANNEXATION_OF_THE_PARTY_OF_NICOYA_TO_COSTA_RICA,
    23602_i32 => _MOTHER_S_DAY,
    23633_i32 => _INDEPENDENCE_DAY,
    23734_i32 => _CHRISTMAS_DAY,
    23741_i32 => _NEW_YEAR_S_DAY,
    23821_i32 => _MAUNDY_THURSDAY,
    23822_i32 => _GOOD_FRIDAY,
    23841_i32 => _JUAN_SANTAMAR_A_DAY,
    23861_i32 => _INTERNATIONAL_LABOR_DAY,
    23946_i32 => _ANNEXATION_OF_THE_PARTY_OF_NICOYA_TO_COSTA_RICA,
    23967_i32 => _MOTHER_S_DAY,
    23998_i32 => _INDEPENDENCE_DAY,
    24099_i32 => _CHRISTMAS_DAY,
    24106_i32 => _NEW_YEAR_S_DAY,
    24206_i32 => _MAUNDY_THURSDAY,
    24207_i32 => _GOOD_FRIDAY__JUAN_SANTAMAR_A_DAY,
    24227_i32 => _INTERNATIONAL_LABOR_DAY,
    24312_i32 => _ANNEXATION_OF_THE_PARTY_OF_NICOYA_TO_COSTA_RICA,
    24333_i32 => _MOTHER_S_DAY,
    24364_i32 => _INDEPENDENCE_DAY,
    24465_i32 => _CHRISTMAS_DAY,
    24472_i32 => _NEW_YEAR_S_DAY,
    24563_i32 => _MAUNDY_THURSDAY,
    24564_i32 => _GOOD_FRIDAY,
    24572_i32 => _JUAN_SANTAMAR_A_DAY,
    24592_i32 => _INTERNATIONAL_LABOR_DAY,
    24677_i32 => _ANNEXATION_OF_THE_PARTY_OF_NICOYA_TO_COSTA_RICA,
    24698_i32 => _MOTHER_S_DAY,
    24729_i32 => _INDEPENDENCE_DAY,
    24830_i32 => _CHRISTMAS_DAY,
    24837_i32 => _NEW_YEAR_S_DAY,
    24937_i32 => _JUAN_SANTAMAR_A_DAY,
    24948_i32 => _MAUNDY_THURSDAY,
    24949_i32 => _GOOD_FRIDAY,
    24957_i32 => _INTERNATIONAL_LABOR_DAY,
    25042_i32 => _ANNEXATION_OF_THE_PARTY_OF_NICOYA_TO_COSTA_RICA,
    25063_i32 => _MOTHER_S_DAY,
    25094_i32 => _INDEPENDENCE_DAY,
    25195_i32 => _CHRISTMAS_DAY,
    25202_i32 => _NEW_YEAR_S_DAY,
    25298_i32 => _MAUNDY_THURSDAY,
    25299_i32 => _GOOD_FRIDAY,
    25302_i32 => _JUAN_SANTAMAR_A_DAY,
    25322_i32 => _INTERNATIONAL_LABOR_DAY,
    25407_i32 => _ANNEXATION_OF_THE_PARTY_OF_NICOYA_TO_COSTA_RICA,
    25428_i32 => _MOTHER_S_DAY,
    25459_i32 => _INDEPENDENCE_DAY,
    25560_i32 => _CHRISTMAS_DAY,
    25567_i32 => _NEW_YEAR_S_DAY,
    25655_i32 => _MAUNDY_THURSDAY,
    25656_i32 => _GOOD_FRIDAY,
    25668_i32 => _JUAN_SANTAMAR_A_DAY,
    25688_i32 => _INTERNATIONAL_LABOR_DAY,
    25773_i32 => _ANNEXATION_OF_THE_PARTY_OF_NICOYA_TO_COSTA_RICA,
    25794_i32 => _MOTHER_S_DAY,
    25825_i32 => _INDEPENDENCE_DAY,
    25926_i32 => _CHRISTMAS_DAY,
    25933_i32 => _NEW_YEAR_S_DAY,
    26033_i32 => _JUAN_SANTAMAR_A_DAY,
    26040_i32 => _MAUNDY_THURSDAY,
    26041_i32 => _GOOD_FRIDAY,
    26053_i32 => _INTERNATIONAL_LABOR_DAY,
    26138_i32 => _ANNEXATION_OF_THE_PARTY_OF_NICOYA_TO_COSTA_RICA,
    26159_i32 => _MOTHER_S_DAY,
    26190_i32 => _INDEPENDENCE_DAY,
    26291_i32 => _CHRISTMAS_DAY,
    26298_i32 => _NEW_YEAR_S_DAY,
    26390_i32 => _MAUNDY_THURSDAY,
    26391_i32 => _GOOD_FRIDAY,
    26398_i32 => _JUAN_SANTAMAR_A_DAY,
    26418_i32 => _INTERNATIONAL_LABOR_DAY,
    26503_i32 => _ANNEXATION_OF_THE_PARTY_OF_NICOYA_TO_COSTA_RICA,
    26524_i32 => _MOTHER_S_DAY,
    26555_i32 => _INDEPENDENCE_DAY,
    26656_i32 => _CHRISTMAS_DAY,
    26663_i32 => _NEW_YEAR_S_DAY,
    26747_i32 => _MAUNDY_THURSDAY,
    26748_i32 => _GOOD_FRIDAY,
    26763_i32 => _JUAN_SANTAMAR_A_DAY,
    26783_i32 => _INTERNATIONAL_LABOR_DAY,
    26868_i32 => _ANNEXATION_OF_THE_PARTY_OF_NICOYA_TO_COSTA_RICA,
    26889_i32 => _MOTHER_S_DAY,
    26920_i32 => _INDEPENDENCE_DAY,
    27021_i32 => _CHRISTMAS_DAY,
    27028_i32 => _NEW_YEAR_S_DAY,
    27129_i32 => _JUAN_SANTAMAR_A_DAY,
    27132_i32 => _MAUNDY_THURSDAY,
    27133_i32 => _GOOD_FRIDAY,
    27149_i32 => _INTERNATIONAL_LABOR_DAY,
    27234_i32 => _ANNEXATION_OF_THE_PARTY_OF_NICOYA_TO_COSTA_RICA,
    27255_i32 => _MOTHER_S_DAY,
    27286_i32 => _INDEPENDENCE_DAY,
    27387_i32 => _CHRISTMAS_DAY,
    27394_i32 => _NEW_YEAR_S_DAY,
    27489_i32 => _MAUNDY_THURSDAY,
    27490_i32 => _GOOD_FRIDAY,
    27494_i32 => _JUAN_SANTAMAR_A_DAY,
    27514_i32 => _INTERNATIONAL_LABOR_DAY,
    27599_i32 => _ANNEXATION_OF_THE_PARTY_OF_NICOYA_TO_COSTA_RICA,
    27620_i32 => _MOTHER_S_DAY,
    27651_i32 => _INDEPENDENCE_DAY,
    27752_i32 => _CHRISTMAS_DAY,
    27759_i32 => _NEW_YEAR_S_DAY,
    27839_i32 => _MAUNDY_THURSDAY,
    27840_i32 => _GOOD_FRIDAY,
    27859_i32 => _JUAN_SANTAMAR_A_DAY,
    27879_i32 => _INTERNATIONAL_LABOR_DAY,
    27964_i32 => _ANNEXATION_OF_THE_PARTY_OF_NICOYA_TO_COSTA_RICA,
    27985_i32 => _MOTHER_S_DAY,
    28016_i32 => _INDEPENDENCE_DAY,
    28117_i32 => _CHRISTMAS_DAY,
    28124_i32 => _NEW_YEAR_S_DAY,
    28224_i32 => _JUAN_SANTAMAR_A_DAY__MAUNDY_THURSDAY,
    28225_i32 => _GOOD_FRIDAY,
    28244_i32 => _INTERNATIONAL_LABOR_DAY,
    28329_i32 => _ANNEXATION_OF_THE_PARTY_OF_NICOYA_TO_COSTA_RICA,
    28350_i32 => _MOTHER_S_DAY,
    28381_i32 => _INDEPENDENCE_DAY,
    28482_i32 => _CHRISTMAS_DAY,
    28489_i32 => _NEW_YEAR_S_DAY,
    28581_i32 => _MAUNDY_THURSDAY,
    28582_i32 => _GOOD_FRIDAY,
    28590_i32 => _JUAN_SANTAMAR_A_DAY,
    28610_i32 => _INTERNATIONAL_LABOR_DAY,
    28695_i32 => _ANNEXATION_OF_THE_PARTY_OF_NICOYA_TO_COSTA_RICA,
    28716_i32 => _MOTHER_S_DAY,
    28747_i32 => _INDEPENDENCE_DAY,
    28848_i32 => _CHRISTMAS_DAY,
    28855_i32 => _NEW_YEAR_S_DAY,
    28955_i32 => _JUAN_SANTAMAR_A_DAY,
    28959_i32 => _MAUNDY_THURSDAY,
    28960_i32 => _GOOD_FRIDAY,
    28975_i32 => _INTERNATIONAL_LABOR_DAY,
    29060_i32 => _ANNEXATION_OF_THE_PARTY_OF_NICOYA_TO_COSTA_RICA,
    29081_i32 => _MOTHER_S_DAY,
    29112_i32 => _INDEPENDENCE_DAY,
    29213_i32 => _CHRISTMAS_DAY,
    29220_i32 => _NEW_YEAR_S_DAY,
};
