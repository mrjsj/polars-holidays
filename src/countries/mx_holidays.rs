use crate::countries::constants::*;
use phf::phf_map;

pub static MX_HOLIDAYS: phf::Map<i32, &'static str> = phf_map! {
    10957_i32 => _NEW_YEAR_S_DAY,
    10992_i32 => _CONSTITUTION_DAY,
    11037_i32 => _BENITO_JU_REZ_S_BIRTHDAY,
    11078_i32 => _LABOR_DAY,
    11216_i32 => _INDEPENDENCE_DAY,
    11281_i32 => _REVOLUTION_DAY,
    11292_i32 => _CHANGE_OF_FEDERAL_GOVERNMENT,
    11316_i32 => _CHRISTMAS_DAY,
    11323_i32 => _NEW_YEAR_S_DAY,
    11358_i32 => _CONSTITUTION_DAY,
    11402_i32 => _BENITO_JU_REZ_S_BIRTHDAY,
    11443_i32 => _LABOR_DAY,
    11581_i32 => _INDEPENDENCE_DAY,
    11646_i32 => _REVOLUTION_DAY,
    11681_i32 => _CHRISTMAS_DAY,
    11688_i32 => _NEW_YEAR_S_DAY,
    11723_i32 => _CONSTITUTION_DAY,
    11767_i32 => _BENITO_JU_REZ_S_BIRTHDAY,
    11808_i32 => _LABOR_DAY,
    11946_i32 => _INDEPENDENCE_DAY,
    12011_i32 => _REVOLUTION_DAY,
    12046_i32 => _CHRISTMAS_DAY,
    12053_i32 => _NEW_YEAR_S_DAY,
    12088_i32 => _CONSTITUTION_DAY,
    12132_i32 => _BENITO_JU_REZ_S_BIRTHDAY,
    12173_i32 => _LABOR_DAY,
    12311_i32 => _INDEPENDENCE_DAY,
    12376_i32 => _REVOLUTION_DAY,
    12411_i32 => _CHRISTMAS_DAY,
    12418_i32 => _NEW_YEAR_S_DAY,
    12453_i32 => _CONSTITUTION_DAY,
    12498_i32 => _BENITO_JU_REZ_S_BIRTHDAY,
    12539_i32 => _LABOR_DAY,
    12677_i32 => _INDEPENDENCE_DAY,
    12742_i32 => _REVOLUTION_DAY,
    12777_i32 => _CHRISTMAS_DAY,
    12784_i32 => _NEW_YEAR_S_DAY,
    12819_i32 => _CONSTITUTION_DAY,
    12863_i32 => _BENITO_JU_REZ_S_BIRTHDAY,
    12904_i32 => _LABOR_DAY,
    13042_i32 => _INDEPENDENCE_DAY,
    13107_i32 => _REVOLUTION_DAY,
    13142_i32 => _CHRISTMAS_DAY,
    13149_i32 => _NEW_YEAR_S_DAY,
    13185_i32 => _CONSTITUTION_DAY,
    13228_i32 => _BENITO_JU_REZ_S_BIRTHDAY,
    13269_i32 => _LABOR_DAY,
    13407_i32 => _INDEPENDENCE_DAY,
    13472_i32 => _REVOLUTION_DAY,
    13483_i32 => _CHANGE_OF_FEDERAL_GOVERNMENT,
    13507_i32 => _CHRISTMAS_DAY,
    13514_i32 => _NEW_YEAR_S_DAY,
    13549_i32 => _CONSTITUTION_DAY,
    13591_i32 => _BENITO_JU_REZ_S_BIRTHDAY,
    13634_i32 => _LABOR_DAY,
    13772_i32 => _INDEPENDENCE_DAY,
    13836_i32 => _REVOLUTION_DAY,
    13872_i32 => _CHRISTMAS_DAY,
    13879_i32 => _NEW_YEAR_S_DAY,
    13913_i32 => _CONSTITUTION_DAY,
    13955_i32 => _BENITO_JU_REZ_S_BIRTHDAY,
    14000_i32 => _LABOR_DAY,
    14138_i32 => _INDEPENDENCE_DAY,
    14200_i32 => _REVOLUTION_DAY,
    14238_i32 => _CHRISTMAS_DAY,
    14245_i32 => _NEW_YEAR_S_DAY,
    14277_i32 => _CONSTITUTION_DAY,
    14319_i32 => _BENITO_JU_REZ_S_BIRTHDAY,
    14365_i32 => _LABOR_DAY,
    14503_i32 => _INDEPENDENCE_DAY,
    14564_i32 => _REVOLUTION_DAY,
    14603_i32 => _CHRISTMAS_DAY,
    14610_i32 => _NEW_YEAR_S_DAY,
    14641_i32 => _CONSTITUTION_DAY,
    14683_i32 => _BENITO_JU_REZ_S_BIRTHDAY,
    14730_i32 => _LABOR_DAY,
    14868_i32 => _INDEPENDENCE_DAY,
    14928_i32 => _REVOLUTION_DAY,
    14968_i32 => _CHRISTMAS_DAY,
    14975_i32 => _NEW_YEAR_S_DAY,
    15012_i32 => _CONSTITUTION_DAY,
    15054_i32 => _BENITO_JU_REZ_S_BIRTHDAY,
    15095_i32 => _LABOR_DAY,
    15233_i32 => _INDEPENDENCE_DAY,
    15299_i32 => _REVOLUTION_DAY,
    15333_i32 => _CHRISTMAS_DAY,
    15340_i32 => _NEW_YEAR_S_DAY,
    15376_i32 => _CONSTITUTION_DAY,
    15418_i32 => _BENITO_JU_REZ_S_BIRTHDAY,
    15461_i32 => _LABOR_DAY,
    15599_i32 => _INDEPENDENCE_DAY,
    15663_i32 => _REVOLUTION_DAY,
    15675_i32 => _CHANGE_OF_FEDERAL_GOVERNMENT,
    15699_i32 => _CHRISTMAS_DAY,
    15706_i32 => _NEW_YEAR_S_DAY,
    15740_i32 => _CONSTITUTION_DAY,
    15782_i32 => _BENITO_JU_REZ_S_BIRTHDAY,
    15826_i32 => _LABOR_DAY,
    15964_i32 => _INDEPENDENCE_DAY,
    16027_i32 => _REVOLUTION_DAY,
    16064_i32 => _CHRISTMAS_DAY,
    16071_i32 => _NEW_YEAR_S_DAY,
    16104_i32 => _CONSTITUTION_DAY,
    16146_i32 => _BENITO_JU_REZ_S_BIRTHDAY,
    16191_i32 => _LABOR_DAY,
    16329_i32 => _INDEPENDENCE_DAY,
    16391_i32 => _REVOLUTION_DAY,
    16429_i32 => _CHRISTMAS_DAY,
    16436_i32 => _NEW_YEAR_S_DAY,
    16468_i32 => _CONSTITUTION_DAY,
    16510_i32 => _BENITO_JU_REZ_S_BIRTHDAY,
    16556_i32 => _LABOR_DAY,
    16694_i32 => _INDEPENDENCE_DAY,
    16755_i32 => _REVOLUTION_DAY,
    16794_i32 => _CHRISTMAS_DAY,
    16801_i32 => _NEW_YEAR_S_DAY,
    16832_i32 => _CONSTITUTION_DAY,
    16881_i32 => _BENITO_JU_REZ_S_BIRTHDAY,
    16922_i32 => _LABOR_DAY,
    17060_i32 => _INDEPENDENCE_DAY,
    17126_i32 => _REVOLUTION_DAY,
    17160_i32 => _CHRISTMAS_DAY,
    17167_i32 => _NEW_YEAR_S_DAY,
    17203_i32 => _CONSTITUTION_DAY,
    17245_i32 => _BENITO_JU_REZ_S_BIRTHDAY,
    17287_i32 => _LABOR_DAY,
    17425_i32 => _INDEPENDENCE_DAY,
    17490_i32 => _REVOLUTION_DAY,
    17525_i32 => _CHRISTMAS_DAY,
    17532_i32 => _NEW_YEAR_S_DAY,
    17567_i32 => _CONSTITUTION_DAY,
    17609_i32 => _BENITO_JU_REZ_S_BIRTHDAY,
    17652_i32 => _LABOR_DAY,
    17790_i32 => _INDEPENDENCE_DAY,
    17854_i32 => _REVOLUTION_DAY,
    17866_i32 => _CHANGE_OF_FEDERAL_GOVERNMENT,
    17890_i32 => _CHRISTMAS_DAY,
    17897_i32 => _NEW_YEAR_S_DAY,
    17931_i32 => _CONSTITUTION_DAY,
    17973_i32 => _BENITO_JU_REZ_S_BIRTHDAY,
    18017_i32 => _LABOR_DAY,
    18155_i32 => _INDEPENDENCE_DAY,
    18218_i32 => _REVOLUTION_DAY,
    18255_i32 => _CHRISTMAS_DAY,
    18262_i32 => _NEW_YEAR_S_DAY,
    18295_i32 => _CONSTITUTION_DAY,
    18337_i32 => _BENITO_JU_REZ_S_BIRTHDAY,
    18383_i32 => _LABOR_DAY,
    18521_i32 => _INDEPENDENCE_DAY,
    18582_i32 => _REVOLUTION_DAY,
    18621_i32 => _CHRISTMAS_DAY,
    18628_i32 => _NEW_YEAR_S_DAY,
    18659_i32 => _CONSTITUTION_DAY,
    18701_i32 => _BENITO_JU_REZ_S_BIRTHDAY,
    18748_i32 => _LABOR_DAY,
    18886_i32 => _INDEPENDENCE_DAY,
    18946_i32 => _REVOLUTION_DAY,
    18986_i32 => _CHRISTMAS_DAY,
    18993_i32 => _NEW_YEAR_S_DAY,
    19030_i32 => _CONSTITUTION_DAY,
    19072_i32 => _BENITO_JU_REZ_S_BIRTHDAY,
    19113_i32 => _LABOR_DAY,
    19251_i32 => _INDEPENDENCE_DAY,
    19317_i32 => _REVOLUTION_DAY,
    19351_i32 => _CHRISTMAS_DAY,
    19358_i32 => _NEW_YEAR_S_DAY,
    19394_i32 => _CONSTITUTION_DAY,
    19436_i32 => _BENITO_JU_REZ_S_BIRTHDAY,
    19478_i32 => _LABOR_DAY,
    19616_i32 => _INDEPENDENCE_DAY,
    19681_i32 => _REVOLUTION_DAY,
    19716_i32 => _CHRISTMAS_DAY,
    19723_i32 => _NEW_YEAR_S_DAY,
    19758_i32 => _CONSTITUTION_DAY,
    19800_i32 => _BENITO_JU_REZ_S_BIRTHDAY,
    19844_i32 => _LABOR_DAY,
    19982_i32 => _INDEPENDENCE_DAY,
    19997_i32 => _CHANGE_OF_FEDERAL_GOVERNMENT,
    20045_i32 => _REVOLUTION_DAY,
    20082_i32 => _CHRISTMAS_DAY,
    20089_i32 => _NEW_YEAR_S_DAY,
    20122_i32 => _CONSTITUTION_DAY,
    20164_i32 => _BENITO_JU_REZ_S_BIRTHDAY,
    20209_i32 => _LABOR_DAY,
    20347_i32 => _INDEPENDENCE_DAY,
    20409_i32 => _REVOLUTION_DAY,
    20447_i32 => _CHRISTMAS_DAY,
    20454_i32 => _NEW_YEAR_S_DAY,
    20486_i32 => _CONSTITUTION_DAY,
    20528_i32 => _BENITO_JU_REZ_S_BIRTHDAY,
    20574_i32 => _LABOR_DAY,
    20712_i32 => _INDEPENDENCE_DAY,
    20773_i32 => _REVOLUTION_DAY,
    20812_i32 => _CHRISTMAS_DAY,
    20819_i32 => _NEW_YEAR_S_DAY,
    20850_i32 => _CONSTITUTION_DAY,
    20892_i32 => _BENITO_JU_REZ_S_BIRTHDAY,
    20939_i32 => _LABOR_DAY,
    21077_i32 => _INDEPENDENCE_DAY,
    21137_i32 => _REVOLUTION_DAY,
    21177_i32 => _CHRISTMAS_DAY,
    21184_i32 => _NEW_YEAR_S_DAY,
    21221_i32 => _CONSTITUTION_DAY,
    21263_i32 => _BENITO_JU_REZ_S_BIRTHDAY,
    21305_i32 => _LABOR_DAY,
    21443_i32 => _INDEPENDENCE_DAY,
    21508_i32 => _REVOLUTION_DAY,
    21543_i32 => _CHRISTMAS_DAY,
    21550_i32 => _NEW_YEAR_S_DAY,
    21585_i32 => _CONSTITUTION_DAY,
    21627_i32 => _BENITO_JU_REZ_S_BIRTHDAY,
    21670_i32 => _LABOR_DAY,
    21808_i32 => _INDEPENDENCE_DAY,
    21872_i32 => _REVOLUTION_DAY,
    21908_i32 => _CHRISTMAS_DAY,
    21915_i32 => _NEW_YEAR_S_DAY,
    21949_i32 => _CONSTITUTION_DAY,
    21991_i32 => _BENITO_JU_REZ_S_BIRTHDAY,
    22035_i32 => _LABOR_DAY,
    22173_i32 => _INDEPENDENCE_DAY,
    22188_i32 => _CHANGE_OF_FEDERAL_GOVERNMENT,
    22236_i32 => _REVOLUTION_DAY,
    22273_i32 => _CHRISTMAS_DAY,
    22280_i32 => _NEW_YEAR_S_DAY,
    22313_i32 => _CONSTITUTION_DAY,
    22355_i32 => _BENITO_JU_REZ_S_BIRTHDAY,
    22400_i32 => _LABOR_DAY,
    22538_i32 => _INDEPENDENCE_DAY,
    22600_i32 => _REVOLUTION_DAY,
    22638_i32 => _CHRISTMAS_DAY,
    22645_i32 => _NEW_YEAR_S_DAY,
    22677_i32 => _CONSTITUTION_DAY,
    22719_i32 => _BENITO_JU_REZ_S_BIRTHDAY,
    22766_i32 => _LABOR_DAY,
    22904_i32 => _INDEPENDENCE_DAY,
    22964_i32 => _REVOLUTION_DAY,
    23004_i32 => _CHRISTMAS_DAY,
    23011_i32 => _NEW_YEAR_S_DAY,
    23048_i32 => _CONSTITUTION_DAY,
    23090_i32 => _BENITO_JU_REZ_S_BIRTHDAY,
    23131_i32 => _LABOR_DAY,
    23269_i32 => _INDEPENDENCE_DAY,
    23335_i32 => _REVOLUTION_DAY,
    23369_i32 => _CHRISTMAS_DAY,
    23376_i32 => _NEW_YEAR_S_DAY,
    23412_i32 => _CONSTITUTION_DAY,
    23454_i32 => _BENITO_JU_REZ_S_BIRTHDAY,
    23496_i32 => _LABOR_DAY,
    23634_i32 => _INDEPENDENCE_DAY,
    23699_i32 => _REVOLUTION_DAY,
    23734_i32 => _CHRISTMAS_DAY,
    23741_i32 => _NEW_YEAR_S_DAY,
    23776_i32 => _CONSTITUTION_DAY,
    23818_i32 => _BENITO_JU_REZ_S_BIRTHDAY,
    23861_i32 => _LABOR_DAY,
    23999_i32 => _INDEPENDENCE_DAY,
    24063_i32 => _REVOLUTION_DAY,
    24099_i32 => _CHRISTMAS_DAY,
    24106_i32 => _NEW_YEAR_S_DAY,
    24140_i32 => _CONSTITUTION_DAY,
    24182_i32 => _BENITO_JU_REZ_S_BIRTHDAY,
    24227_i32 => _LABOR_DAY,
    24365_i32 => _INDEPENDENCE_DAY,
    24380_i32 => _CHANGE_OF_FEDERAL_GOVERNMENT,
    24427_i32 => _REVOLUTION_DAY,
    24465_i32 => _CHRISTMAS_DAY,
    24472_i32 => _NEW_YEAR_S_DAY,
    24504_i32 => _CONSTITUTION_DAY,
    24546_i32 => _BENITO_JU_REZ_S_BIRTHDAY,
    24592_i32 => _LABOR_DAY,
    24730_i32 => _INDEPENDENCE_DAY,
    24791_i32 => _REVOLUTION_DAY,
    24830_i32 => _CHRISTMAS_DAY,
    24837_i32 => _NEW_YEAR_S_DAY,
    24868_i32 => _CONSTITUTION_DAY,
    24910_i32 => _BENITO_JU_REZ_S_BIRTHDAY,
    24957_i32 => _LABOR_DAY,
    25095_i32 => _INDEPENDENCE_DAY,
    25155_i32 => _REVOLUTION_DAY,
    25195_i32 => _CHRISTMAS_DAY,
    25202_i32 => _NEW_YEAR_S_DAY,
    25239_i32 => _CONSTITUTION_DAY,
    25281_i32 => _BENITO_JU_REZ_S_BIRTHDAY,
    25322_i32 => _LABOR_DAY,
    25460_i32 => _INDEPENDENCE_DAY,
    25526_i32 => _REVOLUTION_DAY,
    25560_i32 => _CHRISTMAS_DAY,
    25567_i32 => _NEW_YEAR_S_DAY,
    25603_i32 => _CONSTITUTION_DAY,
    25645_i32 => _BENITO_JU_REZ_S_BIRTHDAY,
    25688_i32 => _LABOR_DAY,
    25826_i32 => _INDEPENDENCE_DAY,
    25890_i32 => _REVOLUTION_DAY,
    25926_i32 => _CHRISTMAS_DAY,
    25933_i32 => _NEW_YEAR_S_DAY,
    25967_i32 => _CONSTITUTION_DAY,
    26009_i32 => _BENITO_JU_REZ_S_BIRTHDAY,
    26053_i32 => _LABOR_DAY,
    26191_i32 => _INDEPENDENCE_DAY,
    26254_i32 => _REVOLUTION_DAY,
    26291_i32 => _CHRISTMAS_DAY,
    26298_i32 => _NEW_YEAR_S_DAY,
    26331_i32 => _CONSTITUTION_DAY,
    26373_i32 => _BENITO_JU_REZ_S_BIRTHDAY,
    26418_i32 => _LABOR_DAY,
    26556_i32 => _INDEPENDENCE_DAY,
    26571_i32 => _CHANGE_OF_FEDERAL_GOVERNMENT,
    26618_i32 => _REVOLUTION_DAY,
    26656_i32 => _CHRISTMAS_DAY,
    26663_i32 => _NEW_YEAR_S_DAY,
    26695_i32 => _CONSTITUTION_DAY,
    26737_i32 => _BENITO_JU_REZ_S_BIRTHDAY,
    26783_i32 => _LABOR_DAY,
    26921_i32 => _INDEPENDENCE_DAY,
    26982_i32 => _REVOLUTION_DAY,
    27021_i32 => _CHRISTMAS_DAY,
    27028_i32 => _NEW_YEAR_S_DAY,
    27059_i32 => _CONSTITUTION_DAY,
    27108_i32 => _BENITO_JU_REZ_S_BIRTHDAY,
    27149_i32 => _LABOR_DAY,
    27287_i32 => _INDEPENDENCE_DAY,
    27353_i32 => _REVOLUTION_DAY,
    27387_i32 => _CHRISTMAS_DAY,
    27394_i32 => _NEW_YEAR_S_DAY,
    27430_i32 => _CONSTITUTION_DAY,
    27472_i32 => _BENITO_JU_REZ_S_BIRTHDAY,
    27514_i32 => _LABOR_DAY,
    27652_i32 => _INDEPENDENCE_DAY,
    27717_i32 => _REVOLUTION_DAY,
    27752_i32 => _CHRISTMAS_DAY,
    27759_i32 => _NEW_YEAR_S_DAY,
    27794_i32 => _CONSTITUTION_DAY,
    27836_i32 => _BENITO_JU_REZ_S_BIRTHDAY,
    27879_i32 => _LABOR_DAY,
    28017_i32 => _INDEPENDENCE_DAY,
    28081_i32 => _REVOLUTION_DAY,
    28117_i32 => _CHRISTMAS_DAY,
    28124_i32 => _NEW_YEAR_S_DAY,
    28158_i32 => _CONSTITUTION_DAY,
    28200_i32 => _BENITO_JU_REZ_S_BIRTHDAY,
    28244_i32 => _LABOR_DAY,
    28382_i32 => _INDEPENDENCE_DAY,
    28445_i32 => _REVOLUTION_DAY,
    28482_i32 => _CHRISTMAS_DAY,
    28489_i32 => _NEW_YEAR_S_DAY,
    28522_i32 => _CONSTITUTION_DAY,
    28564_i32 => _BENITO_JU_REZ_S_BIRTHDAY,
    28610_i32 => _LABOR_DAY,
    28748_i32 => _INDEPENDENCE_DAY,
    28763_i32 => _CHANGE_OF_FEDERAL_GOVERNMENT,
    28809_i32 => _REVOLUTION_DAY,
    28848_i32 => _CHRISTMAS_DAY,
    28855_i32 => _NEW_YEAR_S_DAY,
    28886_i32 => _CONSTITUTION_DAY,
    28928_i32 => _BENITO_JU_REZ_S_BIRTHDAY,
    28975_i32 => _LABOR_DAY,
    29113_i32 => _INDEPENDENCE_DAY,
    29173_i32 => _REVOLUTION_DAY,
    29213_i32 => _CHRISTMAS_DAY,
    29220_i32 => _NEW_YEAR_S_DAY,
};
