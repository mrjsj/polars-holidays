use crate::countries::constants::*;
use phf::phf_map;

pub static ID_HOLIDAYS: phf::Map<i32, &'static str> = phf_map! {
    10957_i32 => _NEW_YEAR_S_DAY,
    10964_i32 => _EID_AL_FITR,
    10965_i32 => _EID_AL_FITR_SECOND_DAY,
    11032_i32 => _EID_AL_ADHA,
    11051_i32 => _DAY_OF_SILENCE,
    11053_i32 => _ISLAMIC_NEW_YEAR,
    11068_i32 => _GOOD_FRIDAY,
    11095_i32 => _VESAK_DAY,
    11109_i32 => _ASCENSION_DAY,
    11123_i32 => _PROPHET_S_BIRTHDAY,
    11186_i32 => _INDEPENDENCE_DAY,
    11255_i32 => _ISRA__AND_MI_RAJ,
    11316_i32 => _CHRISTMAS_DAY,
    11318_i32 => _EID_AL_FITR,
    11319_i32 => _EID_AL_FITR_SECOND_DAY,
    11323_i32 => _NEW_YEAR_S_DAY,
    11386_i32 => _EID_AL_ADHA,
    11406_i32 => _DAY_OF_SILENCE,
    11407_i32 => _ISLAMIC_NEW_YEAR,
    11425_i32 => _GOOD_FRIDAY,
    11449_i32 => _VESAK_DAY,
    11466_i32 => _ASCENSION_DAY,
    11477_i32 => _PROPHET_S_BIRTHDAY,
    11551_i32 => _INDEPENDENCE_DAY,
    11610_i32 => _ISRA__AND_MI_RAJ,
    11672_i32 => _EID_AL_FITR,
    11673_i32 => _EID_AL_FITR_SECOND_DAY,
    11681_i32 => _CHRISTMAS_DAY,
    11688_i32 => _NEW_YEAR_S_DAY,
    11741_i32 => _EID_AL_ADHA,
    11761_i32 => _ISLAMIC_NEW_YEAR,
    11775_i32 => _GOOD_FRIDAY,
    11790_i32 => _DAY_OF_SILENCE,
    11816_i32 => _ASCENSION_DAY,
    11832_i32 => _PROPHET_S_BIRTHDAY,
    11833_i32 => _VESAK_DAY,
    11916_i32 => _INDEPENDENCE_DAY,
    11964_i32 => _ISRA__AND_MI_RAJ,
    12027_i32 => _EID_AL_FITR,
    12028_i32 => _EID_AL_FITR_SECOND_DAY,
    12046_i32 => _CHRISTMAS_DAY,
    12053_i32 => _NEW_YEAR_S_DAY,
    12084_i32 => _LUNAR_NEW_YEAR,
    12095_i32 => _EID_AL_ADHA,
    12114_i32 => _ISLAMIC_NEW_YEAR,
    12144_i32 => _DAY_OF_SILENCE,
    12160_i32 => _GOOD_FRIDAY,
    12187_i32 => _PROPHET_S_BIRTHDAY,
    12188_i32 => _VESAK_DAY,
    12201_i32 => _ASCENSION_DAY,
    12281_i32 => _INDEPENDENCE_DAY,
    12317_i32 => _ISRA__AND_MI_RAJ,
    12381_i32 => _EID_AL_FITR,
    12382_i32 => _EID_AL_FITR_SECOND_DAY,
    12411_i32 => _CHRISTMAS_DAY,
    12418_i32 => _NEW_YEAR_S_DAY,
    12439_i32 => _LUNAR_NEW_YEAR,
    12450_i32 => _EID_AL_ADHA,
    12471_i32 => _ISLAMIC_NEW_YEAR,
    12499_i32 => _DAY_OF_SILENCE,
    12513_i32 => _LEGISLATIVE_ELECTION_DAY,
    12517_i32 => _GOOD_FRIDAY,
    12541_i32 => _PROPHET_S_BIRTHDAY,
    12558_i32 => _ASCENSION_DAY,
    12572_i32 => _VESAK_DAY,
    12604_i32 => _PRESIDENTIAL_ELECTION_DAY,
    12647_i32 => _INDEPENDENCE_DAY,
    12674_i32 => _ISRA__AND_MI_RAJ,
    12681_i32 => _PRESIDENTIAL_ELECTION_DAY,
    12736_i32 => _EID_AL_FITR,
    12737_i32 => _EID_AL_FITR_SECOND_DAY,
    12738_i32 => _EID_AL_FITR__OBSERVED_,
    12777_i32 => _CHRISTMAS_DAY,
    12784_i32 => _NEW_YEAR_S_DAY,
    12804_i32 => _EID_AL_ADHA,
    12823_i32 => _LUNAR_NEW_YEAR,
    12824_i32 => _ISLAMIC_NEW_YEAR,
    12853_i32 => _DAY_OF_SILENCE,
    12867_i32 => _GOOD_FRIDAY,
    12895_i32 => _PROPHET_S_BIRTHDAY,
    12908_i32 => _ASCENSION_DAY,
    12927_i32 => _VESAK_DAY,
    13012_i32 => _INDEPENDENCE_DAY,
    13028_i32 => _ISRA__AND_MI_RAJ,
    13090_i32 => _EID_AL_FITR,
    13091_i32 => _EID_AL_FITR_SECOND_DAY,
    13142_i32 => _CHRISTMAS_DAY,
    13149_i32 => _NEW_YEAR_S_DAY,
    13158_i32 => _EID_AL_ADHA,
    13178_i32 => _LUNAR_NEW_YEAR,
    13179_i32 => _ISLAMIC_NEW_YEAR,
    13237_i32 => _DAY_OF_SILENCE,
    13248_i32 => _PROPHET_S_BIRTHDAY,
    13252_i32 => _GOOD_FRIDAY,
    13281_i32 => _VESAK_DAY,
    13293_i32 => _ASCENSION_DAY,
    13377_i32 => _INDEPENDENCE_DAY,
    13381_i32 => _ISRA__AND_MI_RAJ,
    13445_i32 => _EID_AL_FITR,
    13446_i32 => _EID_AL_FITR_SECOND_DAY,
    13507_i32 => _CHRISTMAS_DAY,
    13513_i32 => _EID_AL_ADHA,
    13514_i32 => _NEW_YEAR_S_DAY,
    13533_i32 => _ISLAMIC_NEW_YEAR,
    13563_i32 => _LUNAR_NEW_YEAR,
    13591_i32 => _DAY_OF_SILENCE,
    13603_i32 => _PROPHET_S_BIRTHDAY,
    13609_i32 => _GOOD_FRIDAY,
    13650_i32 => _ASCENSION_DAY,
    13665_i32 => _VESAK_DAY,
    13736_i32 => _ISRA__AND_MI_RAJ,
    13742_i32 => _INDEPENDENCE_DAY,
    13799_i32 => _EID_AL_FITR,
    13800_i32 => _EID_AL_FITR_SECOND_DAY,
    13867_i32 => _EID_AL_ADHA,
    13872_i32 => _CHRISTMAS_DAY,
    13879_i32 => _NEW_YEAR_S_DAY,
    13888_i32 => _ISLAMIC_NEW_YEAR,
    13916_i32 => _LUNAR_NEW_YEAR,
    13945_i32 => _DAY_OF_SILENCE,
    13958_i32 => _PROPHET_S_BIRTHDAY,
    13959_i32 => _GOOD_FRIDAY,
    14000_i32 => _ASCENSION_DAY,
    14019_i32 => _VESAK_DAY,
    14090_i32 => _ISRA__AND_MI_RAJ,
    14108_i32 => _INDEPENDENCE_DAY,
    14153_i32 => _EID_AL_FITR,
    14154_i32 => _EID_AL_FITR_SECOND_DAY,
    14221_i32 => _EID_AL_ADHA,
    14238_i32 => _CHRISTMAS_DAY,
    14242_i32 => _ISLAMIC_NEW_YEAR,
    14245_i32 => _NEW_YEAR_S_DAY,
    14270_i32 => _LUNAR_NEW_YEAR,
    14312_i32 => _PROPHET_S_BIRTHDAY,
    14329_i32 => _DAY_OF_SILENCE,
    14343_i32 => _LEGISLATIVE_ELECTION_DAY,
    14344_i32 => _GOOD_FRIDAY,
    14373_i32 => _VESAK_DAY,
    14385_i32 => _ASCENSION_DAY,
    14433_i32 => _PRESIDENTIAL_ELECTION_DAY,
    14445_i32 => _ISRA__AND_MI_RAJ,
    14473_i32 => _INDEPENDENCE_DAY,
    14507_i32 => _EID_AL_FITR,
    14508_i32 => _EID_AL_FITR_SECOND_DAY,
    14575_i32 => _EID_AL_ADHA,
    14596_i32 => _ISLAMIC_NEW_YEAR,
    14603_i32 => _CHRISTMAS_DAY,
    14610_i32 => _NEW_YEAR_S_DAY,
    14655_i32 => _LUNAR_NEW_YEAR,
    14666_i32 => _PROPHET_S_BIRTHDAY,
    14684_i32 => _DAY_OF_SILENCE,
    14701_i32 => _GOOD_FRIDAY,
    14742_i32 => _ASCENSION_DAY,
    14757_i32 => _VESAK_DAY,
    14800_i32 => _ISRA__AND_MI_RAJ,
    14838_i32 => _INDEPENDENCE_DAY,
    14862_i32 => _EID_AL_FITR,
    14863_i32 => _EID_AL_FITR_SECOND_DAY,
    14930_i32 => _EID_AL_ADHA,
    14950_i32 => _ISLAMIC_NEW_YEAR,
    14968_i32 => _CHRISTMAS_DAY,
    14975_i32 => _NEW_YEAR_S_DAY,
    15008_i32 => _LUNAR_NEW_YEAR,
    15020_i32 => _PROPHET_S_BIRTHDAY,
    15038_i32 => _DAY_OF_SILENCE,
    15086_i32 => _GOOD_FRIDAY,
    15111_i32 => _VESAK_DAY,
    15127_i32 => _ASCENSION_DAY,
    15154_i32 => _ISRA__AND_MI_RAJ,
    15203_i32 => _INDEPENDENCE_DAY,
    15216_i32 => _EID_AL_FITR,
    15217_i32 => _EID_AL_FITR_SECOND_DAY,
    15284_i32 => _EID_AL_ADHA,
    15305_i32 => _ISLAMIC_NEW_YEAR,
    15333_i32 => _CHRISTMAS_DAY,
    15340_i32 => _NEW_YEAR_S_DAY,
    15362_i32 => _LUNAR_NEW_YEAR,
    15375_i32 => _PROPHET_S_BIRTHDAY,
    15422_i32 => _DAY_OF_SILENCE,
    15436_i32 => _GOOD_FRIDAY,
    15466_i32 => _VESAK_DAY,
    15477_i32 => _ASCENSION_DAY,
    15508_i32 => _ISRA__AND_MI_RAJ,
    15569_i32 => _INDEPENDENCE_DAY,
    15571_i32 => _EID_AL_FITR,
    15572_i32 => _EID_AL_FITR_SECOND_DAY,
    15639_i32 => _EID_AL_ADHA,
    15659_i32 => _ISLAMIC_NEW_YEAR,
    15699_i32 => _CHRISTMAS_DAY,
    15706_i32 => _NEW_YEAR_S_DAY,
    15729_i32 => _PROPHET_S_BIRTHDAY,
    15747_i32 => _LUNAR_NEW_YEAR,
    15776_i32 => _DAY_OF_SILENCE,
    15793_i32 => _GOOD_FRIDAY,
    15834_i32 => _ASCENSION_DAY,
    15850_i32 => _VESAK_DAY,
    15862_i32 => _ISRA__AND_MI_RAJ,
    15925_i32 => _EID_AL_FITR,
    15926_i32 => _EID_AL_FITR_SECOND_DAY,
    15934_i32 => _INDEPENDENCE_DAY,
    15993_i32 => _EID_AL_ADHA,
    16014_i32 => _ISLAMIC_NEW_YEAR,
    16064_i32 => _CHRISTMAS_DAY,
    16071_i32 => _NEW_YEAR_S_DAY,
    16084_i32 => _PROPHET_S_BIRTHDAY,
    16101_i32 => _LUNAR_NEW_YEAR,
    16160_i32 => _DAY_OF_SILENCE,
    16169_i32 => _LEGISLATIVE_ELECTION_DAY,
    16178_i32 => _GOOD_FRIDAY,
    16191_i32 => _INTERNATIONAL_LABOR_DAY,
    16205_i32 => _VESAK_DAY,
    16217_i32 => _ISRA__AND_MI_RAJ,
    16219_i32 => _ASCENSION_DAY,
    16260_i32 => _PRESIDENTIAL_ELECTION_DAY,
    16279_i32 => _EID_AL_FITR,
    16280_i32 => _EID_AL_FITR_SECOND_DAY,
    16299_i32 => _INDEPENDENCE_DAY,
    16348_i32 => _EID_AL_ADHA,
    16368_i32 => _ISLAMIC_NEW_YEAR,
    16429_i32 => _CHRISTMAS_DAY,
    16436_i32 => _NEW_YEAR_S_DAY,
    16438_i32 => _PROPHET_S_BIRTHDAY,
    16485_i32 => _LUNAR_NEW_YEAR,
    16515_i32 => _DAY_OF_SILENCE,
    16528_i32 => _GOOD_FRIDAY,
    16556_i32 => _INTERNATIONAL_LABOR_DAY,
    16569_i32 => _ASCENSION_DAY,
    16571_i32 => _ISRA__AND_MI_RAJ,
    16588_i32 => _VESAK_DAY,
    16633_i32 => _EID_AL_FITR,
    16634_i32 => _EID_AL_FITR_SECOND_DAY,
    16664_i32 => _INDEPENDENCE_DAY,
    16702_i32 => _EID_AL_ADHA,
    16722_i32 => _ISLAMIC_NEW_YEAR,
    16778_i32 => _LOCAL_ELECTION_DAY,
    16793_i32 => _PROPHET_S_BIRTHDAY,
    16794_i32 => _CHRISTMAS_DAY,
    16801_i32 => _NEW_YEAR_S_DAY,
    16839_i32 => _LUNAR_NEW_YEAR,
    16869_i32 => _DAY_OF_SILENCE,
    16885_i32 => _GOOD_FRIDAY,
    16922_i32 => _INTERNATIONAL_LABOR_DAY,
    16926_i32 => _ASCENSION_DAY,
    16927_i32 => _ISRA__AND_MI_RAJ,
    16943_i32 => _VESAK_DAY,
    16953_i32 => _PANCASILA_DAY,
    16988_i32 => _EID_AL_FITR,
    16989_i32 => _EID_AL_FITR_SECOND_DAY,
    17030_i32 => _INDEPENDENCE_DAY,
    17056_i32 => _EID_AL_ADHA,
    17076_i32 => _ISLAMIC_NEW_YEAR,
    17147_i32 => _PROPHET_S_BIRTHDAY,
    17160_i32 => _CHRISTMAS_DAY,
    17167_i32 => _NEW_YEAR_S_DAY,
    17194_i32 => _LUNAR_NEW_YEAR,
    17212_i32 => _LOCAL_ELECTION_DAY,
    17253_i32 => _DAY_OF_SILENCE,
    17270_i32 => _GOOD_FRIDAY,
    17280_i32 => _ISRA__AND_MI_RAJ,
    17287_i32 => _INTERNATIONAL_LABOR_DAY,
    17297_i32 => _VESAK_DAY,
    17311_i32 => _ASCENSION_DAY,
    17318_i32 => _PANCASILA_DAY,
    17342_i32 => _EID_AL_FITR,
    17343_i32 => _EID_AL_FITR_SECOND_DAY,
    17395_i32 => _INDEPENDENCE_DAY,
    17410_i32 => _EID_AL_ADHA,
    17430_i32 => _ISLAMIC_NEW_YEAR,
    17501_i32 => _PROPHET_S_BIRTHDAY,
    17525_i32 => _CHRISTMAS_DAY,
    17532_i32 => _NEW_YEAR_S_DAY,
    17578_i32 => _LUNAR_NEW_YEAR,
    17607_i32 => _DAY_OF_SILENCE,
    17620_i32 => _GOOD_FRIDAY,
    17635_i32 => _ISRA__AND_MI_RAJ,
    17652_i32 => _INTERNATIONAL_LABOR_DAY,
    17661_i32 => _ASCENSION_DAY,
    17680_i32 => _VESAK_DAY,
    17683_i32 => _PANCASILA_DAY,
    17697_i32 => _EID_AL_FITR,
    17698_i32 => _EID_AL_FITR_SECOND_DAY,
    17709_i32 => _LOCAL_ELECTION_DAY,
    17760_i32 => _INDEPENDENCE_DAY,
    17765_i32 => _EID_AL_ADHA,
    17785_i32 => _ISLAMIC_NEW_YEAR,
    17855_i32 => _PROPHET_S_BIRTHDAY,
    17890_i32 => _CHRISTMAS_DAY,
    17897_i32 => _NEW_YEAR_S_DAY,
    17932_i32 => _LUNAR_NEW_YEAR,
    17962_i32 => _DAY_OF_SILENCE,
    17989_i32 => _ISRA__AND_MI_RAJ,
    18003_i32 => _GENERAL_ELECTION_DAY,
    18005_i32 => _GOOD_FRIDAY,
    18017_i32 => _INTERNATIONAL_LABOR_DAY,
    18035_i32 => _VESAK_DAY,
    18046_i32 => _ASCENSION_DAY,
    18048_i32 => _PANCASILA_DAY,
    18052_i32 => _EID_AL_FITR,
    18053_i32 => _EID_AL_FITR_SECOND_DAY,
    18119_i32 => _EID_AL_ADHA,
    18125_i32 => _INDEPENDENCE_DAY,
    18140_i32 => _ISLAMIC_NEW_YEAR,
    18209_i32 => _PROPHET_S_BIRTHDAY,
    18255_i32 => _CHRISTMAS_DAY,
    18262_i32 => _NEW_YEAR_S_DAY,
    18286_i32 => _LUNAR_NEW_YEAR,
    18343_i32 => _ISRA__AND_MI_RAJ,
    18346_i32 => _DAY_OF_SILENCE,
    18362_i32 => _GOOD_FRIDAY,
    18383_i32 => _INTERNATIONAL_LABOR_DAY,
    18389_i32 => _VESAK_DAY,
    18403_i32 => _ASCENSION_DAY,
    18406_i32 => _EID_AL_FITR,
    18407_i32 => _EID_AL_FITR_SECOND_DAY,
    18414_i32 => _PANCASILA_DAY,
    18474_i32 => _EID_AL_ADHA,
    18491_i32 => _INDEPENDENCE_DAY,
    18494_i32 => _ISLAMIC_NEW_YEAR,
    18564_i32 => _PROPHET_S_BIRTHDAY,
    18605_i32 => _LOCAL_ELECTION_DAY,
    18621_i32 => _CHRISTMAS_DAY,
    18628_i32 => _NEW_YEAR_S_DAY,
    18670_i32 => _LUNAR_NEW_YEAR,
    18697_i32 => _ISRA__AND_MI_RAJ,
    18700_i32 => _DAY_OF_SILENCE,
    18719_i32 => _GOOD_FRIDAY,
    18748_i32 => _INTERNATIONAL_LABOR_DAY,
    18760_i32 => _ASCENSION_DAY__EID_AL_FITR,
    18761_i32 => _EID_AL_FITR_SECOND_DAY,
    18773_i32 => _VESAK_DAY,
    18779_i32 => _PANCASILA_DAY,
    18828_i32 => _EID_AL_ADHA,
    18850_i32 => _ISLAMIC_NEW_YEAR,
    18856_i32 => _INDEPENDENCE_DAY,
    18920_i32 => _PROPHET_S_BIRTHDAY,
    18986_i32 => _CHRISTMAS_DAY,
    18993_i32 => _NEW_YEAR_S_DAY,
    19024_i32 => _LUNAR_NEW_YEAR,
    19051_i32 => _ISRA__AND_MI_RAJ,
    19054_i32 => _DAY_OF_SILENCE,
    19097_i32 => _GOOD_FRIDAY,
    19113_i32 => _INTERNATIONAL_LABOR_DAY,
    19114_i32 => _EID_AL_FITR,
    19115_i32 => _EID_AL_FITR_SECOND_DAY,
    19128_i32 => _VESAK_DAY,
    19138_i32 => _ASCENSION_DAY,
    19144_i32 => _PANCASILA_DAY,
    19183_i32 => _EID_AL_ADHA,
    19203_i32 => _ISLAMIC_NEW_YEAR,
    19221_i32 => _INDEPENDENCE_DAY,
    19273_i32 => _PROPHET_S_BIRTHDAY,
    19351_i32 => _CHRISTMAS_DAY,
    19358_i32 => _NEW_YEAR_S_DAY,
    19379_i32 => _LUNAR_NEW_YEAR,
    19406_i32 => _ISRA__AND_MI_RAJ,
    19438_i32 => _DAY_OF_SILENCE,
    19454_i32 => _GOOD_FRIDAY,
    19469_i32 => _EID_AL_FITR,
    19470_i32 => _EID_AL_FITR_SECOND_DAY,
    19478_i32 => _INTERNATIONAL_LABOR_DAY,
    19495_i32 => _ASCENSION_DAY,
    19509_i32 => _PANCASILA_DAY,
    19512_i32 => _VESAK_DAY,
    19537_i32 => _EID_AL_ADHA,
    19557_i32 => _ISLAMIC_NEW_YEAR,
    19586_i32 => _INDEPENDENCE_DAY,
    19628_i32 => _PROPHET_S_BIRTHDAY,
    19716_i32 => _CHRISTMAS_DAY,
    19723_i32 => _NEW_YEAR_S_DAY,
    19761_i32 => _ISRA__AND_MI_RAJ,
    19763_i32 => _LUNAR_NEW_YEAR,
    19767_i32 => _GENERAL_ELECTION_DAY,
    19793_i32 => _DAY_OF_SILENCE,
    19811_i32 => _GOOD_FRIDAY,
    19813_i32 => _EASTER_SUNDAY,
    19823_i32 => _EID_AL_FITR,
    19824_i32 => _EID_AL_FITR_SECOND_DAY,
    19844_i32 => _INTERNATIONAL_LABOR_DAY,
    19852_i32 => _ASCENSION_DAY,
    19866_i32 => _VESAK_DAY,
    19875_i32 => _PANCASILA_DAY,
    19891_i32 => _EID_AL_ADHA,
    19911_i32 => _ISLAMIC_NEW_YEAR,
    19952_i32 => _INDEPENDENCE_DAY,
    19982_i32 => _PROPHET_S_BIRTHDAY,
    20054_i32 => _LOCAL_ELECTION_DAY,
    20082_i32 => _CHRISTMAS_DAY,
    20089_i32 => _NEW_YEAR_S_DAY,
    20115_i32 => _ISRA__AND_MI_RAJ,
    20117_i32 => _LUNAR_NEW_YEAR,
    20176_i32 => _DAY_OF_SILENCE,
    20178_i32 => _EID_AL_FITR,
    20179_i32 => _EID_AL_FITR_SECOND_DAY,
    20196_i32 => _GOOD_FRIDAY,
    20198_i32 => _EASTER_SUNDAY,
    20209_i32 => _INTERNATIONAL_LABOR_DAY,
    20220_i32 => _VESAK_DAY,
    20237_i32 => _ASCENSION_DAY,
    20240_i32 => _PANCASILA_DAY,
    20245_i32 => _EID_AL_ADHA,
    20266_i32 => _ISLAMIC_NEW_YEAR,
    20317_i32 => _INDEPENDENCE_DAY,
    20336_i32 => _PROPHET_S_BIRTHDAY,
    20447_i32 => _CHRISTMAS_DAY,
    20454_i32 => _NEW_YEAR_S_DAY,
    20469_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    20501_i32 => _LUNAR_NEW_YEAR__ESTIMATED_,
    20531_i32 => _DAY_OF_SILENCE,
    20532_i32 => _EID_AL_FITR__ESTIMATED_,
    20533_i32 => _EID_AL_FITR_SECOND_DAY__ESTIMATED_,
    20546_i32 => _GOOD_FRIDAY,
    20548_i32 => _EASTER_SUNDAY,
    20574_i32 => _INTERNATIONAL_LABOR_DAY,
    20587_i32 => _ASCENSION_DAY,
    20600_i32 => _EID_AL_ADHA__ESTIMATED_,
    20604_i32 => _VESAK_DAY__ESTIMATED_,
    20605_i32 => _PANCASILA_DAY,
    20620_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    20682_i32 => _INDEPENDENCE_DAY,
    20690_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    20812_i32 => _CHRISTMAS_DAY,
    20819_i32 => _NEW_YEAR_S_DAY,
    20823_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    20855_i32 => _LUNAR_NEW_YEAR__ESTIMATED_,
    20885_i32 => _DAY_OF_SILENCE,
    20886_i32 => _EID_AL_FITR__ESTIMATED_,
    20887_i32 => _EID_AL_FITR_SECOND_DAY__ESTIMATED_,
    20903_i32 => _GOOD_FRIDAY,
    20905_i32 => _EASTER_SUNDAY,
    20939_i32 => _INTERNATIONAL_LABOR_DAY,
    20944_i32 => _ASCENSION_DAY,
    20954_i32 => _EID_AL_ADHA__ESTIMATED_,
    20958_i32 => _VESAK_DAY__ESTIMATED_,
    20970_i32 => _PANCASILA_DAY,
    20975_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    21044_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    21047_i32 => _INDEPENDENCE_DAY,
    21177_i32 => _CHRISTMAS_DAY__ISRA__AND_MI_RAJ__ESTIMATED_,
    21184_i32 => _NEW_YEAR_S_DAY,
    21209_i32 => _LUNAR_NEW_YEAR__ESTIMATED_,
    21240_i32 => _EID_AL_FITR__ESTIMATED_,
    21241_i32 => _EID_AL_FITR_SECOND_DAY__ESTIMATED_,
    21269_i32 => _DAY_OF_SILENCE,
    21288_i32 => _GOOD_FRIDAY,
    21290_i32 => _EASTER_SUNDAY,
    21305_i32 => _INTERNATIONAL_LABOR_DAY,
    21309_i32 => _EID_AL_ADHA__ESTIMATED_,
    21313_i32 => _VESAK_DAY__ESTIMATED_,
    21329_i32 => _ASCENSION_DAY__ISLAMIC_NEW_YEAR__ESTIMATED_,
    21336_i32 => _PANCASILA_DAY,
    21399_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    21413_i32 => _INDEPENDENCE_DAY,
    21532_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    21543_i32 => _CHRISTMAS_DAY,
    21550_i32 => _NEW_YEAR_S_DAY,
    21593_i32 => _LUNAR_NEW_YEAR__ESTIMATED_,
    21594_i32 => _EID_AL_FITR__ESTIMATED_,
    21595_i32 => _EID_AL_FITR_SECOND_DAY__ESTIMATED_,
    21623_i32 => _DAY_OF_SILENCE,
    21638_i32 => _GOOD_FRIDAY,
    21640_i32 => _EASTER_SUNDAY,
    21663_i32 => _EID_AL_ADHA__ESTIMATED_,
    21670_i32 => _INTERNATIONAL_LABOR_DAY,
    21679_i32 => _ASCENSION_DAY,
    21683_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    21696_i32 => _VESAK_DAY__ESTIMATED_,
    21701_i32 => _PANCASILA_DAY,
    21754_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    21778_i32 => _INDEPENDENCE_DAY,
    21886_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    21908_i32 => _CHRISTMAS_DAY,
    21915_i32 => _NEW_YEAR_S_DAY,
    21948_i32 => _LUNAR_NEW_YEAR__ESTIMATED_,
    21949_i32 => _EID_AL_FITR__ESTIMATED_,
    21950_i32 => _EID_AL_FITR_SECOND_DAY__ESTIMATED_,
    21978_i32 => _DAY_OF_SILENCE,
    22017_i32 => _EID_AL_ADHA__ESTIMATED_,
    22023_i32 => _GOOD_FRIDAY,
    22025_i32 => _EASTER_SUNDAY,
    22035_i32 => _INTERNATIONAL_LABOR_DAY,
    22037_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    22050_i32 => _VESAK_DAY__ESTIMATED_,
    22064_i32 => _ASCENSION_DAY,
    22066_i32 => _PANCASILA_DAY,
    22108_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    22143_i32 => _INDEPENDENCE_DAY,
    22241_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    22273_i32 => _CHRISTMAS_DAY,
    22280_i32 => _NEW_YEAR_S_DAY,
    22302_i32 => _LUNAR_NEW_YEAR__ESTIMATED_,
    22303_i32 => _EID_AL_FITR__ESTIMATED_,
    22304_i32 => _EID_AL_FITR_SECOND_DAY__ESTIMATED_,
    22371_i32 => _EID_AL_ADHA__ESTIMATED_,
    22380_i32 => _GOOD_FRIDAY,
    22382_i32 => _EASTER_SUNDAY,
    22392_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    22400_i32 => _INTERNATIONAL_LABOR_DAY,
    22421_i32 => _ASCENSION_DAY,
    22431_i32 => _PANCASILA_DAY,
    22434_i32 => _VESAK_DAY__ESTIMATED_,
    22462_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    22508_i32 => _INDEPENDENCE_DAY,
    22595_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    22638_i32 => _CHRISTMAS_DAY,
    22645_i32 => _NEW_YEAR_S_DAY,
    22658_i32 => _EID_AL_FITR__ESTIMATED_,
    22659_i32 => _EID_AL_FITR_SECOND_DAY__ESTIMATED_,
    22686_i32 => _LUNAR_NEW_YEAR__ESTIMATED_,
    22726_i32 => _EID_AL_ADHA__ESTIMATED_,
    22730_i32 => _GOOD_FRIDAY,
    22732_i32 => _EASTER_SUNDAY,
    22746_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    22766_i32 => _INTERNATIONAL_LABOR_DAY,
    22771_i32 => _ASCENSION_DAY,
    22788_i32 => _VESAK_DAY__ESTIMATED_,
    22797_i32 => _PANCASILA_DAY,
    22816_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    22874_i32 => _INDEPENDENCE_DAY,
    22950_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    23004_i32 => _CHRISTMAS_DAY,
    23011_i32 => _NEW_YEAR_S_DAY,
    23012_i32 => _EID_AL_FITR__ESTIMATED_,
    23013_i32 => _EID_AL_FITR_SECOND_DAY__ESTIMATED_,
    23041_i32 => _LUNAR_NEW_YEAR__ESTIMATED_,
    23080_i32 => _EID_AL_ADHA__ESTIMATED_,
    23101_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    23115_i32 => _GOOD_FRIDAY,
    23117_i32 => _EASTER_SUNDAY,
    23131_i32 => _INTERNATIONAL_LABOR_DAY,
    23143_i32 => _VESAK_DAY__ESTIMATED_,
    23156_i32 => _ASCENSION_DAY,
    23162_i32 => _PANCASILA_DAY,
    23170_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    23239_i32 => _INDEPENDENCE_DAY,
    23304_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    23367_i32 => _EID_AL_FITR__ESTIMATED_,
    23368_i32 => _EID_AL_FITR_SECOND_DAY__ESTIMATED_,
    23369_i32 => _CHRISTMAS_DAY,
    23376_i32 => _NEW_YEAR_S_DAY,
    23425_i32 => _LUNAR_NEW_YEAR__ESTIMATED_,
    23435_i32 => _EID_AL_ADHA__ESTIMATED_,
    23455_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    23472_i32 => _GOOD_FRIDAY,
    23474_i32 => _EASTER_SUNDAY,
    23496_i32 => _INTERNATIONAL_LABOR_DAY,
    23513_i32 => _ASCENSION_DAY,
    23525_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    23527_i32 => _PANCASILA_DAY__VESAK_DAY__ESTIMATED_,
    23604_i32 => _INDEPENDENCE_DAY,
    23658_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    23721_i32 => _EID_AL_FITR__ESTIMATED_,
    23722_i32 => _EID_AL_FITR_SECOND_DAY__ESTIMATED_,
    23734_i32 => _CHRISTMAS_DAY,
    23741_i32 => _NEW_YEAR_S_DAY,
    23779_i32 => _LUNAR_NEW_YEAR__ESTIMATED_,
    23789_i32 => _EID_AL_ADHA__ESTIMATED_,
    23810_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    23822_i32 => _GOOD_FRIDAY,
    23824_i32 => _EASTER_SUNDAY,
    23861_i32 => _INTERNATIONAL_LABOR_DAY,
    23863_i32 => _ASCENSION_DAY,
    23880_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    23882_i32 => _VESAK_DAY__ESTIMATED_,
    23892_i32 => _PANCASILA_DAY,
    23969_i32 => _INDEPENDENCE_DAY,
    24012_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    24075_i32 => _EID_AL_FITR__ESTIMATED_,
    24076_i32 => _EID_AL_FITR_SECOND_DAY__ESTIMATED_,
    24099_i32 => _CHRISTMAS_DAY,
    24106_i32 => _NEW_YEAR_S_DAY,
    24133_i32 => _LUNAR_NEW_YEAR__ESTIMATED_,
    24143_i32 => _EID_AL_ADHA__ESTIMATED_,
    24164_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    24207_i32 => _GOOD_FRIDAY,
    24209_i32 => _EASTER_SUNDAY,
    24227_i32 => _INTERNATIONAL_LABOR_DAY,
    24234_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    24236_i32 => _VESAK_DAY__ESTIMATED_,
    24248_i32 => _ASCENSION_DAY,
    24258_i32 => _PANCASILA_DAY,
    24335_i32 => _INDEPENDENCE_DAY,
    24367_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    24429_i32 => _EID_AL_FITR__ESTIMATED_,
    24430_i32 => _EID_AL_FITR_SECOND_DAY__ESTIMATED_,
    24465_i32 => _CHRISTMAS_DAY,
    24472_i32 => _NEW_YEAR_S_DAY,
    24497_i32 => _EID_AL_ADHA__ESTIMATED_,
    24517_i32 => _LUNAR_NEW_YEAR__ESTIMATED_,
    24518_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    24564_i32 => _GOOD_FRIDAY,
    24566_i32 => _EASTER_SUNDAY,
    24589_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    24592_i32 => _INTERNATIONAL_LABOR_DAY,
    24605_i32 => _ASCENSION_DAY,
    24620_i32 => _VESAK_DAY__ESTIMATED_,
    24623_i32 => _PANCASILA_DAY,
    24700_i32 => _INDEPENDENCE_DAY,
    24721_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    24783_i32 => _EID_AL_FITR__ESTIMATED_,
    24784_i32 => _EID_AL_FITR_SECOND_DAY__ESTIMATED_,
    24830_i32 => _CHRISTMAS_DAY,
    24837_i32 => _NEW_YEAR_S_DAY,
    24852_i32 => _EID_AL_ADHA__ESTIMATED_,
    24871_i32 => _LUNAR_NEW_YEAR__ESTIMATED_,
    24872_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    24943_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    24949_i32 => _GOOD_FRIDAY,
    24951_i32 => _EASTER_SUNDAY,
    24957_i32 => _INTERNATIONAL_LABOR_DAY,
    24974_i32 => _VESAK_DAY__ESTIMATED_,
    24988_i32 => _PANCASILA_DAY,
    24990_i32 => _ASCENSION_DAY,
    25065_i32 => _INDEPENDENCE_DAY,
    25076_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    25138_i32 => _EID_AL_FITR__ESTIMATED_,
    25139_i32 => _EID_AL_FITR_SECOND_DAY__ESTIMATED_,
    25195_i32 => _CHRISTMAS_DAY,
    25202_i32 => _NEW_YEAR_S_DAY,
    25206_i32 => _EID_AL_ADHA__ESTIMATED_,
    25225_i32 => _LUNAR_NEW_YEAR__ESTIMATED_,
    25227_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    25297_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    25299_i32 => _GOOD_FRIDAY,
    25301_i32 => _EASTER_SUNDAY,
    25322_i32 => _INTERNATIONAL_LABOR_DAY,
    25328_i32 => _VESAK_DAY__ESTIMATED_,
    25340_i32 => _ASCENSION_DAY,
    25353_i32 => _PANCASILA_DAY,
    25430_i32 => _INDEPENDENCE_DAY__ISRA__AND_MI_RAJ__ESTIMATED_,
    25493_i32 => _EID_AL_FITR__ESTIMATED_,
    25494_i32 => _EID_AL_FITR_SECOND_DAY__ESTIMATED_,
    25560_i32 => _CHRISTMAS_DAY,
    25561_i32 => _EID_AL_ADHA__ESTIMATED_,
    25567_i32 => _NEW_YEAR_S_DAY,
    25581_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    25609_i32 => _LUNAR_NEW_YEAR__ESTIMATED_,
    25651_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    25656_i32 => _GOOD_FRIDAY,
    25658_i32 => _EASTER_SUNDAY,
    25688_i32 => _INTERNATIONAL_LABOR_DAY,
    25697_i32 => _ASCENSION_DAY,
    25712_i32 => _VESAK_DAY__ESTIMATED_,
    25719_i32 => _PANCASILA_DAY,
    25784_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    25796_i32 => _INDEPENDENCE_DAY,
    25847_i32 => _EID_AL_FITR__ESTIMATED_,
    25848_i32 => _EID_AL_FITR_SECOND_DAY__ESTIMATED_,
    25915_i32 => _EID_AL_ADHA__ESTIMATED_,
    25926_i32 => _CHRISTMAS_DAY,
    25933_i32 => _NEW_YEAR_S_DAY,
    25936_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    25964_i32 => _LUNAR_NEW_YEAR__ESTIMATED_,
    26006_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    26041_i32 => _GOOD_FRIDAY,
    26043_i32 => _EASTER_SUNDAY,
    26053_i32 => _INTERNATIONAL_LABOR_DAY,
    26066_i32 => _VESAK_DAY__ESTIMATED_,
    26082_i32 => _ASCENSION_DAY,
    26084_i32 => _PANCASILA_DAY,
    26138_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    26161_i32 => _INDEPENDENCE_DAY,
    26201_i32 => _EID_AL_FITR__ESTIMATED_,
    26202_i32 => _EID_AL_FITR_SECOND_DAY__ESTIMATED_,
    26270_i32 => _EID_AL_ADHA__ESTIMATED_,
    26290_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    26291_i32 => _CHRISTMAS_DAY,
    26298_i32 => _NEW_YEAR_S_DAY,
    26319_i32 => _LUNAR_NEW_YEAR__ESTIMATED_,
    26360_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    26391_i32 => _GOOD_FRIDAY,
    26393_i32 => _EASTER_SUNDAY,
    26418_i32 => _INTERNATIONAL_LABOR_DAY,
    26432_i32 => _ASCENSION_DAY,
    26449_i32 => _PANCASILA_DAY,
    26450_i32 => _VESAK_DAY__ESTIMATED_,
    26493_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    26526_i32 => _INDEPENDENCE_DAY,
    26555_i32 => _EID_AL_FITR__ESTIMATED_,
    26556_i32 => _EID_AL_FITR_SECOND_DAY__ESTIMATED_,
    26624_i32 => _EID_AL_ADHA__ESTIMATED_,
    26645_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    26656_i32 => _CHRISTMAS_DAY,
    26663_i32 => _NEW_YEAR_S_DAY,
    26703_i32 => _LUNAR_NEW_YEAR__ESTIMATED_,
    26715_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    26748_i32 => _GOOD_FRIDAY,
    26750_i32 => _EASTER_SUNDAY,
    26783_i32 => _INTERNATIONAL_LABOR_DAY,
    26789_i32 => _ASCENSION_DAY,
    26805_i32 => _VESAK_DAY__ESTIMATED_,
    26814_i32 => _PANCASILA_DAY,
    26847_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    26891_i32 => _INDEPENDENCE_DAY,
    26909_i32 => _EID_AL_FITR__ESTIMATED_,
    26910_i32 => _EID_AL_FITR_SECOND_DAY__ESTIMATED_,
    26978_i32 => _EID_AL_ADHA__ESTIMATED_,
    26999_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    27021_i32 => _CHRISTMAS_DAY,
    27028_i32 => _NEW_YEAR_S_DAY,
    27057_i32 => _LUNAR_NEW_YEAR__ESTIMATED_,
    27069_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    27133_i32 => _GOOD_FRIDAY,
    27135_i32 => _EASTER_SUNDAY,
    27149_i32 => _INTERNATIONAL_LABOR_DAY,
    27160_i32 => _VESAK_DAY__ESTIMATED_,
    27174_i32 => _ASCENSION_DAY,
    27180_i32 => _PANCASILA_DAY,
    27202_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    27257_i32 => _INDEPENDENCE_DAY,
    27264_i32 => _EID_AL_FITR__ESTIMATED_,
    27265_i32 => _EID_AL_FITR_SECOND_DAY__ESTIMATED_,
    27332_i32 => _EID_AL_ADHA__ESTIMATED_,
    27353_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    27387_i32 => _CHRISTMAS_DAY,
    27394_i32 => _NEW_YEAR_S_DAY,
    27423_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    27441_i32 => _LUNAR_NEW_YEAR__ESTIMATED_,
    27490_i32 => _GOOD_FRIDAY,
    27492_i32 => _EASTER_SUNDAY,
    27514_i32 => _INTERNATIONAL_LABOR_DAY,
    27531_i32 => _ASCENSION_DAY,
    27544_i32 => _VESAK_DAY__ESTIMATED_,
    27545_i32 => _PANCASILA_DAY,
    27557_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    27619_i32 => _EID_AL_FITR__ESTIMATED_,
    27620_i32 => _EID_AL_FITR_SECOND_DAY__ESTIMATED_,
    27622_i32 => _INDEPENDENCE_DAY,
    27687_i32 => _EID_AL_ADHA__ESTIMATED_,
    27707_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    27752_i32 => _CHRISTMAS_DAY,
    27759_i32 => _NEW_YEAR_S_DAY,
    27777_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    27795_i32 => _LUNAR_NEW_YEAR__ESTIMATED_,
    27840_i32 => _GOOD_FRIDAY,
    27842_i32 => _EASTER_SUNDAY,
    27879_i32 => _INTERNATIONAL_LABOR_DAY,
    27881_i32 => _ASCENSION_DAY,
    27898_i32 => _VESAK_DAY__ESTIMATED_,
    27910_i32 => _PANCASILA_DAY,
    27911_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    27973_i32 => _EID_AL_FITR__ESTIMATED_,
    27974_i32 => _EID_AL_FITR_SECOND_DAY__ESTIMATED_,
    27987_i32 => _INDEPENDENCE_DAY,
    28041_i32 => _EID_AL_ADHA__ESTIMATED_,
    28062_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    28117_i32 => _CHRISTMAS_DAY,
    28124_i32 => _NEW_YEAR_S_DAY,
    28131_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    28149_i32 => _LUNAR_NEW_YEAR__ESTIMATED_,
    28225_i32 => _GOOD_FRIDAY,
    28227_i32 => _EASTER_SUNDAY,
    28244_i32 => _INTERNATIONAL_LABOR_DAY,
    28252_i32 => _VESAK_DAY__ESTIMATED_,
    28265_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    28266_i32 => _ASCENSION_DAY,
    28275_i32 => _PANCASILA_DAY,
    28328_i32 => _EID_AL_FITR__ESTIMATED_,
    28329_i32 => _EID_AL_FITR_SECOND_DAY__ESTIMATED_,
    28352_i32 => _INDEPENDENCE_DAY,
    28396_i32 => _EID_AL_ADHA__ESTIMATED_,
    28416_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    28482_i32 => _CHRISTMAS_DAY,
    28486_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    28489_i32 => _NEW_YEAR_S_DAY,
    28533_i32 => _LUNAR_NEW_YEAR__ESTIMATED_,
    28582_i32 => _GOOD_FRIDAY,
    28584_i32 => _EASTER_SUNDAY,
    28610_i32 => _INTERNATIONAL_LABOR_DAY,
    28619_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    28623_i32 => _ASCENSION_DAY,
    28636_i32 => _VESAK_DAY__ESTIMATED_,
    28641_i32 => _PANCASILA_DAY,
    28682_i32 => _EID_AL_FITR__ESTIMATED_,
    28683_i32 => _EID_AL_FITR_SECOND_DAY__ESTIMATED_,
    28718_i32 => _INDEPENDENCE_DAY,
    28751_i32 => _EID_AL_ADHA__ESTIMATED_,
    28771_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    28841_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    28848_i32 => _CHRISTMAS_DAY,
    28855_i32 => _NEW_YEAR_S_DAY,
    28887_i32 => _LUNAR_NEW_YEAR__ESTIMATED_,
    28960_i32 => _GOOD_FRIDAY,
    28962_i32 => _EASTER_SUNDAY,
    28973_i32 => _ISRA__AND_MI_RAJ__ESTIMATED_,
    28975_i32 => _INTERNATIONAL_LABOR_DAY,
    28990_i32 => _VESAK_DAY__ESTIMATED_,
    29001_i32 => _ASCENSION_DAY,
    29006_i32 => _PANCASILA_DAY,
    29036_i32 => _EID_AL_FITR__ESTIMATED_,
    29037_i32 => _EID_AL_FITR_SECOND_DAY__ESTIMATED_,
    29083_i32 => _INDEPENDENCE_DAY,
    29105_i32 => _EID_AL_ADHA__ESTIMATED_,
    29125_i32 => _ISLAMIC_NEW_YEAR__ESTIMATED_,
    29195_i32 => _PROPHET_S_BIRTHDAY__ESTIMATED_,
    29213_i32 => _CHRISTMAS_DAY,
    29220_i32 => _NEW_YEAR_S_DAY,
};
