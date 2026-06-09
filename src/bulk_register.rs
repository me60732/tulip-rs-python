//! Bulk registration module for all TulipRS indicators
//!
//! This module provides a centralized way to register all indicator modules
//! using their auto-registration functions. This eliminates the need to
//! manually add each indicator to lib.rs.

use pyo3::types::PyModule;
use pyo3::PyResult;

/// Register all indicator modules in alphabetical order
///
/// This function calls the auto-registration function for every indicator,
/// creating submodules for each one under the main indicators module.
///
/// # Arguments
/// * `indicators_module` - The parent indicators module to register submodules under
///
/// # Returns
/// * `PyResult<()>` - Success or error from registration process
pub fn register_all_indicator_modules(
    indicators_module: &pyo3::Bound<'_, PyModule>,
) -> PyResult<()> {
    // A-D indicators
    crate::indicators::ad::register_ad_module(indicators_module)?;
    crate::indicators::adosc::register_adosc_module(indicators_module)?;
    crate::indicators::adx::register_adx_module(indicators_module)?;
    crate::indicators::adxr::register_adxr_module(indicators_module)?;
    crate::indicators::ao::register_ao_module(indicators_module)?;
    crate::indicators::apo::register_apo_module(indicators_module)?;
    crate::indicators::aroon::register_aroon_module(indicators_module)?;
    crate::indicators::aroonosc::register_aroonosc_module(indicators_module)?;
    crate::indicators::atr::register_atr_module(indicators_module)?;
    crate::indicators::avgprice::register_avgprice_module(indicators_module)?;
    crate::indicators::bbands::register_bbands_module(indicators_module)?;
    crate::indicators::bop::register_bop_module(indicators_module)?;
    crate::indicators::candlestick::register_candlestick_module(indicators_module)?;
    crate::indicators::cci::register_cci_module(indicators_module)?;
    crate::indicators::chaikinmf::register_chaikinmf_module(indicators_module)?;
    crate::indicators::chandelierexit::register_chandelierexit_module(indicators_module)?;
    crate::indicators::cmo::register_cmo_module(indicators_module)?;
    crate::indicators::cvi::register_cvi_module(indicators_module)?;
    crate::indicators::dema::register_dema_module(indicators_module)?;
    crate::indicators::di::register_di_module(indicators_module)?;
    crate::indicators::dm::register_dm_module(indicators_module)?;
    crate::indicators::donchianchannel::register_donchianchannel_module(indicators_module)?;
    crate::indicators::dpo::register_dpo_module(indicators_module)?;
    crate::indicators::dx::register_dx_module(indicators_module)?;
    // E-L indicators
    crate::indicators::ef::register_ef_module(indicators_module)?;
    crate::indicators::elderray::register_elderray_module(indicators_module)?;
    crate::indicators::ema::register_ema_module(indicators_module)?;
    crate::indicators::emv::register_emv_module(indicators_module)?;
    crate::indicators::fisher::register_fisher_module(indicators_module)?;
    crate::indicators::fosc::register_fosc_module(indicators_module)?;
    crate::indicators::hma::register_hma_module(indicators_module)?;
    crate::indicators::kama::register_kama_module(indicators_module)?;
    crate::indicators::keltnerchannel::register_keltnerchannel_module(indicators_module)?;
    crate::indicators::kvo::register_kvo_module(indicators_module)?;
    crate::indicators::linreg::register_linreg_module(indicators_module)?;

    // M-P indicators
    crate::indicators::macd::register_macd_module(indicators_module)?;
    crate::indicators::marketfi::register_marketfi_module(indicators_module)?;
    crate::indicators::mass::register_mass_module(indicators_module)?;
    crate::indicators::max::register_max_module(indicators_module)?;
    crate::indicators::md::register_md_module(indicators_module)?;
    crate::indicators::medprice::register_medprice_module(indicators_module)?;
    crate::indicators::mfi::register_mfi_module(indicators_module)?;
    crate::indicators::min::register_min_module(indicators_module)?;
    crate::indicators::mom::register_mom_module(indicators_module)?;
    crate::indicators::msw::register_msw_module(indicators_module)?;
    crate::indicators::natr::register_natr_module(indicators_module)?;
    crate::indicators::nvi::register_nvi_module(indicators_module)?;
    crate::indicators::obv::register_obv_module(indicators_module)?;
    crate::indicators::pivotpoint::register_pivotpoint_module(indicators_module)?;
    crate::indicators::ppo::register_ppo_module(indicators_module)?;
    crate::indicators::psar::register_psar_module(indicators_module)?;
    crate::indicators::pvi::register_pvi_module(indicators_module)?;

    // Q-S indicators
    crate::indicators::qstick::register_qstick_module(indicators_module)?;
    crate::indicators::roc::register_roc_module(indicators_module)?;
    crate::indicators::rocr::register_rocr_module(indicators_module)?;
    crate::indicators::rsi::register_rsi_module(indicators_module)?;
    crate::indicators::sma::register_sma_module(indicators_module)?;
    crate::indicators::smaenvelope::register_smaenvelope_module(indicators_module)?;
    crate::indicators::stddev::register_stddev_module(indicators_module)?;
    crate::indicators::stoch::register_stoch_module(indicators_module)?;
    crate::indicators::stochrsi::register_stochrsi_module(indicators_module)?;

    // T-Z indicators
    crate::indicators::tema::register_tema_module(indicators_module)?;
    crate::indicators::tr::register_tr_module(indicators_module)?;
    crate::indicators::trvi::register_trvi_module(indicators_module)?;
    crate::indicators::trima::register_trima_module(indicators_module)?;
    crate::indicators::trix::register_trix_module(indicators_module)?;
    crate::indicators::tsf::register_tsf_module(indicators_module)?;
    crate::indicators::typprice::register_typprice_module(indicators_module)?;
    crate::indicators::ultosc::register_ultosc_module(indicators_module)?;
    crate::indicators::vhf::register_vhf_module(indicators_module)?;
    crate::indicators::vidya::register_vidya_module(indicators_module)?;
    crate::indicators::volatility::register_volatility_module(indicators_module)?;
    crate::indicators::vortex::register_vortex_module(indicators_module)?;
    crate::indicators::vosc::register_vosc_module(indicators_module)?;
    crate::indicators::vwma::register_vwma_module(indicators_module)?;
    crate::indicators::wad::register_wad_module(indicators_module)?;
    crate::indicators::wcprice::register_wcprice_module(indicators_module)?;
    crate::indicators::wilders::register_wilders_module(indicators_module)?;
    crate::indicators::willr::register_willr_module(indicators_module)?;
    crate::indicators::wma::register_wma_module(indicators_module)?;
    crate::indicators::zlema::register_zlema_module(indicators_module)?;

    Ok(())
}

