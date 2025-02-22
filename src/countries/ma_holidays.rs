use crate::countries::constants::*;
use phf::phf_map;

pub static MA_HOLIDAYS: phf::Map<i32, &'static str> = phf_map! {
    10957_i32 => _NEW_YEAR_S_DAY,
    10964_i32 => _EID_AL_FITR__ESTIMATED_,
    10965_i32 => _EID_AL_FITR__ESTIMATED_,
    10967_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    11019_i32 => _THRONE_DAY,
    11032_i32 => _EID_AL_ADHA__ESTIMATED_,
    11033_i32 => _EID_AL_ADHA__ESTIMATED_,
    11053_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    11078_i32 => _LABOR_DAY,
    11122_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    11123_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    11147_i32 => _YOUTH_DAY,
    11183_i32 => _OUED_ED_DAHAB_DAY,
    11189_i32 => _REVOLUTION_DAY,
    11267_i32 => _GREEN_MARCH,
    11279_i32 => _INDEPENDENCE_DAY,
    11318_i32 => _EID_AL_FITR__ESTIMATED_,
    11319_i32 => _EID_AL_FITR__ESTIMATED_,
    11323_i32 => _NEW_YEAR_S_DAY,
    11333_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    11386_i32 => _EID_AL_ADHA__ESTIMATED_,
    11387_i32 => _EID_AL_ADHA__ESTIMATED_,
    11407_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    11443_i32 => _LABOR_DAY,
    11477_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    11478_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    11533_i32 => _THRONE_DAY,
    11548_i32 => _OUED_ED_DAHAB_DAY,
    11554_i32 => _REVOLUTION_DAY,
    11555_i32 => _YOUTH_DAY,
    11632_i32 => _GREEN_MARCH,
    11644_i32 => _INDEPENDENCE_DAY,
    11672_i32 => _EID_AL_FITR__ESTIMATED_,
    11673_i32 => _EID_AL_FITR__ESTIMATED_,
    11688_i32 => _NEW_YEAR_S_DAY,
    11698_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    11740_i32 => _EID_AL_ADHA__ESTIMATED_,
    11741_i32 => _EID_AL_ADHA__ESTIMATED_,
    11761_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    11808_i32 => _LABOR_DAY,
    11831_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    11832_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    11898_i32 => _THRONE_DAY,
    11913_i32 => _OUED_ED_DAHAB_DAY,
    11919_i32 => _REVOLUTION_DAY,
    11920_i32 => _YOUTH_DAY,
    11997_i32 => _GREEN_MARCH,
    12009_i32 => _INDEPENDENCE_DAY,
    12026_i32 => _EID_AL_FITR__ESTIMATED_,
    12027_i32 => _EID_AL_FITR__ESTIMATED_,
    12053_i32 => _NEW_YEAR_S_DAY,
    12063_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    12094_i32 => _EID_AL_ADHA__ESTIMATED_,
    12095_i32 => _EID_AL_ADHA__ESTIMATED_,
    12115_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    12173_i32 => _LABOR_DAY,
    12185_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    12186_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    12263_i32 => _THRONE_DAY,
    12278_i32 => _OUED_ED_DAHAB_DAY,
    12284_i32 => _REVOLUTION_DAY,
    12285_i32 => _YOUTH_DAY,
    12362_i32 => _GREEN_MARCH,
    12374_i32 => _INDEPENDENCE_DAY,
    12381_i32 => _EID_AL_FITR__ESTIMATED_,
    12382_i32 => _EID_AL_FITR__ESTIMATED_,
    12418_i32 => _NEW_YEAR_S_DAY,
    12428_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    12449_i32 => _EID_AL_ADHA__ESTIMATED_,
    12450_i32 => _EID_AL_ADHA__ESTIMATED_,
    12469_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    12539_i32 => _LABOR_DAY__PROPHET_S_BIRTHDAY__ESTIMATED_,
    12540_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    12629_i32 => _THRONE_DAY,
    12644_i32 => _OUED_ED_DAHAB_DAY,
    12650_i32 => _REVOLUTION_DAY,
    12651_i32 => _YOUTH_DAY,
    12728_i32 => _GREEN_MARCH,
    12736_i32 => _EID_AL_FITR__ESTIMATED_,
    12737_i32 => _EID_AL_FITR__ESTIMATED_,
    12740_i32 => _INDEPENDENCE_DAY,
    12784_i32 => _NEW_YEAR_S_DAY,
    12794_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    12804_i32 => _EID_AL_ADHA__ESTIMATED_,
    12805_i32 => _EID_AL_ADHA__ESTIMATED_,
    12824_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    12894_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    12895_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    12904_i32 => _LABOR_DAY,
    12994_i32 => _THRONE_DAY,
    13009_i32 => _OUED_ED_DAHAB_DAY,
    13015_i32 => _REVOLUTION_DAY,
    13016_i32 => _YOUTH_DAY,
    13090_i32 => _EID_AL_FITR__ESTIMATED_,
    13091_i32 => _EID_AL_FITR__ESTIMATED_,
    13093_i32 => _GREEN_MARCH,
    13105_i32 => _INDEPENDENCE_DAY,
    13149_i32 => _NEW_YEAR_S_DAY,
    13158_i32 => _EID_AL_ADHA__ESTIMATED_,
    13159_i32 => _EID_AL_ADHA__ESTIMATED___PROCLAMATION_OF_INDEPENDENCE_DAY,
    13179_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    13248_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    13249_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    13269_i32 => _LABOR_DAY,
    13359_i32 => _THRONE_DAY,
    13374_i32 => _OUED_ED_DAHAB_DAY,
    13380_i32 => _REVOLUTION_DAY,
    13381_i32 => _YOUTH_DAY,
    13444_i32 => _EID_AL_FITR__ESTIMATED_,
    13445_i32 => _EID_AL_FITR__ESTIMATED_,
    13458_i32 => _GREEN_MARCH,
    13470_i32 => _INDEPENDENCE_DAY,
    13513_i32 => _EID_AL_ADHA__ESTIMATED_,
    13514_i32 => _EID_AL_ADHA__ESTIMATED___NEW_YEAR_S_DAY,
    13524_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    13533_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    13603_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    13604_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    13634_i32 => _LABOR_DAY,
    13724_i32 => _THRONE_DAY,
    13739_i32 => _OUED_ED_DAHAB_DAY,
    13745_i32 => _REVOLUTION_DAY,
    13746_i32 => _YOUTH_DAY,
    13799_i32 => _EID_AL_FITR__ESTIMATED_,
    13800_i32 => _EID_AL_FITR__ESTIMATED_,
    13823_i32 => _GREEN_MARCH,
    13835_i32 => _INDEPENDENCE_DAY,
    13867_i32 => _EID_AL_ADHA__ESTIMATED_,
    13868_i32 => _EID_AL_ADHA__ESTIMATED_,
    13879_i32 => _NEW_YEAR_S_DAY,
    13888_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    13889_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    13958_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    13959_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    14000_i32 => _LABOR_DAY,
    14090_i32 => _THRONE_DAY,
    14105_i32 => _OUED_ED_DAHAB_DAY,
    14111_i32 => _REVOLUTION_DAY,
    14112_i32 => _YOUTH_DAY,
    14153_i32 => _EID_AL_FITR__ESTIMATED_,
    14154_i32 => _EID_AL_FITR__ESTIMATED_,
    14189_i32 => _GREEN_MARCH,
    14201_i32 => _INDEPENDENCE_DAY,
    14221_i32 => _EID_AL_ADHA__ESTIMATED_,
    14222_i32 => _EID_AL_ADHA__ESTIMATED_,
    14242_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    14245_i32 => _NEW_YEAR_S_DAY,
    14255_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    14312_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    14313_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    14365_i32 => _LABOR_DAY,
    14455_i32 => _THRONE_DAY,
    14470_i32 => _OUED_ED_DAHAB_DAY,
    14476_i32 => _REVOLUTION_DAY,
    14477_i32 => _YOUTH_DAY,
    14507_i32 => _EID_AL_FITR__ESTIMATED_,
    14508_i32 => _EID_AL_FITR__ESTIMATED_,
    14554_i32 => _GREEN_MARCH,
    14566_i32 => _INDEPENDENCE_DAY,
    14575_i32 => _EID_AL_ADHA__ESTIMATED_,
    14576_i32 => _EID_AL_ADHA__ESTIMATED_,
    14596_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    14610_i32 => _NEW_YEAR_S_DAY,
    14620_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    14666_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    14667_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    14730_i32 => _LABOR_DAY,
    14820_i32 => _THRONE_DAY,
    14835_i32 => _OUED_ED_DAHAB_DAY,
    14841_i32 => _REVOLUTION_DAY,
    14842_i32 => _YOUTH_DAY,
    14862_i32 => _EID_AL_FITR__ESTIMATED_,
    14863_i32 => _EID_AL_FITR__ESTIMATED_,
    14919_i32 => _GREEN_MARCH,
    14929_i32 => _EID_AL_ADHA__ESTIMATED_,
    14930_i32 => _EID_AL_ADHA__ESTIMATED_,
    14931_i32 => _INDEPENDENCE_DAY,
    14950_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    14975_i32 => _NEW_YEAR_S_DAY,
    14985_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    15020_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    15021_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    15095_i32 => _LABOR_DAY,
    15185_i32 => _THRONE_DAY,
    15200_i32 => _OUED_ED_DAHAB_DAY,
    15206_i32 => _REVOLUTION_DAY,
    15207_i32 => _YOUTH_DAY,
    15216_i32 => _EID_AL_FITR__ESTIMATED_,
    15217_i32 => _EID_AL_FITR__ESTIMATED_,
    15284_i32 => _EID_AL_ADHA__ESTIMATED___GREEN_MARCH,
    15285_i32 => _EID_AL_ADHA__ESTIMATED_,
    15296_i32 => _INDEPENDENCE_DAY,
    15304_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    15340_i32 => _NEW_YEAR_S_DAY,
    15350_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    15374_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    15375_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    15461_i32 => _LABOR_DAY,
    15551_i32 => _THRONE_DAY,
    15566_i32 => _OUED_ED_DAHAB_DAY,
    15571_i32 => _EID_AL_FITR__ESTIMATED_,
    15572_i32 => _EID_AL_FITR__ESTIMATED___REVOLUTION_DAY,
    15573_i32 => _YOUTH_DAY,
    15639_i32 => _EID_AL_ADHA__ESTIMATED_,
    15640_i32 => _EID_AL_ADHA__ESTIMATED_,
    15650_i32 => _GREEN_MARCH,
    15659_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    15662_i32 => _INDEPENDENCE_DAY,
    15706_i32 => _NEW_YEAR_S_DAY,
    15716_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    15729_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    15730_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    15826_i32 => _LABOR_DAY,
    15916_i32 => _THRONE_DAY,
    15925_i32 => _EID_AL_FITR__ESTIMATED_,
    15926_i32 => _EID_AL_FITR__ESTIMATED_,
    15931_i32 => _OUED_ED_DAHAB_DAY,
    15937_i32 => _REVOLUTION_DAY,
    15938_i32 => _YOUTH_DAY,
    15993_i32 => _EID_AL_ADHA__ESTIMATED_,
    15994_i32 => _EID_AL_ADHA__ESTIMATED_,
    16013_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    16015_i32 => _GREEN_MARCH,
    16027_i32 => _INDEPENDENCE_DAY,
    16071_i32 => _NEW_YEAR_S_DAY,
    16081_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    16083_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    16084_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    16191_i32 => _LABOR_DAY,
    16279_i32 => _EID_AL_FITR__ESTIMATED_,
    16280_i32 => _EID_AL_FITR__ESTIMATED_,
    16281_i32 => _THRONE_DAY,
    16296_i32 => _OUED_ED_DAHAB_DAY,
    16302_i32 => _REVOLUTION_DAY,
    16303_i32 => _YOUTH_DAY,
    16347_i32 => _EID_AL_ADHA__ESTIMATED_,
    16348_i32 => _EID_AL_ADHA__ESTIMATED_,
    16368_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    16380_i32 => _GREEN_MARCH,
    16392_i32 => _INDEPENDENCE_DAY,
    16436_i32 => _NEW_YEAR_S_DAY,
    16438_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    16439_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    16446_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    16556_i32 => _LABOR_DAY,
    16633_i32 => _EID_AL_FITR__ESTIMATED_,
    16634_i32 => _EID_AL_FITR__ESTIMATED_,
    16646_i32 => _THRONE_DAY,
    16661_i32 => _OUED_ED_DAHAB_DAY,
    16667_i32 => _REVOLUTION_DAY,
    16668_i32 => _YOUTH_DAY,
    16701_i32 => _EID_AL_ADHA__ESTIMATED_,
    16702_i32 => _EID_AL_ADHA__ESTIMATED_,
    16722_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    16745_i32 => _GREEN_MARCH,
    16757_i32 => _INDEPENDENCE_DAY,
    16792_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    16793_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    16801_i32 => _NEW_YEAR_S_DAY,
    16811_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    16922_i32 => _LABOR_DAY,
    16988_i32 => _EID_AL_FITR__ESTIMATED_,
    16989_i32 => _EID_AL_FITR__ESTIMATED_,
    17012_i32 => _THRONE_DAY,
    17027_i32 => _OUED_ED_DAHAB_DAY,
    17033_i32 => _REVOLUTION_DAY,
    17034_i32 => _YOUTH_DAY,
    17055_i32 => _EID_AL_ADHA__ESTIMATED_,
    17056_i32 => _EID_AL_ADHA__ESTIMATED_,
    17076_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    17111_i32 => _GREEN_MARCH,
    17123_i32 => _INDEPENDENCE_DAY,
    17146_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    17147_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    17167_i32 => _NEW_YEAR_S_DAY,
    17177_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    17287_i32 => _LABOR_DAY,
    17342_i32 => _EID_AL_FITR__ESTIMATED_,
    17343_i32 => _EID_AL_FITR__ESTIMATED_,
    17377_i32 => _THRONE_DAY,
    17392_i32 => _OUED_ED_DAHAB_DAY,
    17398_i32 => _REVOLUTION_DAY,
    17399_i32 => _YOUTH_DAY,
    17410_i32 => _EID_AL_ADHA__ESTIMATED_,
    17411_i32 => _EID_AL_ADHA__ESTIMATED_,
    17430_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    17476_i32 => _GREEN_MARCH,
    17488_i32 => _INDEPENDENCE_DAY,
    17500_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    17501_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    17532_i32 => _NEW_YEAR_S_DAY,
    17542_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    17652_i32 => _LABOR_DAY,
    17697_i32 => _EID_AL_FITR__ESTIMATED_,
    17698_i32 => _EID_AL_FITR__ESTIMATED_,
    17742_i32 => _THRONE_DAY,
    17757_i32 => _OUED_ED_DAHAB_DAY,
    17763_i32 => _REVOLUTION_DAY,
    17764_i32 => _EID_AL_ADHA__ESTIMATED___YOUTH_DAY,
    17765_i32 => _EID_AL_ADHA__ESTIMATED_,
    17785_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    17841_i32 => _GREEN_MARCH,
    17853_i32 => _INDEPENDENCE_DAY,
    17855_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    17856_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    17897_i32 => _NEW_YEAR_S_DAY,
    17907_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    18017_i32 => _LABOR_DAY,
    18051_i32 => _EID_AL_FITR__ESTIMATED_,
    18052_i32 => _EID_AL_FITR__ESTIMATED_,
    18107_i32 => _THRONE_DAY,
    18119_i32 => _EID_AL_ADHA__ESTIMATED_,
    18120_i32 => _EID_AL_ADHA__ESTIMATED_,
    18122_i32 => _OUED_ED_DAHAB_DAY,
    18128_i32 => _REVOLUTION_DAY,
    18129_i32 => _YOUTH_DAY,
    18139_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    18206_i32 => _GREEN_MARCH,
    18209_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    18210_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    18218_i32 => _INDEPENDENCE_DAY,
    18262_i32 => _NEW_YEAR_S_DAY,
    18272_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    18383_i32 => _LABOR_DAY,
    18406_i32 => _EID_AL_FITR__ESTIMATED_,
    18407_i32 => _EID_AL_FITR__ESTIMATED_,
    18473_i32 => _THRONE_DAY,
    18474_i32 => _EID_AL_ADHA__ESTIMATED_,
    18475_i32 => _EID_AL_ADHA__ESTIMATED_,
    18488_i32 => _OUED_ED_DAHAB_DAY,
    18494_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED___REVOLUTION_DAY,
    18495_i32 => _YOUTH_DAY,
    18564_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    18565_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    18572_i32 => _GREEN_MARCH,
    18584_i32 => _INDEPENDENCE_DAY,
    18628_i32 => _NEW_YEAR_S_DAY,
    18638_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    18748_i32 => _LABOR_DAY,
    18760_i32 => _EID_AL_FITR__ESTIMATED_,
    18761_i32 => _EID_AL_FITR__ESTIMATED_,
    18828_i32 => _EID_AL_ADHA__ESTIMATED_,
    18829_i32 => _EID_AL_ADHA__ESTIMATED_,
    18838_i32 => _THRONE_DAY,
    18848_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    18853_i32 => _OUED_ED_DAHAB_DAY,
    18859_i32 => _REVOLUTION_DAY,
    18860_i32 => _YOUTH_DAY,
    18918_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    18919_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    18937_i32 => _GREEN_MARCH,
    18949_i32 => _INDEPENDENCE_DAY,
    18993_i32 => _NEW_YEAR_S_DAY,
    19003_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    19113_i32 => _LABOR_DAY,
    19114_i32 => _EID_AL_FITR__ESTIMATED_,
    19115_i32 => _EID_AL_FITR__ESTIMATED_,
    19182_i32 => _EID_AL_ADHA__ESTIMATED_,
    19183_i32 => _EID_AL_ADHA__ESTIMATED_,
    19203_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED___THRONE_DAY,
    19218_i32 => _OUED_ED_DAHAB_DAY,
    19224_i32 => _REVOLUTION_DAY,
    19225_i32 => _YOUTH_DAY,
    19273_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    19274_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    19302_i32 => _GREEN_MARCH,
    19314_i32 => _INDEPENDENCE_DAY,
    19358_i32 => _NEW_YEAR_S_DAY,
    19368_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    19468_i32 => _EID_AL_FITR__ESTIMATED_,
    19469_i32 => _EID_AL_FITR__ESTIMATED_,
    19478_i32 => _LABOR_DAY,
    19536_i32 => _EID_AL_ADHA__ESTIMATED_,
    19537_i32 => _EID_AL_ADHA__ESTIMATED_,
    19557_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    19568_i32 => _THRONE_DAY,
    19583_i32 => _OUED_ED_DAHAB_DAY,
    19589_i32 => _REVOLUTION_DAY,
    19590_i32 => _YOUTH_DAY,
    19627_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    19628_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    19667_i32 => _GREEN_MARCH,
    19679_i32 => _INDEPENDENCE_DAY,
    19723_i32 => _NEW_YEAR_S_DAY,
    19733_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    19735_i32 => _AMAZIGH_NEW_YEAR,
    19823_i32 => _EID_AL_FITR__ESTIMATED_,
    19824_i32 => _EID_AL_FITR__ESTIMATED_,
    19844_i32 => _LABOR_DAY,
    19890_i32 => _EID_AL_ADHA__ESTIMATED_,
    19891_i32 => _EID_AL_ADHA__ESTIMATED_,
    19911_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    19934_i32 => _THRONE_DAY,
    19949_i32 => _OUED_ED_DAHAB_DAY,
    19955_i32 => _REVOLUTION_DAY,
    19956_i32 => _YOUTH_DAY,
    19981_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    19982_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    20033_i32 => _GREEN_MARCH,
    20045_i32 => _INDEPENDENCE_DAY,
    20089_i32 => _NEW_YEAR_S_DAY,
    20099_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    20101_i32 => _AMAZIGH_NEW_YEAR,
    20177_i32 => _EID_AL_FITR__ESTIMATED_,
    20178_i32 => _EID_AL_FITR__ESTIMATED_,
    20209_i32 => _LABOR_DAY,
    20245_i32 => _EID_AL_ADHA__ESTIMATED_,
    20246_i32 => _EID_AL_ADHA__ESTIMATED_,
    20265_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    20299_i32 => _THRONE_DAY,
    20314_i32 => _OUED_ED_DAHAB_DAY,
    20320_i32 => _REVOLUTION_DAY,
    20321_i32 => _YOUTH_DAY,
    20335_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    20336_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    20398_i32 => _GREEN_MARCH,
    20410_i32 => _INDEPENDENCE_DAY,
    20454_i32 => _NEW_YEAR_S_DAY,
    20464_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    20466_i32 => _AMAZIGH_NEW_YEAR,
    20532_i32 => _EID_AL_FITR__ESTIMATED_,
    20533_i32 => _EID_AL_FITR__ESTIMATED_,
    20574_i32 => _LABOR_DAY,
    20600_i32 => _EID_AL_ADHA__ESTIMATED_,
    20601_i32 => _EID_AL_ADHA__ESTIMATED_,
    20620_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    20664_i32 => _THRONE_DAY,
    20679_i32 => _OUED_ED_DAHAB_DAY,
    20685_i32 => _REVOLUTION_DAY,
    20686_i32 => _YOUTH_DAY,
    20690_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    20691_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    20763_i32 => _GREEN_MARCH,
    20775_i32 => _INDEPENDENCE_DAY,
    20819_i32 => _NEW_YEAR_S_DAY,
    20829_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    20831_i32 => _AMAZIGH_NEW_YEAR,
    20886_i32 => _EID_AL_FITR__ESTIMATED_,
    20887_i32 => _EID_AL_FITR__ESTIMATED_,
    20939_i32 => _LABOR_DAY,
    20954_i32 => _EID_AL_ADHA__ESTIMATED_,
    20955_i32 => _EID_AL_ADHA__ESTIMATED_,
    20975_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    21029_i32 => _THRONE_DAY,
    21044_i32 => _OUED_ED_DAHAB_DAY__PROPHET_S_BIRTHDAY__ESTIMATED_,
    21045_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    21050_i32 => _REVOLUTION_DAY,
    21051_i32 => _YOUTH_DAY,
    21128_i32 => _GREEN_MARCH,
    21140_i32 => _INDEPENDENCE_DAY,
    21184_i32 => _NEW_YEAR_S_DAY,
    21194_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    21196_i32 => _AMAZIGH_NEW_YEAR,
    21240_i32 => _EID_AL_FITR__ESTIMATED_,
    21241_i32 => _EID_AL_FITR__ESTIMATED_,
    21305_i32 => _LABOR_DAY,
    21309_i32 => _EID_AL_ADHA__ESTIMATED_,
    21310_i32 => _EID_AL_ADHA__ESTIMATED_,
    21329_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    21395_i32 => _THRONE_DAY,
    21399_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    21400_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    21410_i32 => _OUED_ED_DAHAB_DAY,
    21416_i32 => _REVOLUTION_DAY,
    21417_i32 => _YOUTH_DAY,
    21494_i32 => _GREEN_MARCH,
    21506_i32 => _INDEPENDENCE_DAY,
    21550_i32 => _NEW_YEAR_S_DAY,
    21560_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    21562_i32 => _AMAZIGH_NEW_YEAR,
    21594_i32 => _EID_AL_FITR__ESTIMATED_,
    21595_i32 => _EID_AL_FITR__ESTIMATED_,
    21663_i32 => _EID_AL_ADHA__ESTIMATED_,
    21664_i32 => _EID_AL_ADHA__ESTIMATED_,
    21670_i32 => _LABOR_DAY,
    21683_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    21754_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    21755_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    21760_i32 => _THRONE_DAY,
    21775_i32 => _OUED_ED_DAHAB_DAY,
    21781_i32 => _REVOLUTION_DAY,
    21782_i32 => _YOUTH_DAY,
    21859_i32 => _GREEN_MARCH,
    21871_i32 => _INDEPENDENCE_DAY,
    21915_i32 => _NEW_YEAR_S_DAY,
    21925_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    21927_i32 => _AMAZIGH_NEW_YEAR,
    21949_i32 => _EID_AL_FITR__ESTIMATED_,
    21950_i32 => _EID_AL_FITR__ESTIMATED_,
    22017_i32 => _EID_AL_ADHA__ESTIMATED_,
    22018_i32 => _EID_AL_ADHA__ESTIMATED_,
    22035_i32 => _LABOR_DAY,
    22037_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    22108_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    22109_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    22125_i32 => _THRONE_DAY,
    22140_i32 => _OUED_ED_DAHAB_DAY,
    22146_i32 => _REVOLUTION_DAY,
    22147_i32 => _YOUTH_DAY,
    22224_i32 => _GREEN_MARCH,
    22236_i32 => _INDEPENDENCE_DAY,
    22280_i32 => _NEW_YEAR_S_DAY,
    22290_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    22292_i32 => _AMAZIGH_NEW_YEAR,
    22303_i32 => _EID_AL_FITR__ESTIMATED_,
    22304_i32 => _EID_AL_FITR__ESTIMATED_,
    22371_i32 => _EID_AL_ADHA__ESTIMATED_,
    22372_i32 => _EID_AL_ADHA__ESTIMATED_,
    22392_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    22400_i32 => _LABOR_DAY,
    22462_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    22463_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    22490_i32 => _THRONE_DAY,
    22505_i32 => _OUED_ED_DAHAB_DAY,
    22511_i32 => _REVOLUTION_DAY,
    22512_i32 => _YOUTH_DAY,
    22589_i32 => _GREEN_MARCH,
    22601_i32 => _INDEPENDENCE_DAY,
    22645_i32 => _NEW_YEAR_S_DAY,
    22655_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    22657_i32 => _AMAZIGH_NEW_YEAR,
    22658_i32 => _EID_AL_FITR__ESTIMATED_,
    22659_i32 => _EID_AL_FITR__ESTIMATED_,
    22726_i32 => _EID_AL_ADHA__ESTIMATED_,
    22727_i32 => _EID_AL_ADHA__ESTIMATED_,
    22746_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    22766_i32 => _LABOR_DAY,
    22816_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    22817_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    22856_i32 => _THRONE_DAY,
    22871_i32 => _OUED_ED_DAHAB_DAY,
    22877_i32 => _REVOLUTION_DAY,
    22878_i32 => _YOUTH_DAY,
    22955_i32 => _GREEN_MARCH,
    22967_i32 => _INDEPENDENCE_DAY,
    23011_i32 => _NEW_YEAR_S_DAY,
    23012_i32 => _EID_AL_FITR__ESTIMATED_,
    23013_i32 => _EID_AL_FITR__ESTIMATED_,
    23021_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    23023_i32 => _AMAZIGH_NEW_YEAR,
    23080_i32 => _EID_AL_ADHA__ESTIMATED_,
    23081_i32 => _EID_AL_ADHA__ESTIMATED_,
    23101_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    23131_i32 => _LABOR_DAY,
    23170_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    23171_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    23221_i32 => _THRONE_DAY,
    23236_i32 => _OUED_ED_DAHAB_DAY,
    23242_i32 => _REVOLUTION_DAY,
    23243_i32 => _YOUTH_DAY,
    23320_i32 => _GREEN_MARCH,
    23332_i32 => _INDEPENDENCE_DAY,
    23367_i32 => _EID_AL_FITR__ESTIMATED_,
    23368_i32 => _EID_AL_FITR__ESTIMATED_,
    23376_i32 => _NEW_YEAR_S_DAY,
    23386_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    23388_i32 => _AMAZIGH_NEW_YEAR,
    23435_i32 => _EID_AL_ADHA__ESTIMATED_,
    23436_i32 => _EID_AL_ADHA__ESTIMATED_,
    23455_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    23496_i32 => _LABOR_DAY,
    23525_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    23526_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    23586_i32 => _THRONE_DAY,
    23601_i32 => _OUED_ED_DAHAB_DAY,
    23607_i32 => _REVOLUTION_DAY,
    23608_i32 => _YOUTH_DAY,
    23685_i32 => _GREEN_MARCH,
    23697_i32 => _INDEPENDENCE_DAY,
    23721_i32 => _EID_AL_FITR__ESTIMATED_,
    23722_i32 => _EID_AL_FITR__ESTIMATED_,
    23741_i32 => _NEW_YEAR_S_DAY,
    23751_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    23753_i32 => _AMAZIGH_NEW_YEAR,
    23789_i32 => _EID_AL_ADHA__ESTIMATED_,
    23790_i32 => _EID_AL_ADHA__ESTIMATED_,
    23810_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    23861_i32 => _LABOR_DAY,
    23880_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    23881_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    23951_i32 => _THRONE_DAY,
    23966_i32 => _OUED_ED_DAHAB_DAY,
    23972_i32 => _REVOLUTION_DAY,
    23973_i32 => _YOUTH_DAY,
    24050_i32 => _GREEN_MARCH,
    24062_i32 => _INDEPENDENCE_DAY,
    24075_i32 => _EID_AL_FITR__ESTIMATED_,
    24076_i32 => _EID_AL_FITR__ESTIMATED_,
    24106_i32 => _NEW_YEAR_S_DAY,
    24116_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    24118_i32 => _AMAZIGH_NEW_YEAR,
    24143_i32 => _EID_AL_ADHA__ESTIMATED_,
    24144_i32 => _EID_AL_ADHA__ESTIMATED_,
    24164_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    24227_i32 => _LABOR_DAY,
    24234_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    24235_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    24317_i32 => _THRONE_DAY,
    24332_i32 => _OUED_ED_DAHAB_DAY,
    24338_i32 => _REVOLUTION_DAY,
    24339_i32 => _YOUTH_DAY,
    24416_i32 => _GREEN_MARCH,
    24428_i32 => _INDEPENDENCE_DAY,
    24429_i32 => _EID_AL_FITR__ESTIMATED_,
    24430_i32 => _EID_AL_FITR__ESTIMATED_,
    24472_i32 => _NEW_YEAR_S_DAY,
    24482_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    24484_i32 => _AMAZIGH_NEW_YEAR,
    24497_i32 => _EID_AL_ADHA__ESTIMATED_,
    24498_i32 => _EID_AL_ADHA__ESTIMATED_,
    24518_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    24589_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    24590_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    24592_i32 => _LABOR_DAY,
    24682_i32 => _THRONE_DAY,
    24697_i32 => _OUED_ED_DAHAB_DAY,
    24703_i32 => _REVOLUTION_DAY,
    24704_i32 => _YOUTH_DAY,
    24781_i32 => _GREEN_MARCH,
    24783_i32 => _EID_AL_FITR__ESTIMATED_,
    24784_i32 => _EID_AL_FITR__ESTIMATED_,
    24793_i32 => _INDEPENDENCE_DAY,
    24837_i32 => _NEW_YEAR_S_DAY,
    24847_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    24849_i32 => _AMAZIGH_NEW_YEAR,
    24852_i32 => _EID_AL_ADHA__ESTIMATED_,
    24853_i32 => _EID_AL_ADHA__ESTIMATED_,
    24872_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    24943_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    24944_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    24957_i32 => _LABOR_DAY,
    25047_i32 => _THRONE_DAY,
    25062_i32 => _OUED_ED_DAHAB_DAY,
    25068_i32 => _REVOLUTION_DAY,
    25069_i32 => _YOUTH_DAY,
    25138_i32 => _EID_AL_FITR__ESTIMATED_,
    25139_i32 => _EID_AL_FITR__ESTIMATED_,
    25146_i32 => _GREEN_MARCH,
    25158_i32 => _INDEPENDENCE_DAY,
    25202_i32 => _NEW_YEAR_S_DAY,
    25206_i32 => _EID_AL_ADHA__ESTIMATED_,
    25207_i32 => _EID_AL_ADHA__ESTIMATED_,
    25212_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    25214_i32 => _AMAZIGH_NEW_YEAR,
    25227_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    25297_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    25298_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    25322_i32 => _LABOR_DAY,
    25412_i32 => _THRONE_DAY,
    25427_i32 => _OUED_ED_DAHAB_DAY,
    25433_i32 => _REVOLUTION_DAY,
    25434_i32 => _YOUTH_DAY,
    25493_i32 => _EID_AL_FITR__ESTIMATED_,
    25494_i32 => _EID_AL_FITR__ESTIMATED_,
    25511_i32 => _GREEN_MARCH,
    25523_i32 => _INDEPENDENCE_DAY,
    25561_i32 => _EID_AL_ADHA__ESTIMATED_,
    25562_i32 => _EID_AL_ADHA__ESTIMATED_,
    25567_i32 => _NEW_YEAR_S_DAY,
    25577_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    25579_i32 => _AMAZIGH_NEW_YEAR,
    25581_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    25651_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    25652_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    25688_i32 => _LABOR_DAY,
    25778_i32 => _THRONE_DAY,
    25793_i32 => _OUED_ED_DAHAB_DAY,
    25799_i32 => _REVOLUTION_DAY,
    25800_i32 => _YOUTH_DAY,
    25847_i32 => _EID_AL_FITR__ESTIMATED_,
    25848_i32 => _EID_AL_FITR__ESTIMATED_,
    25877_i32 => _GREEN_MARCH,
    25889_i32 => _INDEPENDENCE_DAY,
    25915_i32 => _EID_AL_ADHA__ESTIMATED_,
    25916_i32 => _EID_AL_ADHA__ESTIMATED_,
    25933_i32 => _NEW_YEAR_S_DAY,
    25936_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    25943_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    25945_i32 => _AMAZIGH_NEW_YEAR,
    26006_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    26007_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    26053_i32 => _LABOR_DAY,
    26143_i32 => _THRONE_DAY,
    26158_i32 => _OUED_ED_DAHAB_DAY,
    26164_i32 => _REVOLUTION_DAY,
    26165_i32 => _YOUTH_DAY,
    26201_i32 => _EID_AL_FITR__ESTIMATED_,
    26202_i32 => _EID_AL_FITR__ESTIMATED_,
    26242_i32 => _GREEN_MARCH,
    26254_i32 => _INDEPENDENCE_DAY,
    26270_i32 => _EID_AL_ADHA__ESTIMATED_,
    26271_i32 => _EID_AL_ADHA__ESTIMATED_,
    26290_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    26298_i32 => _NEW_YEAR_S_DAY,
    26308_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    26310_i32 => _AMAZIGH_NEW_YEAR,
    26360_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    26361_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    26418_i32 => _LABOR_DAY,
    26508_i32 => _THRONE_DAY,
    26523_i32 => _OUED_ED_DAHAB_DAY,
    26529_i32 => _REVOLUTION_DAY,
    26530_i32 => _YOUTH_DAY,
    26555_i32 => _EID_AL_FITR__ESTIMATED_,
    26556_i32 => _EID_AL_FITR__ESTIMATED_,
    26607_i32 => _GREEN_MARCH,
    26619_i32 => _INDEPENDENCE_DAY,
    26624_i32 => _EID_AL_ADHA__ESTIMATED_,
    26625_i32 => _EID_AL_ADHA__ESTIMATED_,
    26645_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    26663_i32 => _NEW_YEAR_S_DAY,
    26673_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    26675_i32 => _AMAZIGH_NEW_YEAR,
    26715_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    26716_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    26783_i32 => _LABOR_DAY,
    26873_i32 => _THRONE_DAY,
    26888_i32 => _OUED_ED_DAHAB_DAY,
    26894_i32 => _REVOLUTION_DAY,
    26895_i32 => _YOUTH_DAY,
    26909_i32 => _EID_AL_FITR__ESTIMATED_,
    26910_i32 => _EID_AL_FITR__ESTIMATED_,
    26972_i32 => _GREEN_MARCH,
    26978_i32 => _EID_AL_ADHA__ESTIMATED_,
    26979_i32 => _EID_AL_ADHA__ESTIMATED_,
    26984_i32 => _INDEPENDENCE_DAY,
    26999_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    27028_i32 => _NEW_YEAR_S_DAY,
    27038_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    27040_i32 => _AMAZIGH_NEW_YEAR,
    27069_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    27070_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    27149_i32 => _LABOR_DAY,
    27239_i32 => _THRONE_DAY,
    27254_i32 => _OUED_ED_DAHAB_DAY,
    27260_i32 => _REVOLUTION_DAY,
    27261_i32 => _YOUTH_DAY,
    27264_i32 => _EID_AL_FITR__ESTIMATED_,
    27265_i32 => _EID_AL_FITR__ESTIMATED_,
    27332_i32 => _EID_AL_ADHA__ESTIMATED_,
    27333_i32 => _EID_AL_ADHA__ESTIMATED_,
    27338_i32 => _GREEN_MARCH,
    27350_i32 => _INDEPENDENCE_DAY,
    27353_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    27394_i32 => _NEW_YEAR_S_DAY,
    27404_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    27406_i32 => _AMAZIGH_NEW_YEAR,
    27423_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    27424_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    27514_i32 => _LABOR_DAY,
    27604_i32 => _THRONE_DAY,
    27619_i32 => _EID_AL_FITR__ESTIMATED___OUED_ED_DAHAB_DAY,
    27620_i32 => _EID_AL_FITR__ESTIMATED_,
    27625_i32 => _REVOLUTION_DAY,
    27626_i32 => _YOUTH_DAY,
    27687_i32 => _EID_AL_ADHA__ESTIMATED_,
    27688_i32 => _EID_AL_ADHA__ESTIMATED_,
    27703_i32 => _GREEN_MARCH,
    27707_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    27715_i32 => _INDEPENDENCE_DAY,
    27759_i32 => _NEW_YEAR_S_DAY,
    27769_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    27771_i32 => _AMAZIGH_NEW_YEAR,
    27777_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    27778_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    27879_i32 => _LABOR_DAY,
    27969_i32 => _THRONE_DAY,
    27973_i32 => _EID_AL_FITR__ESTIMATED_,
    27974_i32 => _EID_AL_FITR__ESTIMATED_,
    27984_i32 => _OUED_ED_DAHAB_DAY,
    27990_i32 => _REVOLUTION_DAY,
    27991_i32 => _YOUTH_DAY,
    28041_i32 => _EID_AL_ADHA__ESTIMATED_,
    28042_i32 => _EID_AL_ADHA__ESTIMATED_,
    28062_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    28068_i32 => _GREEN_MARCH,
    28080_i32 => _INDEPENDENCE_DAY,
    28124_i32 => _NEW_YEAR_S_DAY,
    28131_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    28132_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    28134_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    28136_i32 => _AMAZIGH_NEW_YEAR,
    28244_i32 => _LABOR_DAY,
    28328_i32 => _EID_AL_FITR__ESTIMATED_,
    28329_i32 => _EID_AL_FITR__ESTIMATED_,
    28334_i32 => _THRONE_DAY,
    28349_i32 => _OUED_ED_DAHAB_DAY,
    28355_i32 => _REVOLUTION_DAY,
    28356_i32 => _YOUTH_DAY,
    28396_i32 => _EID_AL_ADHA__ESTIMATED_,
    28397_i32 => _EID_AL_ADHA__ESTIMATED_,
    28416_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    28433_i32 => _GREEN_MARCH,
    28445_i32 => _INDEPENDENCE_DAY,
    28486_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    28487_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    28489_i32 => _NEW_YEAR_S_DAY,
    28499_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    28501_i32 => _AMAZIGH_NEW_YEAR,
    28610_i32 => _LABOR_DAY,
    28682_i32 => _EID_AL_FITR__ESTIMATED_,
    28683_i32 => _EID_AL_FITR__ESTIMATED_,
    28700_i32 => _THRONE_DAY,
    28715_i32 => _OUED_ED_DAHAB_DAY,
    28721_i32 => _REVOLUTION_DAY,
    28722_i32 => _YOUTH_DAY,
    28751_i32 => _EID_AL_ADHA__ESTIMATED_,
    28752_i32 => _EID_AL_ADHA__ESTIMATED_,
    28771_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    28799_i32 => _GREEN_MARCH,
    28811_i32 => _INDEPENDENCE_DAY,
    28841_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    28842_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    28855_i32 => _NEW_YEAR_S_DAY,
    28865_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    28867_i32 => _AMAZIGH_NEW_YEAR,
    28975_i32 => _LABOR_DAY,
    29036_i32 => _EID_AL_FITR__ESTIMATED_,
    29037_i32 => _EID_AL_FITR__ESTIMATED_,
    29065_i32 => _THRONE_DAY,
    29080_i32 => _OUED_ED_DAHAB_DAY,
    29086_i32 => _REVOLUTION_DAY,
    29087_i32 => _YOUTH_DAY,
    29105_i32 => _EID_AL_ADHA__ESTIMATED_,
    29106_i32 => _EID_AL_ADHA__ESTIMATED_,
    29125_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    29164_i32 => _GREEN_MARCH,
    29176_i32 => _INDEPENDENCE_DAY,
    29195_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    29196_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    29220_i32 => _NEW_YEAR_S_DAY,
};
