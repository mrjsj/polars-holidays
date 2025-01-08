mod ad_holidays;
mod ae_holidays;
mod af_holidays;
mod al_holidays;
mod am_holidays;
mod ao_holidays;
mod ar_holidays;
mod as_holidays;
mod at_holidays;
mod au_holidays;
mod aw_holidays;
mod az_holidays;
mod ba_holidays;
mod bb_holidays;
mod bd_holidays;
mod be_holidays;
mod bf_holidays;
mod bg_holidays;
mod bh_holidays;
mod bi_holidays;
mod bn_holidays;
mod bo_holidays;
mod br_holidays;
mod bs_holidays;
mod bw_holidays;
mod by_holidays;
mod bz_holidays;
mod ca_holidays;
mod cg_holidays;
mod ch_holidays;
mod cl_holidays;
mod cm_holidays;
mod cn_holidays;
mod co_holidays;
mod cr_holidays;
mod cu_holidays;
mod cw_holidays;
mod cy_holidays;
mod cz_holidays;
mod de_holidays;
mod dj_holidays;
mod dk_holidays;
mod dm_holidays;
mod do_holidays;
mod dz_holidays;
mod ec_holidays;
mod ee_holidays;
mod eg_holidays;
mod es_holidays;
mod et_holidays;
mod fi_holidays;
mod fr_holidays;
mod ga_holidays;
mod gb_holidays;
mod ge_holidays;
mod gg_holidays;
mod gh_holidays;
mod gl_holidays;
mod gr_holidays;
mod gt_holidays;
mod gu_holidays;
mod hk_holidays;
mod hn_holidays;
mod hr_holidays;
mod ht_holidays;
mod hu_holidays;
mod id_holidays;
mod ie_holidays;
mod il_holidays;
mod im_holidays;
mod in_holidays;
mod ir_holidays;
mod is_holidays;
mod it_holidays;
mod je_holidays;
mod jm_holidays;
mod jo_holidays;
mod jp_holidays;
mod ke_holidays;
mod kg_holidays;
mod kh_holidays;
mod kn_holidays;
mod kr_holidays;
mod kw_holidays;
mod kz_holidays;
mod la_holidays;
mod li_holidays;
mod ls_holidays;
mod lt_holidays;
mod lu_holidays;
mod lv_holidays;
mod ma_holidays;
mod mc_holidays;
mod md_holidays;
mod me_holidays;
mod mg_holidays;
mod mh_holidays;
mod mk_holidays;
mod mp_holidays;
mod mr_holidays;
mod mt_holidays;
mod mv_holidays;
mod mw_holidays;
mod mx_holidays;
mod my_holidays;
mod mz_holidays;
mod na_holidays;
mod ng_holidays;
mod ni_holidays;
mod nl_holidays;
mod no_holidays;
mod nz_holidays;
mod pa_holidays;
mod pe_holidays;
mod pg_holidays;
mod ph_holidays;
mod pk_holidays;
mod pl_holidays;
mod pr_holidays;
mod pt_holidays;
mod pw_holidays;
mod py_holidays;
mod ro_holidays;
mod rs_holidays;
mod ru_holidays;
mod sa_holidays;
mod sc_holidays;
mod se_holidays;
mod sg_holidays;
mod si_holidays;
mod sk_holidays;
mod sm_holidays;
mod sv_holidays;
mod sz_holidays;
mod td_holidays;
mod th_holidays;
mod tl_holidays;
mod tn_holidays;
mod to_holidays;
mod tr_holidays;
mod tw_holidays;
mod tz_holidays;
mod ua_holidays;
mod uk_holidays;
mod um_holidays;
mod us_holidays;
mod uy_holidays;
mod uz_holidays;
mod va_holidays;
mod ve_holidays;
mod vi_holidays;
mod vn_holidays;
mod vu_holidays;
mod ws_holidays;
mod za_holidays;
mod zm_holidays;
mod zw_holidays;

use phf::phf_map;

mod constants;