// Get a count of all registered indicators
/*pub fn get_indicator_count() -> usize {
    70 // Total number of indicators with auto-registration
}

/// Get list of all indicator names in alphabetical order
pub fn get_indicator_names() -> Vec<&'static str> {
    vec![
        "ad",
        "adosc",
        "adx",
        "adxr",
        "ao",
        "apo",
        "aroon",
        "aroonosc",
        "atr",
        "avgprice",
        "bbands",
        "bop",
        "cci",
        "cmo",
        "cvi",
        "dema",
        "di",
        "dm",
        "dpo",
        "dx",
        "ema",
        "emv",
        "fisher",
        "fosc",
        "hma",
        "kama",
        "kvo",
        "linreg",
        "macd",
        "marketfi",
        "mass",
        "max",
        "md",
        "medprice",
        "mfi",
        "min",
        "mom",
        "msw",
        "natr",
        "nvi",
        "obv",
        "pivotpoint",
        "ppo",
        "psar",
        "pvi",
        "qstick",
        "roc",
        "rocr",
        "rsi",
        "sma",
        "stddev",
        "stoch",
        "stochrsi",
        "tema",
        "tr",
        "trima",
        "trix",
        "tsf",
        "typprice",
        "ultosc",
        "vhf",
        "vidya",
        "volatility",
        "vosc",
        "vwma",
        "wad",
        "wcprice",
        "wilders",
        "willr",
        "wma",
        "zlema",
    ]
}*/
