use crate::countries::constants::*;
use phf::phf_map;

pub static PR_HOLIDAYS: phf::Map<i32, &'static str> = phf_map! {
    10957_i32 => _NEW_YEAR_S_DAY,
    10962_i32 => _EPIPHANY,
    10973_i32 => _MARTIN_LUTHER_KING_JR__DAY,
    11008_i32 => _PRESIDENTS__DAY,
    11038_i32 => _EMANCIPATION_DAY,
    11068_i32 => _GOOD_FRIDAY,
    11106_i32 => _MEMORIAL_DAY,
    11142_i32 => _INDEPENDENCE_DAY,
    11163_i32 => _CONSTITUTION_DAY,
    11204_i32 => _LABOR_DAY,
    11271_i32 => _VETERANS_DAY__OBSERVED_,
    11272_i32 => _VETERANS_DAY,
    11280_i32 => _DISCOVERY_DAY,
    11281_i32 => _DISCOVERY_DAY__OBSERVED_,
    11284_i32 => _THANKSGIVING,
    11316_i32 => _CHRISTMAS_DAY,
    11323_i32 => _NEW_YEAR_S_DAY,
    11328_i32 => _EPIPHANY,
    11337_i32 => _MARTIN_LUTHER_KING_JR__DAY,
    11372_i32 => _PRESIDENTS__DAY,
    11403_i32 => _EMANCIPATION_DAY,
    11425_i32 => _GOOD_FRIDAY,
    11470_i32 => _MEMORIAL_DAY,
    11507_i32 => _INDEPENDENCE_DAY,
    11528_i32 => _CONSTITUTION_DAY,
    11568_i32 => _LABOR_DAY,
    11637_i32 => _VETERANS_DAY,
    11638_i32 => _VETERANS_DAY__OBSERVED_,
    11645_i32 => _DISCOVERY_DAY,
    11648_i32 => _THANKSGIVING,
    11681_i32 => _CHRISTMAS_DAY,
    11688_i32 => _NEW_YEAR_S_DAY,
    11693_i32 => _EPIPHANY,
    11708_i32 => _MARTIN_LUTHER_KING_JR__DAY,
    11736_i32 => _PRESIDENTS__DAY,
    11768_i32 => _EMANCIPATION_DAY,
    11775_i32 => _GOOD_FRIDAY,
    11834_i32 => _MEMORIAL_DAY,
    11872_i32 => _INDEPENDENCE_DAY,
    11893_i32 => _CONSTITUTION_DAY,
    11932_i32 => _LABOR_DAY,
    12002_i32 => _VETERANS_DAY,
    12010_i32 => _DISCOVERY_DAY,
    12019_i32 => _THANKSGIVING,
    12046_i32 => _CHRISTMAS_DAY,
    12053_i32 => _NEW_YEAR_S_DAY,
    12058_i32 => _EPIPHANY,
    12072_i32 => _MARTIN_LUTHER_KING_JR__DAY,
    12100_i32 => _PRESIDENTS__DAY,
    12133_i32 => _EMANCIPATION_DAY,
    12160_i32 => _GOOD_FRIDAY,
    12198_i32 => _MEMORIAL_DAY,
    12237_i32 => _INDEPENDENCE_DAY,
    12258_i32 => _CONSTITUTION_DAY,
    12296_i32 => _LABOR_DAY,
    12367_i32 => _VETERANS_DAY,
    12375_i32 => _DISCOVERY_DAY,
    12383_i32 => _THANKSGIVING,
    12411_i32 => _CHRISTMAS_DAY,
    12418_i32 => _NEW_YEAR_S_DAY,
    12423_i32 => _EPIPHANY,
    12436_i32 => _MARTIN_LUTHER_KING_JR__DAY,
    12464_i32 => _PRESIDENTS__DAY,
    12499_i32 => _EMANCIPATION_DAY,
    12517_i32 => _GOOD_FRIDAY,
    12569_i32 => _MEMORIAL_DAY,
    12603_i32 => _INDEPENDENCE_DAY,
    12604_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    12624_i32 => _CONSTITUTION_DAY,
    12625_i32 => _CONSTITUTION_DAY__OBSERVED_,
    12667_i32 => _LABOR_DAY,
    12733_i32 => _VETERANS_DAY,
    12741_i32 => _DISCOVERY_DAY,
    12747_i32 => _THANKSGIVING,
    12776_i32 => _CHRISTMAS_DAY__OBSERVED_,
    12777_i32 => _CHRISTMAS_DAY,
    12783_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    12784_i32 => _NEW_YEAR_S_DAY,
    12789_i32 => _EPIPHANY,
    12800_i32 => _MARTIN_LUTHER_KING_JR__DAY,
    12835_i32 => _PRESIDENTS__DAY,
    12864_i32 => _EMANCIPATION_DAY,
    12867_i32 => _GOOD_FRIDAY,
    12933_i32 => _MEMORIAL_DAY,
    12968_i32 => _INDEPENDENCE_DAY,
    12989_i32 => _CONSTITUTION_DAY,
    13031_i32 => _LABOR_DAY,
    13098_i32 => _VETERANS_DAY,
    13106_i32 => _DISCOVERY_DAY,
    13111_i32 => _THANKSGIVING,
    13142_i32 => _CHRISTMAS_DAY,
    13143_i32 => _CHRISTMAS_DAY__OBSERVED_,
    13149_i32 => _NEW_YEAR_S_DAY,
    13150_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    13154_i32 => _EPIPHANY,
    13164_i32 => _MARTIN_LUTHER_KING_JR__DAY,
    13199_i32 => _PRESIDENTS__DAY,
    13229_i32 => _EMANCIPATION_DAY,
    13252_i32 => _GOOD_FRIDAY,
    13297_i32 => _MEMORIAL_DAY,
    13333_i32 => _INDEPENDENCE_DAY,
    13354_i32 => _CONSTITUTION_DAY,
    13395_i32 => _LABOR_DAY,
    13462_i32 => _VETERANS_DAY__OBSERVED_,
    13463_i32 => _VETERANS_DAY,
    13471_i32 => _DISCOVERY_DAY,
    13472_i32 => _DISCOVERY_DAY__OBSERVED_,
    13475_i32 => _THANKSGIVING,
    13507_i32 => _CHRISTMAS_DAY,
    13514_i32 => _NEW_YEAR_S_DAY,
    13519_i32 => _EPIPHANY,
    13528_i32 => _MARTIN_LUTHER_KING_JR__DAY,
    13563_i32 => _PRESIDENTS__DAY,
    13594_i32 => _EMANCIPATION_DAY,
    13609_i32 => _GOOD_FRIDAY,
    13661_i32 => _MEMORIAL_DAY,
    13698_i32 => _INDEPENDENCE_DAY,
    13719_i32 => _CONSTITUTION_DAY,
    13759_i32 => _LABOR_DAY,
    13828_i32 => _VETERANS_DAY,
    13829_i32 => _VETERANS_DAY__OBSERVED_,
    13836_i32 => _DISCOVERY_DAY,
    13839_i32 => _THANKSGIVING,
    13872_i32 => _CHRISTMAS_DAY,
    13879_i32 => _NEW_YEAR_S_DAY,
    13884_i32 => _EPIPHANY,
    13899_i32 => _MARTIN_LUTHER_KING_JR__DAY,
    13927_i32 => _PRESIDENTS__DAY,
    13959_i32 => _GOOD_FRIDAY,
    13960_i32 => _EMANCIPATION_DAY,
    14025_i32 => _MEMORIAL_DAY,
    14064_i32 => _INDEPENDENCE_DAY,
    14085_i32 => _CONSTITUTION_DAY,
    14123_i32 => _LABOR_DAY,
    14194_i32 => _VETERANS_DAY,
    14202_i32 => _DISCOVERY_DAY,
    14210_i32 => _THANKSGIVING,
    14238_i32 => _CHRISTMAS_DAY,
    14245_i32 => _NEW_YEAR_S_DAY,
    14250_i32 => _EPIPHANY,
    14263_i32 => _MARTIN_LUTHER_KING_JR__DAY,
    14291_i32 => _PRESIDENTS__DAY,
    14325_i32 => _EMANCIPATION_DAY,
    14326_i32 => _EMANCIPATION_DAY__OBSERVED_,
    14344_i32 => _GOOD_FRIDAY,
    14389_i32 => _MEMORIAL_DAY,
    14428_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    14429_i32 => _INDEPENDENCE_DAY,
    14450_i32 => _CONSTITUTION_DAY,
    14494_i32 => _LABOR_DAY,
    14559_i32 => _VETERANS_DAY,
    14567_i32 => _DISCOVERY_DAY,
    14574_i32 => _THANKSGIVING,
    14603_i32 => _CHRISTMAS_DAY,
    14610_i32 => _NEW_YEAR_S_DAY,
    14615_i32 => _EPIPHANY,
    14627_i32 => _MARTIN_LUTHER_KING_JR__DAY,
    14655_i32 => _PRESIDENTS__DAY,
    14690_i32 => _EMANCIPATION_DAY,
    14701_i32 => _GOOD_FRIDAY,
    14760_i32 => _MEMORIAL_DAY,
    14794_i32 => _INDEPENDENCE_DAY,
    14795_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    14815_i32 => _CONSTITUTION_DAY,
    14816_i32 => _CONSTITUTION_DAY__OBSERVED_,
    14858_i32 => _LABOR_DAY,
    14924_i32 => _VETERANS_DAY,
    14932_i32 => _DISCOVERY_DAY,
    14938_i32 => _THANKSGIVING,
    14967_i32 => _CHRISTMAS_DAY__OBSERVED_,
    14968_i32 => _CHRISTMAS_DAY,
    14974_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    14975_i32 => _NEW_YEAR_S_DAY,
    14980_i32 => _EPIPHANY,
    14991_i32 => _MARTIN_LUTHER_KING_JR__DAY,
    15026_i32 => _PRESIDENTS__DAY,
    15055_i32 => _EMANCIPATION_DAY,
    15086_i32 => _GOOD_FRIDAY,
    15124_i32 => _MEMORIAL_DAY,
    15159_i32 => _INDEPENDENCE_DAY,
    15180_i32 => _CONSTITUTION_DAY,
    15222_i32 => _LABOR_DAY,
    15289_i32 => _VETERANS_DAY,
    15297_i32 => _DISCOVERY_DAY,
    15302_i32 => _THANKSGIVING,
    15333_i32 => _CHRISTMAS_DAY,
    15334_i32 => _CHRISTMAS_DAY__OBSERVED_,
    15340_i32 => _NEW_YEAR_S_DAY,
    15341_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    15345_i32 => _EPIPHANY,
    15355_i32 => _MARTIN_LUTHER_KING_JR__DAY,
    15390_i32 => _PRESIDENTS__DAY,
    15421_i32 => _EMANCIPATION_DAY,
    15436_i32 => _GOOD_FRIDAY,
    15488_i32 => _MEMORIAL_DAY,
    15525_i32 => _INDEPENDENCE_DAY,
    15546_i32 => _CONSTITUTION_DAY,
    15586_i32 => _LABOR_DAY,
    15655_i32 => _VETERANS_DAY,
    15656_i32 => _VETERANS_DAY__OBSERVED_,
    15663_i32 => _DISCOVERY_DAY,
    15666_i32 => _THANKSGIVING,
    15699_i32 => _CHRISTMAS_DAY,
    15706_i32 => _NEW_YEAR_S_DAY,
    15711_i32 => _EPIPHANY,
    15726_i32 => _MARTIN_LUTHER_KING_JR__DAY,
    15754_i32 => _PRESIDENTS__DAY,
    15786_i32 => _EMANCIPATION_DAY,
    15793_i32 => _GOOD_FRIDAY,
    15852_i32 => _MEMORIAL_DAY,
    15890_i32 => _INDEPENDENCE_DAY,
    15911_i32 => _CONSTITUTION_DAY,
    15950_i32 => _LABOR_DAY,
    16020_i32 => _VETERANS_DAY,
    16028_i32 => _DISCOVERY_DAY,
    16037_i32 => _THANKSGIVING,
    16064_i32 => _CHRISTMAS_DAY,
    16071_i32 => _NEW_YEAR_S_DAY,
    16076_i32 => _EPIPHANY,
    16090_i32 => _MARTIN_LUTHER_KING_JR__DAY,
    16118_i32 => _PRESIDENTS__DAY,
    16151_i32 => _EMANCIPATION_DAY,
    16178_i32 => _GOOD_FRIDAY,
    16216_i32 => _MEMORIAL_DAY,
    16255_i32 => _INDEPENDENCE_DAY,
    16276_i32 => _CONSTITUTION_DAY,
    16314_i32 => _LABOR_DAY,
    16385_i32 => _VETERANS_DAY,
    16393_i32 => _DISCOVERY_DAY,
    16401_i32 => _THANKSGIVING,
    16429_i32 => _CHRISTMAS_DAY,
    16436_i32 => _NEW_YEAR_S_DAY,
    16441_i32 => _EPIPHANY,
    16454_i32 => _MARTIN_LUTHER_KING_JR__DAY,
    16482_i32 => _PRESIDENTS__DAY,
    16516_i32 => _EMANCIPATION_DAY,
    16517_i32 => _EMANCIPATION_DAY__OBSERVED_,
    16528_i32 => _GOOD_FRIDAY,
    16580_i32 => _MEMORIAL_DAY,
    16619_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    16620_i32 => _INDEPENDENCE_DAY,
    16641_i32 => _CONSTITUTION_DAY,
    16685_i32 => _LABOR_DAY,
    16750_i32 => _VETERANS_DAY,
    16758_i32 => _DISCOVERY_DAY,
    16765_i32 => _THANKSGIVING,
    16794_i32 => _CHRISTMAS_DAY,
    16801_i32 => _NEW_YEAR_S_DAY,
    16806_i32 => _EPIPHANY,
    16818_i32 => _MARTIN_LUTHER_KING_JR__DAY,
    16846_i32 => _PRESIDENTS__DAY,
    16882_i32 => _EMANCIPATION_DAY,
    16885_i32 => _GOOD_FRIDAY,
    16951_i32 => _MEMORIAL_DAY,
    16986_i32 => _INDEPENDENCE_DAY,
    17007_i32 => _CONSTITUTION_DAY,
    17049_i32 => _LABOR_DAY,
    17116_i32 => _VETERANS_DAY,
    17124_i32 => _DISCOVERY_DAY,
    17129_i32 => _THANKSGIVING,
    17160_i32 => _CHRISTMAS_DAY,
    17161_i32 => _CHRISTMAS_DAY__OBSERVED_,
    17167_i32 => _NEW_YEAR_S_DAY,
    17168_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    17172_i32 => _EPIPHANY,
    17182_i32 => _MARTIN_LUTHER_KING_JR__DAY,
    17217_i32 => _PRESIDENTS__DAY,
    17247_i32 => _EMANCIPATION_DAY,
    17270_i32 => _GOOD_FRIDAY,
    17315_i32 => _MEMORIAL_DAY,
    17351_i32 => _INDEPENDENCE_DAY,
    17372_i32 => _CONSTITUTION_DAY,
    17413_i32 => _LABOR_DAY,
    17480_i32 => _VETERANS_DAY__OBSERVED_,
    17481_i32 => _VETERANS_DAY,
    17489_i32 => _DISCOVERY_DAY,
    17490_i32 => _DISCOVERY_DAY__OBSERVED_,
    17493_i32 => _THANKSGIVING,
    17525_i32 => _CHRISTMAS_DAY,
    17532_i32 => _NEW_YEAR_S_DAY,
    17537_i32 => _EPIPHANY,
    17546_i32 => _MARTIN_LUTHER_KING_JR__DAY,
    17581_i32 => _PRESIDENTS__DAY,
    17612_i32 => _EMANCIPATION_DAY,
    17620_i32 => _GOOD_FRIDAY,
    17679_i32 => _MEMORIAL_DAY,
    17716_i32 => _INDEPENDENCE_DAY,
    17737_i32 => _CONSTITUTION_DAY,
    17777_i32 => _LABOR_DAY,
    17846_i32 => _VETERANS_DAY,
    17847_i32 => _VETERANS_DAY__OBSERVED_,
    17854_i32 => _DISCOVERY_DAY,
    17857_i32 => _THANKSGIVING,
    17890_i32 => _CHRISTMAS_DAY,
    17897_i32 => _NEW_YEAR_S_DAY,
    17902_i32 => _EPIPHANY,
    17917_i32 => _MARTIN_LUTHER_KING_JR__DAY,
    17945_i32 => _PRESIDENTS__DAY,
    17977_i32 => _EMANCIPATION_DAY,
    18005_i32 => _GOOD_FRIDAY,
    18043_i32 => _MEMORIAL_DAY,
    18081_i32 => _INDEPENDENCE_DAY,
    18102_i32 => _CONSTITUTION_DAY,
    18141_i32 => _LABOR_DAY,
    18211_i32 => _VETERANS_DAY,
    18219_i32 => _DISCOVERY_DAY,
    18228_i32 => _THANKSGIVING,
    18255_i32 => _CHRISTMAS_DAY,
    18262_i32 => _NEW_YEAR_S_DAY,
    18267_i32 => _EPIPHANY,
    18281_i32 => _MARTIN_LUTHER_KING_JR__DAY,
    18309_i32 => _PRESIDENTS__DAY,
    18343_i32 => _EMANCIPATION_DAY,
    18344_i32 => _EMANCIPATION_DAY__OBSERVED_,
    18362_i32 => _GOOD_FRIDAY,
    18407_i32 => _MEMORIAL_DAY,
    18446_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    18447_i32 => _INDEPENDENCE_DAY,
    18468_i32 => _CONSTITUTION_DAY,
    18512_i32 => _LABOR_DAY,
    18577_i32 => _VETERANS_DAY,
    18585_i32 => _DISCOVERY_DAY,
    18592_i32 => _THANKSGIVING,
    18621_i32 => _CHRISTMAS_DAY,
    18628_i32 => _NEW_YEAR_S_DAY,
    18633_i32 => _EPIPHANY,
    18645_i32 => _MARTIN_LUTHER_KING_JR__DAY,
    18673_i32 => _PRESIDENTS__DAY,
    18708_i32 => _EMANCIPATION_DAY,
    18719_i32 => _GOOD_FRIDAY,
    18778_i32 => _MEMORIAL_DAY,
    18796_i32 => _JUNETEENTH_NATIONAL_INDEPENDENCE_DAY__OBSERVED_,
    18797_i32 => _JUNETEENTH_NATIONAL_INDEPENDENCE_DAY,
    18812_i32 => _INDEPENDENCE_DAY,
    18813_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    18833_i32 => _CONSTITUTION_DAY,
    18834_i32 => _CONSTITUTION_DAY__OBSERVED_,
    18876_i32 => _LABOR_DAY,
    18942_i32 => _VETERANS_DAY,
    18950_i32 => _DISCOVERY_DAY,
    18956_i32 => _THANKSGIVING,
    18985_i32 => _CHRISTMAS_DAY__OBSERVED_,
    18986_i32 => _CHRISTMAS_DAY,
    18992_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    18993_i32 => _NEW_YEAR_S_DAY,
    18998_i32 => _EPIPHANY,
    19009_i32 => _MARTIN_LUTHER_KING_JR__DAY,
    19044_i32 => _PRESIDENTS__DAY,
    19073_i32 => _EMANCIPATION_DAY,
    19097_i32 => _GOOD_FRIDAY,
    19142_i32 => _MEMORIAL_DAY,
    19162_i32 => _JUNETEENTH_NATIONAL_INDEPENDENCE_DAY,
    19163_i32 => _JUNETEENTH_NATIONAL_INDEPENDENCE_DAY__OBSERVED_,
    19177_i32 => _INDEPENDENCE_DAY,
    19198_i32 => _CONSTITUTION_DAY,
    19240_i32 => _LABOR_DAY,
    19307_i32 => _VETERANS_DAY,
    19315_i32 => _DISCOVERY_DAY,
    19320_i32 => _THANKSGIVING,
    19351_i32 => _CHRISTMAS_DAY,
    19352_i32 => _CHRISTMAS_DAY__OBSERVED_,
    19358_i32 => _NEW_YEAR_S_DAY,
    19359_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    19363_i32 => _EPIPHANY,
    19373_i32 => _MARTIN_LUTHER_KING_JR__DAY,
    19408_i32 => _PRESIDENTS__DAY,
    19438_i32 => _EMANCIPATION_DAY,
    19454_i32 => _GOOD_FRIDAY,
    19506_i32 => _MEMORIAL_DAY,
    19527_i32 => _JUNETEENTH_NATIONAL_INDEPENDENCE_DAY,
    19542_i32 => _INDEPENDENCE_DAY,
    19563_i32 => _CONSTITUTION_DAY,
    19604_i32 => _LABOR_DAY,
    19671_i32 => _VETERANS_DAY__OBSERVED_,
    19672_i32 => _VETERANS_DAY,
    19680_i32 => _DISCOVERY_DAY,
    19681_i32 => _DISCOVERY_DAY__OBSERVED_,
    19684_i32 => _THANKSGIVING,
    19716_i32 => _CHRISTMAS_DAY,
    19723_i32 => _NEW_YEAR_S_DAY,
    19728_i32 => _EPIPHANY,
    19737_i32 => _MARTIN_LUTHER_KING_JR__DAY,
    19772_i32 => _PRESIDENTS__DAY,
    19804_i32 => _EMANCIPATION_DAY,
    19811_i32 => _GOOD_FRIDAY,
    19870_i32 => _MEMORIAL_DAY,
    19893_i32 => _JUNETEENTH_NATIONAL_INDEPENDENCE_DAY,
    19908_i32 => _INDEPENDENCE_DAY,
    19929_i32 => _CONSTITUTION_DAY,
    19968_i32 => _LABOR_DAY,
    20038_i32 => _VETERANS_DAY,
    20046_i32 => _DISCOVERY_DAY,
    20055_i32 => _THANKSGIVING,
    20082_i32 => _CHRISTMAS_DAY,
    20089_i32 => _NEW_YEAR_S_DAY,
    20094_i32 => _EPIPHANY,
    20108_i32 => _MARTIN_LUTHER_KING_JR__DAY,
    20136_i32 => _PRESIDENTS__DAY,
    20169_i32 => _EMANCIPATION_DAY,
    20196_i32 => _GOOD_FRIDAY,
    20234_i32 => _MEMORIAL_DAY,
    20258_i32 => _JUNETEENTH_NATIONAL_INDEPENDENCE_DAY,
    20273_i32 => _INDEPENDENCE_DAY,
    20294_i32 => _CONSTITUTION_DAY,
    20332_i32 => _LABOR_DAY,
    20403_i32 => _VETERANS_DAY,
    20411_i32 => _DISCOVERY_DAY,
    20419_i32 => _THANKSGIVING,
    20447_i32 => _CHRISTMAS_DAY,
    20454_i32 => _NEW_YEAR_S_DAY,
    20459_i32 => _EPIPHANY,
    20472_i32 => _MARTIN_LUTHER_KING_JR__DAY,
    20500_i32 => _PRESIDENTS__DAY,
    20534_i32 => _EMANCIPATION_DAY,
    20535_i32 => _EMANCIPATION_DAY__OBSERVED_,
    20546_i32 => _GOOD_FRIDAY,
    20598_i32 => _MEMORIAL_DAY,
    20623_i32 => _JUNETEENTH_NATIONAL_INDEPENDENCE_DAY,
    20637_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    20638_i32 => _INDEPENDENCE_DAY,
    20659_i32 => _CONSTITUTION_DAY,
    20703_i32 => _LABOR_DAY,
    20768_i32 => _VETERANS_DAY,
    20776_i32 => _DISCOVERY_DAY,
    20783_i32 => _THANKSGIVING,
    20812_i32 => _CHRISTMAS_DAY,
    20819_i32 => _NEW_YEAR_S_DAY,
    20824_i32 => _EPIPHANY,
    20836_i32 => _MARTIN_LUTHER_KING_JR__DAY,
    20864_i32 => _PRESIDENTS__DAY,
    20899_i32 => _EMANCIPATION_DAY,
    20903_i32 => _GOOD_FRIDAY,
    20969_i32 => _MEMORIAL_DAY,
    20987_i32 => _JUNETEENTH_NATIONAL_INDEPENDENCE_DAY__OBSERVED_,
    20988_i32 => _JUNETEENTH_NATIONAL_INDEPENDENCE_DAY,
    21003_i32 => _INDEPENDENCE_DAY,
    21004_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    21024_i32 => _CONSTITUTION_DAY,
    21025_i32 => _CONSTITUTION_DAY__OBSERVED_,
    21067_i32 => _LABOR_DAY,
    21133_i32 => _VETERANS_DAY,
    21141_i32 => _DISCOVERY_DAY,
    21147_i32 => _THANKSGIVING,
    21176_i32 => _CHRISTMAS_DAY__OBSERVED_,
    21177_i32 => _CHRISTMAS_DAY,
    21183_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    21184_i32 => _NEW_YEAR_S_DAY,
    21189_i32 => _EPIPHANY,
    21200_i32 => _MARTIN_LUTHER_KING_JR__DAY,
    21235_i32 => _PRESIDENTS__DAY,
    21265_i32 => _EMANCIPATION_DAY,
    21288_i32 => _GOOD_FRIDAY,
    21333_i32 => _MEMORIAL_DAY,
    21354_i32 => _JUNETEENTH_NATIONAL_INDEPENDENCE_DAY,
    21369_i32 => _INDEPENDENCE_DAY,
    21390_i32 => _CONSTITUTION_DAY,
    21431_i32 => _LABOR_DAY,
    21498_i32 => _VETERANS_DAY__OBSERVED_,
    21499_i32 => _VETERANS_DAY,
    21507_i32 => _DISCOVERY_DAY,
    21508_i32 => _DISCOVERY_DAY__OBSERVED_,
    21511_i32 => _THANKSGIVING,
    21543_i32 => _CHRISTMAS_DAY,
    21550_i32 => _NEW_YEAR_S_DAY,
    21555_i32 => _EPIPHANY,
    21564_i32 => _MARTIN_LUTHER_KING_JR__DAY,
    21599_i32 => _PRESIDENTS__DAY,
    21630_i32 => _EMANCIPATION_DAY,
    21638_i32 => _GOOD_FRIDAY,
    21697_i32 => _MEMORIAL_DAY,
    21719_i32 => _JUNETEENTH_NATIONAL_INDEPENDENCE_DAY,
    21734_i32 => _INDEPENDENCE_DAY,
    21755_i32 => _CONSTITUTION_DAY,
    21795_i32 => _LABOR_DAY,
    21864_i32 => _VETERANS_DAY,
    21865_i32 => _VETERANS_DAY__OBSERVED_,
    21872_i32 => _DISCOVERY_DAY,
    21875_i32 => _THANKSGIVING,
    21908_i32 => _CHRISTMAS_DAY,
    21915_i32 => _NEW_YEAR_S_DAY,
    21920_i32 => _EPIPHANY,
    21935_i32 => _MARTIN_LUTHER_KING_JR__DAY,
    21963_i32 => _PRESIDENTS__DAY,
    21995_i32 => _EMANCIPATION_DAY,
    22023_i32 => _GOOD_FRIDAY,
    22061_i32 => _MEMORIAL_DAY,
    22084_i32 => _JUNETEENTH_NATIONAL_INDEPENDENCE_DAY,
    22099_i32 => _INDEPENDENCE_DAY,
    22120_i32 => _CONSTITUTION_DAY,
    22159_i32 => _LABOR_DAY,
    22229_i32 => _VETERANS_DAY,
    22237_i32 => _DISCOVERY_DAY,
    22246_i32 => _THANKSGIVING,
    22273_i32 => _CHRISTMAS_DAY,
    22280_i32 => _NEW_YEAR_S_DAY,
    22285_i32 => _EPIPHANY,
    22299_i32 => _MARTIN_LUTHER_KING_JR__DAY,
    22327_i32 => _PRESIDENTS__DAY,
    22360_i32 => _EMANCIPATION_DAY,
    22380_i32 => _GOOD_FRIDAY,
    22425_i32 => _MEMORIAL_DAY,
    22449_i32 => _JUNETEENTH_NATIONAL_INDEPENDENCE_DAY,
    22464_i32 => _INDEPENDENCE_DAY,
    22485_i32 => _CONSTITUTION_DAY,
    22523_i32 => _LABOR_DAY,
    22594_i32 => _VETERANS_DAY,
    22602_i32 => _DISCOVERY_DAY,
    22610_i32 => _THANKSGIVING,
    22638_i32 => _CHRISTMAS_DAY,
    22645_i32 => _NEW_YEAR_S_DAY,
    22650_i32 => _EPIPHANY,
    22663_i32 => _MARTIN_LUTHER_KING_JR__DAY,
    22691_i32 => _PRESIDENTS__DAY,
    22726_i32 => _EMANCIPATION_DAY,
    22730_i32 => _GOOD_FRIDAY,
    22796_i32 => _MEMORIAL_DAY,
    22814_i32 => _JUNETEENTH_NATIONAL_INDEPENDENCE_DAY__OBSERVED_,
    22815_i32 => _JUNETEENTH_NATIONAL_INDEPENDENCE_DAY,
    22830_i32 => _INDEPENDENCE_DAY,
    22831_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    22851_i32 => _CONSTITUTION_DAY,
    22852_i32 => _CONSTITUTION_DAY__OBSERVED_,
    22894_i32 => _LABOR_DAY,
    22960_i32 => _VETERANS_DAY,
    22968_i32 => _DISCOVERY_DAY,
    22974_i32 => _THANKSGIVING,
    23003_i32 => _CHRISTMAS_DAY__OBSERVED_,
    23004_i32 => _CHRISTMAS_DAY,
    23010_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    23011_i32 => _NEW_YEAR_S_DAY,
    23016_i32 => _EPIPHANY,
    23027_i32 => _MARTIN_LUTHER_KING_JR__DAY,
    23062_i32 => _PRESIDENTS__DAY,
    23091_i32 => _EMANCIPATION_DAY,
    23115_i32 => _GOOD_FRIDAY,
    23160_i32 => _MEMORIAL_DAY,
    23180_i32 => _JUNETEENTH_NATIONAL_INDEPENDENCE_DAY,
    23181_i32 => _JUNETEENTH_NATIONAL_INDEPENDENCE_DAY__OBSERVED_,
    23195_i32 => _INDEPENDENCE_DAY,
    23216_i32 => _CONSTITUTION_DAY,
    23258_i32 => _LABOR_DAY,
    23325_i32 => _VETERANS_DAY,
    23333_i32 => _DISCOVERY_DAY,
    23338_i32 => _THANKSGIVING,
    23369_i32 => _CHRISTMAS_DAY,
    23370_i32 => _CHRISTMAS_DAY__OBSERVED_,
    23376_i32 => _NEW_YEAR_S_DAY,
    23377_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    23381_i32 => _EPIPHANY,
    23391_i32 => _MARTIN_LUTHER_KING_JR__DAY,
    23426_i32 => _PRESIDENTS__DAY,
    23456_i32 => _EMANCIPATION_DAY,
    23472_i32 => _GOOD_FRIDAY,
    23524_i32 => _MEMORIAL_DAY,
    23545_i32 => _JUNETEENTH_NATIONAL_INDEPENDENCE_DAY,
    23560_i32 => _INDEPENDENCE_DAY,
    23581_i32 => _CONSTITUTION_DAY,
    23622_i32 => _LABOR_DAY,
    23689_i32 => _VETERANS_DAY__OBSERVED_,
    23690_i32 => _VETERANS_DAY,
    23698_i32 => _DISCOVERY_DAY,
    23699_i32 => _DISCOVERY_DAY__OBSERVED_,
    23702_i32 => _THANKSGIVING,
    23734_i32 => _CHRISTMAS_DAY,
    23741_i32 => _NEW_YEAR_S_DAY,
    23746_i32 => _EPIPHANY,
    23755_i32 => _MARTIN_LUTHER_KING_JR__DAY,
    23790_i32 => _PRESIDENTS__DAY,
    23821_i32 => _EMANCIPATION_DAY,
    23822_i32 => _GOOD_FRIDAY,
    23888_i32 => _MEMORIAL_DAY,
    23910_i32 => _JUNETEENTH_NATIONAL_INDEPENDENCE_DAY,
    23925_i32 => _INDEPENDENCE_DAY,
    23946_i32 => _CONSTITUTION_DAY,
    23986_i32 => _LABOR_DAY,
    24055_i32 => _VETERANS_DAY,
    24056_i32 => _VETERANS_DAY__OBSERVED_,
    24063_i32 => _DISCOVERY_DAY,
    24066_i32 => _THANKSGIVING,
    24099_i32 => _CHRISTMAS_DAY,
    24106_i32 => _NEW_YEAR_S_DAY,
    24111_i32 => _EPIPHANY,
    24126_i32 => _MARTIN_LUTHER_KING_JR__DAY,
    24154_i32 => _PRESIDENTS__DAY,
    24187_i32 => _EMANCIPATION_DAY,
    24207_i32 => _GOOD_FRIDAY,
    24252_i32 => _MEMORIAL_DAY,
    24276_i32 => _JUNETEENTH_NATIONAL_INDEPENDENCE_DAY,
    24291_i32 => _INDEPENDENCE_DAY,
    24312_i32 => _CONSTITUTION_DAY,
    24350_i32 => _LABOR_DAY,
    24421_i32 => _VETERANS_DAY,
    24429_i32 => _DISCOVERY_DAY,
    24437_i32 => _THANKSGIVING,
    24465_i32 => _CHRISTMAS_DAY,
    24472_i32 => _NEW_YEAR_S_DAY,
    24477_i32 => _EPIPHANY,
    24490_i32 => _MARTIN_LUTHER_KING_JR__DAY,
    24518_i32 => _PRESIDENTS__DAY,
    24552_i32 => _EMANCIPATION_DAY,
    24553_i32 => _EMANCIPATION_DAY__OBSERVED_,
    24564_i32 => _GOOD_FRIDAY,
    24616_i32 => _MEMORIAL_DAY,
    24641_i32 => _JUNETEENTH_NATIONAL_INDEPENDENCE_DAY,
    24655_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    24656_i32 => _INDEPENDENCE_DAY,
    24677_i32 => _CONSTITUTION_DAY,
    24721_i32 => _LABOR_DAY,
    24786_i32 => _VETERANS_DAY,
    24794_i32 => _DISCOVERY_DAY,
    24801_i32 => _THANKSGIVING,
    24830_i32 => _CHRISTMAS_DAY,
    24837_i32 => _NEW_YEAR_S_DAY,
    24842_i32 => _EPIPHANY,
    24854_i32 => _MARTIN_LUTHER_KING_JR__DAY,
    24882_i32 => _PRESIDENTS__DAY,
    24917_i32 => _EMANCIPATION_DAY,
    24949_i32 => _GOOD_FRIDAY,
    24987_i32 => _MEMORIAL_DAY,
    25005_i32 => _JUNETEENTH_NATIONAL_INDEPENDENCE_DAY__OBSERVED_,
    25006_i32 => _JUNETEENTH_NATIONAL_INDEPENDENCE_DAY,
    25021_i32 => _INDEPENDENCE_DAY,
    25022_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    25042_i32 => _CONSTITUTION_DAY,
    25043_i32 => _CONSTITUTION_DAY__OBSERVED_,
    25085_i32 => _LABOR_DAY,
    25151_i32 => _VETERANS_DAY,
    25159_i32 => _DISCOVERY_DAY,
    25165_i32 => _THANKSGIVING,
    25194_i32 => _CHRISTMAS_DAY__OBSERVED_,
    25195_i32 => _CHRISTMAS_DAY,
    25201_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    25202_i32 => _NEW_YEAR_S_DAY,
    25207_i32 => _EPIPHANY,
    25218_i32 => _MARTIN_LUTHER_KING_JR__DAY,
    25253_i32 => _PRESIDENTS__DAY,
    25282_i32 => _EMANCIPATION_DAY,
    25299_i32 => _GOOD_FRIDAY,
    25351_i32 => _MEMORIAL_DAY,
    25371_i32 => _JUNETEENTH_NATIONAL_INDEPENDENCE_DAY,
    25372_i32 => _JUNETEENTH_NATIONAL_INDEPENDENCE_DAY__OBSERVED_,
    25386_i32 => _INDEPENDENCE_DAY,
    25407_i32 => _CONSTITUTION_DAY,
    25449_i32 => _LABOR_DAY,
    25516_i32 => _VETERANS_DAY,
    25524_i32 => _DISCOVERY_DAY,
    25529_i32 => _THANKSGIVING,
    25560_i32 => _CHRISTMAS_DAY,
    25561_i32 => _CHRISTMAS_DAY__OBSERVED_,
    25567_i32 => _NEW_YEAR_S_DAY,
    25568_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    25572_i32 => _EPIPHANY,
    25582_i32 => _MARTIN_LUTHER_KING_JR__DAY,
    25617_i32 => _PRESIDENTS__DAY,
    25648_i32 => _EMANCIPATION_DAY,
    25656_i32 => _GOOD_FRIDAY,
    25715_i32 => _MEMORIAL_DAY,
    25737_i32 => _JUNETEENTH_NATIONAL_INDEPENDENCE_DAY,
    25752_i32 => _INDEPENDENCE_DAY,
    25773_i32 => _CONSTITUTION_DAY,
    25813_i32 => _LABOR_DAY,
    25882_i32 => _VETERANS_DAY,
    25883_i32 => _VETERANS_DAY__OBSERVED_,
    25890_i32 => _DISCOVERY_DAY,
    25893_i32 => _THANKSGIVING,
    25926_i32 => _CHRISTMAS_DAY,
    25933_i32 => _NEW_YEAR_S_DAY,
    25938_i32 => _EPIPHANY,
    25953_i32 => _MARTIN_LUTHER_KING_JR__DAY,
    25981_i32 => _PRESIDENTS__DAY,
    26013_i32 => _EMANCIPATION_DAY,
    26041_i32 => _GOOD_FRIDAY,
    26079_i32 => _MEMORIAL_DAY,
    26102_i32 => _JUNETEENTH_NATIONAL_INDEPENDENCE_DAY,
    26117_i32 => _INDEPENDENCE_DAY,
    26138_i32 => _CONSTITUTION_DAY,
    26177_i32 => _LABOR_DAY,
    26247_i32 => _VETERANS_DAY,
    26255_i32 => _DISCOVERY_DAY,
    26264_i32 => _THANKSGIVING,
    26291_i32 => _CHRISTMAS_DAY,
    26298_i32 => _NEW_YEAR_S_DAY,
    26303_i32 => _EPIPHANY,
    26317_i32 => _MARTIN_LUTHER_KING_JR__DAY,
    26345_i32 => _PRESIDENTS__DAY,
    26378_i32 => _EMANCIPATION_DAY,
    26391_i32 => _GOOD_FRIDAY,
    26443_i32 => _MEMORIAL_DAY,
    26467_i32 => _JUNETEENTH_NATIONAL_INDEPENDENCE_DAY,
    26482_i32 => _INDEPENDENCE_DAY,
    26503_i32 => _CONSTITUTION_DAY,
    26541_i32 => _LABOR_DAY,
    26612_i32 => _VETERANS_DAY,
    26620_i32 => _DISCOVERY_DAY,
    26628_i32 => _THANKSGIVING,
    26656_i32 => _CHRISTMAS_DAY,
    26663_i32 => _NEW_YEAR_S_DAY,
    26668_i32 => _EPIPHANY,
    26681_i32 => _MARTIN_LUTHER_KING_JR__DAY,
    26709_i32 => _PRESIDENTS__DAY,
    26743_i32 => _EMANCIPATION_DAY,
    26744_i32 => _EMANCIPATION_DAY__OBSERVED_,
    26748_i32 => _GOOD_FRIDAY,
    26807_i32 => _MEMORIAL_DAY,
    26832_i32 => _JUNETEENTH_NATIONAL_INDEPENDENCE_DAY,
    26846_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    26847_i32 => _INDEPENDENCE_DAY,
    26868_i32 => _CONSTITUTION_DAY,
    26912_i32 => _LABOR_DAY,
    26977_i32 => _VETERANS_DAY,
    26985_i32 => _DISCOVERY_DAY,
    26992_i32 => _THANKSGIVING,
    27021_i32 => _CHRISTMAS_DAY,
    27028_i32 => _NEW_YEAR_S_DAY,
    27033_i32 => _EPIPHANY,
    27045_i32 => _MARTIN_LUTHER_KING_JR__DAY,
    27073_i32 => _PRESIDENTS__DAY,
    27109_i32 => _EMANCIPATION_DAY,
    27133_i32 => _GOOD_FRIDAY,
    27178_i32 => _MEMORIAL_DAY,
    27198_i32 => _JUNETEENTH_NATIONAL_INDEPENDENCE_DAY,
    27199_i32 => _JUNETEENTH_NATIONAL_INDEPENDENCE_DAY__OBSERVED_,
    27213_i32 => _INDEPENDENCE_DAY,
    27234_i32 => _CONSTITUTION_DAY,
    27276_i32 => _LABOR_DAY,
    27343_i32 => _VETERANS_DAY,
    27351_i32 => _DISCOVERY_DAY,
    27356_i32 => _THANKSGIVING,
    27387_i32 => _CHRISTMAS_DAY,
    27388_i32 => _CHRISTMAS_DAY__OBSERVED_,
    27394_i32 => _NEW_YEAR_S_DAY,
    27395_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    27399_i32 => _EPIPHANY,
    27409_i32 => _MARTIN_LUTHER_KING_JR__DAY,
    27444_i32 => _PRESIDENTS__DAY,
    27474_i32 => _EMANCIPATION_DAY,
    27490_i32 => _GOOD_FRIDAY,
    27542_i32 => _MEMORIAL_DAY,
    27563_i32 => _JUNETEENTH_NATIONAL_INDEPENDENCE_DAY,
    27578_i32 => _INDEPENDENCE_DAY,
    27599_i32 => _CONSTITUTION_DAY,
    27640_i32 => _LABOR_DAY,
    27707_i32 => _VETERANS_DAY__OBSERVED_,
    27708_i32 => _VETERANS_DAY,
    27716_i32 => _DISCOVERY_DAY,
    27717_i32 => _DISCOVERY_DAY__OBSERVED_,
    27720_i32 => _THANKSGIVING,
    27752_i32 => _CHRISTMAS_DAY,
    27759_i32 => _NEW_YEAR_S_DAY,
    27764_i32 => _EPIPHANY,
    27773_i32 => _MARTIN_LUTHER_KING_JR__DAY,
    27808_i32 => _PRESIDENTS__DAY,
    27839_i32 => _EMANCIPATION_DAY,
    27840_i32 => _GOOD_FRIDAY,
    27906_i32 => _MEMORIAL_DAY,
    27928_i32 => _JUNETEENTH_NATIONAL_INDEPENDENCE_DAY,
    27943_i32 => _INDEPENDENCE_DAY,
    27964_i32 => _CONSTITUTION_DAY,
    28004_i32 => _LABOR_DAY,
    28073_i32 => _VETERANS_DAY,
    28074_i32 => _VETERANS_DAY__OBSERVED_,
    28081_i32 => _DISCOVERY_DAY,
    28084_i32 => _THANKSGIVING,
    28117_i32 => _CHRISTMAS_DAY,
    28124_i32 => _NEW_YEAR_S_DAY,
    28129_i32 => _EPIPHANY,
    28144_i32 => _MARTIN_LUTHER_KING_JR__DAY,
    28172_i32 => _PRESIDENTS__DAY,
    28204_i32 => _EMANCIPATION_DAY,
    28225_i32 => _GOOD_FRIDAY,
    28270_i32 => _MEMORIAL_DAY,
    28293_i32 => _JUNETEENTH_NATIONAL_INDEPENDENCE_DAY,
    28308_i32 => _INDEPENDENCE_DAY,
    28329_i32 => _CONSTITUTION_DAY,
    28368_i32 => _LABOR_DAY,
    28438_i32 => _VETERANS_DAY,
    28446_i32 => _DISCOVERY_DAY,
    28455_i32 => _THANKSGIVING,
    28482_i32 => _CHRISTMAS_DAY,
    28489_i32 => _NEW_YEAR_S_DAY,
    28494_i32 => _EPIPHANY,
    28508_i32 => _MARTIN_LUTHER_KING_JR__DAY,
    28536_i32 => _PRESIDENTS__DAY,
    28570_i32 => _EMANCIPATION_DAY,
    28571_i32 => _EMANCIPATION_DAY__OBSERVED_,
    28582_i32 => _GOOD_FRIDAY,
    28634_i32 => _MEMORIAL_DAY,
    28659_i32 => _JUNETEENTH_NATIONAL_INDEPENDENCE_DAY,
    28673_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    28674_i32 => _INDEPENDENCE_DAY,
    28695_i32 => _CONSTITUTION_DAY,
    28739_i32 => _LABOR_DAY,
    28804_i32 => _VETERANS_DAY,
    28812_i32 => _DISCOVERY_DAY,
    28819_i32 => _THANKSGIVING,
    28848_i32 => _CHRISTMAS_DAY,
    28855_i32 => _NEW_YEAR_S_DAY,
    28860_i32 => _EPIPHANY,
    28872_i32 => _MARTIN_LUTHER_KING_JR__DAY,
    28900_i32 => _PRESIDENTS__DAY,
    28935_i32 => _EMANCIPATION_DAY,
    28960_i32 => _GOOD_FRIDAY,
    29005_i32 => _MEMORIAL_DAY,
    29023_i32 => _JUNETEENTH_NATIONAL_INDEPENDENCE_DAY__OBSERVED_,
    29024_i32 => _JUNETEENTH_NATIONAL_INDEPENDENCE_DAY,
    29039_i32 => _INDEPENDENCE_DAY,
    29040_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    29060_i32 => _CONSTITUTION_DAY,
    29061_i32 => _CONSTITUTION_DAY__OBSERVED_,
    29103_i32 => _LABOR_DAY,
    29169_i32 => _VETERANS_DAY,
    29177_i32 => _DISCOVERY_DAY,
    29183_i32 => _THANKSGIVING,
    29212_i32 => _CHRISTMAS_DAY__OBSERVED_,
    29213_i32 => _CHRISTMAS_DAY,
    29219_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    29220_i32 => _NEW_YEAR_S_DAY,
};
