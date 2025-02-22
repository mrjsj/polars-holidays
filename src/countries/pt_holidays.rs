use crate::countries::constants::*;
use phf::phf_map;

pub static PT_HOLIDAYS: phf::Map<i32, &'static str> = phf_map! {
    10957_i32 => _NEW_YEAR_S_DAY,
    11068_i32 => _GOOD_FRIDAY,
    11070_i32 => _EASTER_SUNDAY,
    11072_i32 => _FREEDOM_DAY,
    11078_i32 => _LABOR_DAY,
    11118_i32 => _DAY_OF_PORTUGAL__CAM_ES__AND_THE_PORTUGUESE_COMMUNITIES,
    11130_i32 => _CORPUS_CHRISTI,
    11184_i32 => _ASSUMPTION_DAY,
    11235_i32 => _REPUBLIC_DAY,
    11262_i32 => _ALL_SAINTS__DAY,
    11292_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    11299_i32 => _IMMACULATE_CONCEPTION,
    11316_i32 => _CHRISTMAS_DAY,
    11323_i32 => _NEW_YEAR_S_DAY,
    11425_i32 => _GOOD_FRIDAY,
    11427_i32 => _EASTER_SUNDAY,
    11437_i32 => _FREEDOM_DAY,
    11443_i32 => _LABOR_DAY,
    11483_i32 => _DAY_OF_PORTUGAL__CAM_ES__AND_THE_PORTUGUESE_COMMUNITIES,
    11487_i32 => _CORPUS_CHRISTI,
    11549_i32 => _ASSUMPTION_DAY,
    11600_i32 => _REPUBLIC_DAY,
    11627_i32 => _ALL_SAINTS__DAY,
    11657_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    11664_i32 => _IMMACULATE_CONCEPTION,
    11681_i32 => _CHRISTMAS_DAY,
    11688_i32 => _NEW_YEAR_S_DAY,
    11775_i32 => _GOOD_FRIDAY,
    11777_i32 => _EASTER_SUNDAY,
    11802_i32 => _FREEDOM_DAY,
    11808_i32 => _LABOR_DAY,
    11837_i32 => _CORPUS_CHRISTI,
    11848_i32 => _DAY_OF_PORTUGAL__CAM_ES__AND_THE_PORTUGUESE_COMMUNITIES,
    11914_i32 => _ASSUMPTION_DAY,
    11965_i32 => _REPUBLIC_DAY,
    11992_i32 => _ALL_SAINTS__DAY,
    12022_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    12029_i32 => _IMMACULATE_CONCEPTION,
    12046_i32 => _CHRISTMAS_DAY,
    12053_i32 => _NEW_YEAR_S_DAY,
    12160_i32 => _GOOD_FRIDAY,
    12162_i32 => _EASTER_SUNDAY,
    12167_i32 => _FREEDOM_DAY,
    12173_i32 => _LABOR_DAY,
    12213_i32 => _DAY_OF_PORTUGAL__CAM_ES__AND_THE_PORTUGUESE_COMMUNITIES,
    12222_i32 => _CORPUS_CHRISTI,
    12279_i32 => _ASSUMPTION_DAY,
    12330_i32 => _REPUBLIC_DAY,
    12357_i32 => _ALL_SAINTS__DAY,
    12387_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    12394_i32 => _IMMACULATE_CONCEPTION,
    12411_i32 => _CHRISTMAS_DAY,
    12418_i32 => _NEW_YEAR_S_DAY,
    12517_i32 => _GOOD_FRIDAY,
    12519_i32 => _EASTER_SUNDAY,
    12533_i32 => _FREEDOM_DAY,
    12539_i32 => _LABOR_DAY,
    12579_i32 => _CORPUS_CHRISTI__DAY_OF_PORTUGAL__CAM_ES__AND_THE_PORTUGUESE_COMMUNITIES,
    12645_i32 => _ASSUMPTION_DAY,
    12696_i32 => _REPUBLIC_DAY,
    12723_i32 => _ALL_SAINTS__DAY,
    12753_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    12760_i32 => _IMMACULATE_CONCEPTION,
    12777_i32 => _CHRISTMAS_DAY,
    12784_i32 => _NEW_YEAR_S_DAY,
    12867_i32 => _GOOD_FRIDAY,
    12869_i32 => _EASTER_SUNDAY,
    12898_i32 => _FREEDOM_DAY,
    12904_i32 => _LABOR_DAY,
    12929_i32 => _CORPUS_CHRISTI,
    12944_i32 => _DAY_OF_PORTUGAL__CAM_ES__AND_THE_PORTUGUESE_COMMUNITIES,
    13010_i32 => _ASSUMPTION_DAY,
    13061_i32 => _REPUBLIC_DAY,
    13088_i32 => _ALL_SAINTS__DAY,
    13118_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    13125_i32 => _IMMACULATE_CONCEPTION,
    13142_i32 => _CHRISTMAS_DAY,
    13149_i32 => _NEW_YEAR_S_DAY,
    13252_i32 => _GOOD_FRIDAY,
    13254_i32 => _EASTER_SUNDAY,
    13263_i32 => _FREEDOM_DAY,
    13269_i32 => _LABOR_DAY,
    13309_i32 => _DAY_OF_PORTUGAL__CAM_ES__AND_THE_PORTUGUESE_COMMUNITIES,
    13314_i32 => _CORPUS_CHRISTI,
    13375_i32 => _ASSUMPTION_DAY,
    13426_i32 => _REPUBLIC_DAY,
    13453_i32 => _ALL_SAINTS__DAY,
    13483_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    13490_i32 => _IMMACULATE_CONCEPTION,
    13507_i32 => _CHRISTMAS_DAY,
    13514_i32 => _NEW_YEAR_S_DAY,
    13609_i32 => _GOOD_FRIDAY,
    13611_i32 => _EASTER_SUNDAY,
    13628_i32 => _FREEDOM_DAY,
    13634_i32 => _LABOR_DAY,
    13671_i32 => _CORPUS_CHRISTI,
    13674_i32 => _DAY_OF_PORTUGAL__CAM_ES__AND_THE_PORTUGUESE_COMMUNITIES,
    13740_i32 => _ASSUMPTION_DAY,
    13791_i32 => _REPUBLIC_DAY,
    13818_i32 => _ALL_SAINTS__DAY,
    13848_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    13855_i32 => _IMMACULATE_CONCEPTION,
    13872_i32 => _CHRISTMAS_DAY,
    13879_i32 => _NEW_YEAR_S_DAY,
    13959_i32 => _GOOD_FRIDAY,
    13961_i32 => _EASTER_SUNDAY,
    13994_i32 => _FREEDOM_DAY,
    14000_i32 => _LABOR_DAY,
    14021_i32 => _CORPUS_CHRISTI,
    14040_i32 => _DAY_OF_PORTUGAL__CAM_ES__AND_THE_PORTUGUESE_COMMUNITIES,
    14106_i32 => _ASSUMPTION_DAY,
    14157_i32 => _REPUBLIC_DAY,
    14184_i32 => _ALL_SAINTS__DAY,
    14214_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    14221_i32 => _IMMACULATE_CONCEPTION,
    14238_i32 => _CHRISTMAS_DAY,
    14245_i32 => _NEW_YEAR_S_DAY,
    14344_i32 => _GOOD_FRIDAY,
    14346_i32 => _EASTER_SUNDAY,
    14359_i32 => _FREEDOM_DAY,
    14365_i32 => _LABOR_DAY,
    14405_i32 => _DAY_OF_PORTUGAL__CAM_ES__AND_THE_PORTUGUESE_COMMUNITIES,
    14406_i32 => _CORPUS_CHRISTI,
    14471_i32 => _ASSUMPTION_DAY,
    14522_i32 => _REPUBLIC_DAY,
    14549_i32 => _ALL_SAINTS__DAY,
    14579_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    14586_i32 => _IMMACULATE_CONCEPTION,
    14603_i32 => _CHRISTMAS_DAY,
    14610_i32 => _NEW_YEAR_S_DAY,
    14701_i32 => _GOOD_FRIDAY,
    14703_i32 => _EASTER_SUNDAY,
    14724_i32 => _FREEDOM_DAY,
    14730_i32 => _LABOR_DAY,
    14763_i32 => _CORPUS_CHRISTI,
    14770_i32 => _DAY_OF_PORTUGAL__CAM_ES__AND_THE_PORTUGUESE_COMMUNITIES,
    14836_i32 => _ASSUMPTION_DAY,
    14887_i32 => _REPUBLIC_DAY,
    14914_i32 => _ALL_SAINTS__DAY,
    14944_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    14951_i32 => _IMMACULATE_CONCEPTION,
    14968_i32 => _CHRISTMAS_DAY,
    14975_i32 => _NEW_YEAR_S_DAY,
    15086_i32 => _GOOD_FRIDAY,
    15088_i32 => _EASTER_SUNDAY,
    15089_i32 => _FREEDOM_DAY,
    15095_i32 => _LABOR_DAY,
    15135_i32 => _DAY_OF_PORTUGAL__CAM_ES__AND_THE_PORTUGUESE_COMMUNITIES,
    15148_i32 => _CORPUS_CHRISTI,
    15201_i32 => _ASSUMPTION_DAY,
    15252_i32 => _REPUBLIC_DAY,
    15279_i32 => _ALL_SAINTS__DAY,
    15309_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    15316_i32 => _IMMACULATE_CONCEPTION,
    15333_i32 => _CHRISTMAS_DAY,
    15340_i32 => _NEW_YEAR_S_DAY,
    15436_i32 => _GOOD_FRIDAY,
    15438_i32 => _EASTER_SUNDAY,
    15455_i32 => _FREEDOM_DAY,
    15461_i32 => _LABOR_DAY,
    15498_i32 => _CORPUS_CHRISTI,
    15501_i32 => _DAY_OF_PORTUGAL__CAM_ES__AND_THE_PORTUGUESE_COMMUNITIES,
    15567_i32 => _ASSUMPTION_DAY,
    15618_i32 => _REPUBLIC_DAY,
    15645_i32 => _ALL_SAINTS__DAY,
    15675_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    15682_i32 => _IMMACULATE_CONCEPTION,
    15699_i32 => _CHRISTMAS_DAY,
    15706_i32 => _NEW_YEAR_S_DAY,
    15793_i32 => _GOOD_FRIDAY,
    15795_i32 => _EASTER_SUNDAY,
    15820_i32 => _FREEDOM_DAY,
    15826_i32 => _LABOR_DAY,
    15866_i32 => _DAY_OF_PORTUGAL__CAM_ES__AND_THE_PORTUGUESE_COMMUNITIES,
    15932_i32 => _ASSUMPTION_DAY,
    16047_i32 => _IMMACULATE_CONCEPTION,
    16064_i32 => _CHRISTMAS_DAY,
    16071_i32 => _NEW_YEAR_S_DAY,
    16178_i32 => _GOOD_FRIDAY,
    16180_i32 => _EASTER_SUNDAY,
    16185_i32 => _FREEDOM_DAY,
    16191_i32 => _LABOR_DAY,
    16231_i32 => _DAY_OF_PORTUGAL__CAM_ES__AND_THE_PORTUGUESE_COMMUNITIES,
    16297_i32 => _ASSUMPTION_DAY,
    16412_i32 => _IMMACULATE_CONCEPTION,
    16429_i32 => _CHRISTMAS_DAY,
    16436_i32 => _NEW_YEAR_S_DAY,
    16528_i32 => _GOOD_FRIDAY,
    16530_i32 => _EASTER_SUNDAY,
    16550_i32 => _FREEDOM_DAY,
    16556_i32 => _LABOR_DAY,
    16596_i32 => _DAY_OF_PORTUGAL__CAM_ES__AND_THE_PORTUGUESE_COMMUNITIES,
    16662_i32 => _ASSUMPTION_DAY,
    16777_i32 => _IMMACULATE_CONCEPTION,
    16794_i32 => _CHRISTMAS_DAY,
    16801_i32 => _NEW_YEAR_S_DAY,
    16885_i32 => _GOOD_FRIDAY,
    16887_i32 => _EASTER_SUNDAY,
    16916_i32 => _FREEDOM_DAY,
    16922_i32 => _LABOR_DAY,
    16947_i32 => _CORPUS_CHRISTI,
    16962_i32 => _DAY_OF_PORTUGAL__CAM_ES__AND_THE_PORTUGUESE_COMMUNITIES,
    17028_i32 => _ASSUMPTION_DAY,
    17079_i32 => _REPUBLIC_DAY,
    17106_i32 => _ALL_SAINTS__DAY,
    17136_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    17143_i32 => _IMMACULATE_CONCEPTION,
    17160_i32 => _CHRISTMAS_DAY,
    17167_i32 => _NEW_YEAR_S_DAY,
    17270_i32 => _GOOD_FRIDAY,
    17272_i32 => _EASTER_SUNDAY,
    17281_i32 => _FREEDOM_DAY,
    17287_i32 => _LABOR_DAY,
    17327_i32 => _DAY_OF_PORTUGAL__CAM_ES__AND_THE_PORTUGUESE_COMMUNITIES,
    17332_i32 => _CORPUS_CHRISTI,
    17393_i32 => _ASSUMPTION_DAY,
    17444_i32 => _REPUBLIC_DAY,
    17471_i32 => _ALL_SAINTS__DAY,
    17501_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    17508_i32 => _IMMACULATE_CONCEPTION,
    17525_i32 => _CHRISTMAS_DAY,
    17532_i32 => _NEW_YEAR_S_DAY,
    17620_i32 => _GOOD_FRIDAY,
    17622_i32 => _EASTER_SUNDAY,
    17646_i32 => _FREEDOM_DAY,
    17652_i32 => _LABOR_DAY,
    17682_i32 => _CORPUS_CHRISTI,
    17692_i32 => _DAY_OF_PORTUGAL__CAM_ES__AND_THE_PORTUGUESE_COMMUNITIES,
    17758_i32 => _ASSUMPTION_DAY,
    17809_i32 => _REPUBLIC_DAY,
    17836_i32 => _ALL_SAINTS__DAY,
    17866_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    17873_i32 => _IMMACULATE_CONCEPTION,
    17890_i32 => _CHRISTMAS_DAY,
    17897_i32 => _NEW_YEAR_S_DAY,
    18005_i32 => _GOOD_FRIDAY,
    18007_i32 => _EASTER_SUNDAY,
    18011_i32 => _FREEDOM_DAY,
    18017_i32 => _LABOR_DAY,
    18057_i32 => _DAY_OF_PORTUGAL__CAM_ES__AND_THE_PORTUGUESE_COMMUNITIES,
    18067_i32 => _CORPUS_CHRISTI,
    18123_i32 => _ASSUMPTION_DAY,
    18174_i32 => _REPUBLIC_DAY,
    18201_i32 => _ALL_SAINTS__DAY,
    18231_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    18238_i32 => _IMMACULATE_CONCEPTION,
    18255_i32 => _CHRISTMAS_DAY,
    18262_i32 => _NEW_YEAR_S_DAY,
    18362_i32 => _GOOD_FRIDAY,
    18364_i32 => _EASTER_SUNDAY,
    18377_i32 => _FREEDOM_DAY,
    18383_i32 => _LABOR_DAY,
    18423_i32 => _DAY_OF_PORTUGAL__CAM_ES__AND_THE_PORTUGUESE_COMMUNITIES,
    18424_i32 => _CORPUS_CHRISTI,
    18489_i32 => _ASSUMPTION_DAY,
    18540_i32 => _REPUBLIC_DAY,
    18567_i32 => _ALL_SAINTS__DAY,
    18597_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    18604_i32 => _IMMACULATE_CONCEPTION,
    18621_i32 => _CHRISTMAS_DAY,
    18628_i32 => _NEW_YEAR_S_DAY,
    18719_i32 => _GOOD_FRIDAY,
    18721_i32 => _EASTER_SUNDAY,
    18742_i32 => _FREEDOM_DAY,
    18748_i32 => _LABOR_DAY,
    18781_i32 => _CORPUS_CHRISTI,
    18788_i32 => _DAY_OF_PORTUGAL__CAM_ES__AND_THE_PORTUGUESE_COMMUNITIES,
    18854_i32 => _ASSUMPTION_DAY,
    18905_i32 => _REPUBLIC_DAY,
    18932_i32 => _ALL_SAINTS__DAY,
    18962_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    18969_i32 => _IMMACULATE_CONCEPTION,
    18986_i32 => _CHRISTMAS_DAY,
    18993_i32 => _NEW_YEAR_S_DAY,
    19097_i32 => _GOOD_FRIDAY,
    19099_i32 => _EASTER_SUNDAY,
    19107_i32 => _FREEDOM_DAY,
    19113_i32 => _LABOR_DAY,
    19153_i32 => _DAY_OF_PORTUGAL__CAM_ES__AND_THE_PORTUGUESE_COMMUNITIES,
    19159_i32 => _CORPUS_CHRISTI,
    19219_i32 => _ASSUMPTION_DAY,
    19270_i32 => _REPUBLIC_DAY,
    19297_i32 => _ALL_SAINTS__DAY,
    19327_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    19334_i32 => _IMMACULATE_CONCEPTION,
    19351_i32 => _CHRISTMAS_DAY,
    19358_i32 => _NEW_YEAR_S_DAY,
    19454_i32 => _GOOD_FRIDAY,
    19456_i32 => _EASTER_SUNDAY,
    19472_i32 => _FREEDOM_DAY,
    19478_i32 => _LABOR_DAY,
    19516_i32 => _CORPUS_CHRISTI,
    19518_i32 => _DAY_OF_PORTUGAL__CAM_ES__AND_THE_PORTUGUESE_COMMUNITIES,
    19584_i32 => _ASSUMPTION_DAY,
    19635_i32 => _REPUBLIC_DAY,
    19662_i32 => _ALL_SAINTS__DAY,
    19692_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    19699_i32 => _IMMACULATE_CONCEPTION,
    19716_i32 => _CHRISTMAS_DAY,
    19723_i32 => _NEW_YEAR_S_DAY,
    19811_i32 => _GOOD_FRIDAY,
    19813_i32 => _EASTER_SUNDAY,
    19838_i32 => _FREEDOM_DAY,
    19844_i32 => _LABOR_DAY,
    19873_i32 => _CORPUS_CHRISTI,
    19884_i32 => _DAY_OF_PORTUGAL__CAM_ES__AND_THE_PORTUGUESE_COMMUNITIES,
    19950_i32 => _ASSUMPTION_DAY,
    20001_i32 => _REPUBLIC_DAY,
    20028_i32 => _ALL_SAINTS__DAY,
    20058_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    20065_i32 => _IMMACULATE_CONCEPTION,
    20082_i32 => _CHRISTMAS_DAY,
    20089_i32 => _NEW_YEAR_S_DAY,
    20196_i32 => _GOOD_FRIDAY,
    20198_i32 => _EASTER_SUNDAY,
    20203_i32 => _FREEDOM_DAY,
    20209_i32 => _LABOR_DAY,
    20249_i32 => _DAY_OF_PORTUGAL__CAM_ES__AND_THE_PORTUGUESE_COMMUNITIES,
    20258_i32 => _CORPUS_CHRISTI,
    20315_i32 => _ASSUMPTION_DAY,
    20366_i32 => _REPUBLIC_DAY,
    20393_i32 => _ALL_SAINTS__DAY,
    20423_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    20430_i32 => _IMMACULATE_CONCEPTION,
    20447_i32 => _CHRISTMAS_DAY,
    20454_i32 => _NEW_YEAR_S_DAY,
    20546_i32 => _GOOD_FRIDAY,
    20548_i32 => _EASTER_SUNDAY,
    20568_i32 => _FREEDOM_DAY,
    20574_i32 => _LABOR_DAY,
    20608_i32 => _CORPUS_CHRISTI,
    20614_i32 => _DAY_OF_PORTUGAL__CAM_ES__AND_THE_PORTUGUESE_COMMUNITIES,
    20680_i32 => _ASSUMPTION_DAY,
    20731_i32 => _REPUBLIC_DAY,
    20758_i32 => _ALL_SAINTS__DAY,
    20788_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    20795_i32 => _IMMACULATE_CONCEPTION,
    20812_i32 => _CHRISTMAS_DAY,
    20819_i32 => _NEW_YEAR_S_DAY,
    20903_i32 => _GOOD_FRIDAY,
    20905_i32 => _EASTER_SUNDAY,
    20933_i32 => _FREEDOM_DAY,
    20939_i32 => _LABOR_DAY,
    20965_i32 => _CORPUS_CHRISTI,
    20979_i32 => _DAY_OF_PORTUGAL__CAM_ES__AND_THE_PORTUGUESE_COMMUNITIES,
    21045_i32 => _ASSUMPTION_DAY,
    21096_i32 => _REPUBLIC_DAY,
    21123_i32 => _ALL_SAINTS__DAY,
    21153_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    21160_i32 => _IMMACULATE_CONCEPTION,
    21177_i32 => _CHRISTMAS_DAY,
    21184_i32 => _NEW_YEAR_S_DAY,
    21288_i32 => _GOOD_FRIDAY,
    21290_i32 => _EASTER_SUNDAY,
    21299_i32 => _FREEDOM_DAY,
    21305_i32 => _LABOR_DAY,
    21345_i32 => _DAY_OF_PORTUGAL__CAM_ES__AND_THE_PORTUGUESE_COMMUNITIES,
    21350_i32 => _CORPUS_CHRISTI,
    21411_i32 => _ASSUMPTION_DAY,
    21462_i32 => _REPUBLIC_DAY,
    21489_i32 => _ALL_SAINTS__DAY,
    21519_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    21526_i32 => _IMMACULATE_CONCEPTION,
    21543_i32 => _CHRISTMAS_DAY,
    21550_i32 => _NEW_YEAR_S_DAY,
    21638_i32 => _GOOD_FRIDAY,
    21640_i32 => _EASTER_SUNDAY,
    21664_i32 => _FREEDOM_DAY,
    21670_i32 => _LABOR_DAY,
    21700_i32 => _CORPUS_CHRISTI,
    21710_i32 => _DAY_OF_PORTUGAL__CAM_ES__AND_THE_PORTUGUESE_COMMUNITIES,
    21776_i32 => _ASSUMPTION_DAY,
    21827_i32 => _REPUBLIC_DAY,
    21854_i32 => _ALL_SAINTS__DAY,
    21884_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    21891_i32 => _IMMACULATE_CONCEPTION,
    21908_i32 => _CHRISTMAS_DAY,
    21915_i32 => _NEW_YEAR_S_DAY,
    22023_i32 => _GOOD_FRIDAY,
    22025_i32 => _EASTER_SUNDAY,
    22029_i32 => _FREEDOM_DAY,
    22035_i32 => _LABOR_DAY,
    22075_i32 => _DAY_OF_PORTUGAL__CAM_ES__AND_THE_PORTUGUESE_COMMUNITIES,
    22085_i32 => _CORPUS_CHRISTI,
    22141_i32 => _ASSUMPTION_DAY,
    22192_i32 => _REPUBLIC_DAY,
    22219_i32 => _ALL_SAINTS__DAY,
    22249_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    22256_i32 => _IMMACULATE_CONCEPTION,
    22273_i32 => _CHRISTMAS_DAY,
    22280_i32 => _NEW_YEAR_S_DAY,
    22380_i32 => _GOOD_FRIDAY,
    22382_i32 => _EASTER_SUNDAY,
    22394_i32 => _FREEDOM_DAY,
    22400_i32 => _LABOR_DAY,
    22440_i32 => _DAY_OF_PORTUGAL__CAM_ES__AND_THE_PORTUGUESE_COMMUNITIES,
    22442_i32 => _CORPUS_CHRISTI,
    22506_i32 => _ASSUMPTION_DAY,
    22557_i32 => _REPUBLIC_DAY,
    22584_i32 => _ALL_SAINTS__DAY,
    22614_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    22621_i32 => _IMMACULATE_CONCEPTION,
    22638_i32 => _CHRISTMAS_DAY,
    22645_i32 => _NEW_YEAR_S_DAY,
    22730_i32 => _GOOD_FRIDAY,
    22732_i32 => _EASTER_SUNDAY,
    22760_i32 => _FREEDOM_DAY,
    22766_i32 => _LABOR_DAY,
    22792_i32 => _CORPUS_CHRISTI,
    22806_i32 => _DAY_OF_PORTUGAL__CAM_ES__AND_THE_PORTUGUESE_COMMUNITIES,
    22872_i32 => _ASSUMPTION_DAY,
    22923_i32 => _REPUBLIC_DAY,
    22950_i32 => _ALL_SAINTS__DAY,
    22980_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    22987_i32 => _IMMACULATE_CONCEPTION,
    23004_i32 => _CHRISTMAS_DAY,
    23011_i32 => _NEW_YEAR_S_DAY,
    23115_i32 => _GOOD_FRIDAY,
    23117_i32 => _EASTER_SUNDAY,
    23125_i32 => _FREEDOM_DAY,
    23131_i32 => _LABOR_DAY,
    23171_i32 => _DAY_OF_PORTUGAL__CAM_ES__AND_THE_PORTUGUESE_COMMUNITIES,
    23177_i32 => _CORPUS_CHRISTI,
    23237_i32 => _ASSUMPTION_DAY,
    23288_i32 => _REPUBLIC_DAY,
    23315_i32 => _ALL_SAINTS__DAY,
    23345_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    23352_i32 => _IMMACULATE_CONCEPTION,
    23369_i32 => _CHRISTMAS_DAY,
    23376_i32 => _NEW_YEAR_S_DAY,
    23472_i32 => _GOOD_FRIDAY,
    23474_i32 => _EASTER_SUNDAY,
    23490_i32 => _FREEDOM_DAY,
    23496_i32 => _LABOR_DAY,
    23534_i32 => _CORPUS_CHRISTI,
    23536_i32 => _DAY_OF_PORTUGAL__CAM_ES__AND_THE_PORTUGUESE_COMMUNITIES,
    23602_i32 => _ASSUMPTION_DAY,
    23653_i32 => _REPUBLIC_DAY,
    23680_i32 => _ALL_SAINTS__DAY,
    23710_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    23717_i32 => _IMMACULATE_CONCEPTION,
    23734_i32 => _CHRISTMAS_DAY,
    23741_i32 => _NEW_YEAR_S_DAY,
    23822_i32 => _GOOD_FRIDAY,
    23824_i32 => _EASTER_SUNDAY,
    23855_i32 => _FREEDOM_DAY,
    23861_i32 => _LABOR_DAY,
    23884_i32 => _CORPUS_CHRISTI,
    23901_i32 => _DAY_OF_PORTUGAL__CAM_ES__AND_THE_PORTUGUESE_COMMUNITIES,
    23967_i32 => _ASSUMPTION_DAY,
    24018_i32 => _REPUBLIC_DAY,
    24045_i32 => _ALL_SAINTS__DAY,
    24075_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    24082_i32 => _IMMACULATE_CONCEPTION,
    24099_i32 => _CHRISTMAS_DAY,
    24106_i32 => _NEW_YEAR_S_DAY,
    24207_i32 => _GOOD_FRIDAY,
    24209_i32 => _EASTER_SUNDAY,
    24221_i32 => _FREEDOM_DAY,
    24227_i32 => _LABOR_DAY,
    24267_i32 => _DAY_OF_PORTUGAL__CAM_ES__AND_THE_PORTUGUESE_COMMUNITIES,
    24269_i32 => _CORPUS_CHRISTI,
    24333_i32 => _ASSUMPTION_DAY,
    24384_i32 => _REPUBLIC_DAY,
    24411_i32 => _ALL_SAINTS__DAY,
    24441_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    24448_i32 => _IMMACULATE_CONCEPTION,
    24465_i32 => _CHRISTMAS_DAY,
    24472_i32 => _NEW_YEAR_S_DAY,
    24564_i32 => _GOOD_FRIDAY,
    24566_i32 => _EASTER_SUNDAY,
    24586_i32 => _FREEDOM_DAY,
    24592_i32 => _LABOR_DAY,
    24626_i32 => _CORPUS_CHRISTI,
    24632_i32 => _DAY_OF_PORTUGAL__CAM_ES__AND_THE_PORTUGUESE_COMMUNITIES,
    24698_i32 => _ASSUMPTION_DAY,
    24749_i32 => _REPUBLIC_DAY,
    24776_i32 => _ALL_SAINTS__DAY,
    24806_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    24813_i32 => _IMMACULATE_CONCEPTION,
    24830_i32 => _CHRISTMAS_DAY,
    24837_i32 => _NEW_YEAR_S_DAY,
    24949_i32 => _GOOD_FRIDAY,
    24951_i32 => _EASTER_SUNDAY__FREEDOM_DAY,
    24957_i32 => _LABOR_DAY,
    24997_i32 => _DAY_OF_PORTUGAL__CAM_ES__AND_THE_PORTUGUESE_COMMUNITIES,
    25011_i32 => _CORPUS_CHRISTI,
    25063_i32 => _ASSUMPTION_DAY,
    25114_i32 => _REPUBLIC_DAY,
    25141_i32 => _ALL_SAINTS__DAY,
    25171_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    25178_i32 => _IMMACULATE_CONCEPTION,
    25195_i32 => _CHRISTMAS_DAY,
    25202_i32 => _NEW_YEAR_S_DAY,
    25299_i32 => _GOOD_FRIDAY,
    25301_i32 => _EASTER_SUNDAY,
    25316_i32 => _FREEDOM_DAY,
    25322_i32 => _LABOR_DAY,
    25361_i32 => _CORPUS_CHRISTI,
    25362_i32 => _DAY_OF_PORTUGAL__CAM_ES__AND_THE_PORTUGUESE_COMMUNITIES,
    25428_i32 => _ASSUMPTION_DAY,
    25479_i32 => _REPUBLIC_DAY,
    25506_i32 => _ALL_SAINTS__DAY,
    25536_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    25543_i32 => _IMMACULATE_CONCEPTION,
    25560_i32 => _CHRISTMAS_DAY,
    25567_i32 => _NEW_YEAR_S_DAY,
    25656_i32 => _GOOD_FRIDAY,
    25658_i32 => _EASTER_SUNDAY,
    25682_i32 => _FREEDOM_DAY,
    25688_i32 => _LABOR_DAY,
    25718_i32 => _CORPUS_CHRISTI,
    25728_i32 => _DAY_OF_PORTUGAL__CAM_ES__AND_THE_PORTUGUESE_COMMUNITIES,
    25794_i32 => _ASSUMPTION_DAY,
    25845_i32 => _REPUBLIC_DAY,
    25872_i32 => _ALL_SAINTS__DAY,
    25902_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    25909_i32 => _IMMACULATE_CONCEPTION,
    25926_i32 => _CHRISTMAS_DAY,
    25933_i32 => _NEW_YEAR_S_DAY,
    26041_i32 => _GOOD_FRIDAY,
    26043_i32 => _EASTER_SUNDAY,
    26047_i32 => _FREEDOM_DAY,
    26053_i32 => _LABOR_DAY,
    26093_i32 => _DAY_OF_PORTUGAL__CAM_ES__AND_THE_PORTUGUESE_COMMUNITIES,
    26103_i32 => _CORPUS_CHRISTI,
    26159_i32 => _ASSUMPTION_DAY,
    26210_i32 => _REPUBLIC_DAY,
    26237_i32 => _ALL_SAINTS__DAY,
    26267_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    26274_i32 => _IMMACULATE_CONCEPTION,
    26291_i32 => _CHRISTMAS_DAY,
    26298_i32 => _NEW_YEAR_S_DAY,
    26391_i32 => _GOOD_FRIDAY,
    26393_i32 => _EASTER_SUNDAY,
    26412_i32 => _FREEDOM_DAY,
    26418_i32 => _LABOR_DAY,
    26453_i32 => _CORPUS_CHRISTI,
    26458_i32 => _DAY_OF_PORTUGAL__CAM_ES__AND_THE_PORTUGUESE_COMMUNITIES,
    26524_i32 => _ASSUMPTION_DAY,
    26575_i32 => _REPUBLIC_DAY,
    26602_i32 => _ALL_SAINTS__DAY,
    26632_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    26639_i32 => _IMMACULATE_CONCEPTION,
    26656_i32 => _CHRISTMAS_DAY,
    26663_i32 => _NEW_YEAR_S_DAY,
    26748_i32 => _GOOD_FRIDAY,
    26750_i32 => _EASTER_SUNDAY,
    26777_i32 => _FREEDOM_DAY,
    26783_i32 => _LABOR_DAY,
    26810_i32 => _CORPUS_CHRISTI,
    26823_i32 => _DAY_OF_PORTUGAL__CAM_ES__AND_THE_PORTUGUESE_COMMUNITIES,
    26889_i32 => _ASSUMPTION_DAY,
    26940_i32 => _REPUBLIC_DAY,
    26967_i32 => _ALL_SAINTS__DAY,
    26997_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    27004_i32 => _IMMACULATE_CONCEPTION,
    27021_i32 => _CHRISTMAS_DAY,
    27028_i32 => _NEW_YEAR_S_DAY,
    27133_i32 => _GOOD_FRIDAY,
    27135_i32 => _EASTER_SUNDAY,
    27143_i32 => _FREEDOM_DAY,
    27149_i32 => _LABOR_DAY,
    27189_i32 => _DAY_OF_PORTUGAL__CAM_ES__AND_THE_PORTUGUESE_COMMUNITIES,
    27195_i32 => _CORPUS_CHRISTI,
    27255_i32 => _ASSUMPTION_DAY,
    27306_i32 => _REPUBLIC_DAY,
    27333_i32 => _ALL_SAINTS__DAY,
    27363_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    27370_i32 => _IMMACULATE_CONCEPTION,
    27387_i32 => _CHRISTMAS_DAY,
    27394_i32 => _NEW_YEAR_S_DAY,
    27490_i32 => _GOOD_FRIDAY,
    27492_i32 => _EASTER_SUNDAY,
    27508_i32 => _FREEDOM_DAY,
    27514_i32 => _LABOR_DAY,
    27552_i32 => _CORPUS_CHRISTI,
    27554_i32 => _DAY_OF_PORTUGAL__CAM_ES__AND_THE_PORTUGUESE_COMMUNITIES,
    27620_i32 => _ASSUMPTION_DAY,
    27671_i32 => _REPUBLIC_DAY,
    27698_i32 => _ALL_SAINTS__DAY,
    27728_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    27735_i32 => _IMMACULATE_CONCEPTION,
    27752_i32 => _CHRISTMAS_DAY,
    27759_i32 => _NEW_YEAR_S_DAY,
    27840_i32 => _GOOD_FRIDAY,
    27842_i32 => _EASTER_SUNDAY,
    27873_i32 => _FREEDOM_DAY,
    27879_i32 => _LABOR_DAY,
    27902_i32 => _CORPUS_CHRISTI,
    27919_i32 => _DAY_OF_PORTUGAL__CAM_ES__AND_THE_PORTUGUESE_COMMUNITIES,
    27985_i32 => _ASSUMPTION_DAY,
    28036_i32 => _REPUBLIC_DAY,
    28063_i32 => _ALL_SAINTS__DAY,
    28093_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    28100_i32 => _IMMACULATE_CONCEPTION,
    28117_i32 => _CHRISTMAS_DAY,
    28124_i32 => _NEW_YEAR_S_DAY,
    28225_i32 => _GOOD_FRIDAY,
    28227_i32 => _EASTER_SUNDAY,
    28238_i32 => _FREEDOM_DAY,
    28244_i32 => _LABOR_DAY,
    28284_i32 => _DAY_OF_PORTUGAL__CAM_ES__AND_THE_PORTUGUESE_COMMUNITIES,
    28287_i32 => _CORPUS_CHRISTI,
    28350_i32 => _ASSUMPTION_DAY,
    28401_i32 => _REPUBLIC_DAY,
    28428_i32 => _ALL_SAINTS__DAY,
    28458_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    28465_i32 => _IMMACULATE_CONCEPTION,
    28482_i32 => _CHRISTMAS_DAY,
    28489_i32 => _NEW_YEAR_S_DAY,
    28582_i32 => _GOOD_FRIDAY,
    28584_i32 => _EASTER_SUNDAY,
    28604_i32 => _FREEDOM_DAY,
    28610_i32 => _LABOR_DAY,
    28644_i32 => _CORPUS_CHRISTI,
    28650_i32 => _DAY_OF_PORTUGAL__CAM_ES__AND_THE_PORTUGUESE_COMMUNITIES,
    28716_i32 => _ASSUMPTION_DAY,
    28767_i32 => _REPUBLIC_DAY,
    28794_i32 => _ALL_SAINTS__DAY,
    28824_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    28831_i32 => _IMMACULATE_CONCEPTION,
    28848_i32 => _CHRISTMAS_DAY,
    28855_i32 => _NEW_YEAR_S_DAY,
    28960_i32 => _GOOD_FRIDAY,
    28962_i32 => _EASTER_SUNDAY,
    28969_i32 => _FREEDOM_DAY,
    28975_i32 => _LABOR_DAY,
    29015_i32 => _DAY_OF_PORTUGAL__CAM_ES__AND_THE_PORTUGUESE_COMMUNITIES,
    29022_i32 => _CORPUS_CHRISTI,
    29081_i32 => _ASSUMPTION_DAY,
    29132_i32 => _REPUBLIC_DAY,
    29159_i32 => _ALL_SAINTS__DAY,
    29189_i32 => _RESTORATION_OF_INDEPENDENCE_DAY,
    29196_i32 => _IMMACULATE_CONCEPTION,
    29213_i32 => _CHRISTMAS_DAY,
    29220_i32 => _NEW_YEAR_S_DAY,
};
