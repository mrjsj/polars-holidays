use crate::countries::constants::*;
use phf::phf_map;

pub static DJ_HOLIDAYS: phf::Map<i32, &'static str> = phf_map! {
    10957_i32 => _NEW_YEAR_S_DAY,
    10964_i32 => _EID_AL_FITR__ESTIMATED_,
    10965_i32 => _EID_AL_FITR_HOLIDAY__ESTIMATED_,
    11031_i32 => _ARAFAT_DAY__ESTIMATED_,
    11032_i32 => _EID_AL_ADHA__ESTIMATED_,
    11033_i32 => _EID_AL_ADHA_HOLIDAY__ESTIMATED_,
    11053_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    11078_i32 => _LABOR_DAY,
    11122_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    11135_i32 => _INDEPENDENCE_DAY,
    11136_i32 => _INDEPENDENCE_DAY_HOLIDAY,
    11254_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    11316_i32 => _CHRISTMAS_DAY,
    11318_i32 => _EID_AL_FITR__ESTIMATED_,
    11319_i32 => _EID_AL_FITR_HOLIDAY__ESTIMATED_,
    11323_i32 => _NEW_YEAR_S_DAY,
    11385_i32 => _ARAFAT_DAY__ESTIMATED_,
    11386_i32 => _EID_AL_ADHA__ESTIMATED_,
    11387_i32 => _EID_AL_ADHA_HOLIDAY__ESTIMATED_,
    11407_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    11443_i32 => _LABOR_DAY,
    11477_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    11500_i32 => _INDEPENDENCE_DAY,
    11501_i32 => _INDEPENDENCE_DAY_HOLIDAY,
    11609_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    11672_i32 => _EID_AL_FITR__ESTIMATED_,
    11673_i32 => _EID_AL_FITR_HOLIDAY__ESTIMATED_,
    11681_i32 => _CHRISTMAS_DAY,
    11688_i32 => _NEW_YEAR_S_DAY,
    11739_i32 => _ARAFAT_DAY__ESTIMATED_,
    11740_i32 => _EID_AL_ADHA__ESTIMATED_,
    11741_i32 => _EID_AL_ADHA_HOLIDAY__ESTIMATED_,
    11761_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    11808_i32 => _LABOR_DAY,
    11831_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    11865_i32 => _INDEPENDENCE_DAY,
    11866_i32 => _INDEPENDENCE_DAY_HOLIDAY,
    11964_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    12026_i32 => _EID_AL_FITR__ESTIMATED_,
    12027_i32 => _EID_AL_FITR_HOLIDAY__ESTIMATED_,
    12046_i32 => _CHRISTMAS_DAY,
    12053_i32 => _NEW_YEAR_S_DAY,
    12093_i32 => _ARAFAT_DAY__ESTIMATED_,
    12094_i32 => _EID_AL_ADHA__ESTIMATED_,
    12095_i32 => _EID_AL_ADHA_HOLIDAY__ESTIMATED_,
    12115_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    12173_i32 => _LABOR_DAY,
    12185_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    12230_i32 => _INDEPENDENCE_DAY,
    12231_i32 => _INDEPENDENCE_DAY_HOLIDAY,
    12319_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    12381_i32 => _EID_AL_FITR__ESTIMATED_,
    12382_i32 => _EID_AL_FITR_HOLIDAY__ESTIMATED_,
    12411_i32 => _CHRISTMAS_DAY,
    12418_i32 => _NEW_YEAR_S_DAY,
    12448_i32 => _ARAFAT_DAY__ESTIMATED_,
    12449_i32 => _EID_AL_ADHA__ESTIMATED_,
    12450_i32 => _EID_AL_ADHA_HOLIDAY__ESTIMATED_,
    12469_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    12539_i32 => _LABOR_DAY__PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    12596_i32 => _INDEPENDENCE_DAY,
    12597_i32 => _INDEPENDENCE_DAY_HOLIDAY,
    12673_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    12736_i32 => _EID_AL_FITR__ESTIMATED_,
    12737_i32 => _EID_AL_FITR_HOLIDAY__ESTIMATED_,
    12777_i32 => _CHRISTMAS_DAY,
    12784_i32 => _NEW_YEAR_S_DAY,
    12803_i32 => _ARAFAT_DAY__ESTIMATED_,
    12804_i32 => _EID_AL_ADHA__ESTIMATED_,
    12805_i32 => _EID_AL_ADHA_HOLIDAY__ESTIMATED_,
    12824_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    12894_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    12904_i32 => _LABOR_DAY,
    12961_i32 => _INDEPENDENCE_DAY,
    12962_i32 => _INDEPENDENCE_DAY_HOLIDAY,
    13027_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    13090_i32 => _EID_AL_FITR__ESTIMATED_,
    13091_i32 => _EID_AL_FITR_HOLIDAY__ESTIMATED_,
    13142_i32 => _CHRISTMAS_DAY,
    13149_i32 => _NEW_YEAR_S_DAY,
    13157_i32 => _ARAFAT_DAY__ESTIMATED_,
    13158_i32 => _EID_AL_ADHA__ESTIMATED_,
    13159_i32 => _EID_AL_ADHA_HOLIDAY__ESTIMATED_,
    13179_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    13248_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    13269_i32 => _LABOR_DAY,
    13326_i32 => _INDEPENDENCE_DAY,
    13327_i32 => _INDEPENDENCE_DAY_HOLIDAY,
    13381_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    13444_i32 => _EID_AL_FITR__ESTIMATED_,
    13445_i32 => _EID_AL_FITR_HOLIDAY__ESTIMATED_,
    13507_i32 => _CHRISTMAS_DAY,
    13512_i32 => _ARAFAT_DAY__ESTIMATED_,
    13513_i32 => _EID_AL_ADHA__ESTIMATED_,
    13514_i32 => _EID_AL_ADHA_HOLIDAY__ESTIMATED___NEW_YEAR_S_DAY,
    13533_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    13603_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    13634_i32 => _LABOR_DAY,
    13691_i32 => _INDEPENDENCE_DAY,
    13692_i32 => _INDEPENDENCE_DAY_HOLIDAY,
    13735_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    13799_i32 => _EID_AL_FITR__ESTIMATED_,
    13800_i32 => _EID_AL_FITR_HOLIDAY__ESTIMATED_,
    13866_i32 => _ARAFAT_DAY__ESTIMATED_,
    13867_i32 => _EID_AL_ADHA__ESTIMATED_,
    13868_i32 => _EID_AL_ADHA_HOLIDAY__ESTIMATED_,
    13872_i32 => _CHRISTMAS_DAY,
    13879_i32 => _NEW_YEAR_S_DAY,
    13888_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    13958_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    14000_i32 => _LABOR_DAY,
    14057_i32 => _INDEPENDENCE_DAY,
    14058_i32 => _INDEPENDENCE_DAY_HOLIDAY,
    14090_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    14153_i32 => _EID_AL_FITR__ESTIMATED_,
    14154_i32 => _EID_AL_FITR_HOLIDAY__ESTIMATED_,
    14220_i32 => _ARAFAT_DAY__ESTIMATED_,
    14221_i32 => _EID_AL_ADHA__ESTIMATED_,
    14222_i32 => _EID_AL_ADHA_HOLIDAY__ESTIMATED_,
    14238_i32 => _CHRISTMAS_DAY,
    14242_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    14245_i32 => _NEW_YEAR_S_DAY,
    14312_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    14365_i32 => _LABOR_DAY,
    14422_i32 => _INDEPENDENCE_DAY,
    14423_i32 => _INDEPENDENCE_DAY_HOLIDAY,
    14445_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    14507_i32 => _EID_AL_FITR__ESTIMATED_,
    14508_i32 => _EID_AL_FITR_HOLIDAY__ESTIMATED_,
    14574_i32 => _ARAFAT_DAY__ESTIMATED_,
    14575_i32 => _EID_AL_ADHA__ESTIMATED_,
    14576_i32 => _EID_AL_ADHA_HOLIDAY__ESTIMATED_,
    14596_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    14603_i32 => _CHRISTMAS_DAY,
    14610_i32 => _NEW_YEAR_S_DAY,
    14666_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    14730_i32 => _LABOR_DAY,
    14787_i32 => _INDEPENDENCE_DAY,
    14788_i32 => _INDEPENDENCE_DAY_HOLIDAY,
    14799_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    14862_i32 => _EID_AL_FITR__ESTIMATED_,
    14863_i32 => _EID_AL_FITR_HOLIDAY__ESTIMATED_,
    14928_i32 => _ARAFAT_DAY__ESTIMATED_,
    14929_i32 => _EID_AL_ADHA__ESTIMATED_,
    14930_i32 => _EID_AL_ADHA_HOLIDAY__ESTIMATED_,
    14950_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    14968_i32 => _CHRISTMAS_DAY,
    14975_i32 => _NEW_YEAR_S_DAY,
    15020_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    15095_i32 => _LABOR_DAY,
    15152_i32 => _INDEPENDENCE_DAY,
    15153_i32 => _INDEPENDENCE_DAY_HOLIDAY,
    15154_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    15216_i32 => _EID_AL_FITR__ESTIMATED_,
    15217_i32 => _EID_AL_FITR_HOLIDAY__ESTIMATED_,
    15283_i32 => _ARAFAT_DAY__ESTIMATED_,
    15284_i32 => _EID_AL_ADHA__ESTIMATED_,
    15285_i32 => _EID_AL_ADHA_HOLIDAY__ESTIMATED_,
    15304_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    15333_i32 => _CHRISTMAS_DAY,
    15340_i32 => _NEW_YEAR_S_DAY,
    15374_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    15461_i32 => _LABOR_DAY,
    15508_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    15518_i32 => _INDEPENDENCE_DAY,
    15519_i32 => _INDEPENDENCE_DAY_HOLIDAY,
    15571_i32 => _EID_AL_FITR__ESTIMATED_,
    15572_i32 => _EID_AL_FITR_HOLIDAY__ESTIMATED_,
    15638_i32 => _ARAFAT_DAY__ESTIMATED_,
    15639_i32 => _EID_AL_ADHA__ESTIMATED_,
    15640_i32 => _EID_AL_ADHA_HOLIDAY__ESTIMATED_,
    15659_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    15699_i32 => _CHRISTMAS_DAY,
    15706_i32 => _NEW_YEAR_S_DAY,
    15729_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    15826_i32 => _LABOR_DAY,
    15862_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    15883_i32 => _INDEPENDENCE_DAY,
    15884_i32 => _INDEPENDENCE_DAY_HOLIDAY,
    15925_i32 => _EID_AL_FITR__ESTIMATED_,
    15926_i32 => _EID_AL_FITR_HOLIDAY__ESTIMATED_,
    15992_i32 => _ARAFAT_DAY__ESTIMATED_,
    15993_i32 => _EID_AL_ADHA__ESTIMATED_,
    15994_i32 => _EID_AL_ADHA_HOLIDAY__ESTIMATED_,
    16013_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    16064_i32 => _CHRISTMAS_DAY,
    16071_i32 => _NEW_YEAR_S_DAY,
    16083_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    16191_i32 => _LABOR_DAY,
    16216_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    16248_i32 => _INDEPENDENCE_DAY,
    16249_i32 => _INDEPENDENCE_DAY_HOLIDAY,
    16279_i32 => _EID_AL_FITR__ESTIMATED_,
    16280_i32 => _EID_AL_FITR_HOLIDAY__ESTIMATED_,
    16346_i32 => _ARAFAT_DAY__ESTIMATED_,
    16347_i32 => _EID_AL_ADHA__ESTIMATED_,
    16348_i32 => _EID_AL_ADHA_HOLIDAY__ESTIMATED_,
    16368_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    16429_i32 => _CHRISTMAS_DAY,
    16436_i32 => _NEW_YEAR_S_DAY,
    16438_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    16556_i32 => _LABOR_DAY,
    16571_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    16613_i32 => _INDEPENDENCE_DAY,
    16614_i32 => _INDEPENDENCE_DAY_HOLIDAY,
    16633_i32 => _EID_AL_FITR__ESTIMATED_,
    16634_i32 => _EID_AL_FITR_HOLIDAY__ESTIMATED_,
    16700_i32 => _ARAFAT_DAY__ESTIMATED_,
    16701_i32 => _EID_AL_ADHA__ESTIMATED_,
    16702_i32 => _EID_AL_ADHA_HOLIDAY__ESTIMATED_,
    16722_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    16792_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    16794_i32 => _CHRISTMAS_DAY,
    16801_i32 => _NEW_YEAR_S_DAY,
    16922_i32 => _LABOR_DAY,
    16925_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    16979_i32 => _INDEPENDENCE_DAY,
    16980_i32 => _INDEPENDENCE_DAY_HOLIDAY,
    16988_i32 => _EID_AL_FITR__ESTIMATED_,
    16989_i32 => _EID_AL_FITR_HOLIDAY__ESTIMATED_,
    17054_i32 => _ARAFAT_DAY__ESTIMATED_,
    17055_i32 => _EID_AL_ADHA__ESTIMATED_,
    17056_i32 => _EID_AL_ADHA_HOLIDAY__ESTIMATED_,
    17076_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    17146_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    17160_i32 => _CHRISTMAS_DAY,
    17167_i32 => _NEW_YEAR_S_DAY,
    17280_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    17287_i32 => _LABOR_DAY,
    17342_i32 => _EID_AL_FITR__ESTIMATED_,
    17343_i32 => _EID_AL_FITR_HOLIDAY__ESTIMATED_,
    17344_i32 => _INDEPENDENCE_DAY,
    17345_i32 => _INDEPENDENCE_DAY_HOLIDAY,
    17409_i32 => _ARAFAT_DAY__ESTIMATED_,
    17410_i32 => _EID_AL_ADHA__ESTIMATED_,
    17411_i32 => _EID_AL_ADHA_HOLIDAY__ESTIMATED_,
    17430_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    17500_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    17525_i32 => _CHRISTMAS_DAY,
    17532_i32 => _NEW_YEAR_S_DAY,
    17634_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    17652_i32 => _LABOR_DAY,
    17697_i32 => _EID_AL_FITR__ESTIMATED_,
    17698_i32 => _EID_AL_FITR_HOLIDAY__ESTIMATED_,
    17709_i32 => _INDEPENDENCE_DAY,
    17710_i32 => _INDEPENDENCE_DAY_HOLIDAY,
    17763_i32 => _ARAFAT_DAY__ESTIMATED_,
    17764_i32 => _EID_AL_ADHA__ESTIMATED_,
    17765_i32 => _EID_AL_ADHA_HOLIDAY__ESTIMATED_,
    17785_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    17855_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    17890_i32 => _CHRISTMAS_DAY,
    17897_i32 => _NEW_YEAR_S_DAY,
    17989_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    18017_i32 => _LABOR_DAY,
    18051_i32 => _EID_AL_FITR__ESTIMATED_,
    18052_i32 => _EID_AL_FITR_HOLIDAY__ESTIMATED_,
    18074_i32 => _INDEPENDENCE_DAY,
    18075_i32 => _INDEPENDENCE_DAY_HOLIDAY,
    18118_i32 => _ARAFAT_DAY__ESTIMATED_,
    18119_i32 => _EID_AL_ADHA__ESTIMATED_,
    18120_i32 => _EID_AL_ADHA_HOLIDAY__ESTIMATED_,
    18139_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    18209_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    18255_i32 => _CHRISTMAS_DAY,
    18262_i32 => _NEW_YEAR_S_DAY,
    18343_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    18383_i32 => _LABOR_DAY,
    18406_i32 => _EID_AL_FITR__ESTIMATED_,
    18407_i32 => _EID_AL_FITR_HOLIDAY__ESTIMATED_,
    18440_i32 => _INDEPENDENCE_DAY,
    18441_i32 => _INDEPENDENCE_DAY_HOLIDAY,
    18473_i32 => _ARAFAT_DAY__ESTIMATED_,
    18474_i32 => _EID_AL_ADHA__ESTIMATED_,
    18475_i32 => _EID_AL_ADHA_HOLIDAY__ESTIMATED_,
    18494_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    18564_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    18621_i32 => _CHRISTMAS_DAY,
    18628_i32 => _NEW_YEAR_S_DAY,
    18697_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    18748_i32 => _LABOR_DAY,
    18760_i32 => _EID_AL_FITR__ESTIMATED_,
    18761_i32 => _EID_AL_FITR_HOLIDAY__ESTIMATED_,
    18805_i32 => _INDEPENDENCE_DAY,
    18806_i32 => _INDEPENDENCE_DAY_HOLIDAY,
    18827_i32 => _ARAFAT_DAY__ESTIMATED_,
    18828_i32 => _EID_AL_ADHA__ESTIMATED_,
    18829_i32 => _EID_AL_ADHA_HOLIDAY__ESTIMATED_,
    18848_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    18918_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    18986_i32 => _CHRISTMAS_DAY,
    18993_i32 => _NEW_YEAR_S_DAY,
    19051_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    19113_i32 => _LABOR_DAY,
    19114_i32 => _EID_AL_FITR__ESTIMATED_,
    19115_i32 => _EID_AL_FITR_HOLIDAY__ESTIMATED_,
    19170_i32 => _INDEPENDENCE_DAY,
    19171_i32 => _INDEPENDENCE_DAY_HOLIDAY,
    19181_i32 => _ARAFAT_DAY__ESTIMATED_,
    19182_i32 => _EID_AL_ADHA__ESTIMATED_,
    19183_i32 => _EID_AL_ADHA_HOLIDAY__ESTIMATED_,
    19203_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    19273_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    19351_i32 => _CHRISTMAS_DAY,
    19358_i32 => _NEW_YEAR_S_DAY,
    19406_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    19468_i32 => _EID_AL_FITR__ESTIMATED_,
    19469_i32 => _EID_AL_FITR_HOLIDAY__ESTIMATED_,
    19478_i32 => _LABOR_DAY,
    19535_i32 => _ARAFAT_DAY__ESTIMATED___INDEPENDENCE_DAY,
    19536_i32 => _EID_AL_ADHA__ESTIMATED___INDEPENDENCE_DAY_HOLIDAY,
    19537_i32 => _EID_AL_ADHA_HOLIDAY__ESTIMATED_,
    19557_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    19627_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    19716_i32 => _CHRISTMAS_DAY,
    19723_i32 => _NEW_YEAR_S_DAY,
    19761_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    19823_i32 => _EID_AL_FITR__ESTIMATED_,
    19824_i32 => _EID_AL_FITR_HOLIDAY__ESTIMATED_,
    19844_i32 => _LABOR_DAY,
    19889_i32 => _ARAFAT_DAY__ESTIMATED_,
    19890_i32 => _EID_AL_ADHA__ESTIMATED_,
    19891_i32 => _EID_AL_ADHA_HOLIDAY__ESTIMATED_,
    19901_i32 => _INDEPENDENCE_DAY,
    19902_i32 => _INDEPENDENCE_DAY_HOLIDAY,
    19911_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    19981_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    20082_i32 => _CHRISTMAS_DAY,
    20089_i32 => _NEW_YEAR_S_DAY,
    20115_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    20177_i32 => _EID_AL_FITR__ESTIMATED_,
    20178_i32 => _EID_AL_FITR_HOLIDAY__ESTIMATED_,
    20209_i32 => _LABOR_DAY,
    20244_i32 => _ARAFAT_DAY__ESTIMATED_,
    20245_i32 => _EID_AL_ADHA__ESTIMATED_,
    20246_i32 => _EID_AL_ADHA_HOLIDAY__ESTIMATED_,
    20265_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    20266_i32 => _INDEPENDENCE_DAY,
    20267_i32 => _INDEPENDENCE_DAY_HOLIDAY,
    20335_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    20447_i32 => _CHRISTMAS_DAY,
    20454_i32 => _NEW_YEAR_S_DAY,
    20469_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    20532_i32 => _EID_AL_FITR__ESTIMATED_,
    20533_i32 => _EID_AL_FITR_HOLIDAY__ESTIMATED_,
    20574_i32 => _LABOR_DAY,
    20599_i32 => _ARAFAT_DAY__ESTIMATED_,
    20600_i32 => _EID_AL_ADHA__ESTIMATED_,
    20601_i32 => _EID_AL_ADHA_HOLIDAY__ESTIMATED_,
    20620_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    20631_i32 => _INDEPENDENCE_DAY,
    20632_i32 => _INDEPENDENCE_DAY_HOLIDAY,
    20690_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    20812_i32 => _CHRISTMAS_DAY,
    20819_i32 => _NEW_YEAR_S_DAY,
    20823_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    20886_i32 => _EID_AL_FITR__ESTIMATED_,
    20887_i32 => _EID_AL_FITR_HOLIDAY__ESTIMATED_,
    20939_i32 => _LABOR_DAY,
    20953_i32 => _ARAFAT_DAY__ESTIMATED_,
    20954_i32 => _EID_AL_ADHA__ESTIMATED_,
    20955_i32 => _EID_AL_ADHA_HOLIDAY__ESTIMATED_,
    20975_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    20996_i32 => _INDEPENDENCE_DAY,
    20997_i32 => _INDEPENDENCE_DAY_HOLIDAY,
    21044_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    21177_i32 => _CHRISTMAS_DAY__ISRA__AND_MI_RAJ__ESTIMATED_,
    21184_i32 => _NEW_YEAR_S_DAY,
    21240_i32 => _EID_AL_FITR__ESTIMATED_,
    21241_i32 => _EID_AL_FITR_HOLIDAY__ESTIMATED_,
    21305_i32 => _LABOR_DAY,
    21308_i32 => _ARAFAT_DAY__ESTIMATED_,
    21309_i32 => _EID_AL_ADHA__ESTIMATED_,
    21310_i32 => _EID_AL_ADHA_HOLIDAY__ESTIMATED_,
    21329_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    21362_i32 => _INDEPENDENCE_DAY,
    21363_i32 => _INDEPENDENCE_DAY_HOLIDAY,
    21399_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    21532_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    21543_i32 => _CHRISTMAS_DAY,
    21550_i32 => _NEW_YEAR_S_DAY,
    21594_i32 => _EID_AL_FITR__ESTIMATED_,
    21595_i32 => _EID_AL_FITR_HOLIDAY__ESTIMATED_,
    21662_i32 => _ARAFAT_DAY__ESTIMATED_,
    21663_i32 => _EID_AL_ADHA__ESTIMATED_,
    21664_i32 => _EID_AL_ADHA_HOLIDAY__ESTIMATED_,
    21670_i32 => _LABOR_DAY,
    21683_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    21727_i32 => _INDEPENDENCE_DAY,
    21728_i32 => _INDEPENDENCE_DAY_HOLIDAY,
    21754_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    21886_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    21908_i32 => _CHRISTMAS_DAY,
    21915_i32 => _NEW_YEAR_S_DAY,
    21949_i32 => _EID_AL_FITR__ESTIMATED_,
    21950_i32 => _EID_AL_FITR_HOLIDAY__ESTIMATED_,
    22016_i32 => _ARAFAT_DAY__ESTIMATED_,
    22017_i32 => _EID_AL_ADHA__ESTIMATED_,
    22018_i32 => _EID_AL_ADHA_HOLIDAY__ESTIMATED_,
    22035_i32 => _LABOR_DAY,
    22037_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    22092_i32 => _INDEPENDENCE_DAY,
    22093_i32 => _INDEPENDENCE_DAY_HOLIDAY,
    22108_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    22241_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    22273_i32 => _CHRISTMAS_DAY,
    22280_i32 => _NEW_YEAR_S_DAY,
    22303_i32 => _EID_AL_FITR__ESTIMATED_,
    22304_i32 => _EID_AL_FITR_HOLIDAY__ESTIMATED_,
    22370_i32 => _ARAFAT_DAY__ESTIMATED_,
    22371_i32 => _EID_AL_ADHA__ESTIMATED_,
    22372_i32 => _EID_AL_ADHA_HOLIDAY__ESTIMATED_,
    22392_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    22400_i32 => _LABOR_DAY,
    22457_i32 => _INDEPENDENCE_DAY,
    22458_i32 => _INDEPENDENCE_DAY_HOLIDAY,
    22462_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    22595_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    22638_i32 => _CHRISTMAS_DAY,
    22645_i32 => _NEW_YEAR_S_DAY,
    22658_i32 => _EID_AL_FITR__ESTIMATED_,
    22659_i32 => _EID_AL_FITR_HOLIDAY__ESTIMATED_,
    22725_i32 => _ARAFAT_DAY__ESTIMATED_,
    22726_i32 => _EID_AL_ADHA__ESTIMATED_,
    22727_i32 => _EID_AL_ADHA_HOLIDAY__ESTIMATED_,
    22746_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    22766_i32 => _LABOR_DAY,
    22816_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    22823_i32 => _INDEPENDENCE_DAY,
    22824_i32 => _INDEPENDENCE_DAY_HOLIDAY,
    22950_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    23004_i32 => _CHRISTMAS_DAY,
    23011_i32 => _NEW_YEAR_S_DAY,
    23012_i32 => _EID_AL_FITR__ESTIMATED_,
    23013_i32 => _EID_AL_FITR_HOLIDAY__ESTIMATED_,
    23079_i32 => _ARAFAT_DAY__ESTIMATED_,
    23080_i32 => _EID_AL_ADHA__ESTIMATED_,
    23081_i32 => _EID_AL_ADHA_HOLIDAY__ESTIMATED_,
    23101_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    23131_i32 => _LABOR_DAY,
    23170_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    23188_i32 => _INDEPENDENCE_DAY,
    23189_i32 => _INDEPENDENCE_DAY_HOLIDAY,
    23304_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    23367_i32 => _EID_AL_FITR__ESTIMATED_,
    23368_i32 => _EID_AL_FITR_HOLIDAY__ESTIMATED_,
    23369_i32 => _CHRISTMAS_DAY,
    23376_i32 => _NEW_YEAR_S_DAY,
    23434_i32 => _ARAFAT_DAY__ESTIMATED_,
    23435_i32 => _EID_AL_ADHA__ESTIMATED_,
    23436_i32 => _EID_AL_ADHA_HOLIDAY__ESTIMATED_,
    23455_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    23496_i32 => _LABOR_DAY,
    23525_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    23553_i32 => _INDEPENDENCE_DAY,
    23554_i32 => _INDEPENDENCE_DAY_HOLIDAY,
    23658_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    23721_i32 => _EID_AL_FITR__ESTIMATED_,
    23722_i32 => _EID_AL_FITR_HOLIDAY__ESTIMATED_,
    23734_i32 => _CHRISTMAS_DAY,
    23741_i32 => _NEW_YEAR_S_DAY,
    23788_i32 => _ARAFAT_DAY__ESTIMATED_,
    23789_i32 => _EID_AL_ADHA__ESTIMATED_,
    23790_i32 => _EID_AL_ADHA_HOLIDAY__ESTIMATED_,
    23810_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    23861_i32 => _LABOR_DAY,
    23880_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    23918_i32 => _INDEPENDENCE_DAY,
    23919_i32 => _INDEPENDENCE_DAY_HOLIDAY,
    24012_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    24075_i32 => _EID_AL_FITR__ESTIMATED_,
    24076_i32 => _EID_AL_FITR_HOLIDAY__ESTIMATED_,
    24099_i32 => _CHRISTMAS_DAY,
    24106_i32 => _NEW_YEAR_S_DAY,
    24142_i32 => _ARAFAT_DAY__ESTIMATED_,
    24143_i32 => _EID_AL_ADHA__ESTIMATED_,
    24144_i32 => _EID_AL_ADHA_HOLIDAY__ESTIMATED_,
    24164_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    24227_i32 => _LABOR_DAY,
    24234_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    24284_i32 => _INDEPENDENCE_DAY,
    24285_i32 => _INDEPENDENCE_DAY_HOLIDAY,
    24367_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    24429_i32 => _EID_AL_FITR__ESTIMATED_,
    24430_i32 => _EID_AL_FITR_HOLIDAY__ESTIMATED_,
    24465_i32 => _CHRISTMAS_DAY,
    24472_i32 => _NEW_YEAR_S_DAY,
    24496_i32 => _ARAFAT_DAY__ESTIMATED_,
    24497_i32 => _EID_AL_ADHA__ESTIMATED_,
    24498_i32 => _EID_AL_ADHA_HOLIDAY__ESTIMATED_,
    24518_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    24589_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    24592_i32 => _LABOR_DAY,
    24649_i32 => _INDEPENDENCE_DAY,
    24650_i32 => _INDEPENDENCE_DAY_HOLIDAY,
    24721_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    24783_i32 => _EID_AL_FITR__ESTIMATED_,
    24784_i32 => _EID_AL_FITR_HOLIDAY__ESTIMATED_,
    24830_i32 => _CHRISTMAS_DAY,
    24837_i32 => _NEW_YEAR_S_DAY,
    24851_i32 => _ARAFAT_DAY__ESTIMATED_,
    24852_i32 => _EID_AL_ADHA__ESTIMATED_,
    24853_i32 => _EID_AL_ADHA_HOLIDAY__ESTIMATED_,
    24872_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    24943_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    24957_i32 => _LABOR_DAY,
    25014_i32 => _INDEPENDENCE_DAY,
    25015_i32 => _INDEPENDENCE_DAY_HOLIDAY,
    25076_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    25138_i32 => _EID_AL_FITR__ESTIMATED_,
    25139_i32 => _EID_AL_FITR_HOLIDAY__ESTIMATED_,
    25195_i32 => _CHRISTMAS_DAY,
    25202_i32 => _NEW_YEAR_S_DAY,
    25205_i32 => _ARAFAT_DAY__ESTIMATED_,
    25206_i32 => _EID_AL_ADHA__ESTIMATED_,
    25207_i32 => _EID_AL_ADHA_HOLIDAY__ESTIMATED_,
    25227_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    25297_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    25322_i32 => _LABOR_DAY,
    25379_i32 => _INDEPENDENCE_DAY,
    25380_i32 => _INDEPENDENCE_DAY_HOLIDAY,
    25430_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    25493_i32 => _EID_AL_FITR__ESTIMATED_,
    25494_i32 => _EID_AL_FITR_HOLIDAY__ESTIMATED_,
    25560_i32 => _ARAFAT_DAY__ESTIMATED___CHRISTMAS_DAY,
    25561_i32 => _EID_AL_ADHA__ESTIMATED_,
    25562_i32 => _EID_AL_ADHA_HOLIDAY__ESTIMATED_,
    25567_i32 => _NEW_YEAR_S_DAY,
    25581_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    25651_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    25688_i32 => _LABOR_DAY,
    25745_i32 => _INDEPENDENCE_DAY,
    25746_i32 => _INDEPENDENCE_DAY_HOLIDAY,
    25784_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    25847_i32 => _EID_AL_FITR__ESTIMATED_,
    25848_i32 => _EID_AL_FITR_HOLIDAY__ESTIMATED_,
    25914_i32 => _ARAFAT_DAY__ESTIMATED_,
    25915_i32 => _EID_AL_ADHA__ESTIMATED_,
    25916_i32 => _EID_AL_ADHA_HOLIDAY__ESTIMATED_,
    25926_i32 => _CHRISTMAS_DAY,
    25933_i32 => _NEW_YEAR_S_DAY,
    25936_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    26006_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    26053_i32 => _LABOR_DAY,
    26110_i32 => _INDEPENDENCE_DAY,
    26111_i32 => _INDEPENDENCE_DAY_HOLIDAY,
    26138_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    26201_i32 => _EID_AL_FITR__ESTIMATED_,
    26202_i32 => _EID_AL_FITR_HOLIDAY__ESTIMATED_,
    26269_i32 => _ARAFAT_DAY__ESTIMATED_,
    26270_i32 => _EID_AL_ADHA__ESTIMATED_,
    26271_i32 => _EID_AL_ADHA_HOLIDAY__ESTIMATED_,
    26290_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    26291_i32 => _CHRISTMAS_DAY,
    26298_i32 => _NEW_YEAR_S_DAY,
    26360_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    26418_i32 => _LABOR_DAY,
    26475_i32 => _INDEPENDENCE_DAY,
    26476_i32 => _INDEPENDENCE_DAY_HOLIDAY,
    26493_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    26555_i32 => _EID_AL_FITR__ESTIMATED_,
    26556_i32 => _EID_AL_FITR_HOLIDAY__ESTIMATED_,
    26623_i32 => _ARAFAT_DAY__ESTIMATED_,
    26624_i32 => _EID_AL_ADHA__ESTIMATED_,
    26625_i32 => _EID_AL_ADHA_HOLIDAY__ESTIMATED_,
    26645_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    26656_i32 => _CHRISTMAS_DAY,
    26663_i32 => _NEW_YEAR_S_DAY,
    26715_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    26783_i32 => _LABOR_DAY,
    26840_i32 => _INDEPENDENCE_DAY,
    26841_i32 => _INDEPENDENCE_DAY_HOLIDAY,
    26847_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    26909_i32 => _EID_AL_FITR__ESTIMATED_,
    26910_i32 => _EID_AL_FITR_HOLIDAY__ESTIMATED_,
    26977_i32 => _ARAFAT_DAY__ESTIMATED_,
    26978_i32 => _EID_AL_ADHA__ESTIMATED_,
    26979_i32 => _EID_AL_ADHA_HOLIDAY__ESTIMATED_,
    26999_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    27021_i32 => _CHRISTMAS_DAY,
    27028_i32 => _NEW_YEAR_S_DAY,
    27069_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    27149_i32 => _LABOR_DAY,
    27202_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    27206_i32 => _INDEPENDENCE_DAY,
    27207_i32 => _INDEPENDENCE_DAY_HOLIDAY,
    27264_i32 => _EID_AL_FITR__ESTIMATED_,
    27265_i32 => _EID_AL_FITR_HOLIDAY__ESTIMATED_,
    27331_i32 => _ARAFAT_DAY__ESTIMATED_,
    27332_i32 => _EID_AL_ADHA__ESTIMATED_,
    27333_i32 => _EID_AL_ADHA_HOLIDAY__ESTIMATED_,
    27353_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    27387_i32 => _CHRISTMAS_DAY,
    27394_i32 => _NEW_YEAR_S_DAY,
    27423_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    27514_i32 => _LABOR_DAY,
    27557_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    27571_i32 => _INDEPENDENCE_DAY,
    27572_i32 => _INDEPENDENCE_DAY_HOLIDAY,
    27619_i32 => _EID_AL_FITR__ESTIMATED_,
    27620_i32 => _EID_AL_FITR_HOLIDAY__ESTIMATED_,
    27686_i32 => _ARAFAT_DAY__ESTIMATED_,
    27687_i32 => _EID_AL_ADHA__ESTIMATED_,
    27688_i32 => _EID_AL_ADHA_HOLIDAY__ESTIMATED_,
    27707_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    27752_i32 => _CHRISTMAS_DAY,
    27759_i32 => _NEW_YEAR_S_DAY,
    27777_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    27879_i32 => _LABOR_DAY,
    27911_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    27936_i32 => _INDEPENDENCE_DAY,
    27937_i32 => _INDEPENDENCE_DAY_HOLIDAY,
    27973_i32 => _EID_AL_FITR__ESTIMATED_,
    27974_i32 => _EID_AL_FITR_HOLIDAY__ESTIMATED_,
    28040_i32 => _ARAFAT_DAY__ESTIMATED_,
    28041_i32 => _EID_AL_ADHA__ESTIMATED_,
    28042_i32 => _EID_AL_ADHA_HOLIDAY__ESTIMATED_,
    28062_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    28117_i32 => _CHRISTMAS_DAY,
    28124_i32 => _NEW_YEAR_S_DAY,
    28131_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    28244_i32 => _LABOR_DAY,
    28265_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    28301_i32 => _INDEPENDENCE_DAY,
    28302_i32 => _INDEPENDENCE_DAY_HOLIDAY,
    28328_i32 => _EID_AL_FITR__ESTIMATED_,
    28329_i32 => _EID_AL_FITR_HOLIDAY__ESTIMATED_,
    28395_i32 => _ARAFAT_DAY__ESTIMATED_,
    28396_i32 => _EID_AL_ADHA__ESTIMATED_,
    28397_i32 => _EID_AL_ADHA_HOLIDAY__ESTIMATED_,
    28416_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    28482_i32 => _CHRISTMAS_DAY,
    28486_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    28489_i32 => _NEW_YEAR_S_DAY,
    28610_i32 => _LABOR_DAY,
    28619_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    28667_i32 => _INDEPENDENCE_DAY,
    28668_i32 => _INDEPENDENCE_DAY_HOLIDAY,
    28682_i32 => _EID_AL_FITR__ESTIMATED_,
    28683_i32 => _EID_AL_FITR_HOLIDAY__ESTIMATED_,
    28750_i32 => _ARAFAT_DAY__ESTIMATED_,
    28751_i32 => _EID_AL_ADHA__ESTIMATED_,
    28752_i32 => _EID_AL_ADHA_HOLIDAY__ESTIMATED_,
    28771_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    28841_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    28848_i32 => _CHRISTMAS_DAY,
    28855_i32 => _NEW_YEAR_S_DAY,
    28973_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    28975_i32 => _LABOR_DAY,
    29032_i32 => _INDEPENDENCE_DAY,
    29033_i32 => _INDEPENDENCE_DAY_HOLIDAY,
    29036_i32 => _EID_AL_FITR__ESTIMATED_,
    29037_i32 => _EID_AL_FITR_HOLIDAY__ESTIMATED_,
    29104_i32 => _ARAFAT_DAY__ESTIMATED_,
    29105_i32 => _EID_AL_ADHA__ESTIMATED_,
    29106_i32 => _EID_AL_ADHA_HOLIDAY__ESTIMATED_,
    29125_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    29195_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    29213_i32 => _CHRISTMAS_DAY,
    29220_i32 => _NEW_YEAR_S_DAY,
};
