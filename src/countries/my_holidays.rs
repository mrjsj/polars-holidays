use crate::countries::constants::*;
use phf::phf_map;

pub static MY_HOLIDAYS: phf::Map<i32, &'static str> = phf_map! {
    10964_i32 => _EID_AL_FITR__ESTIMATED_,
    10965_i32 => _EID_AL_FITR__SECOND_DAY___ESTIMATED_,
    10966_i32 => _EID_AL_FITR__SECOND_DAY___OBSERVED__ESTIMATED_,
    10992_i32 => _CHINESE_NEW_YEAR__ESTIMATED_,
    10993_i32 => _CHINESE_NEW_YEAR__SECOND_DAY___ESTIMATED_,
    10994_i32 => _CHINESE_NEW_YEAR__SECOND_DAY___OBSERVED__ESTIMATED_,
    11032_i32 => _EID_AL_ADHA__ESTIMATED_,
    11053_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    11078_i32 => _LABOR_DAY,
    11095_i32 => _VESAK_DAY__ESTIMATED_,
    11111_i32 => _BIRTHDAY_OF_HM_YANG_DI_PERTUAN_AGONG,
    11122_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    11200_i32 => _NATIONAL_DAY,
    11316_i32 => _CHRISTMAS_DAY,
    11318_i32 => _EID_AL_FITR__ESTIMATED_,
    11319_i32 => _EID_AL_FITR__SECOND_DAY___ESTIMATED_,
    11346_i32 => _CHINESE_NEW_YEAR,
    11347_i32 => _CHINESE_NEW_YEAR__SECOND_DAY_,
    11387_i32 => _EID_AL_ADHA,
    11407_i32 => _ISLAMIC_NEW_YEAR,
    11443_i32 => _LABOR_DAY,
    11449_i32 => _VESAK_DAY,
    11475_i32 => _BIRTHDAY_OF_HM_YANG_DI_PERTUAN_AGONG,
    11477_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY,
    11565_i32 => _NATIONAL_DAY,
    11673_i32 => _EID_AL_FITR,
    11674_i32 => _EID_AL_FITR__SECOND_DAY_,
    11681_i32 => _CHRISTMAS_DAY,
    11730_i32 => _CHINESE_NEW_YEAR,
    11731_i32 => _CHINESE_NEW_YEAR__SECOND_DAY_,
    11741_i32 => _EID_AL_ADHA,
    11761_i32 => _ISLAMIC_NEW_YEAR,
    11808_i32 => _LABOR_DAY,
    11831_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY,
    11834_i32 => _VESAK_DAY,
    11839_i32 => _BIRTHDAY_OF_HM_YANG_DI_PERTUAN_AGONG,
    11930_i32 => _NATIONAL_DAY,
    12027_i32 => _EID_AL_FITR,
    12028_i32 => _EID_AL_FITR__SECOND_DAY_,
    12046_i32 => _CHRISTMAS_DAY,
    12084_i32 => _CHINESE_NEW_YEAR,
    12085_i32 => _CHINESE_NEW_YEAR__SECOND_DAY_,
    12086_i32 => _CHINESE_NEW_YEAR__SECOND_DAY___OBSERVED_,
    12095_i32 => _EID_AL_ADHA,
    12116_i32 => _ISLAMIC_NEW_YEAR,
    12173_i32 => _LABOR_DAY,
    12186_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY,
    12187_i32 => _VESAK_DAY,
    12210_i32 => _BIRTHDAY_OF_HM_YANG_DI_PERTUAN_AGONG,
    12295_i32 => _NATIONAL_DAY,
    12296_i32 => _NATIONAL_DAY__OBSERVED_,
    12382_i32 => _EID_AL_FITR,
    12383_i32 => _EID_AL_FITR__SECOND_DAY_,
    12411_i32 => _CHRISTMAS_DAY,
    12439_i32 => _CHINESE_NEW_YEAR,
    12440_i32 => _CHINESE_NEW_YEAR__SECOND_DAY_,
    12450_i32 => _EID_AL_ADHA,
    12470_i32 => _ISLAMIC_NEW_YEAR,
    12539_i32 => _LABOR_DAY,
    12540_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY,
    12541_i32 => _VESAK_DAY,
    12542_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__OBSERVED_,
    12574_i32 => _BIRTHDAY_OF_HM_YANG_DI_PERTUAN_AGONG,
    12661_i32 => _NATIONAL_DAY,
    12736_i32 => _EID_AL_FITR,
    12737_i32 => _EID_AL_FITR__SECOND_DAY_,
    12738_i32 => _EID_AL_FITR__OBSERVED_,
    12777_i32 => _CHRISTMAS_DAY,
    12804_i32 => _EID_AL_ADHA,
    12823_i32 => _CHINESE_NEW_YEAR,
    12824_i32 => _CHINESE_NEW_YEAR__SECOND_DAY___ISLAMIC_NEW_YEAR,
    12894_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY,
    12904_i32 => _LABOR_DAY,
    12905_i32 => _LABOR_DAY__OBSERVED_,
    12925_i32 => _VESAK_DAY,
    12926_i32 => _VESAK_DAY__OBSERVED_,
    12938_i32 => _BIRTHDAY_OF_HM_YANG_DI_PERTUAN_AGONG,
    13026_i32 => _NATIONAL_DAY,
    13090_i32 => _EID_AL_FITR,
    13091_i32 => _EID_AL_FITR__SECOND_DAY_,
    13142_i32 => _CHRISTMAS_DAY,
    13143_i32 => _CHRISTMAS_DAY__OBSERVED_,
    13158_i32 => _EID_AL_ADHA,
    13177_i32 => _CHINESE_NEW_YEAR,
    13178_i32 => _CHINESE_NEW_YEAR__SECOND_DAY_,
    13179_i32 => _ISLAMIC_NEW_YEAR,
    13180_i32 => _CHINESE_NEW_YEAR__OBSERVED_,
    13249_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY,
    13269_i32 => _LABOR_DAY,
    13280_i32 => _VESAK_DAY,
    13302_i32 => _BIRTHDAY_OF_HM_YANG_DI_PERTUAN_AGONG,
    13391_i32 => _NATIONAL_DAY,
    13445_i32 => _EID_AL_FITR,
    13446_i32 => _EID_AL_FITR__SECOND_DAY_,
    13507_i32 => _CHRISTMAS_DAY,
    13513_i32 => _EID_AL_ADHA,
    13533_i32 => _ISLAMIC_NEW_YEAR,
    13562_i32 => _CHINESE_NEW_YEAR,
    13563_i32 => _CHINESE_NEW_YEAR__SECOND_DAY_,
    13564_i32 => _CHINESE_NEW_YEAR__OBSERVED_,
    13603_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY,
    13634_i32 => _LABOR_DAY__VESAK_DAY,
    13666_i32 => _BIRTHDAY_OF_HM_YANG_DI_PERTUAN_AGONG,
    13756_i32 => _NATIONAL_DAY,
    13799_i32 => _EID_AL_FITR,
    13800_i32 => _EID_AL_FITR__SECOND_DAY_,
    13801_i32 => _EID_AL_FITR__SECOND_DAY___OBSERVED_,
    13867_i32 => _EID_AL_ADHA,
    13872_i32 => _CHRISTMAS_DAY,
    13888_i32 => _ISLAMIC_NEW_YEAR,
    13916_i32 => _CHINESE_NEW_YEAR,
    13917_i32 => _CHINESE_NEW_YEAR__SECOND_DAY_,
    13958_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY,
    14000_i32 => _LABOR_DAY,
    14018_i32 => _VESAK_DAY,
    14037_i32 => _BIRTHDAY_OF_HM_YANG_DI_PERTUAN_AGONG,
    14122_i32 => _NATIONAL_DAY,
    14123_i32 => _NATIONAL_DAY__OBSERVED_,
    14153_i32 => _EID_AL_FITR,
    14154_i32 => _EID_AL_FITR__SECOND_DAY_,
    14222_i32 => _EID_AL_ADHA,
    14238_i32 => _CHRISTMAS_DAY,
    14242_i32 => _ISLAMIC_NEW_YEAR,
    14270_i32 => _CHINESE_NEW_YEAR,
    14271_i32 => _CHINESE_NEW_YEAR__SECOND_DAY_,
    14312_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY,
    14365_i32 => _LABOR_DAY,
    14373_i32 => _VESAK_DAY,
    14401_i32 => _BIRTHDAY_OF_HM_YANG_DI_PERTUAN_AGONG,
    14487_i32 => _NATIONAL_DAY,
    14507_i32 => _EID_AL_FITR,
    14508_i32 => _EID_AL_FITR__SECOND_DAY_,
    14509_i32 => _EID_AL_FITR__OBSERVED_,
    14576_i32 => _EID_AL_ADHA,
    14596_i32 => _ISLAMIC_NEW_YEAR,
    14603_i32 => _CHRISTMAS_DAY,
    14654_i32 => _CHINESE_NEW_YEAR,
    14655_i32 => _CHINESE_NEW_YEAR__SECOND_DAY_,
    14656_i32 => _CHINESE_NEW_YEAR__OBSERVED_,
    14666_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY,
    14730_i32 => _LABOR_DAY,
    14757_i32 => _VESAK_DAY,
    14765_i32 => _BIRTHDAY_OF_HM_YANG_DI_PERTUAN_AGONG,
    14852_i32 => _NATIONAL_DAY,
    14862_i32 => _EID_AL_FITR,
    14863_i32 => _EID_AL_FITR__SECOND_DAY_,
    14868_i32 => _MALAYSIA_DAY,
    14930_i32 => _EID_AL_ADHA,
    14951_i32 => _ISLAMIC_NEW_YEAR,
    14968_i32 => _CHRISTMAS_DAY,
    15008_i32 => _CHINESE_NEW_YEAR,
    15009_i32 => _CHINESE_NEW_YEAR__SECOND_DAY_,
    15021_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY,
    15095_i32 => _LABOR_DAY,
    15096_i32 => _LABOR_DAY__OBSERVED_,
    15111_i32 => _VESAK_DAY,
    15129_i32 => _BIRTHDAY_OF_HM_YANG_DI_PERTUAN_AGONG,
    15217_i32 => _EID_AL_FITR__NATIONAL_DAY,
    15218_i32 => _EID_AL_FITR__SECOND_DAY_,
    15233_i32 => _MALAYSIA_DAY,
    15285_i32 => _EID_AL_ADHA,
    15305_i32 => _ISLAMIC_NEW_YEAR,
    15333_i32 => _CHRISTMAS_DAY,
    15334_i32 => _CHRISTMAS_DAY__OBSERVED_,
    15362_i32 => _CHINESE_NEW_YEAR,
    15363_i32 => _CHINESE_NEW_YEAR__SECOND_DAY_,
    15375_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY,
    15376_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__OBSERVED_,
    15461_i32 => _LABOR_DAY,
    15465_i32 => _VESAK_DAY,
    15493_i32 => _BIRTHDAY_OF_HM_YANG_DI_PERTUAN_AGONG,
    15571_i32 => _EID_AL_FITR,
    15572_i32 => _EID_AL_FITR__SECOND_DAY_,
    15573_i32 => _EID_AL_FITR__OBSERVED_,
    15583_i32 => _NATIONAL_DAY,
    15599_i32 => _MALAYSIA_DAY,
    15600_i32 => _MALAYSIA_DAY__OBSERVED_,
    15639_i32 => _EID_AL_ADHA,
    15659_i32 => _ISLAMIC_NEW_YEAR,
    15699_i32 => _CHRISTMAS_DAY,
    15729_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY,
    15746_i32 => _CHINESE_NEW_YEAR,
    15747_i32 => _CHINESE_NEW_YEAR__SECOND_DAY_,
    15748_i32 => _CHINESE_NEW_YEAR__OBSERVED_,
    15826_i32 => _LABOR_DAY,
    15849_i32 => _VESAK_DAY,
    15857_i32 => _BIRTHDAY_OF_HM_YANG_DI_PERTUAN_AGONG,
    15925_i32 => _EID_AL_FITR,
    15926_i32 => _EID_AL_FITR__SECOND_DAY_,
    15948_i32 => _NATIONAL_DAY,
    15964_i32 => _MALAYSIA_DAY,
    15993_i32 => _EID_AL_ADHA,
    16014_i32 => _ISLAMIC_NEW_YEAR,
    16064_i32 => _CHRISTMAS_DAY,
    16084_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY,
    16101_i32 => _CHINESE_NEW_YEAR,
    16102_i32 => _CHINESE_NEW_YEAR__SECOND_DAY_,
    16191_i32 => _LABOR_DAY,
    16203_i32 => _VESAK_DAY,
    16228_i32 => _BIRTHDAY_OF_HM_YANG_DI_PERTUAN_AGONG,
    16279_i32 => _EID_AL_FITR,
    16280_i32 => _EID_AL_FITR__SECOND_DAY_,
    16313_i32 => _NATIONAL_DAY,
    16314_i32 => _NATIONAL_DAY__OBSERVED_,
    16329_i32 => _MALAYSIA_DAY,
    16348_i32 => _EID_AL_ADHA,
    16349_i32 => _EID_AL_ADHA__OBSERVED_,
    16368_i32 => _ISLAMIC_NEW_YEAR,
    16429_i32 => _CHRISTMAS_DAY,
    16438_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY,
    16485_i32 => _CHINESE_NEW_YEAR,
    16486_i32 => _CHINESE_NEW_YEAR__SECOND_DAY_,
    16556_i32 => _LABOR_DAY,
    16558_i32 => _VESAK_DAY,
    16559_i32 => _VESAK_DAY__OBSERVED_,
    16592_i32 => _BIRTHDAY_OF_HM_YANG_DI_PERTUAN_AGONG,
    16633_i32 => _EID_AL_FITR,
    16634_i32 => _EID_AL_FITR__SECOND_DAY_,
    16678_i32 => _NATIONAL_DAY,
    16694_i32 => _MALAYSIA_DAY,
    16702_i32 => _EID_AL_ADHA,
    16722_i32 => _ISLAMIC_NEW_YEAR,
    16793_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY,
    16794_i32 => _CHRISTMAS_DAY,
    16839_i32 => _CHINESE_NEW_YEAR,
    16840_i32 => _CHINESE_NEW_YEAR__SECOND_DAY_,
    16922_i32 => _LABOR_DAY,
    16923_i32 => _LABOR_DAY__OBSERVED_,
    16942_i32 => _VESAK_DAY,
    16956_i32 => _BIRTHDAY_OF_HM_YANG_DI_PERTUAN_AGONG,
    16988_i32 => _EID_AL_FITR,
    16989_i32 => _EID_AL_FITR__SECOND_DAY_,
    17044_i32 => _NATIONAL_DAY,
    17056_i32 => _EID_AL_ADHA,
    17060_i32 => _MALAYSIA_DAY,
    17076_i32 => _ISLAMIC_NEW_YEAR,
    17147_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY,
    17160_i32 => _CHRISTMAS_DAY,
    17161_i32 => _CHRISTMAS_DAY__OBSERVED_,
    17194_i32 => _CHINESE_NEW_YEAR,
    17195_i32 => _CHINESE_NEW_YEAR__SECOND_DAY_,
    17196_i32 => _CHINESE_NEW_YEAR__SECOND_DAY___OBSERVED_,
    17280_i32 => _DAY_OF_INSTALLATION_OF_THE_15TH_YANG_DI_PERTUAN_AGONG,
    17287_i32 => _LABOR_DAY,
    17296_i32 => _VESAK_DAY,
    17342_i32 => _EID_AL_FITR,
    17343_i32 => _EID_AL_FITR__SECOND_DAY_,
    17344_i32 => _EID_AL_FITR__OBSERVED_,
    17409_i32 => _NATIONAL_DAY,
    17410_i32 => _EID_AL_ADHA,
    17413_i32 => _ADDITIONAL_HOLIDAY_IN_COMMEMORATION_OF_THE_2017_SEA_GAMES,
    17418_i32 => _BIRTHDAY_OF_HM_YANG_DI_PERTUAN_AGONG,
    17425_i32 => _MALAYSIA_DAY,
    17431_i32 => _ISLAMIC_NEW_YEAR,
    17501_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY,
    17525_i32 => _CHRISTMAS_DAY,
    17578_i32 => _CHINESE_NEW_YEAR,
    17579_i32 => _CHINESE_NEW_YEAR__SECOND_DAY_,
    17652_i32 => _LABOR_DAY,
    17660_i32 => _GENERAL_ELECTION_ADDITIONAL_HOLIDAY,
    17680_i32 => _VESAK_DAY,
    17697_i32 => _EID_AL_FITR,
    17698_i32 => _EID_AL_FITR__SECOND_DAY_,
    17765_i32 => _EID_AL_ADHA,
    17774_i32 => _NATIONAL_DAY,
    17783_i32 => _BIRTHDAY_OF_HM_YANG_DI_PERTUAN_AGONG,
    17784_i32 => _BIRTHDAY_OF_HM_YANG_DI_PERTUAN_AGONG__OBSERVED_,
    17785_i32 => _ISLAMIC_NEW_YEAR,
    17790_i32 => _MALAYSIA_DAY,
    17791_i32 => _MALAYSIA_DAY__OBSERVED_,
    17855_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY,
    17890_i32 => _CHRISTMAS_DAY,
    17932_i32 => _CHINESE_NEW_YEAR,
    17933_i32 => _CHINESE_NEW_YEAR__SECOND_DAY_,
    18017_i32 => _LABOR_DAY,
    18035_i32 => _VESAK_DAY,
    18036_i32 => _VESAK_DAY__OBSERVED_,
    18052_i32 => _EID_AL_FITR,
    18053_i32 => _EID_AL_FITR__SECOND_DAY_,
    18107_i32 => _DAY_OF_INSTALLATION_OF_THE_16TH_YANG_DI_PERTUAN_AGONG,
    18119_i32 => _EID_AL_ADHA,
    18120_i32 => _EID_AL_ADHA__OBSERVED_,
    18139_i32 => _NATIONAL_DAY,
    18140_i32 => _ISLAMIC_NEW_YEAR,
    18148_i32 => _BIRTHDAY_OF_HM_YANG_DI_PERTUAN_AGONG,
    18155_i32 => _MALAYSIA_DAY,
    18209_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY,
    18255_i32 => _CHRISTMAS_DAY,
    18286_i32 => _CHINESE_NEW_YEAR,
    18287_i32 => _CHINESE_NEW_YEAR__SECOND_DAY_,
    18288_i32 => _CHINESE_NEW_YEAR__SECOND_DAY___OBSERVED_,
    18383_i32 => _LABOR_DAY,
    18389_i32 => _VESAK_DAY,
    18406_i32 => _EID_AL_FITR,
    18407_i32 => _EID_AL_FITR__SECOND_DAY_,
    18408_i32 => _EID_AL_FITR__OBSERVED_,
    18421_i32 => _BIRTHDAY_OF_HM_YANG_DI_PERTUAN_AGONG,
    18474_i32 => _EID_AL_ADHA,
    18494_i32 => _ISLAMIC_NEW_YEAR,
    18505_i32 => _NATIONAL_DAY,
    18521_i32 => _MALAYSIA_DAY,
    18564_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY,
    18621_i32 => _CHRISTMAS_DAY,
    18670_i32 => _CHINESE_NEW_YEAR,
    18671_i32 => _CHINESE_NEW_YEAR__SECOND_DAY_,
    18748_i32 => _LABOR_DAY,
    18760_i32 => _EID_AL_FITR,
    18761_i32 => _EID_AL_FITR__SECOND_DAY_,
    18773_i32 => _VESAK_DAY,
    18785_i32 => _BIRTHDAY_OF_HM_YANG_DI_PERTUAN_AGONG,
    18828_i32 => _EID_AL_ADHA,
    18849_i32 => _ISLAMIC_NEW_YEAR,
    18870_i32 => _NATIONAL_DAY,
    18886_i32 => _MALAYSIA_DAY,
    18919_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY,
    18986_i32 => _CHRISTMAS_DAY,
    19024_i32 => _CHINESE_NEW_YEAR,
    19025_i32 => _CHINESE_NEW_YEAR__SECOND_DAY_,
    19113_i32 => _LABOR_DAY,
    19114_i32 => _EID_AL_FITR,
    19115_i32 => _EID_AL_FITR__SECOND_DAY_,
    19116_i32 => _LABOR_DAY__OBSERVED_,
    19127_i32 => _VESAK_DAY,
    19128_i32 => _VESAK_DAY__OBSERVED_,
    19149_i32 => _BIRTHDAY_OF_HM_YANG_DI_PERTUAN_AGONG,
    19183_i32 => _EID_AL_ADHA,
    19184_i32 => _EID_AL_ADHA__OBSERVED_,
    19203_i32 => _ISLAMIC_NEW_YEAR,
    19235_i32 => _NATIONAL_DAY,
    19251_i32 => _MALAYSIA_DAY,
    19275_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY,
    19314_i32 => _GENERAL_ELECTION_ADDITIONAL_HOLIDAY,
    19315_i32 => _GENERAL_ELECTION_ADDITIONAL_HOLIDAY,
    19324_i32 => _CUTI_PERISTIWA,
    19351_i32 => _CHRISTMAS_DAY,
    19352_i32 => _CHRISTMAS_DAY__OBSERVED_,
    19379_i32 => _CHINESE_NEW_YEAR,
    19380_i32 => _CHINESE_NEW_YEAR__SECOND_DAY_,
    19381_i32 => _CHINESE_NEW_YEAR__OBSERVED_,
    19468_i32 => _EID_AL_FITR__ADDITIONAL_HOLIDAY_,
    19469_i32 => _EID_AL_FITR,
    19470_i32 => _EID_AL_FITR__SECOND_DAY_,
    19471_i32 => _EID_AL_FITR__SECOND_DAY___OBSERVED_,
    19478_i32 => _LABOR_DAY,
    19481_i32 => _VESAK_DAY,
    19513_i32 => _BIRTHDAY_OF_HM_YANG_DI_PERTUAN_AGONG,
    19537_i32 => _EID_AL_ADHA,
    19557_i32 => _ISLAMIC_NEW_YEAR,
    19600_i32 => _NATIONAL_DAY,
    19616_i32 => _MALAYSIA_DAY,
    19628_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY,
    19716_i32 => _CHRISTMAS_DAY,
    19763_i32 => _CHINESE_NEW_YEAR,
    19764_i32 => _CHINESE_NEW_YEAR__SECOND_DAY_,
    19765_i32 => _CHINESE_NEW_YEAR__SECOND_DAY___OBSERVED_,
    19823_i32 => _EID_AL_FITR,
    19824_i32 => _EID_AL_FITR__SECOND_DAY_,
    19844_i32 => _LABOR_DAY,
    19865_i32 => _VESAK_DAY,
    19877_i32 => _BIRTHDAY_OF_HM_YANG_DI_PERTUAN_AGONG,
    19891_i32 => _EID_AL_ADHA,
    19911_i32 => _ISLAMIC_NEW_YEAR,
    19966_i32 => _NATIONAL_DAY,
    19982_i32 => _MALAYSIA_DAY__PROPHET_MUHAMMAD_S_BIRTHDAY,
    20082_i32 => _CHRISTMAS_DAY,
    20117_i32 => _CHINESE_NEW_YEAR__ESTIMATED_,
    20118_i32 => _CHINESE_NEW_YEAR__SECOND_DAY___ESTIMATED_,
    20177_i32 => _EID_AL_FITR__ESTIMATED_,
    20178_i32 => _EID_AL_FITR__SECOND_DAY___ESTIMATED_,
    20179_i32 => _EID_AL_FITR__OBSERVED__ESTIMATED_,
    20209_i32 => _LABOR_DAY,
    20219_i32 => _VESAK_DAY__ESTIMATED_,
    20220_i32 => _VESAK_DAY__OBSERVED__ESTIMATED_,
    20241_i32 => _BIRTHDAY_OF_HM_YANG_DI_PERTUAN_AGONG,
    20245_i32 => _EID_AL_ADHA__ESTIMATED_,
    20265_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    20331_i32 => _NATIONAL_DAY,
    20332_i32 => _NATIONAL_DAY__OBSERVED_,
    20335_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    20347_i32 => _MALAYSIA_DAY,
    20447_i32 => _CHRISTMAS_DAY,
    20501_i32 => _CHINESE_NEW_YEAR__ESTIMATED_,
    20502_i32 => _CHINESE_NEW_YEAR__SECOND_DAY___ESTIMATED_,
    20532_i32 => _EID_AL_FITR__ESTIMATED_,
    20533_i32 => _EID_AL_FITR__SECOND_DAY___ESTIMATED_,
    20574_i32 => _LABOR_DAY__VESAK_DAY__ESTIMATED_,
    20600_i32 => _EID_AL_ADHA__ESTIMATED_,
    20605_i32 => _BIRTHDAY_OF_HM_YANG_DI_PERTUAN_AGONG,
    20620_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    20690_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    20696_i32 => _NATIONAL_DAY,
    20712_i32 => _MALAYSIA_DAY,
    20812_i32 => _CHRISTMAS_DAY,
    20855_i32 => _CHINESE_NEW_YEAR__ESTIMATED_,
    20856_i32 => _CHINESE_NEW_YEAR__SECOND_DAY___ESTIMATED_,
    20857_i32 => _CHINESE_NEW_YEAR__SECOND_DAY___OBSERVED__ESTIMATED_,
    20886_i32 => _EID_AL_FITR__ESTIMATED_,
    20887_i32 => _EID_AL_FITR__SECOND_DAY___ESTIMATED_,
    20939_i32 => _LABOR_DAY,
    20954_i32 => _EID_AL_ADHA__ESTIMATED_,
    20955_i32 => _EID_AL_ADHA__OBSERVED__ESTIMATED_,
    20958_i32 => _VESAK_DAY__ESTIMATED_,
    20975_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    20976_i32 => _BIRTHDAY_OF_HM_YANG_DI_PERTUAN_AGONG,
    21044_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    21061_i32 => _NATIONAL_DAY,
    21077_i32 => _MALAYSIA_DAY,
    21177_i32 => _CHRISTMAS_DAY,
    21209_i32 => _CHINESE_NEW_YEAR__ESTIMATED_,
    21210_i32 => _CHINESE_NEW_YEAR__SECOND_DAY___ESTIMATED_,
    21240_i32 => _EID_AL_FITR__ESTIMATED_,
    21241_i32 => _EID_AL_FITR__SECOND_DAY___ESTIMATED_,
    21242_i32 => _EID_AL_FITR__SECOND_DAY___OBSERVED__ESTIMATED_,
    21305_i32 => _LABOR_DAY,
    21309_i32 => _EID_AL_ADHA__ESTIMATED_,
    21313_i32 => _VESAK_DAY__ESTIMATED_,
    21329_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    21340_i32 => _BIRTHDAY_OF_HM_YANG_DI_PERTUAN_AGONG,
    21399_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    21427_i32 => _NATIONAL_DAY,
    21443_i32 => _MALAYSIA_DAY,
    21543_i32 => _CHRISTMAS_DAY,
    21593_i32 => _CHINESE_NEW_YEAR__ESTIMATED_,
    21594_i32 => _CHINESE_NEW_YEAR__SECOND_DAY___ESTIMATED___EID_AL_FITR__ESTIMATED_,
    21595_i32 => _EID_AL_FITR__SECOND_DAY___ESTIMATED_,
    21663_i32 => _EID_AL_ADHA__ESTIMATED_,
    21670_i32 => _LABOR_DAY,
    21683_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    21696_i32 => _VESAK_DAY__ESTIMATED_,
    21697_i32 => _VESAK_DAY__OBSERVED__ESTIMATED_,
    21704_i32 => _BIRTHDAY_OF_HM_YANG_DI_PERTUAN_AGONG,
    21754_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    21792_i32 => _NATIONAL_DAY,
    21808_i32 => _MALAYSIA_DAY,
    21809_i32 => _MALAYSIA_DAY__OBSERVED_,
    21908_i32 => _CHRISTMAS_DAY,
    21948_i32 => _CHINESE_NEW_YEAR__ESTIMATED_,
    21949_i32 => _CHINESE_NEW_YEAR__SECOND_DAY___ESTIMATED___EID_AL_FITR__ESTIMATED_,
    21950_i32 => _EID_AL_FITR__SECOND_DAY___ESTIMATED_,
    21951_i32 => _CHINESE_NEW_YEAR__OBSERVED__ESTIMATED_,
    22017_i32 => _EID_AL_ADHA__ESTIMATED_,
    22035_i32 => _LABOR_DAY,
    22037_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    22050_i32 => _VESAK_DAY__ESTIMATED_,
    22068_i32 => _BIRTHDAY_OF_HM_YANG_DI_PERTUAN_AGONG,
    22108_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    22157_i32 => _NATIONAL_DAY,
    22173_i32 => _MALAYSIA_DAY,
    22273_i32 => _CHRISTMAS_DAY,
    22302_i32 => _CHINESE_NEW_YEAR__ESTIMATED_,
    22303_i32 => _CHINESE_NEW_YEAR__SECOND_DAY___ESTIMATED___EID_AL_FITR__ESTIMATED_,
    22304_i32 => _EID_AL_FITR__SECOND_DAY___ESTIMATED_,
    22371_i32 => _EID_AL_ADHA__ESTIMATED_,
    22392_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    22400_i32 => _LABOR_DAY,
    22405_i32 => _VESAK_DAY__ESTIMATED_,
    22432_i32 => _BIRTHDAY_OF_HM_YANG_DI_PERTUAN_AGONG,
    22462_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    22522_i32 => _NATIONAL_DAY,
    22523_i32 => _NATIONAL_DAY__OBSERVED_,
    22538_i32 => _MALAYSIA_DAY,
    22638_i32 => _CHRISTMAS_DAY,
    22658_i32 => _EID_AL_FITR__ESTIMATED_,
    22659_i32 => _EID_AL_FITR__SECOND_DAY___ESTIMATED_,
    22686_i32 => _CHINESE_NEW_YEAR__ESTIMATED_,
    22687_i32 => _CHINESE_NEW_YEAR__SECOND_DAY___ESTIMATED_,
    22726_i32 => _EID_AL_ADHA__ESTIMATED_,
    22746_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    22766_i32 => _LABOR_DAY,
    22788_i32 => _VESAK_DAY__ESTIMATED_,
    22789_i32 => _VESAK_DAY__OBSERVED__ESTIMATED_,
    22803_i32 => _BIRTHDAY_OF_HM_YANG_DI_PERTUAN_AGONG,
    22816_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    22817_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__OBSERVED__ESTIMATED_,
    22888_i32 => _NATIONAL_DAY,
    22904_i32 => _MALAYSIA_DAY,
    23004_i32 => _CHRISTMAS_DAY,
    23012_i32 => _EID_AL_FITR__ESTIMATED_,
    23013_i32 => _EID_AL_FITR__SECOND_DAY___ESTIMATED_,
    23014_i32 => _EID_AL_FITR__OBSERVED__ESTIMATED_,
    23041_i32 => _CHINESE_NEW_YEAR__ESTIMATED_,
    23042_i32 => _CHINESE_NEW_YEAR__SECOND_DAY___ESTIMATED_,
    23080_i32 => _EID_AL_ADHA__ESTIMATED_,
    23101_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    23131_i32 => _LABOR_DAY,
    23132_i32 => _LABOR_DAY__OBSERVED_,
    23143_i32 => _VESAK_DAY__ESTIMATED_,
    23167_i32 => _BIRTHDAY_OF_HM_YANG_DI_PERTUAN_AGONG,
    23170_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    23253_i32 => _NATIONAL_DAY,
    23269_i32 => _MALAYSIA_DAY,
    23367_i32 => _EID_AL_FITR__ESTIMATED_,
    23368_i32 => _EID_AL_FITR__SECOND_DAY___ESTIMATED_,
    23369_i32 => _CHRISTMAS_DAY,
    23370_i32 => _CHRISTMAS_DAY__OBSERVED_,
    23425_i32 => _CHINESE_NEW_YEAR__ESTIMATED_,
    23426_i32 => _CHINESE_NEW_YEAR__SECOND_DAY___ESTIMATED_,
    23427_i32 => _CHINESE_NEW_YEAR__OBSERVED__ESTIMATED_,
    23435_i32 => _EID_AL_ADHA__ESTIMATED_,
    23455_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    23496_i32 => _LABOR_DAY,
    23498_i32 => _VESAK_DAY__ESTIMATED_,
    23525_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    23531_i32 => _BIRTHDAY_OF_HM_YANG_DI_PERTUAN_AGONG,
    23618_i32 => _NATIONAL_DAY,
    23634_i32 => _MALAYSIA_DAY,
    23721_i32 => _EID_AL_FITR__ESTIMATED_,
    23722_i32 => _EID_AL_FITR__SECOND_DAY___ESTIMATED_,
    23734_i32 => _CHRISTMAS_DAY,
    23779_i32 => _CHINESE_NEW_YEAR__ESTIMATED_,
    23780_i32 => _CHINESE_NEW_YEAR__SECOND_DAY___ESTIMATED_,
    23789_i32 => _EID_AL_ADHA__ESTIMATED_,
    23790_i32 => _EID_AL_ADHA__OBSERVED__ESTIMATED_,
    23810_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    23861_i32 => _LABOR_DAY,
    23880_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    23881_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__OBSERVED__ESTIMATED_,
    23882_i32 => _VESAK_DAY__ESTIMATED_,
    23895_i32 => _BIRTHDAY_OF_HM_YANG_DI_PERTUAN_AGONG,
    23983_i32 => _NATIONAL_DAY,
    23999_i32 => _MALAYSIA_DAY,
    24000_i32 => _MALAYSIA_DAY__OBSERVED_,
    24075_i32 => _EID_AL_FITR__ESTIMATED_,
    24076_i32 => _EID_AL_FITR__SECOND_DAY___ESTIMATED_,
    24077_i32 => _EID_AL_FITR__SECOND_DAY___OBSERVED__ESTIMATED_,
    24099_i32 => _CHRISTMAS_DAY,
    24133_i32 => _CHINESE_NEW_YEAR__ESTIMATED_,
    24134_i32 => _CHINESE_NEW_YEAR__SECOND_DAY___ESTIMATED_,
    24143_i32 => _EID_AL_ADHA__ESTIMATED_,
    24164_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    24227_i32 => _LABOR_DAY,
    24234_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    24236_i32 => _VESAK_DAY__ESTIMATED_,
    24259_i32 => _BIRTHDAY_OF_HM_YANG_DI_PERTUAN_AGONG,
    24349_i32 => _NATIONAL_DAY,
    24350_i32 => _NATIONAL_DAY__OBSERVED_,
    24365_i32 => _MALAYSIA_DAY,
    24429_i32 => _EID_AL_FITR__ESTIMATED_,
    24430_i32 => _EID_AL_FITR__SECOND_DAY___ESTIMATED_,
    24465_i32 => _CHRISTMAS_DAY,
    24497_i32 => _EID_AL_ADHA__ESTIMATED_,
    24517_i32 => _CHINESE_NEW_YEAR__ESTIMATED_,
    24518_i32 => _CHINESE_NEW_YEAR__SECOND_DAY___ESTIMATED___ISLAMIC_NEW_YEAR__ESTIMATED_,
    24519_i32 => _CHINESE_NEW_YEAR__OBSERVED__ESTIMATED_,
    24589_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    24592_i32 => _LABOR_DAY,
    24620_i32 => _VESAK_DAY__ESTIMATED_,
    24623_i32 => _BIRTHDAY_OF_HM_YANG_DI_PERTUAN_AGONG,
    24714_i32 => _NATIONAL_DAY,
    24730_i32 => _MALAYSIA_DAY,
    24783_i32 => _EID_AL_FITR__ESTIMATED_,
    24784_i32 => _EID_AL_FITR__SECOND_DAY___ESTIMATED_,
    24785_i32 => _EID_AL_FITR__OBSERVED__ESTIMATED_,
    24830_i32 => _CHRISTMAS_DAY,
    24852_i32 => _EID_AL_ADHA__ESTIMATED_,
    24871_i32 => _CHINESE_NEW_YEAR__ESTIMATED_,
    24872_i32 => _CHINESE_NEW_YEAR__SECOND_DAY___ESTIMATED___ISLAMIC_NEW_YEAR__ESTIMATED_,
    24943_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    24957_i32 => _LABOR_DAY,
    24974_i32 => _VESAK_DAY__ESTIMATED_,
    24994_i32 => _BIRTHDAY_OF_HM_YANG_DI_PERTUAN_AGONG,
    25079_i32 => _NATIONAL_DAY,
    25095_i32 => _MALAYSIA_DAY,
    25138_i32 => _EID_AL_FITR__ESTIMATED_,
    25139_i32 => _EID_AL_FITR__SECOND_DAY___ESTIMATED_,
    25195_i32 => _CHRISTMAS_DAY,
    25206_i32 => _EID_AL_ADHA__ESTIMATED_,
    25225_i32 => _CHINESE_NEW_YEAR__ESTIMATED_,
    25226_i32 => _CHINESE_NEW_YEAR__SECOND_DAY___ESTIMATED_,
    25227_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    25297_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    25322_i32 => _LABOR_DAY,
    25323_i32 => _LABOR_DAY__OBSERVED_,
    25328_i32 => _VESAK_DAY__ESTIMATED_,
    25358_i32 => _BIRTHDAY_OF_HM_YANG_DI_PERTUAN_AGONG,
    25444_i32 => _NATIONAL_DAY,
    25460_i32 => _MALAYSIA_DAY,
    25493_i32 => _EID_AL_FITR__ESTIMATED_,
    25494_i32 => _EID_AL_FITR__SECOND_DAY___ESTIMATED_,
    25560_i32 => _CHRISTMAS_DAY,
    25561_i32 => _EID_AL_ADHA__ESTIMATED_,
    25562_i32 => _CHRISTMAS_DAY__OBSERVED_,
    25581_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    25609_i32 => _CHINESE_NEW_YEAR__ESTIMATED_,
    25610_i32 => _CHINESE_NEW_YEAR__SECOND_DAY___ESTIMATED_,
    25611_i32 => _CHINESE_NEW_YEAR__OBSERVED__ESTIMATED_,
    25651_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    25652_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__OBSERVED__ESTIMATED_,
    25688_i32 => _LABOR_DAY,
    25712_i32 => _VESAK_DAY__ESTIMATED_,
    25722_i32 => _BIRTHDAY_OF_HM_YANG_DI_PERTUAN_AGONG,
    25810_i32 => _NATIONAL_DAY,
    25826_i32 => _MALAYSIA_DAY,
    25827_i32 => _MALAYSIA_DAY__OBSERVED_,
    25847_i32 => _EID_AL_FITR__ESTIMATED_,
    25848_i32 => _EID_AL_FITR__SECOND_DAY___ESTIMATED_,
    25849_i32 => _EID_AL_FITR__OBSERVED__ESTIMATED_,
    25915_i32 => _EID_AL_ADHA__ESTIMATED_,
    25926_i32 => _CHRISTMAS_DAY,
    25936_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    25964_i32 => _CHINESE_NEW_YEAR__ESTIMATED_,
    25965_i32 => _CHINESE_NEW_YEAR__SECOND_DAY___ESTIMATED_,
    26006_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    26053_i32 => _LABOR_DAY,
    26066_i32 => _VESAK_DAY__ESTIMATED_,
    26086_i32 => _BIRTHDAY_OF_HM_YANG_DI_PERTUAN_AGONG,
    26175_i32 => _NATIONAL_DAY,
    26191_i32 => _MALAYSIA_DAY,
    26201_i32 => _EID_AL_FITR__ESTIMATED_,
    26202_i32 => _EID_AL_FITR__SECOND_DAY___ESTIMATED_,
    26270_i32 => _EID_AL_ADHA__ESTIMATED_,
    26290_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    26291_i32 => _CHRISTMAS_DAY,
    26319_i32 => _CHINESE_NEW_YEAR__ESTIMATED_,
    26320_i32 => _CHINESE_NEW_YEAR__SECOND_DAY___ESTIMATED_,
    26360_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    26418_i32 => _LABOR_DAY,
    26421_i32 => _VESAK_DAY__ESTIMATED_,
    26422_i32 => _VESAK_DAY__OBSERVED__ESTIMATED_,
    26450_i32 => _BIRTHDAY_OF_HM_YANG_DI_PERTUAN_AGONG,
    26540_i32 => _NATIONAL_DAY,
    26541_i32 => _NATIONAL_DAY__OBSERVED_,
    26555_i32 => _EID_AL_FITR__ESTIMATED_,
    26556_i32 => _EID_AL_FITR__SECOND_DAY___ESTIMATED___MALAYSIA_DAY,
    26624_i32 => _EID_AL_ADHA__ESTIMATED_,
    26625_i32 => _EID_AL_ADHA__OBSERVED__ESTIMATED_,
    26645_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    26656_i32 => _CHRISTMAS_DAY,
    26703_i32 => _CHINESE_NEW_YEAR__ESTIMATED_,
    26704_i32 => _CHINESE_NEW_YEAR__SECOND_DAY___ESTIMATED_,
    26715_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    26716_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__OBSERVED__ESTIMATED_,
    26783_i32 => _LABOR_DAY,
    26805_i32 => _VESAK_DAY__ESTIMATED_,
    26814_i32 => _BIRTHDAY_OF_HM_YANG_DI_PERTUAN_AGONG,
    26905_i32 => _NATIONAL_DAY,
    26909_i32 => _EID_AL_FITR__ESTIMATED_,
    26910_i32 => _EID_AL_FITR__SECOND_DAY___ESTIMATED_,
    26921_i32 => _MALAYSIA_DAY,
    26978_i32 => _EID_AL_ADHA__ESTIMATED_,
    26999_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    27021_i32 => _CHRISTMAS_DAY,
    27057_i32 => _CHINESE_NEW_YEAR__ESTIMATED_,
    27058_i32 => _CHINESE_NEW_YEAR__SECOND_DAY___ESTIMATED_,
    27059_i32 => _CHINESE_NEW_YEAR__SECOND_DAY___OBSERVED__ESTIMATED_,
    27069_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    27149_i32 => _LABOR_DAY,
    27150_i32 => _LABOR_DAY__OBSERVED_,
    27160_i32 => _VESAK_DAY__ESTIMATED_,
    27185_i32 => _BIRTHDAY_OF_HM_YANG_DI_PERTUAN_AGONG,
    27264_i32 => _EID_AL_FITR__ESTIMATED_,
    27265_i32 => _EID_AL_FITR__SECOND_DAY___ESTIMATED_,
    27271_i32 => _NATIONAL_DAY,
    27287_i32 => _MALAYSIA_DAY,
    27332_i32 => _EID_AL_ADHA__ESTIMATED_,
    27353_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    27387_i32 => _CHRISTMAS_DAY,
    27388_i32 => _CHRISTMAS_DAY__OBSERVED_,
    27423_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    27441_i32 => _CHINESE_NEW_YEAR__ESTIMATED_,
    27442_i32 => _CHINESE_NEW_YEAR__SECOND_DAY___ESTIMATED_,
    27514_i32 => _LABOR_DAY__VESAK_DAY__ESTIMATED_,
    27549_i32 => _BIRTHDAY_OF_HM_YANG_DI_PERTUAN_AGONG,
    27619_i32 => _EID_AL_FITR__ESTIMATED_,
    27620_i32 => _EID_AL_FITR__SECOND_DAY___ESTIMATED_,
    27636_i32 => _NATIONAL_DAY,
    27652_i32 => _MALAYSIA_DAY,
    27687_i32 => _EID_AL_ADHA__ESTIMATED_,
    27707_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    27752_i32 => _CHRISTMAS_DAY,
    27777_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    27795_i32 => _CHINESE_NEW_YEAR__ESTIMATED_,
    27796_i32 => _CHINESE_NEW_YEAR__SECOND_DAY___ESTIMATED_,
    27879_i32 => _LABOR_DAY,
    27898_i32 => _VESAK_DAY__ESTIMATED_,
    27899_i32 => _VESAK_DAY__OBSERVED__ESTIMATED_,
    27913_i32 => _BIRTHDAY_OF_HM_YANG_DI_PERTUAN_AGONG,
    27973_i32 => _EID_AL_FITR__ESTIMATED_,
    27974_i32 => _EID_AL_FITR__SECOND_DAY___ESTIMATED_,
    28001_i32 => _NATIONAL_DAY,
    28017_i32 => _MALAYSIA_DAY,
    28018_i32 => _MALAYSIA_DAY__OBSERVED_,
    28041_i32 => _EID_AL_ADHA__ESTIMATED_,
    28062_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    28117_i32 => _CHRISTMAS_DAY,
    28131_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    28149_i32 => _CHINESE_NEW_YEAR__ESTIMATED_,
    28150_i32 => _CHINESE_NEW_YEAR__SECOND_DAY___ESTIMATED_,
    28151_i32 => _CHINESE_NEW_YEAR__SECOND_DAY___OBSERVED__ESTIMATED_,
    28244_i32 => _LABOR_DAY,
    28252_i32 => _VESAK_DAY__ESTIMATED_,
    28277_i32 => _BIRTHDAY_OF_HM_YANG_DI_PERTUAN_AGONG,
    28328_i32 => _EID_AL_FITR__ESTIMATED_,
    28329_i32 => _EID_AL_FITR__SECOND_DAY___ESTIMATED_,
    28366_i32 => _NATIONAL_DAY,
    28382_i32 => _MALAYSIA_DAY,
    28396_i32 => _EID_AL_ADHA__ESTIMATED_,
    28416_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    28482_i32 => _CHRISTMAS_DAY,
    28486_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    28487_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__OBSERVED__ESTIMATED_,
    28533_i32 => _CHINESE_NEW_YEAR__ESTIMATED_,
    28534_i32 => _CHINESE_NEW_YEAR__SECOND_DAY___ESTIMATED_,
    28610_i32 => _LABOR_DAY,
    28636_i32 => _VESAK_DAY__ESTIMATED_,
    28641_i32 => _BIRTHDAY_OF_HM_YANG_DI_PERTUAN_AGONG,
    28682_i32 => _EID_AL_FITR__ESTIMATED_,
    28683_i32 => _EID_AL_FITR__SECOND_DAY___ESTIMATED_,
    28684_i32 => _EID_AL_FITR__OBSERVED__ESTIMATED_,
    28732_i32 => _NATIONAL_DAY,
    28748_i32 => _MALAYSIA_DAY,
    28751_i32 => _EID_AL_ADHA__ESTIMATED_,
    28771_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    28841_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    28848_i32 => _CHRISTMAS_DAY,
    28887_i32 => _CHINESE_NEW_YEAR__ESTIMATED_,
    28888_i32 => _CHINESE_NEW_YEAR__SECOND_DAY___ESTIMATED_,
    28975_i32 => _LABOR_DAY,
    28990_i32 => _VESAK_DAY__ESTIMATED_,
    28991_i32 => _VESAK_DAY__OBSERVED__ESTIMATED_,
    29012_i32 => _BIRTHDAY_OF_HM_YANG_DI_PERTUAN_AGONG,
    29036_i32 => _EID_AL_FITR__ESTIMATED_,
    29037_i32 => _EID_AL_FITR__SECOND_DAY___ESTIMATED_,
    29097_i32 => _NATIONAL_DAY,
    29105_i32 => _EID_AL_ADHA__ESTIMATED_,
    29113_i32 => _MALAYSIA_DAY,
    29125_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    29195_i32 => _PROPHET_MUHAMMAD_S_BIRTHDAY__ESTIMATED_,
    29213_i32 => _CHRISTMAS_DAY,
};
