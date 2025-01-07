import holidays
from datetime import date, timedelta

import re

EPOCH = date(1970,1,1)
START_DATE = date(2000,1,1)
END_DATE = date(2050,1,1)

COUNTRY_LIST = holidays.list_supported_countries().keys()


MOD_MOD_TEMPLATE = "mod {country_code}_holidays;"
MOD_USE_TEMPLATE = "pub use {country_code}_holidays::{country_code_upper}_HOLIDAYS;"


HOLIDAYS_START = """use phf::phf_map;
use crate::countries::constants::*;

pub static {country_code}_HOLIDAYS: phf::Map<i32, &'static str> = phf_map!{{
"""

HOLIDAYS_END = """};\n"""

holidays.list_supported_countries()

distinct_holidays = {}

def generate_holidays(country_code: str):

    country_holidays = holidays.country_holidays(country_code)

    holidays_map = {}
    current_date = START_DATE
    while current_date <= END_DATE:
        
        if current_date not in country_holidays:
            current_date = current_date + timedelta(days=1)

            continue
        
        holidays_map[(current_date-EPOCH).days] = country_holidays.get(current_date)


        current_date = current_date + timedelta(days=1)

    with open("./src/countries/mod.rs", mode="a") as f:
        f.write(MOD_MOD_TEMPLATE.format(country_code=country_code.lower()) + "\n")
        # f.write(MOD_USE_TEMPLATE.format(country_code=country_code.lower(), country_code_upper=country_code.upper()) + "\n")


    with open(f"./src/countries/{country_code.lower()}_holidays.rs", mode="w") as f:
        f.write(HOLIDAYS_START.format(country_code=country_code.upper()))
        for holiday_date, holiday_name in holidays_map.items():

            replace_non_alpha_numeric = "[^a-zA-Z0-9]"
            holiday_const = "_" + re.sub(replace_non_alpha_numeric, "_", holiday_name).upper()

            if holiday_const not in distinct_holidays:
                distinct_holidays[holiday_const] = holiday_name

            f.write(f"\t{holiday_date}_i32 => {holiday_const},\n")
        f.write(HOLIDAYS_END)


for country in COUNTRY_LIST:
    generate_holidays(country)

with open("./src/countries/mod.rs", "a") as f:

    f.write("\n" + "use phf::phf_map;" + "\n\n")
    f.write("\n" + "mod constants;" + "\n\n")
    f.write("""pub static HOLIDAYS: phf::Map<&'static str, &'static phf::Map<i32, &'static str>> = phf_map!{""" + "\n")
    for country in COUNTRY_LIST:
        f.write(f"    \"{country.lower()}\" => &{country.lower()}_holidays::{country.upper()}_HOLIDAYS," + "\n")
    f.write("};" + "\n")

with open("./src/countries/constants.rs", mode="w") as f:

    f.write("#![allow(non_upper_case_globals)]" + "\n")

    for holiday_const, holiday_name in sorted(distinct_holidays.items()):
        f.write(f"pub static {holiday_const}: &str = \"{holiday_name}\";" + "\n")