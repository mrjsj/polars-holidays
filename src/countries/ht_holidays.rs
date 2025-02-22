use crate::countries::constants::*;
use phf::phf_map;

pub static HT_HOLIDAYS: phf::Map<i32, &'static str> = phf_map! {
    10957_i32 => _NATIONAL_INDEPENDENCE_DAY__NEW_YEAR_S_DAY,
    10958_i32 => _ANCESTRY_DAY,
    11021_i32 => _CARNIVAL,
    11022_i32 => _SHROVE_MONDAY,
    11023_i32 => _FAT_TUESDAY,
    11068_i32 => _GOOD_FRIDAY,
    11070_i32 => _EASTER,
    11078_i32 => _AGRICULTURE_AND_LABOR_DAY,
    11095_i32 => _FLAG_DAY_AND_UNIVERSITY_DAY,
    11130_i32 => _CORPUS_CHRISTI,
    11184_i32 => _ASSUMPTION_OF_MARY,
    11247_i32 => _DEATH_OF_DESSALINES,
    11262_i32 => _ALL_SAINTS__DAY,
    11263_i32 => _DAY_OF_THE_DEAD,
    11279_i32 => _ARMED_FORCES_DAY__COMMEMORATION_OF_THE_BATTLE_OF_VERTIERES,
    11316_i32 => _CHRISTMAS_DAY,
    11323_i32 => _NATIONAL_INDEPENDENCE_DAY__NEW_YEAR_S_DAY,
    11324_i32 => _ANCESTRY_DAY,
    11378_i32 => _CARNIVAL,
    11379_i32 => _SHROVE_MONDAY,
    11380_i32 => _FAT_TUESDAY,
    11425_i32 => _GOOD_FRIDAY,
    11427_i32 => _EASTER,
    11443_i32 => _AGRICULTURE_AND_LABOR_DAY,
    11460_i32 => _FLAG_DAY_AND_UNIVERSITY_DAY,
    11487_i32 => _CORPUS_CHRISTI,
    11549_i32 => _ASSUMPTION_OF_MARY,
    11612_i32 => _DEATH_OF_DESSALINES,
    11627_i32 => _ALL_SAINTS__DAY,
    11628_i32 => _DAY_OF_THE_DEAD,
    11644_i32 => _ARMED_FORCES_DAY__COMMEMORATION_OF_THE_BATTLE_OF_VERTIERES,
    11681_i32 => _CHRISTMAS_DAY,
    11688_i32 => _NATIONAL_INDEPENDENCE_DAY__NEW_YEAR_S_DAY,
    11689_i32 => _ANCESTRY_DAY,
    11728_i32 => _CARNIVAL,
    11729_i32 => _SHROVE_MONDAY,
    11730_i32 => _FAT_TUESDAY,
    11775_i32 => _GOOD_FRIDAY,
    11777_i32 => _EASTER,
    11808_i32 => _AGRICULTURE_AND_LABOR_DAY,
    11825_i32 => _FLAG_DAY_AND_UNIVERSITY_DAY,
    11837_i32 => _CORPUS_CHRISTI,
    11914_i32 => _ASSUMPTION_OF_MARY,
    11977_i32 => _DEATH_OF_DESSALINES,
    11992_i32 => _ALL_SAINTS__DAY,
    11993_i32 => _DAY_OF_THE_DEAD,
    12009_i32 => _ARMED_FORCES_DAY__COMMEMORATION_OF_THE_BATTLE_OF_VERTIERES,
    12046_i32 => _CHRISTMAS_DAY,
    12053_i32 => _NATIONAL_INDEPENDENCE_DAY__NEW_YEAR_S_DAY,
    12054_i32 => _ANCESTRY_DAY,
    12113_i32 => _CARNIVAL,
    12114_i32 => _SHROVE_MONDAY,
    12115_i32 => _FAT_TUESDAY,
    12160_i32 => _GOOD_FRIDAY,
    12162_i32 => _EASTER,
    12173_i32 => _AGRICULTURE_AND_LABOR_DAY,
    12190_i32 => _FLAG_DAY_AND_UNIVERSITY_DAY,
    12222_i32 => _CORPUS_CHRISTI,
    12279_i32 => _ASSUMPTION_OF_MARY,
    12342_i32 => _DEATH_OF_DESSALINES,
    12357_i32 => _ALL_SAINTS__DAY,
    12358_i32 => _DAY_OF_THE_DEAD,
    12374_i32 => _ARMED_FORCES_DAY__COMMEMORATION_OF_THE_BATTLE_OF_VERTIERES,
    12411_i32 => _CHRISTMAS_DAY,
    12418_i32 => _NATIONAL_INDEPENDENCE_DAY__NEW_YEAR_S_DAY,
    12419_i32 => _ANCESTRY_DAY,
    12470_i32 => _CARNIVAL,
    12471_i32 => _SHROVE_MONDAY,
    12472_i32 => _FAT_TUESDAY,
    12517_i32 => _GOOD_FRIDAY,
    12519_i32 => _EASTER,
    12539_i32 => _AGRICULTURE_AND_LABOR_DAY,
    12556_i32 => _FLAG_DAY_AND_UNIVERSITY_DAY,
    12579_i32 => _CORPUS_CHRISTI,
    12645_i32 => _ASSUMPTION_OF_MARY,
    12708_i32 => _DEATH_OF_DESSALINES,
    12723_i32 => _ALL_SAINTS__DAY,
    12724_i32 => _DAY_OF_THE_DEAD,
    12740_i32 => _ARMED_FORCES_DAY__COMMEMORATION_OF_THE_BATTLE_OF_VERTIERES,
    12777_i32 => _CHRISTMAS_DAY,
    12784_i32 => _NATIONAL_INDEPENDENCE_DAY__NEW_YEAR_S_DAY,
    12785_i32 => _ANCESTRY_DAY,
    12820_i32 => _CARNIVAL,
    12821_i32 => _SHROVE_MONDAY,
    12822_i32 => _FAT_TUESDAY,
    12867_i32 => _GOOD_FRIDAY,
    12869_i32 => _EASTER,
    12904_i32 => _AGRICULTURE_AND_LABOR_DAY,
    12921_i32 => _FLAG_DAY_AND_UNIVERSITY_DAY,
    12929_i32 => _CORPUS_CHRISTI,
    13010_i32 => _ASSUMPTION_OF_MARY,
    13073_i32 => _DEATH_OF_DESSALINES,
    13088_i32 => _ALL_SAINTS__DAY,
    13089_i32 => _DAY_OF_THE_DEAD,
    13105_i32 => _ARMED_FORCES_DAY__COMMEMORATION_OF_THE_BATTLE_OF_VERTIERES,
    13142_i32 => _CHRISTMAS_DAY,
    13149_i32 => _NATIONAL_INDEPENDENCE_DAY__NEW_YEAR_S_DAY,
    13150_i32 => _ANCESTRY_DAY,
    13205_i32 => _CARNIVAL,
    13206_i32 => _SHROVE_MONDAY,
    13207_i32 => _FAT_TUESDAY,
    13252_i32 => _GOOD_FRIDAY,
    13254_i32 => _EASTER,
    13269_i32 => _AGRICULTURE_AND_LABOR_DAY,
    13286_i32 => _FLAG_DAY_AND_UNIVERSITY_DAY,
    13314_i32 => _CORPUS_CHRISTI,
    13375_i32 => _ASSUMPTION_OF_MARY,
    13438_i32 => _DEATH_OF_DESSALINES,
    13453_i32 => _ALL_SAINTS__DAY,
    13454_i32 => _DAY_OF_THE_DEAD,
    13470_i32 => _ARMED_FORCES_DAY__COMMEMORATION_OF_THE_BATTLE_OF_VERTIERES,
    13507_i32 => _CHRISTMAS_DAY,
    13514_i32 => _NATIONAL_INDEPENDENCE_DAY__NEW_YEAR_S_DAY,
    13515_i32 => _ANCESTRY_DAY,
    13562_i32 => _CARNIVAL,
    13563_i32 => _SHROVE_MONDAY,
    13564_i32 => _FAT_TUESDAY,
    13609_i32 => _GOOD_FRIDAY,
    13611_i32 => _EASTER,
    13634_i32 => _AGRICULTURE_AND_LABOR_DAY,
    13651_i32 => _FLAG_DAY_AND_UNIVERSITY_DAY,
    13671_i32 => _CORPUS_CHRISTI,
    13740_i32 => _ASSUMPTION_OF_MARY,
    13803_i32 => _DEATH_OF_DESSALINES,
    13818_i32 => _ALL_SAINTS__DAY,
    13819_i32 => _DAY_OF_THE_DEAD,
    13835_i32 => _ARMED_FORCES_DAY__COMMEMORATION_OF_THE_BATTLE_OF_VERTIERES,
    13872_i32 => _CHRISTMAS_DAY,
    13879_i32 => _NATIONAL_INDEPENDENCE_DAY__NEW_YEAR_S_DAY,
    13880_i32 => _ANCESTRY_DAY,
    13912_i32 => _CARNIVAL,
    13913_i32 => _SHROVE_MONDAY,
    13914_i32 => _FAT_TUESDAY,
    13959_i32 => _GOOD_FRIDAY,
    13961_i32 => _EASTER,
    14000_i32 => _AGRICULTURE_AND_LABOR_DAY,
    14017_i32 => _FLAG_DAY_AND_UNIVERSITY_DAY,
    14021_i32 => _CORPUS_CHRISTI,
    14106_i32 => _ASSUMPTION_OF_MARY,
    14169_i32 => _DEATH_OF_DESSALINES,
    14184_i32 => _ALL_SAINTS__DAY,
    14185_i32 => _DAY_OF_THE_DEAD,
    14201_i32 => _ARMED_FORCES_DAY__COMMEMORATION_OF_THE_BATTLE_OF_VERTIERES,
    14238_i32 => _CHRISTMAS_DAY,
    14245_i32 => _NATIONAL_INDEPENDENCE_DAY__NEW_YEAR_S_DAY,
    14246_i32 => _ANCESTRY_DAY,
    14297_i32 => _CARNIVAL,
    14298_i32 => _SHROVE_MONDAY,
    14299_i32 => _FAT_TUESDAY,
    14344_i32 => _GOOD_FRIDAY,
    14346_i32 => _EASTER,
    14365_i32 => _AGRICULTURE_AND_LABOR_DAY,
    14382_i32 => _FLAG_DAY_AND_UNIVERSITY_DAY,
    14406_i32 => _CORPUS_CHRISTI,
    14471_i32 => _ASSUMPTION_OF_MARY,
    14534_i32 => _DEATH_OF_DESSALINES,
    14549_i32 => _ALL_SAINTS__DAY,
    14550_i32 => _DAY_OF_THE_DEAD,
    14566_i32 => _ARMED_FORCES_DAY__COMMEMORATION_OF_THE_BATTLE_OF_VERTIERES,
    14603_i32 => _CHRISTMAS_DAY,
    14610_i32 => _NATIONAL_INDEPENDENCE_DAY__NEW_YEAR_S_DAY,
    14611_i32 => _ANCESTRY_DAY,
    14654_i32 => _CARNIVAL,
    14655_i32 => _SHROVE_MONDAY,
    14656_i32 => _FAT_TUESDAY,
    14701_i32 => _GOOD_FRIDAY,
    14703_i32 => _EASTER,
    14730_i32 => _AGRICULTURE_AND_LABOR_DAY,
    14747_i32 => _FLAG_DAY_AND_UNIVERSITY_DAY,
    14763_i32 => _CORPUS_CHRISTI,
    14836_i32 => _ASSUMPTION_OF_MARY,
    14899_i32 => _DEATH_OF_DESSALINES,
    14914_i32 => _ALL_SAINTS__DAY,
    14915_i32 => _DAY_OF_THE_DEAD,
    14931_i32 => _ARMED_FORCES_DAY__COMMEMORATION_OF_THE_BATTLE_OF_VERTIERES,
    14968_i32 => _CHRISTMAS_DAY,
    14975_i32 => _NATIONAL_INDEPENDENCE_DAY__NEW_YEAR_S_DAY,
    14976_i32 => _ANCESTRY_DAY,
    15039_i32 => _CARNIVAL,
    15040_i32 => _SHROVE_MONDAY,
    15041_i32 => _FAT_TUESDAY,
    15086_i32 => _GOOD_FRIDAY,
    15088_i32 => _EASTER,
    15095_i32 => _AGRICULTURE_AND_LABOR_DAY,
    15112_i32 => _FLAG_DAY_AND_UNIVERSITY_DAY,
    15148_i32 => _CORPUS_CHRISTI,
    15201_i32 => _ASSUMPTION_OF_MARY,
    15264_i32 => _DEATH_OF_DESSALINES,
    15279_i32 => _ALL_SAINTS__DAY,
    15280_i32 => _DAY_OF_THE_DEAD,
    15296_i32 => _ARMED_FORCES_DAY__COMMEMORATION_OF_THE_BATTLE_OF_VERTIERES,
    15333_i32 => _CHRISTMAS_DAY,
    15340_i32 => _NATIONAL_INDEPENDENCE_DAY__NEW_YEAR_S_DAY,
    15341_i32 => _ANCESTRY_DAY,
    15389_i32 => _CARNIVAL,
    15390_i32 => _SHROVE_MONDAY,
    15391_i32 => _FAT_TUESDAY,
    15436_i32 => _GOOD_FRIDAY,
    15438_i32 => _EASTER,
    15461_i32 => _AGRICULTURE_AND_LABOR_DAY,
    15478_i32 => _FLAG_DAY_AND_UNIVERSITY_DAY,
    15498_i32 => _CORPUS_CHRISTI,
    15567_i32 => _ASSUMPTION_OF_MARY,
    15630_i32 => _DEATH_OF_DESSALINES,
    15645_i32 => _ALL_SAINTS__DAY,
    15646_i32 => _DAY_OF_THE_DEAD,
    15662_i32 => _ARMED_FORCES_DAY__COMMEMORATION_OF_THE_BATTLE_OF_VERTIERES,
    15699_i32 => _CHRISTMAS_DAY,
    15706_i32 => _NATIONAL_INDEPENDENCE_DAY__NEW_YEAR_S_DAY,
    15707_i32 => _ANCESTRY_DAY,
    15746_i32 => _CARNIVAL,
    15747_i32 => _SHROVE_MONDAY,
    15748_i32 => _FAT_TUESDAY,
    15793_i32 => _GOOD_FRIDAY,
    15795_i32 => _EASTER,
    15826_i32 => _AGRICULTURE_AND_LABOR_DAY,
    15843_i32 => _FLAG_DAY_AND_UNIVERSITY_DAY,
    15855_i32 => _CORPUS_CHRISTI,
    15932_i32 => _ASSUMPTION_OF_MARY,
    15995_i32 => _DEATH_OF_DESSALINES,
    16010_i32 => _ALL_SAINTS__DAY,
    16011_i32 => _DAY_OF_THE_DEAD,
    16027_i32 => _ARMED_FORCES_DAY__COMMEMORATION_OF_THE_BATTLE_OF_VERTIERES,
    16064_i32 => _CHRISTMAS_DAY,
    16071_i32 => _NATIONAL_INDEPENDENCE_DAY__NEW_YEAR_S_DAY,
    16072_i32 => _ANCESTRY_DAY,
    16131_i32 => _CARNIVAL,
    16132_i32 => _SHROVE_MONDAY,
    16133_i32 => _FAT_TUESDAY,
    16178_i32 => _GOOD_FRIDAY,
    16180_i32 => _EASTER,
    16191_i32 => _AGRICULTURE_AND_LABOR_DAY,
    16208_i32 => _FLAG_DAY_AND_UNIVERSITY_DAY,
    16240_i32 => _CORPUS_CHRISTI,
    16297_i32 => _ASSUMPTION_OF_MARY,
    16360_i32 => _DEATH_OF_DESSALINES,
    16375_i32 => _ALL_SAINTS__DAY,
    16376_i32 => _DAY_OF_THE_DEAD,
    16392_i32 => _ARMED_FORCES_DAY__COMMEMORATION_OF_THE_BATTLE_OF_VERTIERES,
    16429_i32 => _CHRISTMAS_DAY,
    16436_i32 => _NATIONAL_INDEPENDENCE_DAY__NEW_YEAR_S_DAY,
    16437_i32 => _ANCESTRY_DAY,
    16481_i32 => _CARNIVAL,
    16482_i32 => _SHROVE_MONDAY,
    16483_i32 => _FAT_TUESDAY,
    16528_i32 => _GOOD_FRIDAY,
    16530_i32 => _EASTER,
    16556_i32 => _AGRICULTURE_AND_LABOR_DAY,
    16573_i32 => _FLAG_DAY_AND_UNIVERSITY_DAY,
    16590_i32 => _CORPUS_CHRISTI,
    16662_i32 => _ASSUMPTION_OF_MARY,
    16725_i32 => _DEATH_OF_DESSALINES,
    16740_i32 => _ALL_SAINTS__DAY,
    16741_i32 => _DAY_OF_THE_DEAD,
    16757_i32 => _ARMED_FORCES_DAY__COMMEMORATION_OF_THE_BATTLE_OF_VERTIERES,
    16794_i32 => _CHRISTMAS_DAY,
    16801_i32 => _NATIONAL_INDEPENDENCE_DAY__NEW_YEAR_S_DAY,
    16802_i32 => _ANCESTRY_DAY,
    16838_i32 => _CARNIVAL,
    16839_i32 => _SHROVE_MONDAY,
    16840_i32 => _FAT_TUESDAY,
    16885_i32 => _GOOD_FRIDAY,
    16887_i32 => _EASTER,
    16922_i32 => _AGRICULTURE_AND_LABOR_DAY,
    16939_i32 => _FLAG_DAY_AND_UNIVERSITY_DAY,
    16947_i32 => _CORPUS_CHRISTI,
    17028_i32 => _ASSUMPTION_OF_MARY,
    17091_i32 => _DEATH_OF_DESSALINES,
    17106_i32 => _ALL_SAINTS__DAY,
    17107_i32 => _DAY_OF_THE_DEAD,
    17123_i32 => _ARMED_FORCES_DAY__COMMEMORATION_OF_THE_BATTLE_OF_VERTIERES,
    17160_i32 => _CHRISTMAS_DAY,
    17167_i32 => _NATIONAL_INDEPENDENCE_DAY__NEW_YEAR_S_DAY,
    17168_i32 => _ANCESTRY_DAY,
    17223_i32 => _CARNIVAL,
    17224_i32 => _SHROVE_MONDAY,
    17225_i32 => _FAT_TUESDAY,
    17270_i32 => _GOOD_FRIDAY,
    17272_i32 => _EASTER,
    17287_i32 => _AGRICULTURE_AND_LABOR_DAY,
    17304_i32 => _FLAG_DAY_AND_UNIVERSITY_DAY,
    17332_i32 => _CORPUS_CHRISTI,
    17393_i32 => _ASSUMPTION_OF_MARY,
    17456_i32 => _DEATH_OF_DESSALINES,
    17471_i32 => _ALL_SAINTS__DAY,
    17472_i32 => _DAY_OF_THE_DEAD,
    17488_i32 => _ARMED_FORCES_DAY__COMMEMORATION_OF_THE_BATTLE_OF_VERTIERES,
    17525_i32 => _CHRISTMAS_DAY,
    17532_i32 => _NATIONAL_INDEPENDENCE_DAY__NEW_YEAR_S_DAY,
    17533_i32 => _ANCESTRY_DAY,
    17573_i32 => _CARNIVAL,
    17574_i32 => _SHROVE_MONDAY,
    17575_i32 => _FAT_TUESDAY,
    17620_i32 => _GOOD_FRIDAY,
    17622_i32 => _EASTER,
    17652_i32 => _AGRICULTURE_AND_LABOR_DAY,
    17669_i32 => _FLAG_DAY_AND_UNIVERSITY_DAY,
    17682_i32 => _CORPUS_CHRISTI,
    17758_i32 => _ASSUMPTION_OF_MARY,
    17821_i32 => _DEATH_OF_DESSALINES,
    17836_i32 => _ALL_SAINTS__DAY,
    17837_i32 => _DAY_OF_THE_DEAD,
    17853_i32 => _ARMED_FORCES_DAY__COMMEMORATION_OF_THE_BATTLE_OF_VERTIERES,
    17890_i32 => _CHRISTMAS_DAY,
    17897_i32 => _NATIONAL_INDEPENDENCE_DAY__NEW_YEAR_S_DAY,
    17898_i32 => _ANCESTRY_DAY,
    17958_i32 => _CARNIVAL,
    17959_i32 => _SHROVE_MONDAY,
    17960_i32 => _FAT_TUESDAY,
    18005_i32 => _GOOD_FRIDAY,
    18007_i32 => _EASTER,
    18017_i32 => _AGRICULTURE_AND_LABOR_DAY,
    18034_i32 => _FLAG_DAY_AND_UNIVERSITY_DAY,
    18067_i32 => _CORPUS_CHRISTI,
    18123_i32 => _ASSUMPTION_OF_MARY,
    18186_i32 => _DEATH_OF_DESSALINES,
    18201_i32 => _ALL_SAINTS__DAY,
    18202_i32 => _DAY_OF_THE_DEAD,
    18218_i32 => _ARMED_FORCES_DAY__COMMEMORATION_OF_THE_BATTLE_OF_VERTIERES,
    18255_i32 => _CHRISTMAS_DAY,
    18262_i32 => _NATIONAL_INDEPENDENCE_DAY__NEW_YEAR_S_DAY,
    18263_i32 => _ANCESTRY_DAY,
    18315_i32 => _CARNIVAL,
    18316_i32 => _SHROVE_MONDAY,
    18317_i32 => _FAT_TUESDAY,
    18362_i32 => _GOOD_FRIDAY,
    18364_i32 => _EASTER,
    18383_i32 => _AGRICULTURE_AND_LABOR_DAY,
    18400_i32 => _FLAG_DAY_AND_UNIVERSITY_DAY,
    18424_i32 => _CORPUS_CHRISTI,
    18489_i32 => _ASSUMPTION_OF_MARY,
    18552_i32 => _DEATH_OF_DESSALINES,
    18567_i32 => _ALL_SAINTS__DAY,
    18568_i32 => _DAY_OF_THE_DEAD,
    18584_i32 => _ARMED_FORCES_DAY__COMMEMORATION_OF_THE_BATTLE_OF_VERTIERES,
    18621_i32 => _CHRISTMAS_DAY,
    18628_i32 => _NATIONAL_INDEPENDENCE_DAY__NEW_YEAR_S_DAY,
    18629_i32 => _ANCESTRY_DAY,
    18672_i32 => _CARNIVAL,
    18673_i32 => _SHROVE_MONDAY,
    18674_i32 => _FAT_TUESDAY,
    18719_i32 => _GOOD_FRIDAY,
    18721_i32 => _EASTER,
    18748_i32 => _AGRICULTURE_AND_LABOR_DAY,
    18765_i32 => _FLAG_DAY_AND_UNIVERSITY_DAY,
    18781_i32 => _CORPUS_CHRISTI,
    18854_i32 => _ASSUMPTION_OF_MARY,
    18917_i32 => _DEATH_OF_DESSALINES,
    18932_i32 => _ALL_SAINTS__DAY,
    18933_i32 => _DAY_OF_THE_DEAD,
    18949_i32 => _ARMED_FORCES_DAY__COMMEMORATION_OF_THE_BATTLE_OF_VERTIERES,
    18986_i32 => _CHRISTMAS_DAY,
    18993_i32 => _NATIONAL_INDEPENDENCE_DAY__NEW_YEAR_S_DAY,
    18994_i32 => _ANCESTRY_DAY,
    19050_i32 => _CARNIVAL,
    19051_i32 => _SHROVE_MONDAY,
    19052_i32 => _FAT_TUESDAY,
    19097_i32 => _GOOD_FRIDAY,
    19099_i32 => _EASTER,
    19113_i32 => _AGRICULTURE_AND_LABOR_DAY,
    19130_i32 => _FLAG_DAY_AND_UNIVERSITY_DAY,
    19159_i32 => _CORPUS_CHRISTI,
    19219_i32 => _ASSUMPTION_OF_MARY,
    19282_i32 => _DEATH_OF_DESSALINES,
    19297_i32 => _ALL_SAINTS__DAY,
    19298_i32 => _DAY_OF_THE_DEAD,
    19314_i32 => _ARMED_FORCES_DAY__COMMEMORATION_OF_THE_BATTLE_OF_VERTIERES,
    19351_i32 => _CHRISTMAS_DAY,
    19358_i32 => _NATIONAL_INDEPENDENCE_DAY__NEW_YEAR_S_DAY,
    19359_i32 => _ANCESTRY_DAY,
    19407_i32 => _CARNIVAL,
    19408_i32 => _SHROVE_MONDAY,
    19409_i32 => _FAT_TUESDAY,
    19454_i32 => _GOOD_FRIDAY,
    19456_i32 => _EASTER,
    19478_i32 => _AGRICULTURE_AND_LABOR_DAY,
    19495_i32 => _FLAG_DAY_AND_UNIVERSITY_DAY,
    19516_i32 => _CORPUS_CHRISTI,
    19584_i32 => _ASSUMPTION_OF_MARY,
    19647_i32 => _DEATH_OF_DESSALINES,
    19662_i32 => _ALL_SAINTS__DAY,
    19663_i32 => _DAY_OF_THE_DEAD,
    19679_i32 => _ARMED_FORCES_DAY__COMMEMORATION_OF_THE_BATTLE_OF_VERTIERES,
    19716_i32 => _CHRISTMAS_DAY,
    19723_i32 => _NATIONAL_INDEPENDENCE_DAY__NEW_YEAR_S_DAY,
    19724_i32 => _ANCESTRY_DAY,
    19764_i32 => _CARNIVAL,
    19765_i32 => _SHROVE_MONDAY,
    19766_i32 => _FAT_TUESDAY,
    19811_i32 => _GOOD_FRIDAY,
    19813_i32 => _EASTER,
    19844_i32 => _AGRICULTURE_AND_LABOR_DAY,
    19861_i32 => _FLAG_DAY_AND_UNIVERSITY_DAY,
    19873_i32 => _CORPUS_CHRISTI,
    19950_i32 => _ASSUMPTION_OF_MARY,
    20013_i32 => _DEATH_OF_DESSALINES,
    20028_i32 => _ALL_SAINTS__DAY,
    20029_i32 => _DAY_OF_THE_DEAD,
    20045_i32 => _ARMED_FORCES_DAY__COMMEMORATION_OF_THE_BATTLE_OF_VERTIERES,
    20082_i32 => _CHRISTMAS_DAY,
    20089_i32 => _NATIONAL_INDEPENDENCE_DAY__NEW_YEAR_S_DAY,
    20090_i32 => _ANCESTRY_DAY,
    20149_i32 => _CARNIVAL,
    20150_i32 => _SHROVE_MONDAY,
    20151_i32 => _FAT_TUESDAY,
    20196_i32 => _GOOD_FRIDAY,
    20198_i32 => _EASTER,
    20209_i32 => _AGRICULTURE_AND_LABOR_DAY,
    20226_i32 => _FLAG_DAY_AND_UNIVERSITY_DAY,
    20258_i32 => _CORPUS_CHRISTI,
    20315_i32 => _ASSUMPTION_OF_MARY,
    20378_i32 => _DEATH_OF_DESSALINES,
    20393_i32 => _ALL_SAINTS__DAY,
    20394_i32 => _DAY_OF_THE_DEAD,
    20410_i32 => _ARMED_FORCES_DAY__COMMEMORATION_OF_THE_BATTLE_OF_VERTIERES,
    20447_i32 => _CHRISTMAS_DAY,
    20454_i32 => _NATIONAL_INDEPENDENCE_DAY__NEW_YEAR_S_DAY,
    20455_i32 => _ANCESTRY_DAY,
    20499_i32 => _CARNIVAL,
    20500_i32 => _SHROVE_MONDAY,
    20501_i32 => _FAT_TUESDAY,
    20546_i32 => _GOOD_FRIDAY,
    20548_i32 => _EASTER,
    20574_i32 => _AGRICULTURE_AND_LABOR_DAY,
    20591_i32 => _FLAG_DAY_AND_UNIVERSITY_DAY,
    20608_i32 => _CORPUS_CHRISTI,
    20680_i32 => _ASSUMPTION_OF_MARY,
    20743_i32 => _DEATH_OF_DESSALINES,
    20758_i32 => _ALL_SAINTS__DAY,
    20759_i32 => _DAY_OF_THE_DEAD,
    20775_i32 => _ARMED_FORCES_DAY__COMMEMORATION_OF_THE_BATTLE_OF_VERTIERES,
    20812_i32 => _CHRISTMAS_DAY,
    20819_i32 => _NATIONAL_INDEPENDENCE_DAY__NEW_YEAR_S_DAY,
    20820_i32 => _ANCESTRY_DAY,
    20856_i32 => _CARNIVAL,
    20857_i32 => _SHROVE_MONDAY,
    20858_i32 => _FAT_TUESDAY,
    20903_i32 => _GOOD_FRIDAY,
    20905_i32 => _EASTER,
    20939_i32 => _AGRICULTURE_AND_LABOR_DAY,
    20956_i32 => _FLAG_DAY_AND_UNIVERSITY_DAY,
    20965_i32 => _CORPUS_CHRISTI,
    21045_i32 => _ASSUMPTION_OF_MARY,
    21108_i32 => _DEATH_OF_DESSALINES,
    21123_i32 => _ALL_SAINTS__DAY,
    21124_i32 => _DAY_OF_THE_DEAD,
    21140_i32 => _ARMED_FORCES_DAY__COMMEMORATION_OF_THE_BATTLE_OF_VERTIERES,
    21177_i32 => _CHRISTMAS_DAY,
    21184_i32 => _NATIONAL_INDEPENDENCE_DAY__NEW_YEAR_S_DAY,
    21185_i32 => _ANCESTRY_DAY,
    21241_i32 => _CARNIVAL,
    21242_i32 => _SHROVE_MONDAY,
    21243_i32 => _FAT_TUESDAY,
    21288_i32 => _GOOD_FRIDAY,
    21290_i32 => _EASTER,
    21305_i32 => _AGRICULTURE_AND_LABOR_DAY,
    21322_i32 => _FLAG_DAY_AND_UNIVERSITY_DAY,
    21350_i32 => _CORPUS_CHRISTI,
    21411_i32 => _ASSUMPTION_OF_MARY,
    21474_i32 => _DEATH_OF_DESSALINES,
    21489_i32 => _ALL_SAINTS__DAY,
    21490_i32 => _DAY_OF_THE_DEAD,
    21506_i32 => _ARMED_FORCES_DAY__COMMEMORATION_OF_THE_BATTLE_OF_VERTIERES,
    21543_i32 => _CHRISTMAS_DAY,
    21550_i32 => _NATIONAL_INDEPENDENCE_DAY__NEW_YEAR_S_DAY,
    21551_i32 => _ANCESTRY_DAY,
    21591_i32 => _CARNIVAL,
    21592_i32 => _SHROVE_MONDAY,
    21593_i32 => _FAT_TUESDAY,
    21638_i32 => _GOOD_FRIDAY,
    21640_i32 => _EASTER,
    21670_i32 => _AGRICULTURE_AND_LABOR_DAY,
    21687_i32 => _FLAG_DAY_AND_UNIVERSITY_DAY,
    21700_i32 => _CORPUS_CHRISTI,
    21776_i32 => _ASSUMPTION_OF_MARY,
    21839_i32 => _DEATH_OF_DESSALINES,
    21854_i32 => _ALL_SAINTS__DAY,
    21855_i32 => _DAY_OF_THE_DEAD,
    21871_i32 => _ARMED_FORCES_DAY__COMMEMORATION_OF_THE_BATTLE_OF_VERTIERES,
    21908_i32 => _CHRISTMAS_DAY,
    21915_i32 => _NATIONAL_INDEPENDENCE_DAY__NEW_YEAR_S_DAY,
    21916_i32 => _ANCESTRY_DAY,
    21976_i32 => _CARNIVAL,
    21977_i32 => _SHROVE_MONDAY,
    21978_i32 => _FAT_TUESDAY,
    22023_i32 => _GOOD_FRIDAY,
    22025_i32 => _EASTER,
    22035_i32 => _AGRICULTURE_AND_LABOR_DAY,
    22052_i32 => _FLAG_DAY_AND_UNIVERSITY_DAY,
    22085_i32 => _CORPUS_CHRISTI,
    22141_i32 => _ASSUMPTION_OF_MARY,
    22204_i32 => _DEATH_OF_DESSALINES,
    22219_i32 => _ALL_SAINTS__DAY,
    22220_i32 => _DAY_OF_THE_DEAD,
    22236_i32 => _ARMED_FORCES_DAY__COMMEMORATION_OF_THE_BATTLE_OF_VERTIERES,
    22273_i32 => _CHRISTMAS_DAY,
    22280_i32 => _NATIONAL_INDEPENDENCE_DAY__NEW_YEAR_S_DAY,
    22281_i32 => _ANCESTRY_DAY,
    22333_i32 => _CARNIVAL,
    22334_i32 => _SHROVE_MONDAY,
    22335_i32 => _FAT_TUESDAY,
    22380_i32 => _GOOD_FRIDAY,
    22382_i32 => _EASTER,
    22400_i32 => _AGRICULTURE_AND_LABOR_DAY,
    22417_i32 => _FLAG_DAY_AND_UNIVERSITY_DAY,
    22442_i32 => _CORPUS_CHRISTI,
    22506_i32 => _ASSUMPTION_OF_MARY,
    22569_i32 => _DEATH_OF_DESSALINES,
    22584_i32 => _ALL_SAINTS__DAY,
    22585_i32 => _DAY_OF_THE_DEAD,
    22601_i32 => _ARMED_FORCES_DAY__COMMEMORATION_OF_THE_BATTLE_OF_VERTIERES,
    22638_i32 => _CHRISTMAS_DAY,
    22645_i32 => _NATIONAL_INDEPENDENCE_DAY__NEW_YEAR_S_DAY,
    22646_i32 => _ANCESTRY_DAY,
    22683_i32 => _CARNIVAL,
    22684_i32 => _SHROVE_MONDAY,
    22685_i32 => _FAT_TUESDAY,
    22730_i32 => _GOOD_FRIDAY,
    22732_i32 => _EASTER,
    22766_i32 => _AGRICULTURE_AND_LABOR_DAY,
    22783_i32 => _FLAG_DAY_AND_UNIVERSITY_DAY,
    22792_i32 => _CORPUS_CHRISTI,
    22872_i32 => _ASSUMPTION_OF_MARY,
    22935_i32 => _DEATH_OF_DESSALINES,
    22950_i32 => _ALL_SAINTS__DAY,
    22951_i32 => _DAY_OF_THE_DEAD,
    22967_i32 => _ARMED_FORCES_DAY__COMMEMORATION_OF_THE_BATTLE_OF_VERTIERES,
    23004_i32 => _CHRISTMAS_DAY,
    23011_i32 => _NATIONAL_INDEPENDENCE_DAY__NEW_YEAR_S_DAY,
    23012_i32 => _ANCESTRY_DAY,
    23068_i32 => _CARNIVAL,
    23069_i32 => _SHROVE_MONDAY,
    23070_i32 => _FAT_TUESDAY,
    23115_i32 => _GOOD_FRIDAY,
    23117_i32 => _EASTER,
    23131_i32 => _AGRICULTURE_AND_LABOR_DAY,
    23148_i32 => _FLAG_DAY_AND_UNIVERSITY_DAY,
    23177_i32 => _CORPUS_CHRISTI,
    23237_i32 => _ASSUMPTION_OF_MARY,
    23300_i32 => _DEATH_OF_DESSALINES,
    23315_i32 => _ALL_SAINTS__DAY,
    23316_i32 => _DAY_OF_THE_DEAD,
    23332_i32 => _ARMED_FORCES_DAY__COMMEMORATION_OF_THE_BATTLE_OF_VERTIERES,
    23369_i32 => _CHRISTMAS_DAY,
    23376_i32 => _NATIONAL_INDEPENDENCE_DAY__NEW_YEAR_S_DAY,
    23377_i32 => _ANCESTRY_DAY,
    23425_i32 => _CARNIVAL,
    23426_i32 => _SHROVE_MONDAY,
    23427_i32 => _FAT_TUESDAY,
    23472_i32 => _GOOD_FRIDAY,
    23474_i32 => _EASTER,
    23496_i32 => _AGRICULTURE_AND_LABOR_DAY,
    23513_i32 => _FLAG_DAY_AND_UNIVERSITY_DAY,
    23534_i32 => _CORPUS_CHRISTI,
    23602_i32 => _ASSUMPTION_OF_MARY,
    23665_i32 => _DEATH_OF_DESSALINES,
    23680_i32 => _ALL_SAINTS__DAY,
    23681_i32 => _DAY_OF_THE_DEAD,
    23697_i32 => _ARMED_FORCES_DAY__COMMEMORATION_OF_THE_BATTLE_OF_VERTIERES,
    23734_i32 => _CHRISTMAS_DAY,
    23741_i32 => _NATIONAL_INDEPENDENCE_DAY__NEW_YEAR_S_DAY,
    23742_i32 => _ANCESTRY_DAY,
    23775_i32 => _CARNIVAL,
    23776_i32 => _SHROVE_MONDAY,
    23777_i32 => _FAT_TUESDAY,
    23822_i32 => _GOOD_FRIDAY,
    23824_i32 => _EASTER,
    23861_i32 => _AGRICULTURE_AND_LABOR_DAY,
    23878_i32 => _FLAG_DAY_AND_UNIVERSITY_DAY,
    23884_i32 => _CORPUS_CHRISTI,
    23967_i32 => _ASSUMPTION_OF_MARY,
    24030_i32 => _DEATH_OF_DESSALINES,
    24045_i32 => _ALL_SAINTS__DAY,
    24046_i32 => _DAY_OF_THE_DEAD,
    24062_i32 => _ARMED_FORCES_DAY__COMMEMORATION_OF_THE_BATTLE_OF_VERTIERES,
    24099_i32 => _CHRISTMAS_DAY,
    24106_i32 => _NATIONAL_INDEPENDENCE_DAY__NEW_YEAR_S_DAY,
    24107_i32 => _ANCESTRY_DAY,
    24160_i32 => _CARNIVAL,
    24161_i32 => _SHROVE_MONDAY,
    24162_i32 => _FAT_TUESDAY,
    24207_i32 => _GOOD_FRIDAY,
    24209_i32 => _EASTER,
    24227_i32 => _AGRICULTURE_AND_LABOR_DAY,
    24244_i32 => _FLAG_DAY_AND_UNIVERSITY_DAY,
    24269_i32 => _CORPUS_CHRISTI,
    24333_i32 => _ASSUMPTION_OF_MARY,
    24396_i32 => _DEATH_OF_DESSALINES,
    24411_i32 => _ALL_SAINTS__DAY,
    24412_i32 => _DAY_OF_THE_DEAD,
    24428_i32 => _ARMED_FORCES_DAY__COMMEMORATION_OF_THE_BATTLE_OF_VERTIERES,
    24465_i32 => _CHRISTMAS_DAY,
    24472_i32 => _NATIONAL_INDEPENDENCE_DAY__NEW_YEAR_S_DAY,
    24473_i32 => _ANCESTRY_DAY,
    24517_i32 => _CARNIVAL,
    24518_i32 => _SHROVE_MONDAY,
    24519_i32 => _FAT_TUESDAY,
    24564_i32 => _GOOD_FRIDAY,
    24566_i32 => _EASTER,
    24592_i32 => _AGRICULTURE_AND_LABOR_DAY,
    24609_i32 => _FLAG_DAY_AND_UNIVERSITY_DAY,
    24626_i32 => _CORPUS_CHRISTI,
    24698_i32 => _ASSUMPTION_OF_MARY,
    24761_i32 => _DEATH_OF_DESSALINES,
    24776_i32 => _ALL_SAINTS__DAY,
    24777_i32 => _DAY_OF_THE_DEAD,
    24793_i32 => _ARMED_FORCES_DAY__COMMEMORATION_OF_THE_BATTLE_OF_VERTIERES,
    24830_i32 => _CHRISTMAS_DAY,
    24837_i32 => _NATIONAL_INDEPENDENCE_DAY__NEW_YEAR_S_DAY,
    24838_i32 => _ANCESTRY_DAY,
    24902_i32 => _CARNIVAL,
    24903_i32 => _SHROVE_MONDAY,
    24904_i32 => _FAT_TUESDAY,
    24949_i32 => _GOOD_FRIDAY,
    24951_i32 => _EASTER,
    24957_i32 => _AGRICULTURE_AND_LABOR_DAY,
    24974_i32 => _FLAG_DAY_AND_UNIVERSITY_DAY,
    25011_i32 => _CORPUS_CHRISTI,
    25063_i32 => _ASSUMPTION_OF_MARY,
    25126_i32 => _DEATH_OF_DESSALINES,
    25141_i32 => _ALL_SAINTS__DAY,
    25142_i32 => _DAY_OF_THE_DEAD,
    25158_i32 => _ARMED_FORCES_DAY__COMMEMORATION_OF_THE_BATTLE_OF_VERTIERES,
    25195_i32 => _CHRISTMAS_DAY,
    25202_i32 => _NATIONAL_INDEPENDENCE_DAY__NEW_YEAR_S_DAY,
    25203_i32 => _ANCESTRY_DAY,
    25252_i32 => _CARNIVAL,
    25253_i32 => _SHROVE_MONDAY,
    25254_i32 => _FAT_TUESDAY,
    25299_i32 => _GOOD_FRIDAY,
    25301_i32 => _EASTER,
    25322_i32 => _AGRICULTURE_AND_LABOR_DAY,
    25339_i32 => _FLAG_DAY_AND_UNIVERSITY_DAY,
    25361_i32 => _CORPUS_CHRISTI,
    25428_i32 => _ASSUMPTION_OF_MARY,
    25491_i32 => _DEATH_OF_DESSALINES,
    25506_i32 => _ALL_SAINTS__DAY,
    25507_i32 => _DAY_OF_THE_DEAD,
    25523_i32 => _ARMED_FORCES_DAY__COMMEMORATION_OF_THE_BATTLE_OF_VERTIERES,
    25560_i32 => _CHRISTMAS_DAY,
    25567_i32 => _NATIONAL_INDEPENDENCE_DAY__NEW_YEAR_S_DAY,
    25568_i32 => _ANCESTRY_DAY,
    25609_i32 => _CARNIVAL,
    25610_i32 => _SHROVE_MONDAY,
    25611_i32 => _FAT_TUESDAY,
    25656_i32 => _GOOD_FRIDAY,
    25658_i32 => _EASTER,
    25688_i32 => _AGRICULTURE_AND_LABOR_DAY,
    25705_i32 => _FLAG_DAY_AND_UNIVERSITY_DAY,
    25718_i32 => _CORPUS_CHRISTI,
    25794_i32 => _ASSUMPTION_OF_MARY,
    25857_i32 => _DEATH_OF_DESSALINES,
    25872_i32 => _ALL_SAINTS__DAY,
    25873_i32 => _DAY_OF_THE_DEAD,
    25889_i32 => _ARMED_FORCES_DAY__COMMEMORATION_OF_THE_BATTLE_OF_VERTIERES,
    25926_i32 => _CHRISTMAS_DAY,
    25933_i32 => _NATIONAL_INDEPENDENCE_DAY__NEW_YEAR_S_DAY,
    25934_i32 => _ANCESTRY_DAY,
    25994_i32 => _CARNIVAL,
    25995_i32 => _SHROVE_MONDAY,
    25996_i32 => _FAT_TUESDAY,
    26041_i32 => _GOOD_FRIDAY,
    26043_i32 => _EASTER,
    26053_i32 => _AGRICULTURE_AND_LABOR_DAY,
    26070_i32 => _FLAG_DAY_AND_UNIVERSITY_DAY,
    26103_i32 => _CORPUS_CHRISTI,
    26159_i32 => _ASSUMPTION_OF_MARY,
    26222_i32 => _DEATH_OF_DESSALINES,
    26237_i32 => _ALL_SAINTS__DAY,
    26238_i32 => _DAY_OF_THE_DEAD,
    26254_i32 => _ARMED_FORCES_DAY__COMMEMORATION_OF_THE_BATTLE_OF_VERTIERES,
    26291_i32 => _CHRISTMAS_DAY,
    26298_i32 => _NATIONAL_INDEPENDENCE_DAY__NEW_YEAR_S_DAY,
    26299_i32 => _ANCESTRY_DAY,
    26344_i32 => _CARNIVAL,
    26345_i32 => _SHROVE_MONDAY,
    26346_i32 => _FAT_TUESDAY,
    26391_i32 => _GOOD_FRIDAY,
    26393_i32 => _EASTER,
    26418_i32 => _AGRICULTURE_AND_LABOR_DAY,
    26435_i32 => _FLAG_DAY_AND_UNIVERSITY_DAY,
    26453_i32 => _CORPUS_CHRISTI,
    26524_i32 => _ASSUMPTION_OF_MARY,
    26587_i32 => _DEATH_OF_DESSALINES,
    26602_i32 => _ALL_SAINTS__DAY,
    26603_i32 => _DAY_OF_THE_DEAD,
    26619_i32 => _ARMED_FORCES_DAY__COMMEMORATION_OF_THE_BATTLE_OF_VERTIERES,
    26656_i32 => _CHRISTMAS_DAY,
    26663_i32 => _NATIONAL_INDEPENDENCE_DAY__NEW_YEAR_S_DAY,
    26664_i32 => _ANCESTRY_DAY,
    26701_i32 => _CARNIVAL,
    26702_i32 => _SHROVE_MONDAY,
    26703_i32 => _FAT_TUESDAY,
    26748_i32 => _GOOD_FRIDAY,
    26750_i32 => _EASTER,
    26783_i32 => _AGRICULTURE_AND_LABOR_DAY,
    26800_i32 => _FLAG_DAY_AND_UNIVERSITY_DAY,
    26810_i32 => _CORPUS_CHRISTI,
    26889_i32 => _ASSUMPTION_OF_MARY,
    26952_i32 => _DEATH_OF_DESSALINES,
    26967_i32 => _ALL_SAINTS__DAY,
    26968_i32 => _DAY_OF_THE_DEAD,
    26984_i32 => _ARMED_FORCES_DAY__COMMEMORATION_OF_THE_BATTLE_OF_VERTIERES,
    27021_i32 => _CHRISTMAS_DAY,
    27028_i32 => _NATIONAL_INDEPENDENCE_DAY__NEW_YEAR_S_DAY,
    27029_i32 => _ANCESTRY_DAY,
    27086_i32 => _CARNIVAL,
    27087_i32 => _SHROVE_MONDAY,
    27088_i32 => _FAT_TUESDAY,
    27133_i32 => _GOOD_FRIDAY,
    27135_i32 => _EASTER,
    27149_i32 => _AGRICULTURE_AND_LABOR_DAY,
    27166_i32 => _FLAG_DAY_AND_UNIVERSITY_DAY,
    27195_i32 => _CORPUS_CHRISTI,
    27255_i32 => _ASSUMPTION_OF_MARY,
    27318_i32 => _DEATH_OF_DESSALINES,
    27333_i32 => _ALL_SAINTS__DAY,
    27334_i32 => _DAY_OF_THE_DEAD,
    27350_i32 => _ARMED_FORCES_DAY__COMMEMORATION_OF_THE_BATTLE_OF_VERTIERES,
    27387_i32 => _CHRISTMAS_DAY,
    27394_i32 => _NATIONAL_INDEPENDENCE_DAY__NEW_YEAR_S_DAY,
    27395_i32 => _ANCESTRY_DAY,
    27443_i32 => _CARNIVAL,
    27444_i32 => _SHROVE_MONDAY,
    27445_i32 => _FAT_TUESDAY,
    27490_i32 => _GOOD_FRIDAY,
    27492_i32 => _EASTER,
    27514_i32 => _AGRICULTURE_AND_LABOR_DAY,
    27531_i32 => _FLAG_DAY_AND_UNIVERSITY_DAY,
    27552_i32 => _CORPUS_CHRISTI,
    27620_i32 => _ASSUMPTION_OF_MARY,
    27683_i32 => _DEATH_OF_DESSALINES,
    27698_i32 => _ALL_SAINTS__DAY,
    27699_i32 => _DAY_OF_THE_DEAD,
    27715_i32 => _ARMED_FORCES_DAY__COMMEMORATION_OF_THE_BATTLE_OF_VERTIERES,
    27752_i32 => _CHRISTMAS_DAY,
    27759_i32 => _NATIONAL_INDEPENDENCE_DAY__NEW_YEAR_S_DAY,
    27760_i32 => _ANCESTRY_DAY,
    27793_i32 => _CARNIVAL,
    27794_i32 => _SHROVE_MONDAY,
    27795_i32 => _FAT_TUESDAY,
    27840_i32 => _GOOD_FRIDAY,
    27842_i32 => _EASTER,
    27879_i32 => _AGRICULTURE_AND_LABOR_DAY,
    27896_i32 => _FLAG_DAY_AND_UNIVERSITY_DAY,
    27902_i32 => _CORPUS_CHRISTI,
    27985_i32 => _ASSUMPTION_OF_MARY,
    28048_i32 => _DEATH_OF_DESSALINES,
    28063_i32 => _ALL_SAINTS__DAY,
    28064_i32 => _DAY_OF_THE_DEAD,
    28080_i32 => _ARMED_FORCES_DAY__COMMEMORATION_OF_THE_BATTLE_OF_VERTIERES,
    28117_i32 => _CHRISTMAS_DAY,
    28124_i32 => _NATIONAL_INDEPENDENCE_DAY__NEW_YEAR_S_DAY,
    28125_i32 => _ANCESTRY_DAY,
    28178_i32 => _CARNIVAL,
    28179_i32 => _SHROVE_MONDAY,
    28180_i32 => _FAT_TUESDAY,
    28225_i32 => _GOOD_FRIDAY,
    28227_i32 => _EASTER,
    28244_i32 => _AGRICULTURE_AND_LABOR_DAY,
    28261_i32 => _FLAG_DAY_AND_UNIVERSITY_DAY,
    28287_i32 => _CORPUS_CHRISTI,
    28350_i32 => _ASSUMPTION_OF_MARY,
    28413_i32 => _DEATH_OF_DESSALINES,
    28428_i32 => _ALL_SAINTS__DAY,
    28429_i32 => _DAY_OF_THE_DEAD,
    28445_i32 => _ARMED_FORCES_DAY__COMMEMORATION_OF_THE_BATTLE_OF_VERTIERES,
    28482_i32 => _CHRISTMAS_DAY,
    28489_i32 => _NATIONAL_INDEPENDENCE_DAY__NEW_YEAR_S_DAY,
    28490_i32 => _ANCESTRY_DAY,
    28535_i32 => _CARNIVAL,
    28536_i32 => _SHROVE_MONDAY,
    28537_i32 => _FAT_TUESDAY,
    28582_i32 => _GOOD_FRIDAY,
    28584_i32 => _EASTER,
    28610_i32 => _AGRICULTURE_AND_LABOR_DAY,
    28627_i32 => _FLAG_DAY_AND_UNIVERSITY_DAY,
    28644_i32 => _CORPUS_CHRISTI,
    28716_i32 => _ASSUMPTION_OF_MARY,
    28779_i32 => _DEATH_OF_DESSALINES,
    28794_i32 => _ALL_SAINTS__DAY,
    28795_i32 => _DAY_OF_THE_DEAD,
    28811_i32 => _ARMED_FORCES_DAY__COMMEMORATION_OF_THE_BATTLE_OF_VERTIERES,
    28848_i32 => _CHRISTMAS_DAY,
    28855_i32 => _NATIONAL_INDEPENDENCE_DAY__NEW_YEAR_S_DAY,
    28856_i32 => _ANCESTRY_DAY,
    28913_i32 => _CARNIVAL,
    28914_i32 => _SHROVE_MONDAY,
    28915_i32 => _FAT_TUESDAY,
    28960_i32 => _GOOD_FRIDAY,
    28962_i32 => _EASTER,
    28975_i32 => _AGRICULTURE_AND_LABOR_DAY,
    28992_i32 => _FLAG_DAY_AND_UNIVERSITY_DAY,
    29022_i32 => _CORPUS_CHRISTI,
    29081_i32 => _ASSUMPTION_OF_MARY,
    29144_i32 => _DEATH_OF_DESSALINES,
    29159_i32 => _ALL_SAINTS__DAY,
    29160_i32 => _DAY_OF_THE_DEAD,
    29176_i32 => _ARMED_FORCES_DAY__COMMEMORATION_OF_THE_BATTLE_OF_VERTIERES,
    29213_i32 => _CHRISTMAS_DAY,
    29220_i32 => _NATIONAL_INDEPENDENCE_DAY__NEW_YEAR_S_DAY,
};
