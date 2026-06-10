# tulip-rs-bench

Python benchmark suite comparing the **tulip-rs Python binding** against
[pandas-ta](https://github.com/twopirllc/pandas-ta) — a pure Python/numpy/pandas
technical-analysis library.

This package lives inside the `tulip_rs_python` repository, mirroring the way
`tulip_test/` sits inside the main Rust workspace — a self-contained sub-package
with its own dependency declaration.

Libraries that wrap C Tulip or TA-Lib (e.g. `tulipy`) are intentionally excluded;
the interesting comparison is pure-Python implementations vs a compiled Rust binding.

---

## Setup

### 1. Build and install the tulip-rs Python binding

```bash
cd ..                          # tulip_rs_python/
maturin develop --release
```

### 2. Install the benchmark package

```bash
cd bench/
pip install -e .
```

`pip install -e .` reads `pyproject.toml` and installs all dependencies:
`tulip-rs`, `pandas-ta`, `psycopg2-binary`, `python-dotenv`, `numpy`, `pandas`.
The `tulip-rs-bench` console script is registered automatically.

### 3. Configure environment

```bash
cp .env.example .env
# Edit .env — set STOCKS_DATABASE_URL, BENCHMARK_DATABASE_URL
# Set BENCHMARK_LOG_TO_DB=1 to write results to the database
```

### 4. Start the database (required for real OHLCV data)

```bash
cd ../../tulip_rs/tulip_test/docker
docker compose up -d
```

---

## Running

```bash
# All indicators — dry run (stdout only)
tulip-rs-bench

# Specific indicators only
tulip-rs-bench ema rsi macd

# Write results to the indicator_benchmark database
BENCHMARK_LOG_TO_DB=1 tulip-rs-bench

# Or equivalently
python -m tulip_rs_bench.run_all
python -m tulip_rs_bench.run_all ema rsi
```

### Example output

```
════════════════════════════════════════════════════════════════
  tulip-rs Python Benchmark Suite
════════════════════════════════════════════════════════════════

[1/3] Loading stock data …
  loaded 6,705 bars  BHP/ASX
  loaded 6,705 bars  CBA/ASX
  loaded 6,705 bars  AAPL/NYSE
  loaded 6,705 bars  MSFT/NYSE

[2/3] DB logging disabled (BENCHMARK_LOG_TO_DB=0) — stdout only

[3/3] Running benchmarks …

────────────────────────────────────────────────────────────────
  EMA
────────────────────────────────────────────────────────────────
    tulip_rs_python      BHP_ASX        [14]       45.2 µs ± 1.1
    pandas_ta            BHP_ASX        [14]      872.4 µs ± 18.3  ×19.3 slower than tulip_rs
    ...
```

---

## Timing methodology

| Item | Detail |
|------|--------|
| Data | Real OHLCV, **6,705 bars** per stock (BHP/ASX, CBA/ASX, AAPL/NYSE, MSFT/NYSE) |
| Samples | 30 independent timing runs (`BENCH_REPEAT`) |
| Calls/sample | 10 back-to-back calls (`BENCH_NUMBER`) |
| Reported time | Mean of 30 samples, each averaged over 10 calls |
| Unit | Microseconds (µs) on screen; nanoseconds (ns) in the database |
| Conversion | `pd.Series` built **once before** the timed region — conversion excluded |

---

## Database views

Apply the Python-specific views to the existing `indicator_benchmark` database:

```bash
psql -U postgres -h localhost -d indicator_benchmark \
     -f ../../tulip_rs/tulip_test/docker/scripts/03_python_benchmark_views.sql
```

Query results:

```sql
-- Averaged across all option sets, sorted by speedup
SELECT indicator_name, tulip_rs_python_avg_ns, pandas_ta_avg_ns, pandas_ta_to_tulip_ratio
FROM python_avg_options_comparison
WHERE run_id = (SELECT max(id) FROM benchmark_runs)
ORDER BY pandas_ta_to_tulip_ratio DESC;
```

---

## Adding a new indicator

1. Create `tulip_rs_bench/indicators/bench_<name>.py`.
2. Define `_tulip`, `_pandas_ta`, and `BENCHMARK = BenchmarkDef(...)`.
3. `tulip-rs-bench` discovers it automatically — no registration needed.

```python
# tulip_rs_bench/indicators/bench_myindicator.py
from __future__ import annotations
from typing import Any, List
import tulip_rs, pandas_ta
from tulip_rs_bench.common import BenchmarkDef, OhlcvArrays, PdOhlcvArrays

def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.myindicator.indicator([data.close], options)

def _pandas_ta(data: PdOhlcvArrays, options: List[float]) -> Any:
    return pandas_ta.myindicator(data.close, length=int(options[0]))

BENCHMARK = BenchmarkDef(
    name         = "myindicator",
    options_list = [[14.0], [20.0], [30.0], [50.0]],
    tulip_fn     = _tulip,
    pandas_ta_fn = _pandas_ta,  # None if pandas_ta doesn't implement it
)
```

---

## Indicators covered (20)

| File | Indicator | Inputs | pandas-ta fn |
|------|-----------|--------|--------------|
| bench_ema.py | EMA | close | `ema` |
| bench_sma.py | SMA | close | `sma` |
| bench_rsi.py | RSI | close | `rsi` |
| bench_roc.py | ROC | close | `roc` |
| bench_mom.py | Momentum | close | `mom` |
| bench_wma.py | WMA | close | `wma` |
| bench_dema.py | DEMA | close | `dema` |
| bench_tema.py | TEMA | close | `tema` |
| bench_hma.py | HMA | close | `hma` |
| bench_kama.py | KAMA | close | `kama` |
| bench_macd.py | MACD | close | `macd` |
| bench_bbands.py | Bollinger Bands | close | `bbands` |
| bench_atr.py | ATR | H/L/C | `atr` |
| bench_adx.py | ADX | H/L/C | `adx` |
| bench_cci.py | CCI | H/L/C | `cci` |
| bench_stoch.py | Stochastic | H/L/C | `stoch` |
| bench_willr.py | Williams %R | H/L/C | `willr` |
| bench_aroon.py | Aroon | H/L | `aroon` |
| bench_obv.py | OBV | close+vol | `obv` |
| bench_mfi.py | MFI | H/L/C/V | `mfi` |
