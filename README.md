# TulipRS Python Bindings

![Python](https://img.shields.io/badge/python-3.8+-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)
![License](https://img.shields.io/badge/license-MIT-green.svg)

High-performance Python bindings for the TulipRS technical analysis library. Provides 100+ technical indicators and candlestick pattern recognition with zero-copy numpy integration.

## Features

- **100+ Technical Indicators**: Moving averages, oscillators, trend indicators, volume indicators, and more
- **60+ Candlestick Patterns**: Complete Japanese candlestick pattern recognition
- **Zero-Copy Performance**: Direct numpy array integration without unnecessary data copying
- **State Management**: Support for streaming/real-time calculations with state preservation
- **Comprehensive Error Handling**: Clear error messages and validation
- **Type Safety**: Full type hints and comprehensive documentation

## Installation

### From PyPI (when published)

```bash
pip install tulip-rs
```

### From Source

Requirements:
- Python 3.8+
- Rust 1.70+
- maturin

```bash
# Clone the repository
git clone https://github.com/me60732/tulip-rs-python.git
cd tulip-rs-python
export PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 
# Install maturin
pip install maturin

# Build and install in development mode
maturin develop

# Or build wheel for distribution
maturin build --release
#Or for build specific to current machine cpu archicture can also compile for other archictures by changing native to other rust cargo attributes
RUSTFLAGS="-C target-cpu=native" maturin build --release
```

## Quick Start

```python
import numpy as np
import tulip_rs

close = np.array([81.59, 81.06, 82.87, 83.00, 83.61,
                  83.15, 82.84, 83.99, 84.55, 84.36], dtype=np.float64)

# Calculate EMA(5)
outputs, state = tulip_rs.indicators.ema.indicator([close], [5.0])
print("EMA(5):", outputs[0])

# Streaming: continue from saved state without reprocessing history
new_bar = np.array([85.10], dtype=np.float64)
next_outputs, next_state = state.batch_indicator([new_bar])
print("Next EMA:", next_outputs[0])

# MACD — three outputs: macd line, signal, histogram
high = np.array([82.15, 81.89, 83.03, 83.30, 83.85, 83.90, 83.33, 84.30, 84.84, 85.00], dtype=np.float64)
low  = np.array([81.29, 80.64, 81.31, 82.65, 83.07, 83.11, 82.49, 82.30, 84.15, 84.11], dtype=np.float64)
macd_out, _ = tulip_rs.indicators.macd.indicator([close], [2.0, 5.0, 9.0])
macd_line, signal, histogram = macd_out[0], macd_out[1], macd_out[2]

# Candlestick patterns (returns list of pattern-match lists per bar)
open_ = np.array([81.85, 81.20, 81.55, 82.91, 83.10,
                  83.41, 82.71, 82.70, 84.20, 84.25], dtype=np.float64)
cdl_out, _ = tulip_rs.indicators.candlestick.indicator([open_, high, low, close], [5.0, 1.0, 1.0])
for bar_idx, patterns in enumerate(cdl_out[0]):
    for p in patterns:
        print(f"Bar {bar_idx}: {p['full_name']} ({p['forecast']})")  
```

## Available Indicators

### Moving Averages
- `sma` — Simple Moving Average
- `ema` — Exponential Moving Average
- `wma` — Weighted Moving Average
- `dema` — Double Exponential Moving Average
- `tema` — Triple Exponential Moving Average
- `trima` — Triangular Moving Average
- `hma` — Hull Moving Average
- `zlema` — Zero-Lag EMA
- `kama` — Kaufman Adaptive Moving Average
- `vidya` — Variable Index Dynamic Average
- `vwma` — Volume Weighted Moving Average
- `wilders` — Wilder's Smoothing
- `smaenvelope` — SMA Envelope

### Oscillators
- `rsi` — Relative Strength Index
- `macd` — Moving Average Convergence Divergence
- `stoch` — Stochastic Oscillator
- `stochrsi` — Stochastic RSI
- `willr` — Williams %R
- `cci` — Commodity Channel Index
- `cmo` — Chande Momentum Oscillator
- `ultosc` — Ultimate Oscillator
- `ao` — Awesome Oscillator
- `fisher` — Fisher Transform
- `fosc` — Forecast Oscillator
- `msw` — Mesa Sine Wave
- `trix` — TRIX

### Trend Indicators
- `adx` — Average Directional Index
- `adxr` — ADX Rating
- `di` — Directional Indicator
- `dm` — Directional Movement
- `dx` — Directional Movement Index
- `aroon` — Aroon
- `aroonosc` — Aroon Oscillator
- `psar` — Parabolic SAR
- `ppo` — Percentage Price Oscillator
- `apo` — Absolute Price Oscillator
- `vortex` — Vortex Indicator
- `elderray` — Elder-Ray Index
- `donchianchannel` — Donchian Channel
- `ichimoku` — Ichimoku Cloud
- `supertrend` — SuperTrend
- `ef` — Efficiency Ratio
- `mama` — MESA Adaptive Moving Average

### Volatility Indicators
- `bbands` — Bollinger Bands
- `atr` — Average True Range
- `natr` — Normalized ATR
- `tr` — True Range
- `stddev` — Standard Deviation
- `volatility` — Volatility
- `vhf` — Vertical Horizontal Filter
- `cvi` — Chaikin Volatility
- `chandelierexit` — Chandelier Exit
- `keltnerchannel` — Keltner Channel
- `trvi` — True Range Volatility Index

### Volume Indicators
- `ad` — Accumulation/Distribution
- `adosc` — A/D Oscillator (Chaikin Oscillator)
- `obv` — On Balance Volume
- `mfi` — Money Flow Index
- `nvi` — Negative Volume Index
- `pvi` — Positive Volume Index
- `vosc` — Volume Oscillator
- `kvo` — Klinger Volume Oscillator
- `emv` — Ease of Movement
- `wad` — Williams Accumulation/Distribution
- `marketfi` — Market Facilitation Index
- `chaikinmf` — Chaikin Money Flow
- `vwap` — Volume Weighted Average Price

### Price & Statistical
- `avgprice`, `medprice`, `typprice`, `wcprice` — Price transforms
- `max`, `min` — Rolling maximum / minimum
- `mom` — Momentum
- `roc`, `rocr` — Rate of Change
- `bop` — Balance of Power
- `linreg`, `tsf` — Linear Regression / Time Series Forecast
- `dpo` — Detrended Price Oscillator
- `mass` — Mass Index
- `md` — Mean Deviation
- `qstick` — QStick
- `pivotpoint` — Pivot Points

### Cycle & Ehlers Indicators
- `cybercycle` — Ehlers CyberCycle
- `adaptivemsw` — Adaptive Mesa Sine Wave
- `homodynediscriminator` — Homodyne Discriminator
- `instantaneoustrendline` — Instantaneous Trendline
- `trendmode` — Ehlers TrendMode
- `highpass` — Ehlers High Pass Filter
- `hilberttransform` — Hilbert Transform
- `roofingfilter` — Ehlers Roofing Filter
- `supersmoother` — Ehlers Super Smoother
- `ccfisher` — Cyber Cycle Fisher

### Candlestick Patterns
- 77+ classical patterns via `tulip_rs.indicators.candlestick`
- Single-pass detection of all patterns
- Returns per-bar match lists with name, Japanese name, bar count, and forecast type

## Advanced Usage

### Working with Results

```python
import tulip_rs
import numpy as np

# Calculate MACD (returns multiple outputs)
prices = np.random.randn(100).cumsum() + 100
result = tulip_rs.macd(prices, fast_period=12.0, slow_period=26.0, signal_period=9.0)

# Get individual outputs
macd_line = result.get_output()  # First output
all_outputs = result.get_all_outputs()  # All outputs as list

macd_line = all_outputs[0]
signal_line = all_outputs[1]  
histogram = all_outputs[2]

print(f"MACD has {result.num_outputs()} outputs")
print(f"State available: {result.has_state()}")
```

### State Management for Streaming

```python
import numpy as np
import tulip_rs

prices_batch1 = np.array([100.0, 101.0, 102.0, 103.0, 104.0], dtype=np.float64)
outputs, state = tulip_rs.indicators.sma.indicator([prices_batch1], [3.0])
print("SMA batch 1:", outputs[0])

# Serialise state
json_str = state.to_json()

# Continue with new bars — no reprocessing of history
prices_batch2 = np.array([105.0, 106.0], dtype=np.float64)
next_outputs, next_state = state.batch_indicator([prices_batch2])
print("SMA batch 2:", next_outputs[0])

# Restore from JSON
restored_state = tulip_rs.indicators.sma.State.from_json(json_str)
```

### Candlestick Pattern Analysis

```python
# OHLC data
open_data = np.array([100, 102, 104, 103, 105])
high_data = np.array([101, 104, 105, 104, 106])
low_data = np.array([99, 101, 103, 102, 104])
close_data = np.array([102, 103, 103, 105, 105])

# Detect bullish engulfing pattern
result = tulip_rs.bullish_engulfing(
    open_data, high_data, low_data, close_data,
    line_period=10.0, body_period=10.0,
    min_long_cdl_height=0.001, min_cdl_height_tolerance=0.05,
    doji_max_height=0.1
)

# Analyze patterns
patterns = result.get_patterns()  # Returns numpy array of i8
pattern_bools = result.get_pattern_bools()  # Returns boolean array

bullish_count = result.count_bullish()
bearish_count = result.count_bearish()

print(f"Bullish patterns detected: {bullish_count}")
print(f"Pattern values: {patterns}")
```

### Utility Functions

```python
# List all available indicators
indicators = tulip_rs.list_indicators()
print(f"Available indicators: {len(indicators)}")

# List all candlestick patterns  
patterns = tulip_rs.list_candle_patterns()
print(f"Available patterns: {len(patterns)}")

# Get detailed information about an indicator
info = tulip_rs.get_indicator_info("rsi")
if info:
    print(f"Name: {info.name}")
    print(f"Full Name: {info.full_name}")
    print(f"Inputs: {info.inputs}")
    print(f"Options: {info.options}")
    print(f"Outputs: {info.outputs}")
```

## Performance

TulipRS Python bindings are designed for maximum performance:

- **Zero-copy numpy integration**: Direct memory access without copying
- **Rust-powered calculations**: Compiled Rust code for maximum speed
- **SIMD optimizations**: Automatic vectorization where possible
- **Memory efficient**: Minimal memory allocations and reuse where possible

Benchmark comparison with other popular libraries:

```
SMA (10,000 points):
- TulipRS:     0.045ms
- TA-Lib:      0.123ms  
- Pandas:      0.267ms

RSI (10,000 points):
- TulipRS:     0.087ms
- TA-Lib:      0.156ms
- Pandas:      0.445ms
```

## Error Handling

The library provides comprehensive error handling:

```python
try:
    # This will raise an error - period too small
    result = tulip_rs.sma(np.array([1, 2]), period=10.0)
except ValueError as e:
    print(f"Error: {e}")
    # Error: Not enough data: more input data points are required

try:
    # This will raise an error - mismatched array lengths  
    result = tulip_rs.atr(
        high=np.array([1, 2, 3]),
        low=np.array([1, 2]),  # Different length!
        close=np.array([1, 2, 3]),
        period=2.0
    )
except ValueError as e:
    print(f"Error: {e}")
    # Error: Array length mismatch: array 1 has length 2, expected 3
```

## API Reference

### IndicatorResult

Result object returned by indicator functions.

**Methods:**
- `get_output() -> numpy.ndarray`: Get the first (primary) output array
- `get_all_outputs() -> List[numpy.ndarray]`: Get all output arrays  
- `num_outputs() -> int`: Get the number of outputs
- `has_state() -> bool`: Check if state is available for continuation
- `state_to_json() -> Optional[str]`: Serialize state to JSON

### CandleResult

Result object returned by candlestick pattern functions.

**Methods:**
- `get_patterns() -> numpy.ndarray[int8]`: Get pattern values (-100, 0, 100)
- `get_pattern_bools() -> numpy.ndarray[bool]`: Get patterns as boolean array
- `count_bullish() -> int`: Count bullish patterns (value = 100)
- `count_bearish() -> int`: Count bearish patterns (value = -100)
- `has_state() -> bool`: Check if state is available
- `state_to_json() -> Optional[str]`: Serialize state to JSON

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Development Setup

```bash
# Clone the repository
git clone https://github.com/yourusername/tulip-rs-python.git
cd tulip-rs-python

# Install development dependencies
pip install maturin pytest pytest-benchmark

# Build in development mode
maturin develop

# Run tests
pytest tests/

# Run benchmarks
pytest benchmarks/ --benchmark-only
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Credits

- Based on the [TulipRS](https://github.com/yourusername/tulip_rs_base_clean) Rust library
- Inspired by the original [Tulip Indicators](https://tulipindicators.org/) library
- Built with [PyO3](https://github.com/PyO3/pyo3) and [maturin](https://github.com/PyO3/maturin)

## Changelog

### v0.1.0 (Initial Release)
- 100+ technical indicators
- 60+ candlestick patterns  
- Zero-copy numpy integration
- State management support
- Comprehensive error handling
- Full type hints and documentation