pub static HOLIDAYS: phf::Map<&'static str, &'static phf::Map<i32, &'static str>> = phf_map! {
    "af" => &af_holidays::AF_HOLIDAYS,
    "al" => &al_holidays::AL_HOLIDAYS,
    "dz" => &dz_holidays::DZ_HOLIDAYS,
    "as" => &as_holidays::AS_HOLIDAYS,
    "ad" => &ad_holidays::AD_HOLIDAYS,
    "ao" => &ao_holidays::AO_HOLIDAYS,
    "ar" => &ar_holidays::AR_HOLIDAYS,
    "am" => &am_holidays::AM_HOLIDAYS,
    "aw" => &aw_holidays::AW_HOLIDAYS,
    "au" => &au_holidays::AU_HOLIDAYS,
    "at" => &at_holidays::AT_HOLIDAYS,
    "az" => &az_holidays::AZ_HOLIDAYS,
    "bs" => &bs_holidays::BS_HOLIDAYS,
    "bh" => &bh_holidays::BH_HOLIDAYS,
    "bd" => &bd_holidays::BD_HOLIDAYS,
    "bb" => &bb_holidays::BB_HOLIDAYS,
    "by" => &by_holidays::BY_HOLIDAYS,
    "be" => &be_holidays::BE_HOLIDAYS,
    "bz" => &bz_holidays::BZ_HOLIDAYS,
    "bo" => &bo_holidays::BO_HOLIDAYS,
    "ba" => &ba_holidays::BA_HOLIDAYS,
    "bw" => &bw_holidays::BW_HOLIDAYS,
    "br" => &br_holidays::BR_HOLIDAYS,
    "bn" => &bn_holidays::BN_HOLIDAYS,
    "bg" => &bg_holidays::BG_HOLIDAYS,
    "bf" => &bf_holidays::BF_HOLIDAYS,
    "bi" => &bi_holidays::BI_HOLIDAYS,
    "kh" => &kh_holidays::KH_HOLIDAYS,
    "cm" => &cm_holidays::CM_HOLIDAYS,
    "ca" => &ca_holidays::CA_HOLIDAYS,
    "td" => &td_holidays::TD_HOLIDAYS,
    "cl" => &cl_holidays::CL_HOLIDAYS,
    "cn" => &cn_holidays::CN_HOLIDAYS,
    "co" => &co_holidays::CO_HOLIDAYS,
    "cg" => &cg_holidays::CG_HOLIDAYS,
    "cr" => &cr_holidays::CR_HOLIDAYS,
    "hr" => &hr_holidays::HR_HOLIDAYS,
    "cu" => &cu_holidays::CU_HOLIDAYS,
    "cw" => &cw_holidays::CW_HOLIDAYS,
    "cy" => &cy_holidays::CY_HOLIDAYS,
    "cz" => &cz_holidays::CZ_HOLIDAYS,
    "dk" => &dk_holidays::DK_HOLIDAYS,
    "dj" => &dj_holidays::DJ_HOLIDAYS,
    "dm" => &dm_holidays::DM_HOLIDAYS,
    "do" => &do_holidays::DO_HOLIDAYS,
    "ec" => &ec_holidays::EC_HOLIDAYS,
    "eg" => &eg_holidays::EG_HOLIDAYS,
    "jo" => &jo_holidays::JO_HOLIDAYS,
    "sv" => &sv_holidays::SV_HOLIDAYS,
    "ee" => &ee_holidays::EE_HOLIDAYS,
    "sz" => &sz_holidays::SZ_HOLIDAYS,
    "et" => &et_holidays::ET_HOLIDAYS,
    "fi" => &fi_holidays::FI_HOLIDAYS,
    "fr" => &fr_holidays::FR_HOLIDAYS,
    "ga" => &ga_holidays::GA_HOLIDAYS,
    "ge" => &ge_holidays::GE_HOLIDAYS,
    "de" => &de_holidays::DE_HOLIDAYS,
    "gh" => &gh_holidays::GH_HOLIDAYS,
    "gr" => &gr_holidays::GR_HOLIDAYS,
    "gl" => &gl_holidays::GL_HOLIDAYS,
    "gu" => &gu_holidays::GU_HOLIDAYS,
    "gt" => &gt_holidays::GT_HOLIDAYS,
    "gg" => &gg_holidays::GG_HOLIDAYS,
    "ht" => &ht_holidays::HT_HOLIDAYS,
    "hn" => &hn_holidays::HN_HOLIDAYS,
    "hk" => &hk_holidays::HK_HOLIDAYS,
    "hu" => &hu_holidays::HU_HOLIDAYS,
    "is" => &is_holidays::IS_HOLIDAYS,
    "in" => &in_holidays::IN_HOLIDAYS,
    "id" => &id_holidays::ID_HOLIDAYS,
    "ir" => &ir_holidays::IR_HOLIDAYS,
    "ie" => &ie_holidays::IE_HOLIDAYS,
    "im" => &im_holidays::IM_HOLIDAYS,
    "il" => &il_holidays::IL_HOLIDAYS,
    "it" => &it_holidays::IT_HOLIDAYS,
    "jm" => &jm_holidays::JM_HOLIDAYS,
    "jp" => &jp_holidays::JP_HOLIDAYS,
    "je" => &je_holidays::JE_HOLIDAYS,
    "kz" => &kz_holidays::KZ_HOLIDAYS,
    "ke" => &ke_holidays::KE_HOLIDAYS,
    "kw" => &kw_holidays::KW_HOLIDAYS,
    "kg" => &kg_holidays::KG_HOLIDAYS,
    "la" => &la_holidays::LA_HOLIDAYS,
    "lv" => &lv_holidays::LV_HOLIDAYS,
    "ls" => &ls_holidays::LS_HOLIDAYS,
    "li" => &li_holidays::LI_HOLIDAYS,
    "lt" => &lt_holidays::LT_HOLIDAYS,
    "lu" => &lu_holidays::LU_HOLIDAYS,
    "mg" => &mg_holidays::MG_HOLIDAYS,
    "mw" => &mw_holidays::MW_HOLIDAYS,
    "my" => &my_holidays::MY_HOLIDAYS,
    "mv" => &mv_holidays::MV_HOLIDAYS,
    "mt" => &mt_holidays::MT_HOLIDAYS,
    "mh" => &mh_holidays::MH_HOLIDAYS,
    "mr" => &mr_holidays::MR_HOLIDAYS,
    "mx" => &mx_holidays::MX_HOLIDAYS,
    "md" => &md_holidays::MD_HOLIDAYS,
    "mc" => &mc_holidays::MC_HOLIDAYS,
    "me" => &me_holidays::ME_HOLIDAYS,
    "ma" => &ma_holidays::MA_HOLIDAYS,
    "mz" => &mz_holidays::MZ_HOLIDAYS,
    "na" => &na_holidays::NA_HOLIDAYS,
    "nl" => &nl_holidays::NL_HOLIDAYS,
    "nz" => &nz_holidays::NZ_HOLIDAYS,
    "ni" => &ni_holidays::NI_HOLIDAYS,
    "ng" => &ng_holidays::NG_HOLIDAYS,
    "mk" => &mk_holidays::MK_HOLIDAYS,
    "mp" => &mp_holidays::MP_HOLIDAYS,
    "no" => &no_holidays::NO_HOLIDAYS,
    "pk" => &pk_holidays::PK_HOLIDAYS,
    "pw" => &pw_holidays::PW_HOLIDAYS,
    "pa" => &pa_holidays::PA_HOLIDAYS,
    "pg" => &pg_holidays::PG_HOLIDAYS,
    "py" => &py_holidays::PY_HOLIDAYS,
    "pe" => &pe_holidays::PE_HOLIDAYS,
    "ph" => &ph_holidays::PH_HOLIDAYS,
    "pl" => &pl_holidays::PL_HOLIDAYS,
    "pt" => &pt_holidays::PT_HOLIDAYS,
    "pr" => &pr_holidays::PR_HOLIDAYS,
    "ro" => &ro_holidays::RO_HOLIDAYS,
    "ru" => &ru_holidays::RU_HOLIDAYS,
    "kn" => &kn_holidays::KN_HOLIDAYS,
    "ws" => &ws_holidays::WS_HOLIDAYS,
    "sm" => &sm_holidays::SM_HOLIDAYS,
    "sa" => &sa_holidays::SA_HOLIDAYS,
    "rs" => &rs_holidays::RS_HOLIDAYS,
    "sc" => &sc_holidays::SC_HOLIDAYS,
    "sg" => &sg_holidays::SG_HOLIDAYS,
    "sk" => &sk_holidays::SK_HOLIDAYS,
    "si" => &si_holidays::SI_HOLIDAYS,
    "za" => &za_holidays::ZA_HOLIDAYS,
    "kr" => &kr_holidays::KR_HOLIDAYS,
    "es" => &es_holidays::ES_HOLIDAYS,
    "se" => &se_holidays::SE_HOLIDAYS,
    "ch" => &ch_holidays::CH_HOLIDAYS,
    "tw" => &tw_holidays::TW_HOLIDAYS,
    "tz" => &tz_holidays::TZ_HOLIDAYS,
    "th" => &th_holidays::TH_HOLIDAYS,
    "tl" => &tl_holidays::TL_HOLIDAYS,
    "to" => &to_holidays::TO_HOLIDAYS,
    "tn" => &tn_holidays::TN_HOLIDAYS,
    "tr" => &tr_holidays::TR_HOLIDAYS,
    "ua" => &ua_holidays::UA_HOLIDAYS,
    "ae" => &ae_holidays::AE_HOLIDAYS,
    "gb" => &gb_holidays::GB_HOLIDAYS,
    "uk" => &uk_holidays::UK_HOLIDAYS,
    "um" => &um_holidays::UM_HOLIDAYS,
    "vi" => &vi_holidays::VI_HOLIDAYS,
    "us" => &us_holidays::US_HOLIDAYS,
    "uy" => &uy_holidays::UY_HOLIDAYS,
    "uz" => &uz_holidays::UZ_HOLIDAYS,
    "vu" => &vu_holidays::VU_HOLIDAYS,
    "va" => &va_holidays::VA_HOLIDAYS,
    "ve" => &ve_holidays::VE_HOLIDAYS,
    "vn" => &vn_holidays::VN_HOLIDAYS,
    "zm" => &zm_holidays::ZM_HOLIDAYS,
    "zw" => &zw_holidays::ZW_HOLIDAYS,
};
