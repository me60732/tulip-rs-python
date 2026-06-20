use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

mod bulk_register;
mod indicators;
mod utils;

/// List all available indicators
#[pyfunction]
fn list_indicators() -> Vec<String> {
    vec![
        "ad".to_string(),
        "adaptivemsw".to_string(),
        "adosc".to_string(),
        "adx".to_string(),
        "adxr".to_string(),
        "ao".to_string(),
        "apo".to_string(),
        "aroon".to_string(),
        "aroonosc".to_string(),
        "atr".to_string(),
        "avgprice".to_string(),
        "bbands".to_string(),
        "bop".to_string(),
        "candlestick".to_string(),
        "ccfisher".to_string(),
        "cci".to_string(),
        "chaikinmf".to_string(),
        "cmo".to_string(),
        "cvi".to_string(),
        "cybercycle".to_string(),
        "dema".to_string(),
        "di".to_string(),
        "dm".to_string(),
        "donchianchannel".to_string(),
        "dpo".to_string(),
        "dx".to_string(),
        "elderray".to_string(),
        "ema".to_string(),
        "emv".to_string(),
        "fisher".to_string(),
        "fosc".to_string(),
        "highpass".to_string(),
        "hilberttransform".to_string(),
        "hma".to_string(),
        "homodynediscriminator".to_string(),
        "ichimoku".to_string(),
        "instantaneoustrendline".to_string(),
        "kama".to_string(),
        "kvo".to_string(),
        "linreg".to_string(),
        "macd".to_string(),
        "mama".to_string(),
        "marketfi".to_string(),
        "mass".to_string(),
        "max".to_string(),
        "md".to_string(),
        "medprice".to_string(),
        "mfi".to_string(),
        "min".to_string(),
        "mom".to_string(),
        "msw".to_string(),
        "natr".to_string(),
        "nvi".to_string(),
        "obv".to_string(),
        "pivotpoint".to_string(),
        "ppo".to_string(),
        "psar".to_string(),
        "pvi".to_string(),
        "qstick".to_string(),
        "roc".to_string(),
        "rocr".to_string(),
        "roofingfilter".to_string(),
        "rsi".to_string(),
        "sma".to_string(),
        "smaenvelope".to_string(),
        "stddev".to_string(),
        "stoch".to_string(),
        "stochrsi".to_string(),
        "supersmoother".to_string(),
        "supertrend".to_string(),
        "tema".to_string(),
        "tr".to_string(),
        "trendmode".to_string(),
        "trima".to_string(),
        "trvi".to_string(),
        "trix".to_string(),
        "tsf".to_string(),
        "typprice".to_string(),
        "ultosc".to_string(),
        "vhf".to_string(),
        "vidya".to_string(),
        "volatility".to_string(),
        "vortex".to_string(),
        "vosc".to_string(),
        "vwap".to_string(),
        "vwma".to_string(),
        "wad".to_string(),
        "wcprice".to_string(),
        "wilders".to_string(),
        "willr".to_string(),
        "wma".to_string(),
        "zlema".to_string(),
    ]
}

/// Python module for TulipRS
#[pymodule]
fn tulip_rs(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Module metadata
    m.add("__version__", "0.1.10")?;
    m.add("__author__", "TulipRS Contributors")?;
    m.add(
        "__description__",
        "Python bindings for TulipRS Technical Analysis Library",
    )?;

    // Create indicators submodule
    let indicators_module = PyModule::new(py, "indicators")?;

    // Register all 84 indicators using bulk auto-registration
    bulk_register::register_all_indicator_modules(&indicators_module)?;

    // Add the indicators module to the main module
    m.add_submodule(&indicators_module)?;

    // Utility functions at module level
    m.add_function(wrap_pyfunction!(list_indicators, m)?)?;

    Ok(())
}
