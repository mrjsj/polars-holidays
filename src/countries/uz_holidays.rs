use crate::countries::constants::*;
use phf::phf_map;

pub static UZ_HOLIDAYS: phf::Map<i32, &'static str> = phf_map! {
    10957_i32 => _NEW_YEAR_S_DAY,
    10964_i32 => _EID_AL_FITR__ESTIMATED_,
    11024_i32 => _WOMEN_S_DAY,
    11032_i32 => _EID_AL_ADHA__ESTIMATED_,
    11037_i32 => _NOWRUZ,
    11086_i32 => _DAY_OF_MEMORY_AND_HONOR,
    11201_i32 => _INDEPENDENCE_DAY,
    11231_i32 => _TEACHERS_AND_INSTRUCTORS_DAY,
    11299_i32 => _CONSTITUTION_DAY,
    11318_i32 => _EID_AL_FITR__ESTIMATED_,
    11323_i32 => _NEW_YEAR_S_DAY,
    11386_i32 => _EID_AL_ADHA__ESTIMATED_,
    11389_i32 => _WOMEN_S_DAY,
    11402_i32 => _NOWRUZ,
    11451_i32 => _DAY_OF_MEMORY_AND_HONOR,
    11566_i32 => _INDEPENDENCE_DAY,
    11596_i32 => _TEACHERS_AND_INSTRUCTORS_DAY,
    11664_i32 => _CONSTITUTION_DAY,
    11672_i32 => _EID_AL_FITR__ESTIMATED_,
    11688_i32 => _NEW_YEAR_S_DAY,
    11740_i32 => _EID_AL_ADHA__ESTIMATED_,
    11754_i32 => _WOMEN_S_DAY,
    11767_i32 => _NOWRUZ,
    11816_i32 => _DAY_OF_MEMORY_AND_HONOR,
    11931_i32 => _INDEPENDENCE_DAY,
    11961_i32 => _TEACHERS_AND_INSTRUCTORS_DAY,
    12026_i32 => _EID_AL_FITR__ESTIMATED_,
    12029_i32 => _CONSTITUTION_DAY,
    12053_i32 => _NEW_YEAR_S_DAY,
    12094_i32 => _EID_AL_ADHA__ESTIMATED_,
    12119_i32 => _WOMEN_S_DAY,
    12132_i32 => _NOWRUZ,
    12181_i32 => _DAY_OF_MEMORY_AND_HONOR,
    12296_i32 => _INDEPENDENCE_DAY,
    12326_i32 => _TEACHERS_AND_INSTRUCTORS_DAY,
    12381_i32 => _EID_AL_FITR__ESTIMATED_,
    12394_i32 => _CONSTITUTION_DAY,
    12418_i32 => _NEW_YEAR_S_DAY,
    12449_i32 => _EID_AL_ADHA__ESTIMATED_,
    12485_i32 => _WOMEN_S_DAY,
    12498_i32 => _NOWRUZ,
    12547_i32 => _DAY_OF_MEMORY_AND_HONOR,
    12662_i32 => _INDEPENDENCE_DAY,
    12692_i32 => _TEACHERS_AND_INSTRUCTORS_DAY,
    12736_i32 => _EID_AL_FITR__ESTIMATED_,
    12760_i32 => _CONSTITUTION_DAY,
    12784_i32 => _NEW_YEAR_S_DAY,
    12804_i32 => _EID_AL_ADHA__ESTIMATED_,
    12850_i32 => _WOMEN_S_DAY,
    12863_i32 => _NOWRUZ,
    12912_i32 => _DAY_OF_MEMORY_AND_HONOR,
    13027_i32 => _INDEPENDENCE_DAY,
    13057_i32 => _TEACHERS_AND_INSTRUCTORS_DAY,
    13090_i32 => _EID_AL_FITR__ESTIMATED_,
    13125_i32 => _CONSTITUTION_DAY,
    13149_i32 => _NEW_YEAR_S_DAY,
    13158_i32 => _EID_AL_ADHA,
    13215_i32 => _WOMEN_S_DAY,
    13228_i32 => _NOWRUZ,
    13277_i32 => _DAY_OF_MEMORY_AND_HONOR,
    13392_i32 => _INDEPENDENCE_DAY,
    13422_i32 => _TEACHERS_AND_INSTRUCTORS_DAY,
    13444_i32 => _EID_AL_FITR,
    13490_i32 => _CONSTITUTION_DAY,
    13512_i32 => _EID_AL_ADHA,
    13514_i32 => _NEW_YEAR_S_DAY,
    13580_i32 => _WOMEN_S_DAY,
    13593_i32 => _NOWRUZ,
    13642_i32 => _DAY_OF_MEMORY_AND_HONOR,
    13757_i32 => _INDEPENDENCE_DAY,
    13787_i32 => _TEACHERS_AND_INSTRUCTORS_DAY,
    13799_i32 => _EID_AL_FITR,
    13855_i32 => _CONSTITUTION_DAY,
    13866_i32 => _EID_AL_ADHA,
    13879_i32 => _NEW_YEAR_S_DAY,
    13946_i32 => _WOMEN_S_DAY,
    13959_i32 => _NOWRUZ,
    14008_i32 => _DAY_OF_MEMORY_AND_HONOR,
    14123_i32 => _INDEPENDENCE_DAY,
    14153_i32 => _EID_AL_FITR__TEACHERS_AND_INSTRUCTORS_DAY,
    14221_i32 => _CONSTITUTION_DAY__EID_AL_ADHA,
    14245_i32 => _NEW_YEAR_S_DAY,
    14311_i32 => _WOMEN_S_DAY,
    14324_i32 => _NOWRUZ,
    14373_i32 => _DAY_OF_MEMORY_AND_HONOR,
    14488_i32 => _INDEPENDENCE_DAY,
    14508_i32 => _EID_AL_FITR,
    14518_i32 => _TEACHERS_AND_INSTRUCTORS_DAY,
    14575_i32 => _EID_AL_ADHA,
    14586_i32 => _CONSTITUTION_DAY,
    14610_i32 => _NEW_YEAR_S_DAY,
    14676_i32 => _WOMEN_S_DAY,
    14689_i32 => _NOWRUZ,
    14738_i32 => _DAY_OF_MEMORY_AND_HONOR,
    14853_i32 => _INDEPENDENCE_DAY,
    14862_i32 => _EID_AL_FITR,
    14883_i32 => _TEACHERS_AND_INSTRUCTORS_DAY,
    14929_i32 => _EID_AL_ADHA,
    14951_i32 => _CONSTITUTION_DAY,
    14975_i32 => _NEW_YEAR_S_DAY,
    15041_i32 => _WOMEN_S_DAY,
    15054_i32 => _NOWRUZ,
    15103_i32 => _DAY_OF_MEMORY_AND_HONOR,
    15217_i32 => _EID_AL_FITR,
    15218_i32 => _INDEPENDENCE_DAY,
    15248_i32 => _TEACHERS_AND_INSTRUCTORS_DAY,
    15284_i32 => _EID_AL_ADHA,
    15316_i32 => _CONSTITUTION_DAY,
    15340_i32 => _NEW_YEAR_S_DAY,
    15407_i32 => _WOMEN_S_DAY,
    15420_i32 => _NOWRUZ,
    15469_i32 => _DAY_OF_MEMORY_AND_HONOR,
    15571_i32 => _EID_AL_FITR,
    15584_i32 => _INDEPENDENCE_DAY,
    15614_i32 => _TEACHERS_AND_INSTRUCTORS_DAY,
    15639_i32 => _EID_AL_ADHA,
    15682_i32 => _CONSTITUTION_DAY,
    15706_i32 => _NEW_YEAR_S_DAY,
    15772_i32 => _WOMEN_S_DAY,
    15785_i32 => _NOWRUZ,
    15834_i32 => _DAY_OF_MEMORY_AND_HONOR,
    15926_i32 => _EID_AL_FITR,
    15949_i32 => _INDEPENDENCE_DAY,
    15979_i32 => _TEACHERS_AND_INSTRUCTORS_DAY,
    15993_i32 => _EID_AL_ADHA,
    16047_i32 => _CONSTITUTION_DAY,
    16071_i32 => _NEW_YEAR_S_DAY,
    16137_i32 => _WOMEN_S_DAY,
    16150_i32 => _NOWRUZ,
    16199_i32 => _DAY_OF_MEMORY_AND_HONOR,
    16279_i32 => _EID_AL_FITR,
    16314_i32 => _INDEPENDENCE_DAY,
    16344_i32 => _TEACHERS_AND_INSTRUCTORS_DAY,
    16347_i32 => _EID_AL_ADHA,
    16412_i32 => _CONSTITUTION_DAY,
    16436_i32 => _NEW_YEAR_S_DAY,
    16502_i32 => _WOMEN_S_DAY,
    16515_i32 => _NOWRUZ,
    16564_i32 => _DAY_OF_MEMORY_AND_HONOR,
    16634_i32 => _EID_AL_FITR,
    16679_i32 => _INDEPENDENCE_DAY,
    16702_i32 => _EID_AL_ADHA,
    16709_i32 => _TEACHERS_AND_INSTRUCTORS_DAY,
    16777_i32 => _CONSTITUTION_DAY,
    16801_i32 => _NEW_YEAR_S_DAY,
    16868_i32 => _WOMEN_S_DAY,
    16881_i32 => _NOWRUZ,
    16930_i32 => _DAY_OF_MEMORY_AND_HONOR,
    16988_i32 => _EID_AL_FITR,
    17045_i32 => _INDEPENDENCE_DAY,
    17056_i32 => _EID_AL_ADHA,
    17075_i32 => _TEACHERS_AND_INSTRUCTORS_DAY,
    17143_i32 => _CONSTITUTION_DAY,
    17167_i32 => _NEW_YEAR_S_DAY,
    17233_i32 => _WOMEN_S_DAY,
    17246_i32 => _NOWRUZ,
    17295_i32 => _DAY_OF_MEMORY_AND_HONOR,
    17343_i32 => _EID_AL_FITR,
    17410_i32 => _EID_AL_ADHA__INDEPENDENCE_DAY,
    17440_i32 => _TEACHERS_AND_INSTRUCTORS_DAY,
    17508_i32 => _CONSTITUTION_DAY,
    17532_i32 => _NEW_YEAR_S_DAY,
    17533_i32 => _ADDITIONAL_DAY_OFF_BY_PRESIDENTIAL_DECREE,
    17534_i32 => _DAY_OFF__SUBSTITUTED_FROM_01_06_2018_,
    17598_i32 => _WOMEN_S_DAY,
    17609_i32 => _DAY_OFF__SUBSTITUTED_FROM_03_17_2018_,
    17611_i32 => _NOWRUZ,
    17612_i32 => _DAY_OFF__SUBSTITUTED_FROM_03_24_2018_,
    17620_i32 => _ADDITIONAL_DAY_OFF_BY_PRESIDENTIAL_DECREE,
    17660_i32 => _DAY_OF_MEMORY_AND_HONOR,
    17697_i32 => _EID_AL_FITR,
    17764_i32 => _EID_AL_ADHA,
    17766_i32 => _DAY_OFF__SUBSTITUTED_FROM_08_25_2018_,
    17767_i32 => _DAY_OFF__SUBSTITUTED_FROM_08_26_2018_,
    17774_i32 => _ADDITIONAL_DAY_OFF_BY_PRESIDENTIAL_DECREE,
    17775_i32 => _INDEPENDENCE_DAY,
    17777_i32 => _DAY_OFF__SUBSTITUTED_FROM_09_08_2018_,
    17778_i32 => _DAY_OFF__SUBSTITUTED_FROM_09_15_2018_,
    17805_i32 => _TEACHERS_AND_INSTRUCTORS_DAY,
    17873_i32 => _CONSTITUTION_DAY,
    17896_i32 => _DAY_OFF__SUBSTITUTED_FROM_12_29_2018_,
    17897_i32 => _NEW_YEAR_S_DAY,
    17898_i32 => _ADDITIONAL_DAY_OFF_BY_PRESIDENTIAL_DECREE,
    17899_i32 => _DAY_OFF__SUBSTITUTED_FROM_01_05_2019_,
    17963_i32 => _WOMEN_S_DAY,
    17976_i32 => _NOWRUZ,
    17977_i32 => _ADDITIONAL_DAY_OFF_BY_PRESIDENTIAL_DECREE,
    18025_i32 => _DAY_OF_MEMORY_AND_HONOR,
    18052_i32 => _EID_AL_FITR,
    18053_i32 => _DAY_OFF__SUBSTITUTED_FROM_06_01_2019_,
    18119_i32 => _EID_AL_ADHA,
    18140_i32 => _INDEPENDENCE_DAY,
    18141_i32 => _ADDITIONAL_DAY_OFF_BY_PRESIDENTIAL_DECREE,
    18142_i32 => _DAY_OFF__SUBSTITUTED_FROM_09_07_2019_,
    18170_i32 => _TEACHERS_AND_INSTRUCTORS_DAY,
    18238_i32 => _CONSTITUTION_DAY,
    18261_i32 => _DAY_OFF__SUBSTITUTED_FROM_12_28_2019_,
    18262_i32 => _NEW_YEAR_S_DAY,
    18263_i32 => _DAY_OFF__SUBSTITUTED_FROM_01_04_2020_,
    18329_i32 => _WOMEN_S_DAY,
    18342_i32 => _NOWRUZ,
    18344_i32 => _ADDITIONAL_DAY_OFF_BY_PRESIDENTIAL_DECREE,
    18391_i32 => _DAY_OF_MEMORY_AND_HONOR,
    18406_i32 => _EID_AL_FITR,
    18474_i32 => _EID_AL_ADHA,
    18505_i32 => _DAY_OFF__SUBSTITUTED_FROM_08_29_2020_,
    18506_i32 => _INDEPENDENCE_DAY,
    18536_i32 => _TEACHERS_AND_INSTRUCTORS_DAY,
    18604_i32 => _CONSTITUTION_DAY,
    18628_i32 => _NEW_YEAR_S_DAY,
    18694_i32 => _WOMEN_S_DAY,
    18707_i32 => _NOWRUZ,
    18708_i32 => _DAY_OFF__SUBSTITUTED_FROM_03_27_2021_,
    18756_i32 => _DAY_OF_MEMORY_AND_HONOR,
    18760_i32 => _EID_AL_FITR,
    18761_i32 => _ADDITIONAL_DAY_OFF_BY_PRESIDENTIAL_DECREE,
    18828_i32 => _EID_AL_ADHA,
    18829_i32 => _DAY_OFF__SUBSTITUTED_FROM_07_17_2021_,
    18830_i32 => _DAY_OFF__SUBSTITUTED_FROM_07_24_2021_,
    18871_i32 => _INDEPENDENCE_DAY,
    18872_i32 => _ADDITIONAL_DAY_OFF_BY_PRESIDENTIAL_DECREE,
    18873_i32 => _ADDITIONAL_DAY_OFF_BY_PRESIDENTIAL_DECREE,
    18901_i32 => _TEACHERS_AND_INSTRUCTORS_DAY,
    18969_i32 => _CONSTITUTION_DAY,
    18992_i32 => _ADDITIONAL_DAY_OFF_BY_PRESIDENTIAL_DECREE,
    18993_i32 => _NEW_YEAR_S_DAY,
    18995_i32 => _ADDITIONAL_DAY_OFF_BY_PRESIDENTIAL_DECREE,
    18996_i32 => _DAY_OFF__SUBSTITUTED_FROM_01_08_2022_,
    19059_i32 => _WOMEN_S_DAY,
    19072_i32 => _NOWRUZ,
    19073_i32 => _ADDITIONAL_DAY_OFF_BY_PRESIDENTIAL_DECREE,
    19074_i32 => _ADDITIONAL_DAY_OFF_BY_PRESIDENTIAL_DECREE,
    19114_i32 => _EID_AL_FITR,
    19115_i32 => _ADDITIONAL_DAY_OFF_BY_PRESIDENTIAL_DECREE,
    19116_i32 => _DAY_OFF__SUBSTITUTED_FROM_05_07_2022_,
    19121_i32 => _DAY_OF_MEMORY_AND_HONOR,
    19182_i32 => _EID_AL_ADHA,
    19184_i32 => _ADDITIONAL_DAY_OFF_BY_PRESIDENTIAL_DECREE,
    19185_i32 => _DAY_OFF__SUBSTITUTED_FROM_07_16_2022_,
    19236_i32 => _INDEPENDENCE_DAY,
    19237_i32 => _ADDITIONAL_DAY_OFF_BY_PRESIDENTIAL_DECREE,
    19266_i32 => _TEACHERS_AND_INSTRUCTORS_DAY,
    19334_i32 => _CONSTITUTION_DAY,
    19358_i32 => _NEW_YEAR_S_DAY,
    19359_i32 => _ADDITIONAL_DAY_OFF_BY_PRESIDENTIAL_DECREE,
    19360_i32 => _DAY_OFF__SUBSTITUTED_FROM_01_07_2023_,
    19424_i32 => _WOMEN_S_DAY,
    19436_i32 => _DAY_OFF__SUBSTITUTED_FROM_03_11_2023_,
    19437_i32 => _NOWRUZ,
    19438_i32 => _DAY_OFF__SUBSTITUTED_FROM_03_25_2023_,
    19468_i32 => _EID_AL_FITR,
    19471_i32 => _ADDITIONAL_DAY_OFF_BY_PRESIDENTIAL_DECREE,
    19486_i32 => _DAY_OF_MEMORY_AND_HONOR,
    19536_i32 => _EID_AL_ADHA,
    19537_i32 => _ADDITIONAL_DAY_OFF_BY_PRESIDENTIAL_DECREE,
    19538_i32 => _ADDITIONAL_DAY_OFF_BY_PRESIDENTIAL_DECREE,
    19601_i32 => _INDEPENDENCE_DAY,
    19631_i32 => _TEACHERS_AND_INSTRUCTORS_DAY,
    19632_i32 => _TEACHERS_AND_INSTRUCTORS_DAY__OBSERVED_,
    19699_i32 => _CONSTITUTION_DAY,
    19723_i32 => _NEW_YEAR_S_DAY,
    19724_i32 => _DAY_OFF__SUBSTITUTED_FROM_01_06_2024_,
    19790_i32 => _WOMEN_S_DAY,
    19803_i32 => _NOWRUZ,
    19804_i32 => _ADDITIONAL_DAY_OFF_BY_PRESIDENTIAL_DECREE,
    19823_i32 => _EID_AL_FITR,
    19824_i32 => _ADDITIONAL_DAY_OFF_BY_PRESIDENTIAL_DECREE,
    19825_i32 => _DAY_OFF__SUBSTITUTED_FROM_04_13_2024_,
    19852_i32 => _DAY_OF_MEMORY_AND_HONOR,
    19890_i32 => _EID_AL_ADHA__ESTIMATED_,
    19891_i32 => _EID_AL_ADHA__OBSERVED__ESTIMATED_,
    19892_i32 => _ADDITIONAL_DAY_OFF_BY_PRESIDENTIAL_DECREE,
    19967_i32 => _INDEPENDENCE_DAY,
    19968_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    19969_i32 => _ADDITIONAL_DAY_OFF_BY_PRESIDENTIAL_DECREE,
    19997_i32 => _TEACHERS_AND_INSTRUCTORS_DAY,
    20065_i32 => _CONSTITUTION_DAY,
    20066_i32 => _CONSTITUTION_DAY__OBSERVED_,
    20087_i32 => _DAY_OFF__SUBSTITUTED_FROM_12_14_2024_,
    20088_i32 => _ADDITIONAL_DAY_OFF_BY_PRESIDENTIAL_DECREE,
    20089_i32 => _NEW_YEAR_S_DAY,
    20155_i32 => _WOMEN_S_DAY,
    20157_i32 => _WOMEN_S_DAY__OBSERVED_,
    20168_i32 => _NOWRUZ,
    20177_i32 => _EID_AL_FITR__ESTIMATED_,
    20178_i32 => _EID_AL_FITR__OBSERVED__ESTIMATED_,
    20217_i32 => _DAY_OF_MEMORY_AND_HONOR,
    20245_i32 => _EID_AL_ADHA__ESTIMATED_,
    20332_i32 => _INDEPENDENCE_DAY,
    20362_i32 => _TEACHERS_AND_INSTRUCTORS_DAY,
    20430_i32 => _CONSTITUTION_DAY,
    20454_i32 => _NEW_YEAR_S_DAY,
    20520_i32 => _WOMEN_S_DAY,
    20521_i32 => _WOMEN_S_DAY__OBSERVED_,
    20532_i32 => _EID_AL_FITR__ESTIMATED_,
    20533_i32 => _NOWRUZ,
    20535_i32 => _NOWRUZ__OBSERVED_,
    20582_i32 => _DAY_OF_MEMORY_AND_HONOR,
    20584_i32 => _DAY_OF_MEMORY_AND_HONOR__OBSERVED_,
    20600_i32 => _EID_AL_ADHA__ESTIMATED_,
    20697_i32 => _INDEPENDENCE_DAY,
    20727_i32 => _TEACHERS_AND_INSTRUCTORS_DAY,
    20795_i32 => _CONSTITUTION_DAY,
    20819_i32 => _NEW_YEAR_S_DAY,
    20885_i32 => _WOMEN_S_DAY,
    20886_i32 => _EID_AL_FITR__ESTIMATED_,
    20898_i32 => _NOWRUZ,
    20899_i32 => _NOWRUZ__OBSERVED_,
    20947_i32 => _DAY_OF_MEMORY_AND_HONOR,
    20948_i32 => _DAY_OF_MEMORY_AND_HONOR__OBSERVED_,
    20954_i32 => _EID_AL_ADHA__ESTIMATED_,
    20955_i32 => _EID_AL_ADHA__OBSERVED__ESTIMATED_,
    21062_i32 => _INDEPENDENCE_DAY,
    21092_i32 => _TEACHERS_AND_INSTRUCTORS_DAY,
    21160_i32 => _CONSTITUTION_DAY,
    21184_i32 => _NEW_YEAR_S_DAY,
    21186_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    21240_i32 => _EID_AL_FITR__ESTIMATED_,
    21242_i32 => _EID_AL_FITR__OBSERVED__ESTIMATED_,
    21251_i32 => _WOMEN_S_DAY,
    21264_i32 => _NOWRUZ,
    21309_i32 => _EID_AL_ADHA__ESTIMATED_,
    21313_i32 => _DAY_OF_MEMORY_AND_HONOR,
    21428_i32 => _INDEPENDENCE_DAY,
    21458_i32 => _TEACHERS_AND_INSTRUCTORS_DAY,
    21459_i32 => _TEACHERS_AND_INSTRUCTORS_DAY__OBSERVED_,
    21526_i32 => _CONSTITUTION_DAY,
    21550_i32 => _NEW_YEAR_S_DAY,
    21594_i32 => _EID_AL_FITR__ESTIMATED_,
    21616_i32 => _WOMEN_S_DAY,
    21629_i32 => _NOWRUZ,
    21663_i32 => _EID_AL_ADHA__ESTIMATED_,
    21678_i32 => _DAY_OF_MEMORY_AND_HONOR,
    21793_i32 => _INDEPENDENCE_DAY,
    21795_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    21823_i32 => _TEACHERS_AND_INSTRUCTORS_DAY,
    21891_i32 => _CONSTITUTION_DAY,
    21893_i32 => _CONSTITUTION_DAY__OBSERVED_,
    21915_i32 => _NEW_YEAR_S_DAY,
    21949_i32 => _EID_AL_FITR__ESTIMATED_,
    21981_i32 => _WOMEN_S_DAY,
    21994_i32 => _NOWRUZ,
    22017_i32 => _EID_AL_ADHA__ESTIMATED_,
    22019_i32 => _EID_AL_ADHA__OBSERVED__ESTIMATED_,
    22043_i32 => _DAY_OF_MEMORY_AND_HONOR,
    22158_i32 => _INDEPENDENCE_DAY,
    22159_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    22188_i32 => _TEACHERS_AND_INSTRUCTORS_DAY,
    22256_i32 => _CONSTITUTION_DAY,
    22257_i32 => _CONSTITUTION_DAY__OBSERVED_,
    22280_i32 => _NEW_YEAR_S_DAY,
    22303_i32 => _EID_AL_FITR__ESTIMATED_,
    22346_i32 => _WOMEN_S_DAY,
    22348_i32 => _WOMEN_S_DAY__OBSERVED_,
    22359_i32 => _NOWRUZ,
    22371_i32 => _EID_AL_ADHA__ESTIMATED_,
    22408_i32 => _DAY_OF_MEMORY_AND_HONOR,
    22523_i32 => _INDEPENDENCE_DAY,
    22553_i32 => _TEACHERS_AND_INSTRUCTORS_DAY,
    22621_i32 => _CONSTITUTION_DAY,
    22645_i32 => _NEW_YEAR_S_DAY,
    22658_i32 => _EID_AL_FITR__ESTIMATED_,
    22712_i32 => _WOMEN_S_DAY,
    22725_i32 => _NOWRUZ,
    22726_i32 => _EID_AL_ADHA__ESTIMATED_,
    22727_i32 => _NOWRUZ__OBSERVED_,
    22774_i32 => _DAY_OF_MEMORY_AND_HONOR,
    22775_i32 => _DAY_OF_MEMORY_AND_HONOR__OBSERVED_,
    22889_i32 => _INDEPENDENCE_DAY,
    22919_i32 => _TEACHERS_AND_INSTRUCTORS_DAY,
    22987_i32 => _CONSTITUTION_DAY,
    23011_i32 => _NEW_YEAR_S_DAY,
    23012_i32 => _EID_AL_FITR__ESTIMATED_,
    23013_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    23014_i32 => _EID_AL_FITR__OBSERVED__ESTIMATED_,
    23077_i32 => _WOMEN_S_DAY,
    23080_i32 => _EID_AL_ADHA__ESTIMATED_,
    23090_i32 => _NOWRUZ,
    23139_i32 => _DAY_OF_MEMORY_AND_HONOR,
    23254_i32 => _INDEPENDENCE_DAY,
    23284_i32 => _TEACHERS_AND_INSTRUCTORS_DAY,
    23286_i32 => _TEACHERS_AND_INSTRUCTORS_DAY__OBSERVED_,
    23352_i32 => _CONSTITUTION_DAY,
    23367_i32 => _EID_AL_FITR__ESTIMATED_,
    23376_i32 => _NEW_YEAR_S_DAY,
    23377_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    23435_i32 => _EID_AL_ADHA__ESTIMATED_,
    23442_i32 => _WOMEN_S_DAY,
    23455_i32 => _NOWRUZ,
    23504_i32 => _DAY_OF_MEMORY_AND_HONOR,
    23619_i32 => _INDEPENDENCE_DAY,
    23649_i32 => _TEACHERS_AND_INSTRUCTORS_DAY,
    23650_i32 => _TEACHERS_AND_INSTRUCTORS_DAY__OBSERVED_,
    23717_i32 => _CONSTITUTION_DAY,
    23721_i32 => _EID_AL_FITR__ESTIMATED_,
    23741_i32 => _NEW_YEAR_S_DAY,
    23789_i32 => _EID_AL_ADHA__ESTIMATED_,
    23790_i32 => _EID_AL_ADHA__OBSERVED__ESTIMATED_,
    23807_i32 => _WOMEN_S_DAY,
    23820_i32 => _NOWRUZ,
    23869_i32 => _DAY_OF_MEMORY_AND_HONOR,
    23984_i32 => _INDEPENDENCE_DAY,
    23986_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    24014_i32 => _TEACHERS_AND_INSTRUCTORS_DAY,
    24075_i32 => _EID_AL_FITR__ESTIMATED_,
    24077_i32 => _EID_AL_FITR__OBSERVED__ESTIMATED_,
    24082_i32 => _CONSTITUTION_DAY,
    24084_i32 => _CONSTITUTION_DAY__OBSERVED_,
    24106_i32 => _NEW_YEAR_S_DAY,
    24143_i32 => _EID_AL_ADHA__ESTIMATED_,
    24173_i32 => _WOMEN_S_DAY,
    24175_i32 => _WOMEN_S_DAY__OBSERVED_,
    24186_i32 => _NOWRUZ,
    24235_i32 => _DAY_OF_MEMORY_AND_HONOR,
    24350_i32 => _INDEPENDENCE_DAY,
    24380_i32 => _TEACHERS_AND_INSTRUCTORS_DAY,
    24429_i32 => _EID_AL_FITR__ESTIMATED_,
    24448_i32 => _CONSTITUTION_DAY,
    24472_i32 => _NEW_YEAR_S_DAY,
    24497_i32 => _EID_AL_ADHA__ESTIMATED_,
    24538_i32 => _WOMEN_S_DAY,
    24539_i32 => _WOMEN_S_DAY__OBSERVED_,
    24551_i32 => _NOWRUZ,
    24553_i32 => _NOWRUZ__OBSERVED_,
    24600_i32 => _DAY_OF_MEMORY_AND_HONOR,
    24602_i32 => _DAY_OF_MEMORY_AND_HONOR__OBSERVED_,
    24715_i32 => _INDEPENDENCE_DAY,
    24745_i32 => _TEACHERS_AND_INSTRUCTORS_DAY,
    24783_i32 => _EID_AL_FITR__ESTIMATED_,
    24784_i32 => _EID_AL_FITR__OBSERVED__ESTIMATED_,
    24813_i32 => _CONSTITUTION_DAY,
    24837_i32 => _NEW_YEAR_S_DAY,
    24852_i32 => _EID_AL_ADHA__ESTIMATED_,
    24854_i32 => _EID_AL_ADHA__OBSERVED__ESTIMATED_,
    24903_i32 => _WOMEN_S_DAY,
    24916_i32 => _NOWRUZ,
    24917_i32 => _NOWRUZ__OBSERVED_,
    24965_i32 => _DAY_OF_MEMORY_AND_HONOR,
    24966_i32 => _DAY_OF_MEMORY_AND_HONOR__OBSERVED_,
    25080_i32 => _INDEPENDENCE_DAY,
    25110_i32 => _TEACHERS_AND_INSTRUCTORS_DAY,
    25138_i32 => _EID_AL_FITR__ESTIMATED_,
    25178_i32 => _CONSTITUTION_DAY,
    25202_i32 => _NEW_YEAR_S_DAY,
    25204_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    25206_i32 => _EID_AL_ADHA__ESTIMATED_,
    25268_i32 => _WOMEN_S_DAY,
    25281_i32 => _NOWRUZ,
    25330_i32 => _DAY_OF_MEMORY_AND_HONOR,
    25445_i32 => _INDEPENDENCE_DAY,
    25475_i32 => _TEACHERS_AND_INSTRUCTORS_DAY,
    25477_i32 => _TEACHERS_AND_INSTRUCTORS_DAY__OBSERVED_,
    25493_i32 => _EID_AL_FITR__ESTIMATED_,
    25543_i32 => _CONSTITUTION_DAY,
    25561_i32 => _EID_AL_ADHA__ESTIMATED_,
    25567_i32 => _NEW_YEAR_S_DAY,
    25568_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    25634_i32 => _WOMEN_S_DAY,
    25647_i32 => _NOWRUZ,
    25696_i32 => _DAY_OF_MEMORY_AND_HONOR,
    25811_i32 => _INDEPENDENCE_DAY,
    25813_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    25841_i32 => _TEACHERS_AND_INSTRUCTORS_DAY,
    25847_i32 => _EID_AL_FITR__ESTIMATED_,
    25848_i32 => _EID_AL_FITR__OBSERVED__ESTIMATED_,
    25909_i32 => _CONSTITUTION_DAY,
    25911_i32 => _CONSTITUTION_DAY__OBSERVED_,
    25915_i32 => _EID_AL_ADHA__ESTIMATED_,
    25933_i32 => _NEW_YEAR_S_DAY,
    25999_i32 => _WOMEN_S_DAY,
    26012_i32 => _NOWRUZ,
    26061_i32 => _DAY_OF_MEMORY_AND_HONOR,
    26176_i32 => _INDEPENDENCE_DAY,
    26177_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    26201_i32 => _EID_AL_FITR__ESTIMATED_,
    26206_i32 => _TEACHERS_AND_INSTRUCTORS_DAY,
    26270_i32 => _EID_AL_ADHA__ESTIMATED_,
    26274_i32 => _CONSTITUTION_DAY,
    26275_i32 => _CONSTITUTION_DAY__OBSERVED_,
    26298_i32 => _NEW_YEAR_S_DAY,
    26364_i32 => _WOMEN_S_DAY,
    26366_i32 => _WOMEN_S_DAY__OBSERVED_,
    26377_i32 => _NOWRUZ,
    26426_i32 => _DAY_OF_MEMORY_AND_HONOR,
    26541_i32 => _INDEPENDENCE_DAY,
    26555_i32 => _EID_AL_FITR__ESTIMATED_,
    26571_i32 => _TEACHERS_AND_INSTRUCTORS_DAY,
    26624_i32 => _EID_AL_ADHA__ESTIMATED_,
    26625_i32 => _EID_AL_ADHA__OBSERVED__ESTIMATED_,
    26639_i32 => _CONSTITUTION_DAY,
    26663_i32 => _NEW_YEAR_S_DAY,
    26729_i32 => _WOMEN_S_DAY,
    26730_i32 => _WOMEN_S_DAY__OBSERVED_,
    26742_i32 => _NOWRUZ,
    26744_i32 => _NOWRUZ__OBSERVED_,
    26791_i32 => _DAY_OF_MEMORY_AND_HONOR,
    26793_i32 => _DAY_OF_MEMORY_AND_HONOR__OBSERVED_,
    26906_i32 => _INDEPENDENCE_DAY,
    26909_i32 => _EID_AL_FITR__ESTIMATED_,
    26936_i32 => _TEACHERS_AND_INSTRUCTORS_DAY,
    26978_i32 => _EID_AL_ADHA__ESTIMATED_,
    27004_i32 => _CONSTITUTION_DAY,
    27028_i32 => _NEW_YEAR_S_DAY,
    27095_i32 => _WOMEN_S_DAY,
    27108_i32 => _NOWRUZ,
    27157_i32 => _DAY_OF_MEMORY_AND_HONOR,
    27264_i32 => _EID_AL_FITR__ESTIMATED_,
    27272_i32 => _INDEPENDENCE_DAY,
    27302_i32 => _TEACHERS_AND_INSTRUCTORS_DAY,
    27304_i32 => _TEACHERS_AND_INSTRUCTORS_DAY__OBSERVED_,
    27332_i32 => _EID_AL_ADHA__ESTIMATED_,
    27370_i32 => _CONSTITUTION_DAY,
    27394_i32 => _NEW_YEAR_S_DAY,
    27395_i32 => _NEW_YEAR_S_DAY__OBSERVED_,
    27460_i32 => _WOMEN_S_DAY,
    27473_i32 => _NOWRUZ,
    27522_i32 => _DAY_OF_MEMORY_AND_HONOR,
    27619_i32 => _EID_AL_FITR__ESTIMATED_,
    27637_i32 => _INDEPENDENCE_DAY,
    27667_i32 => _TEACHERS_AND_INSTRUCTORS_DAY,
    27668_i32 => _TEACHERS_AND_INSTRUCTORS_DAY__OBSERVED_,
    27687_i32 => _EID_AL_ADHA__ESTIMATED_,
    27689_i32 => _EID_AL_ADHA__OBSERVED__ESTIMATED_,
    27735_i32 => _CONSTITUTION_DAY,
    27759_i32 => _NEW_YEAR_S_DAY,
    27825_i32 => _WOMEN_S_DAY,
    27838_i32 => _NOWRUZ,
    27887_i32 => _DAY_OF_MEMORY_AND_HONOR,
    27973_i32 => _EID_AL_FITR__ESTIMATED_,
    28002_i32 => _INDEPENDENCE_DAY,
    28004_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    28032_i32 => _TEACHERS_AND_INSTRUCTORS_DAY,
    28041_i32 => _EID_AL_ADHA__ESTIMATED_,
    28100_i32 => _CONSTITUTION_DAY,
    28102_i32 => _CONSTITUTION_DAY__OBSERVED_,
    28124_i32 => _NEW_YEAR_S_DAY,
    28190_i32 => _WOMEN_S_DAY,
    28203_i32 => _NOWRUZ,
    28252_i32 => _DAY_OF_MEMORY_AND_HONOR,
    28328_i32 => _EID_AL_FITR__ESTIMATED_,
    28367_i32 => _INDEPENDENCE_DAY,
    28368_i32 => _INDEPENDENCE_DAY__OBSERVED_,
    28396_i32 => _EID_AL_ADHA__ESTIMATED_,
    28397_i32 => _TEACHERS_AND_INSTRUCTORS_DAY,
    28465_i32 => _CONSTITUTION_DAY,
    28466_i32 => _CONSTITUTION_DAY__OBSERVED_,
    28489_i32 => _NEW_YEAR_S_DAY,
    28556_i32 => _WOMEN_S_DAY,
    28557_i32 => _WOMEN_S_DAY__OBSERVED_,
    28569_i32 => _NOWRUZ,
    28571_i32 => _NOWRUZ__OBSERVED_,
    28618_i32 => _DAY_OF_MEMORY_AND_HONOR,
    28620_i32 => _DAY_OF_MEMORY_AND_HONOR__OBSERVED_,
    28682_i32 => _EID_AL_FITR__ESTIMATED_,
    28683_i32 => _EID_AL_FITR__OBSERVED__ESTIMATED_,
    28733_i32 => _INDEPENDENCE_DAY,
    28751_i32 => _EID_AL_ADHA__ESTIMATED_,
    28753_i32 => _EID_AL_ADHA__OBSERVED__ESTIMATED_,
    28763_i32 => _TEACHERS_AND_INSTRUCTORS_DAY,
    28831_i32 => _CONSTITUTION_DAY,
    28855_i32 => _NEW_YEAR_S_DAY,
    28921_i32 => _WOMEN_S_DAY,
    28934_i32 => _NOWRUZ,
    28935_i32 => _NOWRUZ__OBSERVED_,
    28983_i32 => _DAY_OF_MEMORY_AND_HONOR,
    28984_i32 => _DAY_OF_MEMORY_AND_HONOR__OBSERVED_,
    29036_i32 => _EID_AL_FITR__ESTIMATED_,
    29098_i32 => _INDEPENDENCE_DAY,
    29105_i32 => _EID_AL_ADHA__ESTIMATED_,
    29128_i32 => _TEACHERS_AND_INSTRUCTORS_DAY,
    29196_i32 => _CONSTITUTION_DAY,
    29220_i32 => _NEW_YEAR_S_DAY,
};
