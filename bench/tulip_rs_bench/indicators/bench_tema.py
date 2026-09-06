# Benchmark: tema (Triple Exponential Moving Average)
from __future__ import annotations

from typing import Any, List



import tulip_rs
from tulip_rs_bench.common import (
    BenchmarkDef,
    OhlcvArrays,
    PdOhlcvArrays,
)


import pandas as pd
import pandas_ta as pta
def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.tema.indicator([data.close], options)


def _simd_assets(stocks: List[OhlcvArrays], options: List[float]) -> Any:
    """Process every loaded stock's close series together via SIMD lanes."""
    inputs = [[stock.close] for stock in stocks]
    return tulip_rs.indicators.tema.simd_by_assets(inputs, options, None)


def _simd_options(data: OhlcvArrays, options_list: List[List[float]]) -> Any:
    """Process every option set together via SIMD lanes for one stock."""
    return tulip_rs.indicators.tema.simd_by_options([data.close], options_list, None)


def _ref(data: PdOhlcvArrays, options: List[float]) -> Any:
    n = int(options[0])
    ema1 = data.close.ewm(span=n, adjust=False).mean()
    ema2 = ema1.ewm(span=n, adjust=False).mean()
    ema3 = ema2.ewm(span=n, adjust=False).mean()
    return 3 * ema1 - 3 * ema2 + ema3





def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.tema(pd.Series(data.close), length=int(options[0]))

BENCHMARK = BenchmarkDef(
    name="tema",
    options_list=[[5.0], [14.0], [20.0], [50.0]],
    tulip_fn=_tulip,
    ref_fn=_ref,
    extra_refs={"pandas_ta": _pta},
    simd_assets_fn=_simd_assets,
    simd_options_fn=_simd_options,
)
