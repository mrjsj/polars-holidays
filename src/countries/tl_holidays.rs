use crate::countries::constants::*;
use phf::phf_map;

pub static TL_HOLIDAYS: phf::Map<i32, &'static str> = phf_map! {
    13149_i32 => _NEW_YEAR_S_DAY,
    13158_i32 => _EID_AL_ADHA__ESTIMATED_,
    13252_i32 => _HOLY_FRIDAY,
    13269_i32 => _WORLD_LABOR_DAY,
    13288_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    13314_i32 => _CORPUS_CHRISTI,
    13390_i32 => _POPULAR_CONSULTATION_DAY,
    13444_i32 => _EID_AL_FITR__ESTIMATED_,
    13453_i32 => _ALL_SAINTS__DAY,
    13454_i32 => _ALL_SOULS__DAY,
    13464_i32 => _NATIONAL_YOUTH_DAY,
    13480_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    13489_i32 => _NATIONAL_HEROES_DAY,
    13490_i32 => _DAY_OF_OUR_LADY_OF_IMMACULATE_CONCEPTION_AND_TIMOR_LESTE_PATRONESS,
    13507_i32 => _CHRISTMAS_DAY,
    13513_i32 => _EID_AL_ADHA__ESTIMATED_,
    13514_i32 => _NEW_YEAR_S_DAY,
    13609_i32 => _HOLY_FRIDAY,
    13634_i32 => _WORLD_LABOR_DAY,
    13653_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    13671_i32 => _CORPUS_CHRISTI,
    13755_i32 => _POPULAR_CONSULTATION_DAY,
    13799_i32 => _EID_AL_FITR__ESTIMATED_,
    13818_i32 => _ALL_SAINTS__DAY,
    13819_i32 => _ALL_SOULS__DAY,
    13829_i32 => _NATIONAL_YOUTH_DAY,
    13845_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    13854_i32 => _NATIONAL_HEROES_DAY,
    13855_i32 => _DAY_OF_OUR_LADY_OF_IMMACULATE_CONCEPTION_AND_TIMOR_LESTE_PATRONESS,
    13867_i32 => _EID_AL_ADHA__ESTIMATED_,
    13872_i32 => _CHRISTMAS_DAY,
    13879_i32 => _NEW_YEAR_S_DAY,
    13959_i32 => _HOLY_FRIDAY,
    14000_i32 => _WORLD_LABOR_DAY,
    14019_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    14021_i32 => _CORPUS_CHRISTI,
    14121_i32 => _POPULAR_CONSULTATION_DAY,
    14153_i32 => _EID_AL_FITR__ESTIMATED_,
    14184_i32 => _ALL_SAINTS__DAY,
    14185_i32 => _ALL_SOULS__DAY,
    14195_i32 => _NATIONAL_YOUTH_DAY,
    14211_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    14220_i32 => _NATIONAL_HEROES_DAY,
    14221_i32 => _DAY_OF_OUR_LADY_OF_IMMACULATE_CONCEPTION_AND_TIMOR_LESTE_PATRONESS__EID_AL_ADHA__ESTIMATED_,
    14238_i32 => _CHRISTMAS_DAY,
    14245_i32 => _NEW_YEAR_S_DAY,
    14344_i32 => _HOLY_FRIDAY,
    14365_i32 => _WORLD_LABOR_DAY,
    14384_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    14406_i32 => _CORPUS_CHRISTI,
    14486_i32 => _POPULAR_CONSULTATION_DAY,
    14507_i32 => _EID_AL_FITR__ESTIMATED_,
    14549_i32 => _ALL_SAINTS__DAY,
    14550_i32 => _ALL_SOULS__DAY,
    14560_i32 => _NATIONAL_YOUTH_DAY,
    14575_i32 => _EID_AL_ADHA__ESTIMATED_,
    14576_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    14585_i32 => _NATIONAL_HEROES_DAY,
    14586_i32 => _DAY_OF_OUR_LADY_OF_IMMACULATE_CONCEPTION_AND_TIMOR_LESTE_PATRONESS,
    14603_i32 => _CHRISTMAS_DAY,
    14610_i32 => _NEW_YEAR_S_DAY,
    14701_i32 => _HOLY_FRIDAY,
    14730_i32 => _WORLD_LABOR_DAY,
    14749_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    14763_i32 => _CORPUS_CHRISTI,
    14851_i32 => _POPULAR_CONSULTATION_DAY,
    14862_i32 => _EID_AL_FITR__ESTIMATED_,
    14914_i32 => _ALL_SAINTS__DAY,
    14915_i32 => _ALL_SOULS__DAY,
    14925_i32 => _NATIONAL_YOUTH_DAY,
    14929_i32 => _EID_AL_ADHA__ESTIMATED_,
    14941_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    14950_i32 => _NATIONAL_HEROES_DAY,
    14951_i32 => _DAY_OF_OUR_LADY_OF_IMMACULATE_CONCEPTION_AND_TIMOR_LESTE_PATRONESS,
    14968_i32 => _CHRISTMAS_DAY,
    14975_i32 => _NEW_YEAR_S_DAY,
    15086_i32 => _HOLY_FRIDAY,
    15095_i32 => _WORLD_LABOR_DAY,
    15114_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    15148_i32 => _CORPUS_CHRISTI,
    15216_i32 => _POPULAR_CONSULTATION_DAY,
    15217_i32 => _EID_AL_FITR,
    15279_i32 => _ALL_SAINTS__DAY,
    15280_i32 => _ALL_SOULS__DAY,
    15285_i32 => _EID_AL_ADHA,
    15290_i32 => _NATIONAL_YOUTH_DAY,
    15306_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    15315_i32 => _NATIONAL_HEROES_DAY,
    15316_i32 => _DAY_OF_OUR_LADY_OF_IMMACULATE_CONCEPTION_AND_TIMOR_LESTE_PATRONESS,
    15333_i32 => _CHRISTMAS_DAY,
    15340_i32 => _NEW_YEAR_S_DAY,
    15436_i32 => _HOLY_FRIDAY,
    15461_i32 => _WORLD_LABOR_DAY,
    15480_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    15498_i32 => _CORPUS_CHRISTI,
    15572_i32 => _EID_AL_FITR,
    15582_i32 => _POPULAR_CONSULTATION_DAY,
    15639_i32 => _EID_AL_ADHA,
    15645_i32 => _ALL_SAINTS__DAY,
    15646_i32 => _ALL_SOULS__DAY,
    15656_i32 => _NATIONAL_YOUTH_DAY,
    15672_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    15681_i32 => _NATIONAL_HEROES_DAY,
    15682_i32 => _DAY_OF_OUR_LADY_OF_IMMACULATE_CONCEPTION_AND_TIMOR_LESTE_PATRONESS,
    15699_i32 => _CHRISTMAS_DAY,
    15706_i32 => _NEW_YEAR_S_DAY,
    15793_i32 => _HOLY_FRIDAY,
    15826_i32 => _WORLD_LABOR_DAY,
    15845_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    15855_i32 => _CORPUS_CHRISTI,
    15925_i32 => _EID_AL_FITR,
    15947_i32 => _POPULAR_CONSULTATION_DAY,
    15993_i32 => _EID_AL_ADHA,
    16010_i32 => _ALL_SAINTS__DAY,
    16011_i32 => _ALL_SOULS__DAY,
    16021_i32 => _NATIONAL_YOUTH_DAY,
    16037_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    16046_i32 => _NATIONAL_HEROES_DAY,
    16047_i32 => _DAY_OF_OUR_LADY_OF_IMMACULATE_CONCEPTION_AND_TIMOR_LESTE_PATRONESS,
    16064_i32 => _CHRISTMAS_DAY,
    16071_i32 => _NEW_YEAR_S_DAY,
    16178_i32 => _HOLY_FRIDAY,
    16191_i32 => _WORLD_LABOR_DAY,
    16210_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    16240_i32 => _CORPUS_CHRISTI,
    16279_i32 => _EID_AL_FITR,
    16312_i32 => _POPULAR_CONSULTATION_DAY,
    16347_i32 => _EID_AL_ADHA,
    16375_i32 => _ALL_SAINTS__DAY,
    16376_i32 => _ALL_SOULS__DAY,
    16386_i32 => _NATIONAL_YOUTH_DAY,
    16402_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    16411_i32 => _NATIONAL_HEROES_DAY,
    16412_i32 => _DAY_OF_OUR_LADY_OF_IMMACULATE_CONCEPTION_AND_TIMOR_LESTE_PATRONESS,
    16429_i32 => _CHRISTMAS_DAY,
    16436_i32 => _NEW_YEAR_S_DAY,
    16528_i32 => _HOLY_FRIDAY,
    16556_i32 => _WORLD_LABOR_DAY,
    16575_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    16590_i32 => _CORPUS_CHRISTI,
    16633_i32 => _EID_AL_FITR,
    16677_i32 => _POPULAR_CONSULTATION_DAY,
    16702_i32 => _EID_AL_ADHA,
    16740_i32 => _ALL_SAINTS__DAY,
    16741_i32 => _ALL_SOULS__DAY,
    16751_i32 => _NATIONAL_YOUTH_DAY,
    16767_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    16776_i32 => _NATIONAL_HEROES_DAY,
    16777_i32 => _DAY_OF_OUR_LADY_OF_IMMACULATE_CONCEPTION_AND_TIMOR_LESTE_PATRONESS,
    16794_i32 => _CHRISTMAS_DAY,
    16801_i32 => _NEW_YEAR_S_DAY,
    16885_i32 => _HOLY_FRIDAY,
    16922_i32 => _WORLD_LABOR_DAY,
    16941_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    16947_i32 => _CORPUS_CHRISTI,
    16989_i32 => _EID_AL_FITR,
    17043_i32 => _POPULAR_CONSULTATION_DAY,
    17062_i32 => _EID_AL_ADHA,
    17106_i32 => _ALL_SAINTS__DAY,
    17107_i32 => _ALL_SOULS__DAY,
    17117_i32 => _NATIONAL_YOUTH_DAY,
    17133_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    17142_i32 => _NATIONAL_HEROES_DAY,
    17143_i32 => _DAY_OF_OUR_LADY_OF_IMMACULATE_CONCEPTION_AND_TIMOR_LESTE_PATRONESS,
    17160_i32 => _CHRISTMAS_DAY,
    17167_i32 => _NEW_YEAR_S_DAY,
    17228_i32 => _VETERAN_S_DAY,
    17270_i32 => _HOLY_FRIDAY,
    17287_i32 => _WORLD_LABOR_DAY,
    17306_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    17332_i32 => _CORPUS_CHRISTI,
    17343_i32 => _EID_AL_FITR,
    17408_i32 => _POPULAR_CONSULTATION_DAY,
    17410_i32 => _EID_AL_ADHA,
    17471_i32 => _ALL_SAINTS__DAY,
    17472_i32 => _ALL_SOULS__DAY,
    17482_i32 => _NATIONAL_YOUTH_DAY,
    17498_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    17507_i32 => _MEMORIAL_DAY,
    17508_i32 => _DAY_OF_OUR_LADY_OF_IMMACULATE_CONCEPTION_AND_TIMOR_LESTE_PATRONESS,
    17525_i32 => _CHRISTMAS_DAY,
    17531_i32 => _NATIONAL_HEROES_DAY,
    17532_i32 => _NEW_YEAR_S_DAY,
    17593_i32 => _VETERAN_S_DAY,
    17620_i32 => _HOLY_FRIDAY,
    17652_i32 => _WORLD_LABOR_DAY,
    17671_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    17682_i32 => _CORPUS_CHRISTI,
    17697_i32 => _EID_AL_FITR,
    17764_i32 => _EID_AL_ADHA,
    17773_i32 => _POPULAR_CONSULTATION_DAY,
    17836_i32 => _ALL_SAINTS__DAY,
    17837_i32 => _ALL_SOULS__DAY,
    17847_i32 => _NATIONAL_YOUTH_DAY,
    17863_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    17872_i32 => _MEMORIAL_DAY,
    17873_i32 => _DAY_OF_OUR_LADY_OF_IMMACULATE_CONCEPTION_AND_TIMOR_LESTE_PATRONESS,
    17890_i32 => _CHRISTMAS_DAY,
    17896_i32 => _NATIONAL_HEROES_DAY,
    17897_i32 => _NEW_YEAR_S_DAY,
    17958_i32 => _VETERAN_S_DAY,
    18005_i32 => _HOLY_FRIDAY,
    18017_i32 => _WORLD_LABOR_DAY,
    18036_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    18053_i32 => _EID_AL_FITR,
    18067_i32 => _CORPUS_CHRISTI,
    18119_i32 => _EID_AL_ADHA,
    18138_i32 => _POPULAR_CONSULTATION_DAY,
    18201_i32 => _ALL_SAINTS__DAY,
    18202_i32 => _ALL_SOULS__DAY,
    18212_i32 => _NATIONAL_YOUTH_DAY,
    18228_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    18237_i32 => _MEMORIAL_DAY,
    18238_i32 => _DAY_OF_OUR_LADY_OF_IMMACULATE_CONCEPTION_AND_TIMOR_LESTE_PATRONESS,
    18255_i32 => _CHRISTMAS_DAY,
    18261_i32 => _NATIONAL_HEROES_DAY,
    18262_i32 => _NEW_YEAR_S_DAY,
    18324_i32 => _VETERAN_S_DAY,
    18362_i32 => _HOLY_FRIDAY,
    18383_i32 => _WORLD_LABOR_DAY,
    18402_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    18406_i32 => _EID_AL_FITR,
    18424_i32 => _CORPUS_CHRISTI,
    18474_i32 => _EID_AL_ADHA,
    18504_i32 => _POPULAR_CONSULTATION_DAY,
    18567_i32 => _ALL_SAINTS__DAY,
    18568_i32 => _ALL_SOULS__DAY,
    18578_i32 => _NATIONAL_YOUTH_DAY,
    18594_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    18603_i32 => _MEMORIAL_DAY,
    18604_i32 => _DAY_OF_OUR_LADY_OF_IMMACULATE_CONCEPTION_AND_TIMOR_LESTE_PATRONESS,
    18621_i32 => _CHRISTMAS_DAY,
    18627_i32 => _NATIONAL_HEROES_DAY,
    18628_i32 => _NEW_YEAR_S_DAY,
    18689_i32 => _VETERAN_S_DAY,
    18719_i32 => _HOLY_FRIDAY,
    18748_i32 => _WORLD_LABOR_DAY,
    18760_i32 => _EID_AL_FITR,
    18767_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    18781_i32 => _CORPUS_CHRISTI,
    18827_i32 => _EID_AL_ADHA,
    18869_i32 => _POPULAR_CONSULTATION_DAY,
    18932_i32 => _ALL_SAINTS__DAY,
    18933_i32 => _ALL_SOULS__DAY,
    18943_i32 => _NATIONAL_YOUTH_DAY,
    18959_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    18968_i32 => _MEMORIAL_DAY,
    18969_i32 => _DAY_OF_OUR_LADY_OF_IMMACULATE_CONCEPTION_AND_TIMOR_LESTE_PATRONESS,
    18986_i32 => _CHRISTMAS_DAY,
    18992_i32 => _NATIONAL_HEROES_DAY,
    18993_i32 => _NEW_YEAR_S_DAY,
    19054_i32 => _VETERAN_S_DAY,
    19097_i32 => _HOLY_FRIDAY,
    19113_i32 => _WORLD_LABOR_DAY,
    19114_i32 => _EID_AL_FITR,
    19132_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    19159_i32 => _CORPUS_CHRISTI,
    19182_i32 => _EID_AL_ADHA,
    19234_i32 => _POPULAR_CONSULTATION_DAY,
    19297_i32 => _ALL_SAINTS__DAY,
    19298_i32 => _ALL_SOULS__DAY,
    19308_i32 => _NATIONAL_YOUTH_DAY,
    19324_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    19333_i32 => _MEMORIAL_DAY,
    19334_i32 => _DAY_OF_OUR_LADY_OF_IMMACULATE_CONCEPTION_AND_TIMOR_LESTE_PATRONESS,
    19351_i32 => _CHRISTMAS_DAY,
    19357_i32 => _NATIONAL_HEROES_DAY,
    19358_i32 => _NEW_YEAR_S_DAY,
    19419_i32 => _VETERAN_S_DAY,
    19454_i32 => _HOLY_FRIDAY,
    19469_i32 => _EID_AL_FITR,
    19478_i32 => _WORLD_LABOR_DAY,
    19497_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    19516_i32 => _CORPUS_CHRISTI,
    19537_i32 => _EID_AL_ADHA,
    19599_i32 => _POPULAR_CONSULTATION_DAY,
    19662_i32 => _ALL_SAINTS__DAY,
    19663_i32 => _ALL_SOULS__DAY,
    19664_i32 => _NATIONAL_WOMEN_S_DAY,
    19673_i32 => _NATIONAL_YOUTH_DAY,
    19689_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    19698_i32 => _MEMORIAL_DAY,
    19699_i32 => _DAY_OF_OUR_LADY_OF_IMMACULATE_CONCEPTION_AND_TIMOR_LESTE_PATRONESS,
    19716_i32 => _CHRISTMAS_DAY,
    19722_i32 => _NATIONAL_HEROES_DAY,
    19723_i32 => _NEW_YEAR_S_DAY,
    19785_i32 => _VETERAN_S_DAY,
    19811_i32 => _HOLY_FRIDAY,
    19823_i32 => _EID_AL_FITR,
    19844_i32 => _WORLD_LABOR_DAY,
    19863_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    19873_i32 => _CORPUS_CHRISTI,
    19891_i32 => _EID_AL_ADHA,
    19965_i32 => _POPULAR_CONSULTATION_DAY,
    20028_i32 => _ALL_SAINTS__DAY,
    20029_i32 => _ALL_SOULS__DAY,
    20030_i32 => _NATIONAL_WOMEN_S_DAY,
    20039_i32 => _NATIONAL_YOUTH_DAY,
    20055_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    20064_i32 => _MEMORIAL_DAY,
    20065_i32 => _DAY_OF_OUR_LADY_OF_IMMACULATE_CONCEPTION_AND_TIMOR_LESTE_PATRONESS,
    20082_i32 => _CHRISTMAS_DAY,
    20088_i32 => _NATIONAL_HEROES_DAY,
    20089_i32 => _NEW_YEAR_S_DAY,
    20150_i32 => _VETERAN_S_DAY,
    20177_i32 => _EID_AL_FITR__ESTIMATED_,
    20196_i32 => _HOLY_FRIDAY,
    20209_i32 => _WORLD_LABOR_DAY,
    20228_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    20245_i32 => _EID_AL_ADHA__ESTIMATED_,
    20258_i32 => _CORPUS_CHRISTI,
    20330_i32 => _POPULAR_CONSULTATION_DAY,
    20393_i32 => _ALL_SAINTS__DAY,
    20394_i32 => _ALL_SOULS__DAY,
    20395_i32 => _NATIONAL_WOMEN_S_DAY,
    20404_i32 => _NATIONAL_YOUTH_DAY,
    20420_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    20429_i32 => _MEMORIAL_DAY,
    20430_i32 => _DAY_OF_OUR_LADY_OF_IMMACULATE_CONCEPTION_AND_TIMOR_LESTE_PATRONESS,
    20447_i32 => _CHRISTMAS_DAY,
    20453_i32 => _NATIONAL_HEROES_DAY,
    20454_i32 => _NEW_YEAR_S_DAY,
    20515_i32 => _VETERAN_S_DAY,
    20532_i32 => _EID_AL_FITR__ESTIMATED_,
    20546_i32 => _HOLY_FRIDAY,
    20574_i32 => _WORLD_LABOR_DAY,
    20593_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    20600_i32 => _EID_AL_ADHA__ESTIMATED_,
    20608_i32 => _CORPUS_CHRISTI,
    20695_i32 => _POPULAR_CONSULTATION_DAY,
    20758_i32 => _ALL_SAINTS__DAY,
    20759_i32 => _ALL_SOULS__DAY,
    20760_i32 => _NATIONAL_WOMEN_S_DAY,
    20769_i32 => _NATIONAL_YOUTH_DAY,
    20785_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    20794_i32 => _MEMORIAL_DAY,
    20795_i32 => _DAY_OF_OUR_LADY_OF_IMMACULATE_CONCEPTION_AND_TIMOR_LESTE_PATRONESS,
    20812_i32 => _CHRISTMAS_DAY,
    20818_i32 => _NATIONAL_HEROES_DAY,
    20819_i32 => _NEW_YEAR_S_DAY,
    20880_i32 => _VETERAN_S_DAY,
    20886_i32 => _EID_AL_FITR__ESTIMATED_,
    20903_i32 => _HOLY_FRIDAY,
    20939_i32 => _WORLD_LABOR_DAY,
    20954_i32 => _EID_AL_ADHA__ESTIMATED_,
    20958_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    20965_i32 => _CORPUS_CHRISTI,
    21060_i32 => _POPULAR_CONSULTATION_DAY,
    21123_i32 => _ALL_SAINTS__DAY,
    21124_i32 => _ALL_SOULS__DAY,
    21125_i32 => _NATIONAL_WOMEN_S_DAY,
    21134_i32 => _NATIONAL_YOUTH_DAY,
    21150_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    21159_i32 => _MEMORIAL_DAY,
    21160_i32 => _DAY_OF_OUR_LADY_OF_IMMACULATE_CONCEPTION_AND_TIMOR_LESTE_PATRONESS,
    21177_i32 => _CHRISTMAS_DAY,
    21183_i32 => _NATIONAL_HEROES_DAY,
    21184_i32 => _NEW_YEAR_S_DAY,
    21240_i32 => _EID_AL_FITR__ESTIMATED_,
    21246_i32 => _VETERAN_S_DAY,
    21288_i32 => _HOLY_FRIDAY,
    21305_i32 => _WORLD_LABOR_DAY,
    21309_i32 => _EID_AL_ADHA__ESTIMATED_,
    21324_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    21350_i32 => _CORPUS_CHRISTI,
    21426_i32 => _POPULAR_CONSULTATION_DAY,
    21489_i32 => _ALL_SAINTS__DAY,
    21490_i32 => _ALL_SOULS__DAY,
    21491_i32 => _NATIONAL_WOMEN_S_DAY,
    21500_i32 => _NATIONAL_YOUTH_DAY,
    21516_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    21525_i32 => _MEMORIAL_DAY,
    21526_i32 => _DAY_OF_OUR_LADY_OF_IMMACULATE_CONCEPTION_AND_TIMOR_LESTE_PATRONESS,
    21543_i32 => _CHRISTMAS_DAY,
    21549_i32 => _NATIONAL_HEROES_DAY,
    21550_i32 => _NEW_YEAR_S_DAY,
    21594_i32 => _EID_AL_FITR__ESTIMATED_,
    21611_i32 => _VETERAN_S_DAY,
    21638_i32 => _HOLY_FRIDAY,
    21663_i32 => _EID_AL_ADHA__ESTIMATED_,
    21670_i32 => _WORLD_LABOR_DAY,
    21689_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    21700_i32 => _CORPUS_CHRISTI,
    21791_i32 => _POPULAR_CONSULTATION_DAY,
    21854_i32 => _ALL_SAINTS__DAY,
    21855_i32 => _ALL_SOULS__DAY,
    21856_i32 => _NATIONAL_WOMEN_S_DAY,
    21865_i32 => _NATIONAL_YOUTH_DAY,
    21881_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    21890_i32 => _MEMORIAL_DAY,
    21891_i32 => _DAY_OF_OUR_LADY_OF_IMMACULATE_CONCEPTION_AND_TIMOR_LESTE_PATRONESS,
    21908_i32 => _CHRISTMAS_DAY,
    21914_i32 => _NATIONAL_HEROES_DAY,
    21915_i32 => _NEW_YEAR_S_DAY,
    21949_i32 => _EID_AL_FITR__ESTIMATED_,
    21976_i32 => _VETERAN_S_DAY,
    22017_i32 => _EID_AL_ADHA__ESTIMATED_,
    22023_i32 => _HOLY_FRIDAY,
    22035_i32 => _WORLD_LABOR_DAY,
    22054_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    22085_i32 => _CORPUS_CHRISTI,
    22156_i32 => _POPULAR_CONSULTATION_DAY,
    22219_i32 => _ALL_SAINTS__DAY,
    22220_i32 => _ALL_SOULS__DAY,
    22221_i32 => _NATIONAL_WOMEN_S_DAY,
    22230_i32 => _NATIONAL_YOUTH_DAY,
    22246_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    22255_i32 => _MEMORIAL_DAY,
    22256_i32 => _DAY_OF_OUR_LADY_OF_IMMACULATE_CONCEPTION_AND_TIMOR_LESTE_PATRONESS,
    22273_i32 => _CHRISTMAS_DAY,
    22279_i32 => _NATIONAL_HEROES_DAY,
    22280_i32 => _NEW_YEAR_S_DAY,
    22303_i32 => _EID_AL_FITR__ESTIMATED_,
    22341_i32 => _VETERAN_S_DAY,
    22371_i32 => _EID_AL_ADHA__ESTIMATED_,
    22380_i32 => _HOLY_FRIDAY,
    22400_i32 => _WORLD_LABOR_DAY,
    22419_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    22442_i32 => _CORPUS_CHRISTI,
    22521_i32 => _POPULAR_CONSULTATION_DAY,
    22584_i32 => _ALL_SAINTS__DAY,
    22585_i32 => _ALL_SOULS__DAY,
    22586_i32 => _NATIONAL_WOMEN_S_DAY,
    22595_i32 => _NATIONAL_YOUTH_DAY,
    22611_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    22620_i32 => _MEMORIAL_DAY,
    22621_i32 => _DAY_OF_OUR_LADY_OF_IMMACULATE_CONCEPTION_AND_TIMOR_LESTE_PATRONESS,
    22638_i32 => _CHRISTMAS_DAY,
    22644_i32 => _NATIONAL_HEROES_DAY,
    22645_i32 => _NEW_YEAR_S_DAY,
    22658_i32 => _EID_AL_FITR__ESTIMATED_,
    22707_i32 => _VETERAN_S_DAY,
    22726_i32 => _EID_AL_ADHA__ESTIMATED_,
    22730_i32 => _HOLY_FRIDAY,
    22766_i32 => _WORLD_LABOR_DAY,
    22785_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    22792_i32 => _CORPUS_CHRISTI,
    22887_i32 => _POPULAR_CONSULTATION_DAY,
    22950_i32 => _ALL_SAINTS__DAY,
    22951_i32 => _ALL_SOULS__DAY,
    22952_i32 => _NATIONAL_WOMEN_S_DAY,
    22961_i32 => _NATIONAL_YOUTH_DAY,
    22977_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    22986_i32 => _MEMORIAL_DAY,
    22987_i32 => _DAY_OF_OUR_LADY_OF_IMMACULATE_CONCEPTION_AND_TIMOR_LESTE_PATRONESS,
    23004_i32 => _CHRISTMAS_DAY,
    23010_i32 => _NATIONAL_HEROES_DAY,
    23011_i32 => _NEW_YEAR_S_DAY,
    23012_i32 => _EID_AL_FITR__ESTIMATED_,
    23072_i32 => _VETERAN_S_DAY,
    23080_i32 => _EID_AL_ADHA__ESTIMATED_,
    23115_i32 => _HOLY_FRIDAY,
    23131_i32 => _WORLD_LABOR_DAY,
    23150_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    23177_i32 => _CORPUS_CHRISTI,
    23252_i32 => _POPULAR_CONSULTATION_DAY,
    23315_i32 => _ALL_SAINTS__DAY,
    23316_i32 => _ALL_SOULS__DAY,
    23317_i32 => _NATIONAL_WOMEN_S_DAY,
    23326_i32 => _NATIONAL_YOUTH_DAY,
    23342_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    23351_i32 => _MEMORIAL_DAY,
    23352_i32 => _DAY_OF_OUR_LADY_OF_IMMACULATE_CONCEPTION_AND_TIMOR_LESTE_PATRONESS,
    23367_i32 => _EID_AL_FITR__ESTIMATED_,
    23369_i32 => _CHRISTMAS_DAY,
    23375_i32 => _NATIONAL_HEROES_DAY,
    23376_i32 => _NEW_YEAR_S_DAY,
    23435_i32 => _EID_AL_ADHA__ESTIMATED_,
    23437_i32 => _VETERAN_S_DAY,
    23472_i32 => _HOLY_FRIDAY,
    23496_i32 => _WORLD_LABOR_DAY,
    23515_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    23534_i32 => _CORPUS_CHRISTI,
    23617_i32 => _POPULAR_CONSULTATION_DAY,
    23680_i32 => _ALL_SAINTS__DAY,
    23681_i32 => _ALL_SOULS__DAY,
    23682_i32 => _NATIONAL_WOMEN_S_DAY,
    23691_i32 => _NATIONAL_YOUTH_DAY,
    23707_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    23716_i32 => _MEMORIAL_DAY,
    23717_i32 => _DAY_OF_OUR_LADY_OF_IMMACULATE_CONCEPTION_AND_TIMOR_LESTE_PATRONESS,
    23721_i32 => _EID_AL_FITR__ESTIMATED_,
    23734_i32 => _CHRISTMAS_DAY,
    23740_i32 => _NATIONAL_HEROES_DAY,
    23741_i32 => _NEW_YEAR_S_DAY,
    23789_i32 => _EID_AL_ADHA__ESTIMATED_,
    23802_i32 => _VETERAN_S_DAY,
    23822_i32 => _HOLY_FRIDAY,
    23861_i32 => _WORLD_LABOR_DAY,
    23880_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    23884_i32 => _CORPUS_CHRISTI,
    23982_i32 => _POPULAR_CONSULTATION_DAY,
    24045_i32 => _ALL_SAINTS__DAY,
    24046_i32 => _ALL_SOULS__DAY,
    24047_i32 => _NATIONAL_WOMEN_S_DAY,
    24056_i32 => _NATIONAL_YOUTH_DAY,
    24072_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    24075_i32 => _EID_AL_FITR__ESTIMATED_,
    24081_i32 => _MEMORIAL_DAY,
    24082_i32 => _DAY_OF_OUR_LADY_OF_IMMACULATE_CONCEPTION_AND_TIMOR_LESTE_PATRONESS,
    24099_i32 => _CHRISTMAS_DAY,
    24105_i32 => _NATIONAL_HEROES_DAY,
    24106_i32 => _NEW_YEAR_S_DAY,
    24143_i32 => _EID_AL_ADHA__ESTIMATED_,
    24168_i32 => _VETERAN_S_DAY,
    24207_i32 => _HOLY_FRIDAY,
    24227_i32 => _WORLD_LABOR_DAY,
    24246_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    24269_i32 => _CORPUS_CHRISTI,
    24348_i32 => _POPULAR_CONSULTATION_DAY,
    24411_i32 => _ALL_SAINTS__DAY,
    24412_i32 => _ALL_SOULS__DAY,
    24413_i32 => _NATIONAL_WOMEN_S_DAY,
    24422_i32 => _NATIONAL_YOUTH_DAY,
    24429_i32 => _EID_AL_FITR__ESTIMATED_,
    24438_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    24447_i32 => _MEMORIAL_DAY,
    24448_i32 => _DAY_OF_OUR_LADY_OF_IMMACULATE_CONCEPTION_AND_TIMOR_LESTE_PATRONESS,
    24465_i32 => _CHRISTMAS_DAY,
    24471_i32 => _NATIONAL_HEROES_DAY,
    24472_i32 => _NEW_YEAR_S_DAY,
    24497_i32 => _EID_AL_ADHA__ESTIMATED_,
    24533_i32 => _VETERAN_S_DAY,
    24564_i32 => _HOLY_FRIDAY,
    24592_i32 => _WORLD_LABOR_DAY,
    24611_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    24626_i32 => _CORPUS_CHRISTI,
    24713_i32 => _POPULAR_CONSULTATION_DAY,
    24776_i32 => _ALL_SAINTS__DAY,
    24777_i32 => _ALL_SOULS__DAY,
    24778_i32 => _NATIONAL_WOMEN_S_DAY,
    24783_i32 => _EID_AL_FITR__ESTIMATED_,
    24787_i32 => _NATIONAL_YOUTH_DAY,
    24803_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    24812_i32 => _MEMORIAL_DAY,
    24813_i32 => _DAY_OF_OUR_LADY_OF_IMMACULATE_CONCEPTION_AND_TIMOR_LESTE_PATRONESS,
    24830_i32 => _CHRISTMAS_DAY,
    24836_i32 => _NATIONAL_HEROES_DAY,
    24837_i32 => _NEW_YEAR_S_DAY,
    24852_i32 => _EID_AL_ADHA__ESTIMATED_,
    24898_i32 => _VETERAN_S_DAY,
    24949_i32 => _HOLY_FRIDAY,
    24957_i32 => _WORLD_LABOR_DAY,
    24976_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    25011_i32 => _CORPUS_CHRISTI,
    25078_i32 => _POPULAR_CONSULTATION_DAY,
    25138_i32 => _EID_AL_FITR__ESTIMATED_,
    25141_i32 => _ALL_SAINTS__DAY,
    25142_i32 => _ALL_SOULS__DAY,
    25143_i32 => _NATIONAL_WOMEN_S_DAY,
    25152_i32 => _NATIONAL_YOUTH_DAY,
    25168_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    25177_i32 => _MEMORIAL_DAY,
    25178_i32 => _DAY_OF_OUR_LADY_OF_IMMACULATE_CONCEPTION_AND_TIMOR_LESTE_PATRONESS,
    25195_i32 => _CHRISTMAS_DAY,
    25201_i32 => _NATIONAL_HEROES_DAY,
    25202_i32 => _NEW_YEAR_S_DAY,
    25206_i32 => _EID_AL_ADHA__ESTIMATED_,
    25263_i32 => _VETERAN_S_DAY,
    25299_i32 => _HOLY_FRIDAY,
    25322_i32 => _WORLD_LABOR_DAY,
    25341_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    25361_i32 => _CORPUS_CHRISTI,
    25443_i32 => _POPULAR_CONSULTATION_DAY,
    25493_i32 => _EID_AL_FITR__ESTIMATED_,
    25506_i32 => _ALL_SAINTS__DAY,
    25507_i32 => _ALL_SOULS__DAY,
    25508_i32 => _NATIONAL_WOMEN_S_DAY,
    25517_i32 => _NATIONAL_YOUTH_DAY,
    25533_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    25542_i32 => _MEMORIAL_DAY,
    25543_i32 => _DAY_OF_OUR_LADY_OF_IMMACULATE_CONCEPTION_AND_TIMOR_LESTE_PATRONESS,
    25560_i32 => _CHRISTMAS_DAY,
    25561_i32 => _EID_AL_ADHA__ESTIMATED_,
    25566_i32 => _NATIONAL_HEROES_DAY,
    25567_i32 => _NEW_YEAR_S_DAY,
    25629_i32 => _VETERAN_S_DAY,
    25656_i32 => _HOLY_FRIDAY,
    25688_i32 => _WORLD_LABOR_DAY,
    25707_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    25718_i32 => _CORPUS_CHRISTI,
    25809_i32 => _POPULAR_CONSULTATION_DAY,
    25847_i32 => _EID_AL_FITR__ESTIMATED_,
    25872_i32 => _ALL_SAINTS__DAY,
    25873_i32 => _ALL_SOULS__DAY,
    25874_i32 => _NATIONAL_WOMEN_S_DAY,
    25883_i32 => _NATIONAL_YOUTH_DAY,
    25899_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    25908_i32 => _MEMORIAL_DAY,
    25909_i32 => _DAY_OF_OUR_LADY_OF_IMMACULATE_CONCEPTION_AND_TIMOR_LESTE_PATRONESS,
    25915_i32 => _EID_AL_ADHA__ESTIMATED_,
    25926_i32 => _CHRISTMAS_DAY,
    25932_i32 => _NATIONAL_HEROES_DAY,
    25933_i32 => _NEW_YEAR_S_DAY,
    25994_i32 => _VETERAN_S_DAY,
    26041_i32 => _HOLY_FRIDAY,
    26053_i32 => _WORLD_LABOR_DAY,
    26072_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    26103_i32 => _CORPUS_CHRISTI,
    26174_i32 => _POPULAR_CONSULTATION_DAY,
    26201_i32 => _EID_AL_FITR__ESTIMATED_,
    26237_i32 => _ALL_SAINTS__DAY,
    26238_i32 => _ALL_SOULS__DAY,
    26239_i32 => _NATIONAL_WOMEN_S_DAY,
    26248_i32 => _NATIONAL_YOUTH_DAY,
    26264_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    26270_i32 => _EID_AL_ADHA__ESTIMATED_,
    26273_i32 => _MEMORIAL_DAY,
    26274_i32 => _DAY_OF_OUR_LADY_OF_IMMACULATE_CONCEPTION_AND_TIMOR_LESTE_PATRONESS,
    26291_i32 => _CHRISTMAS_DAY,
    26297_i32 => _NATIONAL_HEROES_DAY,
    26298_i32 => _NEW_YEAR_S_DAY,
    26359_i32 => _VETERAN_S_DAY,
    26391_i32 => _HOLY_FRIDAY,
    26418_i32 => _WORLD_LABOR_DAY,
    26437_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    26453_i32 => _CORPUS_CHRISTI,
    26539_i32 => _POPULAR_CONSULTATION_DAY,
    26555_i32 => _EID_AL_FITR__ESTIMATED_,
    26602_i32 => _ALL_SAINTS__DAY,
    26603_i32 => _ALL_SOULS__DAY,
    26604_i32 => _NATIONAL_WOMEN_S_DAY,
    26613_i32 => _NATIONAL_YOUTH_DAY,
    26624_i32 => _EID_AL_ADHA__ESTIMATED_,
    26629_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    26638_i32 => _MEMORIAL_DAY,
    26639_i32 => _DAY_OF_OUR_LADY_OF_IMMACULATE_CONCEPTION_AND_TIMOR_LESTE_PATRONESS,
    26656_i32 => _CHRISTMAS_DAY,
    26662_i32 => _NATIONAL_HEROES_DAY,
    26663_i32 => _NEW_YEAR_S_DAY,
    26724_i32 => _VETERAN_S_DAY,
    26748_i32 => _HOLY_FRIDAY,
    26783_i32 => _WORLD_LABOR_DAY,
    26802_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    26810_i32 => _CORPUS_CHRISTI,
    26904_i32 => _POPULAR_CONSULTATION_DAY,
    26909_i32 => _EID_AL_FITR__ESTIMATED_,
    26967_i32 => _ALL_SAINTS__DAY,
    26968_i32 => _ALL_SOULS__DAY,
    26969_i32 => _NATIONAL_WOMEN_S_DAY,
    26978_i32 => _EID_AL_ADHA__ESTIMATED___NATIONAL_YOUTH_DAY,
    26994_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    27003_i32 => _MEMORIAL_DAY,
    27004_i32 => _DAY_OF_OUR_LADY_OF_IMMACULATE_CONCEPTION_AND_TIMOR_LESTE_PATRONESS,
    27021_i32 => _CHRISTMAS_DAY,
    27027_i32 => _NATIONAL_HEROES_DAY,
    27028_i32 => _NEW_YEAR_S_DAY,
    27090_i32 => _VETERAN_S_DAY,
    27133_i32 => _HOLY_FRIDAY,
    27149_i32 => _WORLD_LABOR_DAY,
    27168_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    27195_i32 => _CORPUS_CHRISTI,
    27264_i32 => _EID_AL_FITR__ESTIMATED_,
    27270_i32 => _POPULAR_CONSULTATION_DAY,
    27332_i32 => _EID_AL_ADHA__ESTIMATED_,
    27333_i32 => _ALL_SAINTS__DAY,
    27334_i32 => _ALL_SOULS__DAY,
    27335_i32 => _NATIONAL_WOMEN_S_DAY,
    27344_i32 => _NATIONAL_YOUTH_DAY,
    27360_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    27369_i32 => _MEMORIAL_DAY,
    27370_i32 => _DAY_OF_OUR_LADY_OF_IMMACULATE_CONCEPTION_AND_TIMOR_LESTE_PATRONESS,
    27387_i32 => _CHRISTMAS_DAY,
    27393_i32 => _NATIONAL_HEROES_DAY,
    27394_i32 => _NEW_YEAR_S_DAY,
    27455_i32 => _VETERAN_S_DAY,
    27490_i32 => _HOLY_FRIDAY,
    27514_i32 => _WORLD_LABOR_DAY,
    27533_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    27552_i32 => _CORPUS_CHRISTI,
    27619_i32 => _EID_AL_FITR__ESTIMATED_,
    27635_i32 => _POPULAR_CONSULTATION_DAY,
    27687_i32 => _EID_AL_ADHA__ESTIMATED_,
    27698_i32 => _ALL_SAINTS__DAY,
    27699_i32 => _ALL_SOULS__DAY,
    27700_i32 => _NATIONAL_WOMEN_S_DAY,
    27709_i32 => _NATIONAL_YOUTH_DAY,
    27725_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    27734_i32 => _MEMORIAL_DAY,
    27735_i32 => _DAY_OF_OUR_LADY_OF_IMMACULATE_CONCEPTION_AND_TIMOR_LESTE_PATRONESS,
    27752_i32 => _CHRISTMAS_DAY,
    27758_i32 => _NATIONAL_HEROES_DAY,
    27759_i32 => _NEW_YEAR_S_DAY,
    27820_i32 => _VETERAN_S_DAY,
    27840_i32 => _HOLY_FRIDAY,
    27879_i32 => _WORLD_LABOR_DAY,
    27898_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    27902_i32 => _CORPUS_CHRISTI,
    27973_i32 => _EID_AL_FITR__ESTIMATED_,
    28000_i32 => _POPULAR_CONSULTATION_DAY,
    28041_i32 => _EID_AL_ADHA__ESTIMATED_,
    28063_i32 => _ALL_SAINTS__DAY,
    28064_i32 => _ALL_SOULS__DAY,
    28065_i32 => _NATIONAL_WOMEN_S_DAY,
    28074_i32 => _NATIONAL_YOUTH_DAY,
    28090_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    28099_i32 => _MEMORIAL_DAY,
    28100_i32 => _DAY_OF_OUR_LADY_OF_IMMACULATE_CONCEPTION_AND_TIMOR_LESTE_PATRONESS,
    28117_i32 => _CHRISTMAS_DAY,
    28123_i32 => _NATIONAL_HEROES_DAY,
    28124_i32 => _NEW_YEAR_S_DAY,
    28185_i32 => _VETERAN_S_DAY,
    28225_i32 => _HOLY_FRIDAY,
    28244_i32 => _WORLD_LABOR_DAY,
    28263_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    28287_i32 => _CORPUS_CHRISTI,
    28328_i32 => _EID_AL_FITR__ESTIMATED_,
    28365_i32 => _POPULAR_CONSULTATION_DAY,
    28396_i32 => _EID_AL_ADHA__ESTIMATED_,
    28428_i32 => _ALL_SAINTS__DAY,
    28429_i32 => _ALL_SOULS__DAY,
    28430_i32 => _NATIONAL_WOMEN_S_DAY,
    28439_i32 => _NATIONAL_YOUTH_DAY,
    28455_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    28464_i32 => _MEMORIAL_DAY,
    28465_i32 => _DAY_OF_OUR_LADY_OF_IMMACULATE_CONCEPTION_AND_TIMOR_LESTE_PATRONESS,
    28482_i32 => _CHRISTMAS_DAY,
    28488_i32 => _NATIONAL_HEROES_DAY,
    28489_i32 => _NEW_YEAR_S_DAY,
    28551_i32 => _VETERAN_S_DAY,
    28582_i32 => _HOLY_FRIDAY,
    28610_i32 => _WORLD_LABOR_DAY,
    28629_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    28644_i32 => _CORPUS_CHRISTI,
    28682_i32 => _EID_AL_FITR__ESTIMATED_,
    28731_i32 => _POPULAR_CONSULTATION_DAY,
    28751_i32 => _EID_AL_ADHA__ESTIMATED_,
    28794_i32 => _ALL_SAINTS__DAY,
    28795_i32 => _ALL_SOULS__DAY,
    28796_i32 => _NATIONAL_WOMEN_S_DAY,
    28805_i32 => _NATIONAL_YOUTH_DAY,
    28821_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    28830_i32 => _MEMORIAL_DAY,
    28831_i32 => _DAY_OF_OUR_LADY_OF_IMMACULATE_CONCEPTION_AND_TIMOR_LESTE_PATRONESS,
    28848_i32 => _CHRISTMAS_DAY,
    28854_i32 => _NATIONAL_HEROES_DAY,
    28855_i32 => _NEW_YEAR_S_DAY,
    28916_i32 => _VETERAN_S_DAY,
    28960_i32 => _HOLY_FRIDAY,
    28975_i32 => _WORLD_LABOR_DAY,
    28994_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    29022_i32 => _CORPUS_CHRISTI,
    29036_i32 => _EID_AL_FITR__ESTIMATED_,
    29096_i32 => _POPULAR_CONSULTATION_DAY,
    29105_i32 => _EID_AL_ADHA__ESTIMATED_,
    29159_i32 => _ALL_SAINTS__DAY,
    29160_i32 => _ALL_SOULS__DAY,
    29161_i32 => _NATIONAL_WOMEN_S_DAY,
    29170_i32 => _NATIONAL_YOUTH_DAY,
    29186_i32 => _PROCLAMATION_OF_INDEPENDENCE_DAY,
    29195_i32 => _MEMORIAL_DAY,
    29196_i32 => _DAY_OF_OUR_LADY_OF_IMMACULATE_CONCEPTION_AND_TIMOR_LESTE_PATRONESS,
    29213_i32 => _CHRISTMAS_DAY,
    29219_i32 => _NATIONAL_HEROES_DAY,
    29220_i32 => _NEW_YEAR_S_DAY,
};
