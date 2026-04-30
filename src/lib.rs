use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

mod indicators;
mod utils;

/// List all available indicators
#[pyfunction]
fn list_indicators() -> Vec<&'static str> {
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
}

/// Python module for TulipRS
#[pymodule]
fn tulip_rs(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Module metadata
    m.add("__version__", "0.1.0")?;
    m.add("__author__", "TulipRS Contributors")?;
    m.add(
        "__description__",
        "Python bindings for TulipRS Technical Analysis Library",
    )?;

    // Create indicators submodule
    let indicators_module = PyModule::new(py, "indicators")?;

    // Add AD indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::ad_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(indicators::ad_info, &indicators_module)?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::ad_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::ad_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::ad_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::AdState>()?;

    // Add ADOSC indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::adosc_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::adosc_info,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::adosc_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::adosc_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::adosc_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::AdoscState>()?;

    // Add ADX indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::adx_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(indicators::adx_info, &indicators_module)?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::adx_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::adx_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::adx_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::AdxState>()?;

    // Add ADXR indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::adxr_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(indicators::adxr_info, &indicators_module)?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::adxr_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::adxr_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::adxr_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::AdxrState>()?;

    // Add AO indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::ao_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(indicators::ao_info, &indicators_module)?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::ao_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::ao_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::ao_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::AoState>()?;

    // Add APO indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::apo_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(indicators::apo_info, &indicators_module)?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::apo_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::apo_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::apo_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::ApoState>()?;

    // Add AROON indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::aroon_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::aroon_info,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::aroon_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::aroon_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::aroon_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::AroonState>()?;

    // Add AROONOSC indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::aroonosc_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::aroonosc_info,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::aroonosc_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::aroonosc_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::aroonosc_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::AroonoscState>()?;

    // Add ATR indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::atr_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(indicators::atr_info, &indicators_module)?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::atr_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::atr_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::atr_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::AtrState>()?;

    // Add AVGPRICE indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::avgprice_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::avgprice_info,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::avgprice_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::avgprice_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::avgprice_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::AvgpriceState>()?;

    // Add BBANDS indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::bbands_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::bbands_info,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::bbands_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::bbands_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::bbands_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::BbandsState>()?;

    // Add BOP indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::bop_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(indicators::bop_info, &indicators_module)?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::bop_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::bop_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::bop_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::BopState>()?;

    // Add CCI indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::cci_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(indicators::cci_info, &indicators_module)?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::cci_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::cci_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::cci_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::CciState>()?;

    // Add CMO indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::cmo_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(indicators::cmo_info, &indicators_module)?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::cmo_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::cmo_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::cmo_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::CmoState>()?;

    // Add CVI indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::cvi_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(indicators::cvi_info, &indicators_module)?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::cvi_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::cvi_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::cvi_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::CviState>()?;

    // Add DEMA indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::dema_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(indicators::dema_info, &indicators_module)?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::dema_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::dema_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::dema_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::DemaState>()?;

    // Add DI indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::di_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(indicators::di_info, &indicators_module)?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::di_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::di_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::di_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::DiState>()?;

    // Add DM indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::dm_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(indicators::dm_info, &indicators_module)?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::dm_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::dm_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::dm_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::DmState>()?;

    // Add DPO indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::dpo_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(indicators::dpo_info, &indicators_module)?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::dpo_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::dpo_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::dpo_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::DpoState>()?;

    // Add DX indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::dx_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(indicators::dx_info, &indicators_module)?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::dx_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::dx_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::dx_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::DxState>()?;

    // Add EMA indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::ema_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(indicators::ema_info, &indicators_module)?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::ema_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::ema_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::ema_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::EmaState>()?;

    // Add EMV indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::emv_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(indicators::emv_info, &indicators_module)?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::emv_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::emv_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::emv_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::EmvState>()?;

    // Add FISHER indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::fisher_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::fisher_info,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::fisher_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::fisher_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::fisher_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::FisherState>()?;

    // Add FOSC indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::fosc_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(indicators::fosc_info, &indicators_module)?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::fosc_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::fosc_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::fosc_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::FoscState>()?;

    // Add HMA indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::hma_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(indicators::hma_info, &indicators_module)?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::hma_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::hma_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::hma_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::HmaState>()?;

    // Add KAMA indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::kama_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(indicators::kama_info, &indicators_module)?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::kama_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::kama_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::kama_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::KamaState>()?;

    // Add KVO indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::kvo_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(indicators::kvo_info, &indicators_module)?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::kvo_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::kvo_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::kvo_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::KvoState>()?;

    // Add LINREG indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::linreg_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::linreg_info,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::linreg_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::linreg_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::linreg_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::LinregState>()?;

    // Add MACD indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::macd_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(indicators::macd_info, &indicators_module)?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::macd_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::macd_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::macd_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::MacdState>()?;

    // Add MARKETFI indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::marketfi_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::marketfi_info,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::marketfi_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::marketfi_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::marketfi_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::MarketfiState>()?;

    // Add MASS indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::mass_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(indicators::mass_info, &indicators_module)?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::mass_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::mass_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::mass_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::MassState>()?;

    // Add MAX indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::max_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(indicators::max_info, &indicators_module)?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::max_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::max_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::max_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::MaxState>()?;

    // Add MD indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::md_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(indicators::md_info, &indicators_module)?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::md_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::md_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::md_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::MdState>()?;

    // Add MEDPRICE indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::medprice_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::medprice_info,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::medprice_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::medprice_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::medprice_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::MedpriceState>()?;

    // Add MFI indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::mfi_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(indicators::mfi_info, &indicators_module)?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::mfi_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::mfi_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::mfi_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::MfiState>()?;

    // Add MIN indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::min_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(indicators::min_info, &indicators_module)?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::min_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::min_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::min_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::MinState>()?;

    // Add MOM indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::mom_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(indicators::mom_info, &indicators_module)?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::mom_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::mom_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::mom_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::MomState>()?;

    // Add MSW indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::msw_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(indicators::msw_info, &indicators_module)?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::msw_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::msw_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::msw_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::MswState>()?;

    // Add NATR indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::natr_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(indicators::natr_info, &indicators_module)?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::natr_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::natr_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::natr_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::NatrState>()?;

    // Add NVI indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::nvi_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(indicators::nvi_info, &indicators_module)?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::nvi_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::nvi_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::nvi_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::NviState>()?;

    // Add OBV indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::obv_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(indicators::obv_info, &indicators_module)?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::obv_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::obv_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::obv_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::ObvState>()?;

    // Add PIVOTPOINT indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::pivotpoint_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::pivotpoint_info,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::pivotpoint_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::pivotpoint_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::pivotpoint_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::PivotpointState>()?;

    // Add PPO indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::ppo_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(indicators::ppo_info, &indicators_module)?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::ppo_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::ppo_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::ppo_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::PpoState>()?;

    // Add PSAR indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::psar_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(indicators::psar_info, &indicators_module)?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::psar_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::psar_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::psar_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::PsarState>()?;

    // Add PVI indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::pvi_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(indicators::pvi_info, &indicators_module)?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::pvi_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::pvi_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::pvi_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::PviState>()?;

    // Add QSTICK indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::qstick_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::qstick_info,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::qstick_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::qstick_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::qstick_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::QstickState>()?;

    // Add RANGE indicator functions
    // ROC functions
    // Add ROC indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::roc_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(indicators::roc_info, &indicators_module)?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::roc_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::roc_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::roc_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::RocState>()?;

    // Add ROCR indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::rocr_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(indicators::rocr_info, &indicators_module)?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::rocr_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::rocr_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::rocr_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::RocrState>()?;

    // Add RSI indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::rsi_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(indicators::rsi_info, &indicators_module)?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::rsi_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::rsi_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::rsi_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::RsiState>()?;

    // Add SMA indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::sma_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(indicators::sma_info, &indicators_module)?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::sma_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::sma_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::sma_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::SmaState>()?;

    // Add STDDEV indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::stddev_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::stddev_info,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::stddev_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::stddev_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::stddev_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::StddevState>()?;

    // Add STOCH indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::stoch_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::stoch_info,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::stoch_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::stoch_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::stoch_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::StochState>()?;

    // Add STOCHRSI indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::stochrsi_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::stochrsi_info,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::stochrsi_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::stochrsi_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::stochrsi_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::StochrsiState>()?;

    // Add TEMA indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::tema_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(indicators::tema_info, &indicators_module)?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::tema_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::tema_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::tema_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::TemaState>()?;

    // Add TR indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::tr_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(indicators::tr_info, &indicators_module)?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::tr_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::tr_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::tr_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::TrState>()?;

    // Add TRIMA indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::trima_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::trima_info,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::trima_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::trima_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::trima_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::TrimaState>()?;

    // Add TRIX indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::trix_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(indicators::trix_info, &indicators_module)?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::trix_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::trix_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::trix_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::TrixState>()?;

    // Add TSF indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::tsf_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(indicators::tsf_info, &indicators_module)?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::tsf_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::tsf_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::tsf_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::TsfState>()?;

    // Add TYPPRICE indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::typprice_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::typprice_info,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::typprice_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::typprice_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::typprice_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::TyppriceState>()?;

    // Add ULTOSC indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::ultosc_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::ultosc_info,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::ultosc_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::ultosc_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::ultosc_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::UltoscState>()?;

    // Add VHF indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::vhf_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(indicators::vhf_info, &indicators_module)?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::vhf_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::vhf_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::vhf_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::VhfState>()?;

    // Add VIDYA indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::vidya_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::vidya_info,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::vidya_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::vidya_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::vidya_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::VidyaState>()?;

    // Add VOLATILITY indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::volatility_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::volatility_info,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::volatility_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::volatility_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::volatility_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::VolatilityState>()?;

    // Add VOSC indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::vosc_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(indicators::vosc_info, &indicators_module)?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::vosc_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::vosc_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::vosc_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::VoscState>()?;

    // Add VWMA indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::vwma_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(indicators::vwma_info, &indicators_module)?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::vwma_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::vwma_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::vwma_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::VwmaState>()?;

    // Add WAD indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::wad_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(indicators::wad_info, &indicators_module)?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::wad_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::wad_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::wad_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::WadState>()?;

    // Add WCPRICE indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::wcprice_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::wcprice_info,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::wcprice_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::wcprice_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::wcprice_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::WcpriceState>()?;

    // Add WILDERS indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::wilders_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::wilders_info,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::wilders_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::wilders_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::wilders_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::WildersState>()?;

    // Add WILLR indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::willr_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::willr_info,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::willr_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::willr_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::willr_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::WillrState>()?;

    // Add WMA indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::wma_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(indicators::wma_info, &indicators_module)?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::wma_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::wma_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::wma_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::WmaState>()?;

    // Add ZLEMA indicator functions
    indicators_module.add_function(wrap_pyfunction!(
        indicators::zlema_indicator,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::zlema_info,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::zlema_min_data,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::zlema_min_data_accuracy,
        &indicators_module
    )?)?;
    indicators_module.add_function(wrap_pyfunction!(
        indicators::zlema_output_length,
        &indicators_module
    )?)?;
    indicators_module.add_class::<indicators::ZlemaState>()?;

    // Create AD submodule for ad::indicator() access
    let ad_submodule = PyModule::new(py, "ad")?;
    ad_submodule.add_function(wrap_pyfunction!(indicators::ad_indicator, &ad_submodule)?)?;
    ad_submodule.add_function(wrap_pyfunction!(indicators::ad_info, &ad_submodule)?)?;
    ad_submodule.add_function(wrap_pyfunction!(indicators::ad_min_data, &ad_submodule)?)?;
    ad_submodule.add_function(wrap_pyfunction!(
        indicators::ad_min_data_accuracy,
        &ad_submodule
    )?)?;
    ad_submodule.add_function(wrap_pyfunction!(
        indicators::ad_output_length,
        &ad_submodule
    )?)?;
    ad_submodule.add_class::<indicators::AdState>()?;
    indicators_module.add_submodule(&ad_submodule)?;

    // Create ADOSC submodule for adosc::indicator() access
    let adosc_submodule = PyModule::new(py, "adosc")?;
    adosc_submodule.add_function(wrap_pyfunction!(
        indicators::adosc_indicator,
        &adosc_submodule
    )?)?;
    adosc_submodule.add_function(wrap_pyfunction!(indicators::adosc_info, &adosc_submodule)?)?;
    adosc_submodule.add_function(wrap_pyfunction!(
        indicators::adosc_min_data,
        &adosc_submodule
    )?)?;
    adosc_submodule.add_function(wrap_pyfunction!(
        indicators::adosc_min_data_accuracy,
        &adosc_submodule
    )?)?;
    adosc_submodule.add_function(wrap_pyfunction!(
        indicators::adosc_output_length,
        &adosc_submodule
    )?)?;
    adosc_submodule.add_class::<indicators::AdoscState>()?;
    indicators_module.add_submodule(&adosc_submodule)?;

    // Create ADX submodule for adx::indicator() access
    let adx_submodule = PyModule::new(py, "adx")?;
    adx_submodule.add_function(wrap_pyfunction!(indicators::adx_indicator, &adx_submodule)?)?;
    adx_submodule.add_function(wrap_pyfunction!(indicators::adx_info, &adx_submodule)?)?;
    adx_submodule.add_function(wrap_pyfunction!(indicators::adx_min_data, &adx_submodule)?)?;
    adx_submodule.add_function(wrap_pyfunction!(
        indicators::adx_min_data_accuracy,
        &adx_submodule
    )?)?;
    adx_submodule.add_function(wrap_pyfunction!(
        indicators::adx_output_length,
        &adx_submodule
    )?)?;
    adx_submodule.add_class::<indicators::AdxState>()?;
    indicators_module.add_submodule(&adx_submodule)?;

    // Create ADXR submodule for adxr::indicator() access
    let adxr_submodule = PyModule::new(py, "adxr")?;
    adxr_submodule.add_function(wrap_pyfunction!(
        indicators::adxr_indicator,
        &adxr_submodule
    )?)?;
    adxr_submodule.add_function(wrap_pyfunction!(indicators::adxr_info, &adxr_submodule)?)?;
    adxr_submodule.add_function(wrap_pyfunction!(
        indicators::adxr_min_data,
        &adxr_submodule
    )?)?;
    adxr_submodule.add_function(wrap_pyfunction!(
        indicators::adxr_min_data_accuracy,
        &adxr_submodule
    )?)?;
    adxr_submodule.add_function(wrap_pyfunction!(
        indicators::adxr_output_length,
        &adxr_submodule
    )?)?;
    adxr_submodule.add_class::<indicators::AdxrState>()?;
    indicators_module.add_submodule(&adxr_submodule)?;

    // Create AO submodule for ao::indicator() access
    let ao_submodule = PyModule::new(py, "ao")?;
    ao_submodule.add_function(wrap_pyfunction!(indicators::ao_indicator, &ao_submodule)?)?;
    ao_submodule.add_function(wrap_pyfunction!(indicators::ao_info, &ao_submodule)?)?;
    ao_submodule.add_function(wrap_pyfunction!(indicators::ao_min_data, &ao_submodule)?)?;
    ao_submodule.add_function(wrap_pyfunction!(
        indicators::ao_min_data_accuracy,
        &ao_submodule
    )?)?;
    ao_submodule.add_function(wrap_pyfunction!(
        indicators::ao_output_length,
        &ao_submodule
    )?)?;
    ao_submodule.add_class::<indicators::AoState>()?;
    indicators_module.add_submodule(&ao_submodule)?;

    // Create APO submodule for apo::indicator() access
    let apo_submodule = PyModule::new(py, "apo")?;
    apo_submodule.add_function(wrap_pyfunction!(indicators::apo_indicator, &apo_submodule)?)?;
    apo_submodule.add_function(wrap_pyfunction!(indicators::apo_info, &apo_submodule)?)?;
    apo_submodule.add_function(wrap_pyfunction!(indicators::apo_min_data, &apo_submodule)?)?;
    apo_submodule.add_function(wrap_pyfunction!(
        indicators::apo_min_data_accuracy,
        &apo_submodule
    )?)?;
    apo_submodule.add_function(wrap_pyfunction!(
        indicators::apo_output_length,
        &apo_submodule
    )?)?;
    apo_submodule.add_class::<indicators::ApoState>()?;
    indicators_module.add_submodule(&apo_submodule)?;

    // Create AROON submodule for aroon::indicator() access
    let aroon_submodule = PyModule::new(py, "aroon")?;
    aroon_submodule.add_function(wrap_pyfunction!(
        indicators::aroon_indicator,
        &aroon_submodule
    )?)?;
    aroon_submodule.add_function(wrap_pyfunction!(indicators::aroon_info, &aroon_submodule)?)?;
    aroon_submodule.add_function(wrap_pyfunction!(
        indicators::aroon_min_data,
        &aroon_submodule
    )?)?;
    aroon_submodule.add_function(wrap_pyfunction!(
        indicators::aroon_min_data_accuracy,
        &aroon_submodule
    )?)?;
    aroon_submodule.add_function(wrap_pyfunction!(
        indicators::aroon_output_length,
        &aroon_submodule
    )?)?;
    aroon_submodule.add_class::<indicators::AroonState>()?;
    indicators_module.add_submodule(&aroon_submodule)?;

    // Create AROONOSC submodule for aroonosc::indicator() access
    let aroonosc_submodule = PyModule::new(py, "aroonosc")?;
    aroonosc_submodule.add_function(wrap_pyfunction!(
        indicators::aroonosc_indicator,
        &aroonosc_submodule
    )?)?;
    aroonosc_submodule.add_function(wrap_pyfunction!(
        indicators::aroonosc_info,
        &aroonosc_submodule
    )?)?;
    aroonosc_submodule.add_function(wrap_pyfunction!(
        indicators::aroonosc_min_data,
        &aroonosc_submodule
    )?)?;
    aroonosc_submodule.add_function(wrap_pyfunction!(
        indicators::aroonosc_min_data_accuracy,
        &aroonosc_submodule
    )?)?;
    aroonosc_submodule.add_function(wrap_pyfunction!(
        indicators::aroonosc_output_length,
        &aroonosc_submodule
    )?)?;
    aroonosc_submodule.add_class::<indicators::AroonoscState>()?;
    indicators_module.add_submodule(&aroonosc_submodule)?;

    // Create ATR submodule for atr::indicator() access
    let atr_submodule = PyModule::new(py, "atr")?;
    atr_submodule.add_function(wrap_pyfunction!(indicators::atr_indicator, &atr_submodule)?)?;
    atr_submodule.add_function(wrap_pyfunction!(indicators::atr_info, &atr_submodule)?)?;
    atr_submodule.add_function(wrap_pyfunction!(indicators::atr_min_data, &atr_submodule)?)?;
    atr_submodule.add_function(wrap_pyfunction!(
        indicators::atr_min_data_accuracy,
        &atr_submodule
    )?)?;
    atr_submodule.add_function(wrap_pyfunction!(
        indicators::atr_output_length,
        &atr_submodule
    )?)?;
    atr_submodule.add_class::<indicators::AtrState>()?;
    indicators_module.add_submodule(&atr_submodule)?;

    // Create AVGPRICE submodule for avgprice::indicator() access
    let avgprice_submodule = PyModule::new(py, "avgprice")?;
    avgprice_submodule.add_function(wrap_pyfunction!(
        indicators::avgprice_indicator,
        &avgprice_submodule
    )?)?;
    avgprice_submodule.add_function(wrap_pyfunction!(
        indicators::avgprice_info,
        &avgprice_submodule
    )?)?;
    avgprice_submodule.add_function(wrap_pyfunction!(
        indicators::avgprice_min_data,
        &avgprice_submodule
    )?)?;
    avgprice_submodule.add_function(wrap_pyfunction!(
        indicators::avgprice_min_data_accuracy,
        &avgprice_submodule
    )?)?;
    avgprice_submodule.add_function(wrap_pyfunction!(
        indicators::avgprice_output_length,
        &avgprice_submodule
    )?)?;
    avgprice_submodule.add_class::<indicators::AvgpriceState>()?;
    indicators_module.add_submodule(&avgprice_submodule)?;

    // Create BBANDS submodule for bbands::indicator() access
    let bbands_submodule = PyModule::new(py, "bbands")?;
    bbands_submodule.add_function(wrap_pyfunction!(
        indicators::bbands_indicator,
        &bbands_submodule
    )?)?;
    bbands_submodule.add_function(wrap_pyfunction!(
        indicators::bbands_info,
        &bbands_submodule
    )?)?;
    bbands_submodule.add_function(wrap_pyfunction!(
        indicators::bbands_min_data,
        &bbands_submodule
    )?)?;
    bbands_submodule.add_function(wrap_pyfunction!(
        indicators::bbands_min_data_accuracy,
        &bbands_submodule
    )?)?;
    bbands_submodule.add_function(wrap_pyfunction!(
        indicators::bbands_output_length,
        &bbands_submodule
    )?)?;
    bbands_submodule.add_class::<indicators::BbandsState>()?;
    indicators_module.add_submodule(&bbands_submodule)?;

    // Create BOP submodule for bop::indicator() access
    let bop_submodule = PyModule::new(py, "bop")?;
    bop_submodule.add_function(wrap_pyfunction!(indicators::bop_indicator, &bop_submodule)?)?;
    bop_submodule.add_function(wrap_pyfunction!(indicators::bop_info, &bop_submodule)?)?;
    bop_submodule.add_function(wrap_pyfunction!(indicators::bop_min_data, &bop_submodule)?)?;
    bop_submodule.add_function(wrap_pyfunction!(
        indicators::bop_min_data_accuracy,
        &bop_submodule
    )?)?;
    bop_submodule.add_function(wrap_pyfunction!(
        indicators::bop_output_length,
        &bop_submodule
    )?)?;
    bop_submodule.add_class::<indicators::BopState>()?;
    indicators_module.add_submodule(&bop_submodule)?;

    // Create CCI submodule for cci::indicator() access
    let cci_submodule = PyModule::new(py, "cci")?;
    cci_submodule.add_function(wrap_pyfunction!(indicators::cci_indicator, &cci_submodule)?)?;
    cci_submodule.add_function(wrap_pyfunction!(indicators::cci_info, &cci_submodule)?)?;
    cci_submodule.add_function(wrap_pyfunction!(indicators::cci_min_data, &cci_submodule)?)?;
    cci_submodule.add_function(wrap_pyfunction!(
        indicators::cci_min_data_accuracy,
        &cci_submodule
    )?)?;
    cci_submodule.add_function(wrap_pyfunction!(
        indicators::cci_output_length,
        &cci_submodule
    )?)?;
    cci_submodule.add_class::<indicators::CciState>()?;
    indicators_module.add_submodule(&cci_submodule)?;

    // Create CMO submodule for cmo::indicator() access
    let cmo_submodule = PyModule::new(py, "cmo")?;
    cmo_submodule.add_function(wrap_pyfunction!(indicators::cmo_indicator, &cmo_submodule)?)?;
    cmo_submodule.add_function(wrap_pyfunction!(indicators::cmo_info, &cmo_submodule)?)?;
    cmo_submodule.add_function(wrap_pyfunction!(indicators::cmo_min_data, &cmo_submodule)?)?;
    cmo_submodule.add_function(wrap_pyfunction!(
        indicators::cmo_min_data_accuracy,
        &cmo_submodule
    )?)?;
    cmo_submodule.add_function(wrap_pyfunction!(
        indicators::cmo_output_length,
        &cmo_submodule
    )?)?;
    cmo_submodule.add_class::<indicators::CmoState>()?;
    indicators_module.add_submodule(&cmo_submodule)?;

    // Create CVI submodule for cvi::indicator() access
    let cvi_submodule = PyModule::new(py, "cvi")?;
    cvi_submodule.add_function(wrap_pyfunction!(indicators::cvi_indicator, &cvi_submodule)?)?;
    cvi_submodule.add_function(wrap_pyfunction!(indicators::cvi_info, &cvi_submodule)?)?;
    cvi_submodule.add_function(wrap_pyfunction!(indicators::cvi_min_data, &cvi_submodule)?)?;
    cvi_submodule.add_function(wrap_pyfunction!(
        indicators::cvi_min_data_accuracy,
        &cvi_submodule
    )?)?;
    cvi_submodule.add_function(wrap_pyfunction!(
        indicators::cvi_output_length,
        &cvi_submodule
    )?)?;
    cvi_submodule.add_class::<indicators::CviState>()?;
    indicators_module.add_submodule(&cvi_submodule)?;

    // Create DEMA submodule for dema::indicator() access
    let dema_submodule = PyModule::new(py, "dema")?;
    dema_submodule.add_function(wrap_pyfunction!(
        indicators::dema_indicator,
        &dema_submodule
    )?)?;
    dema_submodule.add_function(wrap_pyfunction!(indicators::dema_info, &dema_submodule)?)?;
    dema_submodule.add_function(wrap_pyfunction!(
        indicators::dema_min_data,
        &dema_submodule
    )?)?;
    dema_submodule.add_function(wrap_pyfunction!(
        indicators::dema_min_data_accuracy,
        &dema_submodule
    )?)?;
    dema_submodule.add_function(wrap_pyfunction!(
        indicators::dema_output_length,
        &dema_submodule
    )?)?;
    dema_submodule.add_class::<indicators::DemaState>()?;
    indicators_module.add_submodule(&dema_submodule)?;

    // Create DI submodule for di::indicator() access
    let di_submodule = PyModule::new(py, "di")?;
    di_submodule.add_function(wrap_pyfunction!(indicators::di_indicator, &di_submodule)?)?;
    di_submodule.add_function(wrap_pyfunction!(indicators::di_info, &di_submodule)?)?;
    di_submodule.add_function(wrap_pyfunction!(indicators::di_min_data, &di_submodule)?)?;
    di_submodule.add_function(wrap_pyfunction!(
        indicators::di_min_data_accuracy,
        &di_submodule
    )?)?;
    di_submodule.add_function(wrap_pyfunction!(
        indicators::di_output_length,
        &di_submodule
    )?)?;
    di_submodule.add_class::<indicators::DiState>()?;
    indicators_module.add_submodule(&di_submodule)?;

    // Create DM submodule for dm::indicator() access
    let dm_submodule = PyModule::new(py, "dm")?;
    dm_submodule.add_function(wrap_pyfunction!(indicators::dm_indicator, &dm_submodule)?)?;
    dm_submodule.add_function(wrap_pyfunction!(indicators::dm_info, &dm_submodule)?)?;
    dm_submodule.add_function(wrap_pyfunction!(indicators::dm_min_data, &dm_submodule)?)?;
    dm_submodule.add_function(wrap_pyfunction!(
        indicators::dm_min_data_accuracy,
        &dm_submodule
    )?)?;
    dm_submodule.add_function(wrap_pyfunction!(
        indicators::dm_output_length,
        &dm_submodule
    )?)?;
    dm_submodule.add_class::<indicators::DmState>()?;
    indicators_module.add_submodule(&dm_submodule)?;

    // Create DPO submodule for dpo::indicator() access
    let dpo_submodule = PyModule::new(py, "dpo")?;
    dpo_submodule.add_function(wrap_pyfunction!(indicators::dpo_indicator, &dpo_submodule)?)?;
    dpo_submodule.add_function(wrap_pyfunction!(indicators::dpo_info, &dpo_submodule)?)?;
    dpo_submodule.add_function(wrap_pyfunction!(indicators::dpo_min_data, &dpo_submodule)?)?;
    dpo_submodule.add_function(wrap_pyfunction!(
        indicators::dpo_min_data_accuracy,
        &dpo_submodule
    )?)?;
    dpo_submodule.add_function(wrap_pyfunction!(
        indicators::dpo_output_length,
        &dpo_submodule
    )?)?;
    dpo_submodule.add_class::<indicators::DpoState>()?;
    indicators_module.add_submodule(&dpo_submodule)?;

    // Create DX submodule for dx::indicator() access
    let dx_submodule = PyModule::new(py, "dx")?;
    dx_submodule.add_function(wrap_pyfunction!(indicators::dx_indicator, &dx_submodule)?)?;
    dx_submodule.add_function(wrap_pyfunction!(indicators::dx_info, &dx_submodule)?)?;
    dx_submodule.add_function(wrap_pyfunction!(indicators::dx_min_data, &dx_submodule)?)?;
    dx_submodule.add_function(wrap_pyfunction!(
        indicators::dx_min_data_accuracy,
        &dx_submodule
    )?)?;
    dx_submodule.add_function(wrap_pyfunction!(
        indicators::dx_output_length,
        &dx_submodule
    )?)?;
    dx_submodule.add_class::<indicators::DxState>()?;
    indicators_module.add_submodule(&dx_submodule)?;

    // Create EMA submodule for ema::indicator() access
    let ema_submodule = PyModule::new(py, "ema")?;
    ema_submodule.add_function(wrap_pyfunction!(indicators::ema_indicator, &ema_submodule)?)?;
    ema_submodule.add_function(wrap_pyfunction!(indicators::ema_info, &ema_submodule)?)?;
    ema_submodule.add_function(wrap_pyfunction!(indicators::ema_min_data, &ema_submodule)?)?;
    ema_submodule.add_function(wrap_pyfunction!(
        indicators::ema_min_data_accuracy,
        &ema_submodule
    )?)?;
    ema_submodule.add_function(wrap_pyfunction!(
        indicators::ema_output_length,
        &ema_submodule
    )?)?;
    ema_submodule.add_class::<indicators::EmaState>()?;
    indicators_module.add_submodule(&ema_submodule)?;

    // Create EMV submodule for emv::indicator() access
    let emv_submodule = PyModule::new(py, "emv")?;
    emv_submodule.add_function(wrap_pyfunction!(indicators::emv_indicator, &emv_submodule)?)?;
    emv_submodule.add_function(wrap_pyfunction!(indicators::emv_info, &emv_submodule)?)?;
    emv_submodule.add_function(wrap_pyfunction!(indicators::emv_min_data, &emv_submodule)?)?;
    emv_submodule.add_function(wrap_pyfunction!(
        indicators::emv_min_data_accuracy,
        &emv_submodule
    )?)?;
    emv_submodule.add_function(wrap_pyfunction!(
        indicators::emv_output_length,
        &emv_submodule
    )?)?;
    emv_submodule.add_class::<indicators::EmvState>()?;
    indicators_module.add_submodule(&emv_submodule)?;

    // Create FISHER submodule for fisher::indicator() access
    let fisher_submodule = PyModule::new(py, "fisher")?;
    fisher_submodule.add_function(wrap_pyfunction!(
        indicators::fisher_indicator,
        &fisher_submodule
    )?)?;
    fisher_submodule.add_function(wrap_pyfunction!(
        indicators::fisher_info,
        &fisher_submodule
    )?)?;
    fisher_submodule.add_function(wrap_pyfunction!(
        indicators::fisher_min_data,
        &fisher_submodule
    )?)?;
    fisher_submodule.add_function(wrap_pyfunction!(
        indicators::fisher_min_data_accuracy,
        &fisher_submodule
    )?)?;
    fisher_submodule.add_function(wrap_pyfunction!(
        indicators::fisher_output_length,
        &fisher_submodule
    )?)?;
    fisher_submodule.add_class::<indicators::FisherState>()?;
    indicators_module.add_submodule(&fisher_submodule)?;

    // Create FOSC submodule for fosc::indicator() access
    let fosc_submodule = PyModule::new(py, "fosc")?;
    fosc_submodule.add_function(wrap_pyfunction!(
        indicators::fosc_indicator,
        &fosc_submodule
    )?)?;
    fosc_submodule.add_function(wrap_pyfunction!(indicators::fosc_info, &fosc_submodule)?)?;
    fosc_submodule.add_function(wrap_pyfunction!(
        indicators::fosc_min_data,
        &fosc_submodule
    )?)?;
    fosc_submodule.add_function(wrap_pyfunction!(
        indicators::fosc_min_data_accuracy,
        &fosc_submodule
    )?)?;
    fosc_submodule.add_function(wrap_pyfunction!(
        indicators::fosc_output_length,
        &fosc_submodule
    )?)?;
    fosc_submodule.add_class::<indicators::FoscState>()?;
    indicators_module.add_submodule(&fosc_submodule)?;

    // Create HMA submodule for hma::indicator() access
    let hma_submodule = PyModule::new(py, "hma")?;
    hma_submodule.add_function(wrap_pyfunction!(indicators::hma_indicator, &hma_submodule)?)?;
    hma_submodule.add_function(wrap_pyfunction!(indicators::hma_info, &hma_submodule)?)?;
    hma_submodule.add_function(wrap_pyfunction!(indicators::hma_min_data, &hma_submodule)?)?;
    hma_submodule.add_function(wrap_pyfunction!(
        indicators::hma_min_data_accuracy,
        &hma_submodule
    )?)?;
    hma_submodule.add_function(wrap_pyfunction!(
        indicators::hma_output_length,
        &hma_submodule
    )?)?;
    hma_submodule.add_class::<indicators::HmaState>()?;
    indicators_module.add_submodule(&hma_submodule)?;

    // Create KAMA submodule for kama::indicator() access
    let kama_submodule = PyModule::new(py, "kama")?;
    kama_submodule.add_function(wrap_pyfunction!(
        indicators::kama_indicator,
        &kama_submodule
    )?)?;
    kama_submodule.add_function(wrap_pyfunction!(indicators::kama_info, &kama_submodule)?)?;
    kama_submodule.add_function(wrap_pyfunction!(
        indicators::kama_min_data,
        &kama_submodule
    )?)?;
    kama_submodule.add_function(wrap_pyfunction!(
        indicators::kama_min_data_accuracy,
        &kama_submodule
    )?)?;
    kama_submodule.add_function(wrap_pyfunction!(
        indicators::kama_output_length,
        &kama_submodule
    )?)?;
    kama_submodule.add_class::<indicators::KamaState>()?;
    indicators_module.add_submodule(&kama_submodule)?;

    // Create KVO submodule for kvo::indicator() access
    let kvo_submodule = PyModule::new(py, "kvo")?;
    kvo_submodule.add_function(wrap_pyfunction!(indicators::kvo_indicator, &kvo_submodule)?)?;
    kvo_submodule.add_function(wrap_pyfunction!(indicators::kvo_info, &kvo_submodule)?)?;
    kvo_submodule.add_function(wrap_pyfunction!(indicators::kvo_min_data, &kvo_submodule)?)?;
    kvo_submodule.add_function(wrap_pyfunction!(
        indicators::kvo_min_data_accuracy,
        &kvo_submodule
    )?)?;
    kvo_submodule.add_function(wrap_pyfunction!(
        indicators::kvo_output_length,
        &kvo_submodule
    )?)?;
    kvo_submodule.add_class::<indicators::KvoState>()?;
    indicators_module.add_submodule(&kvo_submodule)?;

    // Create LINREG submodule for linreg::indicator() access
    let linreg_submodule = PyModule::new(py, "linreg")?;
    linreg_submodule.add_function(wrap_pyfunction!(
        indicators::linreg_indicator,
        &linreg_submodule
    )?)?;
    linreg_submodule.add_function(wrap_pyfunction!(
        indicators::linreg_info,
        &linreg_submodule
    )?)?;
    linreg_submodule.add_function(wrap_pyfunction!(
        indicators::linreg_min_data,
        &linreg_submodule
    )?)?;
    linreg_submodule.add_function(wrap_pyfunction!(
        indicators::linreg_min_data_accuracy,
        &linreg_submodule
    )?)?;
    linreg_submodule.add_function(wrap_pyfunction!(
        indicators::linreg_output_length,
        &linreg_submodule
    )?)?;
    linreg_submodule.add_class::<indicators::LinregState>()?;
    indicators_module.add_submodule(&linreg_submodule)?;

    // Create MACD submodule for macd::indicator() access
    let macd_submodule = PyModule::new(py, "macd")?;
    macd_submodule.add_function(wrap_pyfunction!(
        indicators::macd_indicator,
        &macd_submodule
    )?)?;
    macd_submodule.add_function(wrap_pyfunction!(indicators::macd_info, &macd_submodule)?)?;
    macd_submodule.add_function(wrap_pyfunction!(
        indicators::macd_min_data,
        &macd_submodule
    )?)?;
    macd_submodule.add_function(wrap_pyfunction!(
        indicators::macd_min_data_accuracy,
        &macd_submodule
    )?)?;
    macd_submodule.add_function(wrap_pyfunction!(
        indicators::macd_output_length,
        &macd_submodule
    )?)?;
    macd_submodule.add_class::<indicators::MacdState>()?;
    indicators_module.add_submodule(&macd_submodule)?;

    // Create MARKETFI submodule for marketfi::indicator() access
    let marketfi_submodule = PyModule::new(py, "marketfi")?;
    marketfi_submodule.add_function(wrap_pyfunction!(
        indicators::marketfi_indicator,
        &marketfi_submodule
    )?)?;
    marketfi_submodule.add_function(wrap_pyfunction!(
        indicators::marketfi_info,
        &marketfi_submodule
    )?)?;
    marketfi_submodule.add_function(wrap_pyfunction!(
        indicators::marketfi_min_data,
        &marketfi_submodule
    )?)?;
    marketfi_submodule.add_function(wrap_pyfunction!(
        indicators::marketfi_min_data_accuracy,
        &marketfi_submodule
    )?)?;
    marketfi_submodule.add_function(wrap_pyfunction!(
        indicators::marketfi_output_length,
        &marketfi_submodule
    )?)?;
    marketfi_submodule.add_class::<indicators::MarketfiState>()?;
    indicators_module.add_submodule(&marketfi_submodule)?;

    // Create MASS submodule for mass::indicator() access
    let mass_submodule = PyModule::new(py, "mass")?;
    mass_submodule.add_function(wrap_pyfunction!(
        indicators::mass_indicator,
        &mass_submodule
    )?)?;
    mass_submodule.add_function(wrap_pyfunction!(indicators::mass_info, &mass_submodule)?)?;
    mass_submodule.add_function(wrap_pyfunction!(
        indicators::mass_min_data,
        &mass_submodule
    )?)?;
    mass_submodule.add_function(wrap_pyfunction!(
        indicators::mass_min_data_accuracy,
        &mass_submodule
    )?)?;
    mass_submodule.add_function(wrap_pyfunction!(
        indicators::mass_output_length,
        &mass_submodule
    )?)?;
    mass_submodule.add_class::<indicators::MassState>()?;
    indicators_module.add_submodule(&mass_submodule)?;

    // Create MAX submodule for max::indicator() access
    let max_submodule = PyModule::new(py, "max")?;
    max_submodule.add_function(wrap_pyfunction!(indicators::max_indicator, &max_submodule)?)?;
    max_submodule.add_function(wrap_pyfunction!(indicators::max_info, &max_submodule)?)?;
    max_submodule.add_function(wrap_pyfunction!(indicators::max_min_data, &max_submodule)?)?;
    max_submodule.add_function(wrap_pyfunction!(
        indicators::max_min_data_accuracy,
        &max_submodule
    )?)?;
    max_submodule.add_function(wrap_pyfunction!(
        indicators::max_output_length,
        &max_submodule
    )?)?;
    max_submodule.add_class::<indicators::MaxState>()?;
    indicators_module.add_submodule(&max_submodule)?;

    // Create MD submodule for md::indicator() access
    let md_submodule = PyModule::new(py, "md")?;
    md_submodule.add_function(wrap_pyfunction!(indicators::md_indicator, &md_submodule)?)?;
    md_submodule.add_function(wrap_pyfunction!(indicators::md_info, &md_submodule)?)?;
    md_submodule.add_function(wrap_pyfunction!(indicators::md_min_data, &md_submodule)?)?;
    md_submodule.add_function(wrap_pyfunction!(
        indicators::md_min_data_accuracy,
        &md_submodule
    )?)?;
    md_submodule.add_function(wrap_pyfunction!(
        indicators::md_output_length,
        &md_submodule
    )?)?;
    md_submodule.add_class::<indicators::MdState>()?;
    indicators_module.add_submodule(&md_submodule)?;

    // Create MEDPRICE submodule for medprice::indicator() access
    let medprice_submodule = PyModule::new(py, "medprice")?;
    medprice_submodule.add_function(wrap_pyfunction!(
        indicators::medprice_indicator,
        &medprice_submodule
    )?)?;
    medprice_submodule.add_function(wrap_pyfunction!(
        indicators::medprice_info,
        &medprice_submodule
    )?)?;
    medprice_submodule.add_function(wrap_pyfunction!(
        indicators::medprice_min_data,
        &medprice_submodule
    )?)?;
    medprice_submodule.add_function(wrap_pyfunction!(
        indicators::medprice_min_data_accuracy,
        &medprice_submodule
    )?)?;
    medprice_submodule.add_function(wrap_pyfunction!(
        indicators::medprice_output_length,
        &medprice_submodule
    )?)?;
    medprice_submodule.add_class::<indicators::MedpriceState>()?;
    indicators_module.add_submodule(&medprice_submodule)?;

    // Create MFI submodule for mfi::indicator() access
    let mfi_submodule = PyModule::new(py, "mfi")?;
    mfi_submodule.add_function(wrap_pyfunction!(indicators::mfi_indicator, &mfi_submodule)?)?;
    mfi_submodule.add_function(wrap_pyfunction!(indicators::mfi_info, &mfi_submodule)?)?;
    mfi_submodule.add_function(wrap_pyfunction!(indicators::mfi_min_data, &mfi_submodule)?)?;
    mfi_submodule.add_function(wrap_pyfunction!(
        indicators::mfi_min_data_accuracy,
        &mfi_submodule
    )?)?;
    mfi_submodule.add_function(wrap_pyfunction!(
        indicators::mfi_output_length,
        &mfi_submodule
    )?)?;
    mfi_submodule.add_class::<indicators::MfiState>()?;
    indicators_module.add_submodule(&mfi_submodule)?;

    // Create MIN submodule for min::indicator() access
    let min_submodule = PyModule::new(py, "min")?;
    min_submodule.add_function(wrap_pyfunction!(indicators::min_indicator, &min_submodule)?)?;
    min_submodule.add_function(wrap_pyfunction!(indicators::min_info, &min_submodule)?)?;
    min_submodule.add_function(wrap_pyfunction!(indicators::min_min_data, &min_submodule)?)?;
    min_submodule.add_function(wrap_pyfunction!(
        indicators::min_min_data_accuracy,
        &min_submodule
    )?)?;
    min_submodule.add_function(wrap_pyfunction!(
        indicators::min_output_length,
        &min_submodule
    )?)?;
    min_submodule.add_class::<indicators::MinState>()?;
    indicators_module.add_submodule(&min_submodule)?;

    // Create MOM submodule for mom::indicator() access
    let mom_submodule = PyModule::new(py, "mom")?;
    mom_submodule.add_function(wrap_pyfunction!(indicators::mom_indicator, &mom_submodule)?)?;
    mom_submodule.add_function(wrap_pyfunction!(indicators::mom_info, &mom_submodule)?)?;
    mom_submodule.add_function(wrap_pyfunction!(indicators::mom_min_data, &mom_submodule)?)?;
    mom_submodule.add_function(wrap_pyfunction!(
        indicators::mom_min_data_accuracy,
        &mom_submodule
    )?)?;
    mom_submodule.add_function(wrap_pyfunction!(
        indicators::mom_output_length,
        &mom_submodule
    )?)?;
    mom_submodule.add_class::<indicators::MomState>()?;
    indicators_module.add_submodule(&mom_submodule)?;

    // Create MSW submodule for msw::indicator() access
    let msw_submodule = PyModule::new(py, "msw")?;
    msw_submodule.add_function(wrap_pyfunction!(indicators::msw_indicator, &msw_submodule)?)?;
    msw_submodule.add_function(wrap_pyfunction!(indicators::msw_info, &msw_submodule)?)?;
    msw_submodule.add_function(wrap_pyfunction!(indicators::msw_min_data, &msw_submodule)?)?;
    msw_submodule.add_function(wrap_pyfunction!(
        indicators::msw_min_data_accuracy,
        &msw_submodule
    )?)?;
    msw_submodule.add_function(wrap_pyfunction!(
        indicators::msw_output_length,
        &msw_submodule
    )?)?;
    msw_submodule.add_class::<indicators::MswState>()?;
    indicators_module.add_submodule(&msw_submodule)?;

    // Create NATR submodule for natr::indicator() access
    let natr_submodule = PyModule::new(py, "natr")?;
    natr_submodule.add_function(wrap_pyfunction!(
        indicators::natr_indicator,
        &natr_submodule
    )?)?;
    natr_submodule.add_function(wrap_pyfunction!(indicators::natr_info, &natr_submodule)?)?;
    natr_submodule.add_function(wrap_pyfunction!(
        indicators::natr_min_data,
        &natr_submodule
    )?)?;
    natr_submodule.add_function(wrap_pyfunction!(
        indicators::natr_min_data_accuracy,
        &natr_submodule
    )?)?;
    natr_submodule.add_function(wrap_pyfunction!(
        indicators::natr_output_length,
        &natr_submodule
    )?)?;
    natr_submodule.add_class::<indicators::NatrState>()?;
    indicators_module.add_submodule(&natr_submodule)?;

    // Create NVI submodule for nvi::indicator() access
    let nvi_submodule = PyModule::new(py, "nvi")?;
    nvi_submodule.add_function(wrap_pyfunction!(indicators::nvi_indicator, &nvi_submodule)?)?;
    nvi_submodule.add_function(wrap_pyfunction!(indicators::nvi_info, &nvi_submodule)?)?;
    nvi_submodule.add_function(wrap_pyfunction!(indicators::nvi_min_data, &nvi_submodule)?)?;
    nvi_submodule.add_function(wrap_pyfunction!(
        indicators::nvi_min_data_accuracy,
        &nvi_submodule
    )?)?;
    nvi_submodule.add_function(wrap_pyfunction!(
        indicators::nvi_output_length,
        &nvi_submodule
    )?)?;
    nvi_submodule.add_class::<indicators::NviState>()?;
    indicators_module.add_submodule(&nvi_submodule)?;

    // Create OBV submodule for obv::indicator() access
    let obv_submodule = PyModule::new(py, "obv")?;
    obv_submodule.add_function(wrap_pyfunction!(indicators::obv_indicator, &obv_submodule)?)?;
    obv_submodule.add_function(wrap_pyfunction!(indicators::obv_info, &obv_submodule)?)?;
    obv_submodule.add_function(wrap_pyfunction!(indicators::obv_min_data, &obv_submodule)?)?;
    obv_submodule.add_function(wrap_pyfunction!(
        indicators::obv_min_data_accuracy,
        &obv_submodule
    )?)?;
    obv_submodule.add_function(wrap_pyfunction!(
        indicators::obv_output_length,
        &obv_submodule
    )?)?;
    obv_submodule.add_class::<indicators::ObvState>()?;
    indicators_module.add_submodule(&obv_submodule)?;

    // Create PIVOTPOINT submodule for pivotpoint::indicator() access
    let pivotpoint_submodule = PyModule::new(py, "pivotpoint")?;
    pivotpoint_submodule.add_function(wrap_pyfunction!(
        indicators::pivotpoint_indicator,
        &pivotpoint_submodule
    )?)?;
    pivotpoint_submodule.add_function(wrap_pyfunction!(
        indicators::pivotpoint_info,
        &pivotpoint_submodule
    )?)?;
    pivotpoint_submodule.add_function(wrap_pyfunction!(
        indicators::pivotpoint_min_data,
        &pivotpoint_submodule
    )?)?;
    pivotpoint_submodule.add_function(wrap_pyfunction!(
        indicators::pivotpoint_min_data_accuracy,
        &pivotpoint_submodule
    )?)?;
    pivotpoint_submodule.add_function(wrap_pyfunction!(
        indicators::pivotpoint_output_length,
        &pivotpoint_submodule
    )?)?;
    pivotpoint_submodule.add_class::<indicators::PivotpointState>()?;
    indicators_module.add_submodule(&pivotpoint_submodule)?;

    // Create PPO submodule for ppo::indicator() access
    let ppo_submodule = PyModule::new(py, "ppo")?;
    ppo_submodule.add_function(wrap_pyfunction!(indicators::ppo_indicator, &ppo_submodule)?)?;
    ppo_submodule.add_function(wrap_pyfunction!(indicators::ppo_info, &ppo_submodule)?)?;
    ppo_submodule.add_function(wrap_pyfunction!(indicators::ppo_min_data, &ppo_submodule)?)?;
    ppo_submodule.add_function(wrap_pyfunction!(
        indicators::ppo_min_data_accuracy,
        &ppo_submodule
    )?)?;
    ppo_submodule.add_function(wrap_pyfunction!(
        indicators::ppo_output_length,
        &ppo_submodule
    )?)?;
    ppo_submodule.add_class::<indicators::PpoState>()?;
    indicators_module.add_submodule(&ppo_submodule)?;

    // Create PSAR submodule for psar::indicator() access
    let psar_submodule = PyModule::new(py, "psar")?;
    psar_submodule.add_function(wrap_pyfunction!(
        indicators::psar_indicator,
        &psar_submodule
    )?)?;
    psar_submodule.add_function(wrap_pyfunction!(indicators::psar_info, &psar_submodule)?)?;
    psar_submodule.add_function(wrap_pyfunction!(
        indicators::psar_min_data,
        &psar_submodule
    )?)?;
    psar_submodule.add_function(wrap_pyfunction!(
        indicators::psar_min_data_accuracy,
        &psar_submodule
    )?)?;
    psar_submodule.add_function(wrap_pyfunction!(
        indicators::psar_output_length,
        &psar_submodule
    )?)?;
    psar_submodule.add_class::<indicators::PsarState>()?;
    indicators_module.add_submodule(&psar_submodule)?;

    // Create PVI submodule for pvi::indicator() access
    let pvi_submodule = PyModule::new(py, "pvi")?;
    pvi_submodule.add_function(wrap_pyfunction!(indicators::pvi_indicator, &pvi_submodule)?)?;
    pvi_submodule.add_function(wrap_pyfunction!(indicators::pvi_info, &pvi_submodule)?)?;
    pvi_submodule.add_function(wrap_pyfunction!(indicators::pvi_min_data, &pvi_submodule)?)?;
    pvi_submodule.add_function(wrap_pyfunction!(
        indicators::pvi_min_data_accuracy,
        &pvi_submodule
    )?)?;
    pvi_submodule.add_function(wrap_pyfunction!(
        indicators::pvi_output_length,
        &pvi_submodule
    )?)?;
    pvi_submodule.add_class::<indicators::PviState>()?;
    indicators_module.add_submodule(&pvi_submodule)?;

    // Create QSTICK submodule for qstick::indicator() access
    let qstick_submodule = PyModule::new(py, "qstick")?;
    qstick_submodule.add_function(wrap_pyfunction!(
        indicators::qstick_indicator,
        &qstick_submodule
    )?)?;
    qstick_submodule.add_function(wrap_pyfunction!(
        indicators::qstick_info,
        &qstick_submodule
    )?)?;
    qstick_submodule.add_function(wrap_pyfunction!(
        indicators::qstick_min_data,
        &qstick_submodule
    )?)?;
    qstick_submodule.add_function(wrap_pyfunction!(
        indicators::qstick_min_data_accuracy,
        &qstick_submodule
    )?)?;
    qstick_submodule.add_function(wrap_pyfunction!(
        indicators::qstick_output_length,
        &qstick_submodule
    )?)?;
    qstick_submodule.add_class::<indicators::QstickState>()?;
    indicators_module.add_submodule(&qstick_submodule)?;

    // Create ROC submodule for roc::indicator() access
    let roc_submodule = PyModule::new(py, "roc")?;
    roc_submodule.add_function(wrap_pyfunction!(indicators::roc_indicator, &roc_submodule)?)?;
    roc_submodule.add_function(wrap_pyfunction!(indicators::roc_info, &roc_submodule)?)?;
    roc_submodule.add_function(wrap_pyfunction!(indicators::roc_min_data, &roc_submodule)?)?;
    roc_submodule.add_function(wrap_pyfunction!(
        indicators::roc_min_data_accuracy,
        &roc_submodule
    )?)?;
    roc_submodule.add_function(wrap_pyfunction!(
        indicators::roc_output_length,
        &roc_submodule
    )?)?;
    roc_submodule.add_class::<indicators::RocState>()?;
    indicators_module.add_submodule(&roc_submodule)?;

    // Create ROCR submodule for rocr::indicator() access
    let rocr_submodule = PyModule::new(py, "rocr")?;
    rocr_submodule.add_function(wrap_pyfunction!(
        indicators::rocr_indicator,
        &rocr_submodule
    )?)?;
    rocr_submodule.add_function(wrap_pyfunction!(indicators::rocr_info, &rocr_submodule)?)?;
    rocr_submodule.add_function(wrap_pyfunction!(
        indicators::rocr_min_data,
        &rocr_submodule
    )?)?;
    rocr_submodule.add_function(wrap_pyfunction!(
        indicators::rocr_min_data_accuracy,
        &rocr_submodule
    )?)?;
    rocr_submodule.add_function(wrap_pyfunction!(
        indicators::rocr_output_length,
        &rocr_submodule
    )?)?;
    rocr_submodule.add_class::<indicators::RocrState>()?;
    indicators_module.add_submodule(&rocr_submodule)?;

    // Create RSI submodule for rsi::indicator() access
    let rsi_submodule = PyModule::new(py, "rsi")?;
    rsi_submodule.add_function(wrap_pyfunction!(indicators::rsi_indicator, &rsi_submodule)?)?;
    rsi_submodule.add_function(wrap_pyfunction!(indicators::rsi_info, &rsi_submodule)?)?;
    rsi_submodule.add_function(wrap_pyfunction!(indicators::rsi_min_data, &rsi_submodule)?)?;
    rsi_submodule.add_function(wrap_pyfunction!(
        indicators::rsi_min_data_accuracy,
        &rsi_submodule
    )?)?;
    rsi_submodule.add_function(wrap_pyfunction!(
        indicators::rsi_output_length,
        &rsi_submodule
    )?)?;
    rsi_submodule.add_class::<indicators::RsiState>()?;
    indicators_module.add_submodule(&rsi_submodule)?;

    // Create SMA submodule for sma::indicator() access
    let sma_submodule = PyModule::new(py, "sma")?;
    sma_submodule.add_function(wrap_pyfunction!(indicators::sma_indicator, &sma_submodule)?)?;
    sma_submodule.add_function(wrap_pyfunction!(indicators::sma_info, &sma_submodule)?)?;
    sma_submodule.add_function(wrap_pyfunction!(indicators::sma_min_data, &sma_submodule)?)?;
    sma_submodule.add_function(wrap_pyfunction!(
        indicators::sma_min_data_accuracy,
        &sma_submodule
    )?)?;
    sma_submodule.add_function(wrap_pyfunction!(
        indicators::sma_output_length,
        &sma_submodule
    )?)?;
    sma_submodule.add_class::<indicators::SmaState>()?;
    indicators_module.add_submodule(&sma_submodule)?;

    // Create STDDEV submodule for stddev::indicator() access
    let stddev_submodule = PyModule::new(py, "stddev")?;
    stddev_submodule.add_function(wrap_pyfunction!(
        indicators::stddev_indicator,
        &stddev_submodule
    )?)?;
    stddev_submodule.add_function(wrap_pyfunction!(
        indicators::stddev_info,
        &stddev_submodule
    )?)?;
    stddev_submodule.add_function(wrap_pyfunction!(
        indicators::stddev_min_data,
        &stddev_submodule
    )?)?;
    stddev_submodule.add_function(wrap_pyfunction!(
        indicators::stddev_min_data_accuracy,
        &stddev_submodule
    )?)?;
    stddev_submodule.add_function(wrap_pyfunction!(
        indicators::stddev_output_length,
        &stddev_submodule
    )?)?;
    stddev_submodule.add_class::<indicators::StddevState>()?;
    indicators_module.add_submodule(&stddev_submodule)?;

    // Create STOCH submodule for stoch::indicator() access
    let stoch_submodule = PyModule::new(py, "stoch")?;
    stoch_submodule.add_function(wrap_pyfunction!(
        indicators::stoch_indicator,
        &stoch_submodule
    )?)?;
    stoch_submodule.add_function(wrap_pyfunction!(indicators::stoch_info, &stoch_submodule)?)?;
    stoch_submodule.add_function(wrap_pyfunction!(
        indicators::stoch_min_data,
        &stoch_submodule
    )?)?;
    stoch_submodule.add_function(wrap_pyfunction!(
        indicators::stoch_min_data_accuracy,
        &stoch_submodule
    )?)?;
    stoch_submodule.add_function(wrap_pyfunction!(
        indicators::stoch_output_length,
        &stoch_submodule
    )?)?;
    stoch_submodule.add_class::<indicators::StochState>()?;
    indicators_module.add_submodule(&stoch_submodule)?;

    // Create STOCHRSI submodule for stochrsi::indicator() access
    let stochrsi_submodule = PyModule::new(py, "stochrsi")?;
    stochrsi_submodule.add_function(wrap_pyfunction!(
        indicators::stochrsi_indicator,
        &stochrsi_submodule
    )?)?;
    stochrsi_submodule.add_function(wrap_pyfunction!(
        indicators::stochrsi_info,
        &stochrsi_submodule
    )?)?;
    stochrsi_submodule.add_function(wrap_pyfunction!(
        indicators::stochrsi_min_data,
        &stochrsi_submodule
    )?)?;
    stochrsi_submodule.add_function(wrap_pyfunction!(
        indicators::stochrsi_min_data_accuracy,
        &stochrsi_submodule
    )?)?;
    stochrsi_submodule.add_function(wrap_pyfunction!(
        indicators::stochrsi_output_length,
        &stochrsi_submodule
    )?)?;
    stochrsi_submodule.add_class::<indicators::StochrsiState>()?;
    indicators_module.add_submodule(&stochrsi_submodule)?;

    // Create TEMA submodule for tema::indicator() access
    let tema_submodule = PyModule::new(py, "tema")?;
    tema_submodule.add_function(wrap_pyfunction!(
        indicators::tema_indicator,
        &tema_submodule
    )?)?;
    tema_submodule.add_function(wrap_pyfunction!(indicators::tema_info, &tema_submodule)?)?;
    tema_submodule.add_function(wrap_pyfunction!(
        indicators::tema_min_data,
        &tema_submodule
    )?)?;
    tema_submodule.add_function(wrap_pyfunction!(
        indicators::tema_min_data_accuracy,
        &tema_submodule
    )?)?;
    tema_submodule.add_function(wrap_pyfunction!(
        indicators::tema_output_length,
        &tema_submodule
    )?)?;
    tema_submodule.add_class::<indicators::TemaState>()?;
    indicators_module.add_submodule(&tema_submodule)?;

    // Create TR submodule for tr::indicator() access
    let tr_submodule = PyModule::new(py, "tr")?;
    tr_submodule.add_function(wrap_pyfunction!(indicators::tr_indicator, &tr_submodule)?)?;
    tr_submodule.add_function(wrap_pyfunction!(indicators::tr_info, &tr_submodule)?)?;
    tr_submodule.add_function(wrap_pyfunction!(indicators::tr_min_data, &tr_submodule)?)?;
    tr_submodule.add_function(wrap_pyfunction!(
        indicators::tr_min_data_accuracy,
        &tr_submodule
    )?)?;
    tr_submodule.add_function(wrap_pyfunction!(
        indicators::tr_output_length,
        &tr_submodule
    )?)?;
    tr_submodule.add_class::<indicators::TrState>()?;
    indicators_module.add_submodule(&tr_submodule)?;

    // Create TRIMA submodule for trima::indicator() access
    let trima_submodule = PyModule::new(py, "trima")?;
    trima_submodule.add_function(wrap_pyfunction!(
        indicators::trima_indicator,
        &trima_submodule
    )?)?;
    trima_submodule.add_function(wrap_pyfunction!(indicators::trima_info, &trima_submodule)?)?;
    trima_submodule.add_function(wrap_pyfunction!(
        indicators::trima_min_data,
        &trima_submodule
    )?)?;
    trima_submodule.add_function(wrap_pyfunction!(
        indicators::trima_min_data_accuracy,
        &trima_submodule
    )?)?;
    trima_submodule.add_function(wrap_pyfunction!(
        indicators::trima_output_length,
        &trima_submodule
    )?)?;
    trima_submodule.add_class::<indicators::TrimaState>()?;
    indicators_module.add_submodule(&trima_submodule)?;

    // Create TRIX submodule for trix::indicator() access
    let trix_submodule = PyModule::new(py, "trix")?;
    trix_submodule.add_function(wrap_pyfunction!(
        indicators::trix_indicator,
        &trix_submodule
    )?)?;
    trix_submodule.add_function(wrap_pyfunction!(indicators::trix_info, &trix_submodule)?)?;
    trix_submodule.add_function(wrap_pyfunction!(
        indicators::trix_min_data,
        &trix_submodule
    )?)?;
    trix_submodule.add_function(wrap_pyfunction!(
        indicators::trix_min_data_accuracy,
        &trix_submodule
    )?)?;
    trix_submodule.add_function(wrap_pyfunction!(
        indicators::trix_output_length,
        &trix_submodule
    )?)?;
    trix_submodule.add_class::<indicators::TrixState>()?;
    indicators_module.add_submodule(&trix_submodule)?;

    // Create TSF submodule for tsf::indicator() access
    let tsf_submodule = PyModule::new(py, "tsf")?;
    tsf_submodule.add_function(wrap_pyfunction!(indicators::tsf_indicator, &tsf_submodule)?)?;
    tsf_submodule.add_function(wrap_pyfunction!(indicators::tsf_info, &tsf_submodule)?)?;
    tsf_submodule.add_function(wrap_pyfunction!(indicators::tsf_min_data, &tsf_submodule)?)?;
    tsf_submodule.add_function(wrap_pyfunction!(
        indicators::tsf_min_data_accuracy,
        &tsf_submodule
    )?)?;
    tsf_submodule.add_function(wrap_pyfunction!(
        indicators::tsf_output_length,
        &tsf_submodule
    )?)?;
    tsf_submodule.add_class::<indicators::TsfState>()?;
    indicators_module.add_submodule(&tsf_submodule)?;

    // Create TYPPRICE submodule for typprice::indicator() access
    let typprice_submodule = PyModule::new(py, "typprice")?;
    typprice_submodule.add_function(wrap_pyfunction!(
        indicators::typprice_indicator,
        &typprice_submodule
    )?)?;
    typprice_submodule.add_function(wrap_pyfunction!(
        indicators::typprice_info,
        &typprice_submodule
    )?)?;
    typprice_submodule.add_function(wrap_pyfunction!(
        indicators::typprice_min_data,
        &typprice_submodule
    )?)?;
    typprice_submodule.add_function(wrap_pyfunction!(
        indicators::typprice_min_data_accuracy,
        &typprice_submodule
    )?)?;
    typprice_submodule.add_function(wrap_pyfunction!(
        indicators::typprice_output_length,
        &typprice_submodule
    )?)?;
    typprice_submodule.add_class::<indicators::TyppriceState>()?;
    indicators_module.add_submodule(&typprice_submodule)?;

    // Create ULTOSC submodule for ultosc::indicator() access
    let ultosc_submodule = PyModule::new(py, "ultosc")?;
    ultosc_submodule.add_function(wrap_pyfunction!(
        indicators::ultosc_indicator,
        &ultosc_submodule
    )?)?;
    ultosc_submodule.add_function(wrap_pyfunction!(
        indicators::ultosc_info,
        &ultosc_submodule
    )?)?;
    ultosc_submodule.add_function(wrap_pyfunction!(
        indicators::ultosc_min_data,
        &ultosc_submodule
    )?)?;
    ultosc_submodule.add_function(wrap_pyfunction!(
        indicators::ultosc_min_data_accuracy,
        &ultosc_submodule
    )?)?;
    ultosc_submodule.add_function(wrap_pyfunction!(
        indicators::ultosc_output_length,
        &ultosc_submodule
    )?)?;
    ultosc_submodule.add_class::<indicators::UltoscState>()?;
    indicators_module.add_submodule(&ultosc_submodule)?;

    // Create VHF submodule for vhf::indicator() access
    let vhf_submodule = PyModule::new(py, "vhf")?;
    vhf_submodule.add_function(wrap_pyfunction!(indicators::vhf_indicator, &vhf_submodule)?)?;
    vhf_submodule.add_function(wrap_pyfunction!(indicators::vhf_info, &vhf_submodule)?)?;
    vhf_submodule.add_function(wrap_pyfunction!(indicators::vhf_min_data, &vhf_submodule)?)?;
    vhf_submodule.add_function(wrap_pyfunction!(
        indicators::vhf_min_data_accuracy,
        &vhf_submodule
    )?)?;
    vhf_submodule.add_function(wrap_pyfunction!(
        indicators::vhf_output_length,
        &vhf_submodule
    )?)?;
    vhf_submodule.add_class::<indicators::VhfState>()?;
    indicators_module.add_submodule(&vhf_submodule)?;

    // Create VIDYA submodule for vidya::indicator() access
    let vidya_submodule = PyModule::new(py, "vidya")?;
    vidya_submodule.add_function(wrap_pyfunction!(
        indicators::vidya_indicator,
        &vidya_submodule
    )?)?;
    vidya_submodule.add_function(wrap_pyfunction!(indicators::vidya_info, &vidya_submodule)?)?;
    vidya_submodule.add_function(wrap_pyfunction!(
        indicators::vidya_min_data,
        &vidya_submodule
    )?)?;
    vidya_submodule.add_function(wrap_pyfunction!(
        indicators::vidya_min_data_accuracy,
        &vidya_submodule
    )?)?;
    vidya_submodule.add_function(wrap_pyfunction!(
        indicators::vidya_output_length,
        &vidya_submodule
    )?)?;
    vidya_submodule.add_class::<indicators::VidyaState>()?;
    indicators_module.add_submodule(&vidya_submodule)?;

    // Create VOLATILITY submodule for volatility::indicator() access
    let volatility_submodule = PyModule::new(py, "volatility")?;
    volatility_submodule.add_function(wrap_pyfunction!(
        indicators::volatility_indicator,
        &volatility_submodule
    )?)?;
    volatility_submodule.add_function(wrap_pyfunction!(
        indicators::volatility_info,
        &volatility_submodule
    )?)?;
    volatility_submodule.add_function(wrap_pyfunction!(
        indicators::volatility_min_data,
        &volatility_submodule
    )?)?;
    volatility_submodule.add_function(wrap_pyfunction!(
        indicators::volatility_min_data_accuracy,
        &volatility_submodule
    )?)?;
    volatility_submodule.add_function(wrap_pyfunction!(
        indicators::volatility_output_length,
        &volatility_submodule
    )?)?;
    volatility_submodule.add_class::<indicators::VolatilityState>()?;
    indicators_module.add_submodule(&volatility_submodule)?;

    // Create VOSC submodule for vosc::indicator() access
    let vosc_submodule = PyModule::new(py, "vosc")?;
    vosc_submodule.add_function(wrap_pyfunction!(
        indicators::vosc_indicator,
        &vosc_submodule
    )?)?;
    vosc_submodule.add_function(wrap_pyfunction!(indicators::vosc_info, &vosc_submodule)?)?;
    vosc_submodule.add_function(wrap_pyfunction!(
        indicators::vosc_min_data,
        &vosc_submodule
    )?)?;
    vosc_submodule.add_function(wrap_pyfunction!(
        indicators::vosc_min_data_accuracy,
        &vosc_submodule
    )?)?;
    vosc_submodule.add_function(wrap_pyfunction!(
        indicators::vosc_output_length,
        &vosc_submodule
    )?)?;
    vosc_submodule.add_class::<indicators::VoscState>()?;
    indicators_module.add_submodule(&vosc_submodule)?;

    // Create VWMA submodule for vwma::indicator() access
    let vwma_submodule = PyModule::new(py, "vwma")?;
    vwma_submodule.add_function(wrap_pyfunction!(
        indicators::vwma_indicator,
        &vwma_submodule
    )?)?;
    vwma_submodule.add_function(wrap_pyfunction!(indicators::vwma_info, &vwma_submodule)?)?;
    vwma_submodule.add_function(wrap_pyfunction!(
        indicators::vwma_min_data,
        &vwma_submodule
    )?)?;
    vwma_submodule.add_function(wrap_pyfunction!(
        indicators::vwma_min_data_accuracy,
        &vwma_submodule
    )?)?;
    vwma_submodule.add_function(wrap_pyfunction!(
        indicators::vwma_output_length,
        &vwma_submodule
    )?)?;
    vwma_submodule.add_class::<indicators::VwmaState>()?;
    indicators_module.add_submodule(&vwma_submodule)?;

    // Create WAD submodule for wad::indicator() access
    let wad_submodule = PyModule::new(py, "wad")?;
    wad_submodule.add_function(wrap_pyfunction!(indicators::wad_indicator, &wad_submodule)?)?;
    wad_submodule.add_function(wrap_pyfunction!(indicators::wad_info, &wad_submodule)?)?;
    wad_submodule.add_function(wrap_pyfunction!(indicators::wad_min_data, &wad_submodule)?)?;
    wad_submodule.add_function(wrap_pyfunction!(
        indicators::wad_min_data_accuracy,
        &wad_submodule
    )?)?;
    wad_submodule.add_function(wrap_pyfunction!(
        indicators::wad_output_length,
        &wad_submodule
    )?)?;
    wad_submodule.add_class::<indicators::WadState>()?;
    indicators_module.add_submodule(&wad_submodule)?;

    // Create WCPRICE submodule for wcprice::indicator() access
    let wcprice_submodule = PyModule::new(py, "wcprice")?;
    wcprice_submodule.add_function(wrap_pyfunction!(
        indicators::wcprice_indicator,
        &wcprice_submodule
    )?)?;
    wcprice_submodule.add_function(wrap_pyfunction!(
        indicators::wcprice_info,
        &wcprice_submodule
    )?)?;
    wcprice_submodule.add_function(wrap_pyfunction!(
        indicators::wcprice_min_data,
        &wcprice_submodule
    )?)?;
    wcprice_submodule.add_function(wrap_pyfunction!(
        indicators::wcprice_min_data_accuracy,
        &wcprice_submodule
    )?)?;
    wcprice_submodule.add_function(wrap_pyfunction!(
        indicators::wcprice_output_length,
        &wcprice_submodule
    )?)?;
    wcprice_submodule.add_class::<indicators::WcpriceState>()?;
    indicators_module.add_submodule(&wcprice_submodule)?;

    // Create WILDERS submodule for wilders::indicator() access
    let wilders_submodule = PyModule::new(py, "wilders")?;
    wilders_submodule.add_function(wrap_pyfunction!(
        indicators::wilders_indicator,
        &wilders_submodule
    )?)?;
    wilders_submodule.add_function(wrap_pyfunction!(
        indicators::wilders_info,
        &wilders_submodule
    )?)?;
    wilders_submodule.add_function(wrap_pyfunction!(
        indicators::wilders_min_data,
        &wilders_submodule
    )?)?;
    wilders_submodule.add_function(wrap_pyfunction!(
        indicators::wilders_min_data_accuracy,
        &wilders_submodule
    )?)?;
    wilders_submodule.add_function(wrap_pyfunction!(
        indicators::wilders_output_length,
        &wilders_submodule
    )?)?;
    wilders_submodule.add_class::<indicators::WildersState>()?;
    indicators_module.add_submodule(&wilders_submodule)?;

    // Create WILLR submodule for willr::indicator() access
    let willr_submodule = PyModule::new(py, "willr")?;
    willr_submodule.add_function(wrap_pyfunction!(
        indicators::willr_indicator,
        &willr_submodule
    )?)?;
    willr_submodule.add_function(wrap_pyfunction!(indicators::willr_info, &willr_submodule)?)?;
    willr_submodule.add_function(wrap_pyfunction!(
        indicators::willr_min_data,
        &willr_submodule
    )?)?;
    willr_submodule.add_function(wrap_pyfunction!(
        indicators::willr_min_data_accuracy,
        &willr_submodule
    )?)?;
    willr_submodule.add_function(wrap_pyfunction!(
        indicators::willr_output_length,
        &willr_submodule
    )?)?;
    willr_submodule.add_class::<indicators::WillrState>()?;
    indicators_module.add_submodule(&willr_submodule)?;

    // Create WMA submodule for wma::indicator() access
    let wma_submodule = PyModule::new(py, "wma")?;
    wma_submodule.add_function(wrap_pyfunction!(indicators::wma_indicator, &wma_submodule)?)?;
    wma_submodule.add_function(wrap_pyfunction!(indicators::wma_info, &wma_submodule)?)?;
    wma_submodule.add_function(wrap_pyfunction!(indicators::wma_min_data, &wma_submodule)?)?;
    wma_submodule.add_function(wrap_pyfunction!(
        indicators::wma_min_data_accuracy,
        &wma_submodule
    )?)?;
    wma_submodule.add_function(wrap_pyfunction!(
        indicators::wma_output_length,
        &wma_submodule
    )?)?;
    wma_submodule.add_class::<indicators::WmaState>()?;
    indicators_module.add_submodule(&wma_submodule)?;

    // Create ZLEMA submodule for zlema::indicator() access
    let zlema_submodule = PyModule::new(py, "zlema")?;
    zlema_submodule.add_function(wrap_pyfunction!(
        indicators::zlema_indicator,
        &zlema_submodule
    )?)?;
    zlema_submodule.add_function(wrap_pyfunction!(indicators::zlema_info, &zlema_submodule)?)?;
    zlema_submodule.add_function(wrap_pyfunction!(
        indicators::zlema_min_data,
        &zlema_submodule
    )?)?;
    zlema_submodule.add_function(wrap_pyfunction!(
        indicators::zlema_min_data_accuracy,
        &zlema_submodule
    )?)?;
    zlema_submodule.add_function(wrap_pyfunction!(
        indicators::zlema_output_length,
        &zlema_submodule
    )?)?;
    zlema_submodule.add_class::<indicators::ZlemaState>()?;
    indicators_module.add_submodule(&zlema_submodule)?;

    // TODO: Add other indicators as they are implemented
    // indicators_module.add_function(wrap_pyfunction!(indicators::adx::adx, &indicators_module)?)?;
    // etc.

    m.add_submodule(&indicators_module)?;

    // Utility functions at module level
    m.add_function(wrap_pyfunction!(list_indicators, m)?)?;

    Ok(())
}
