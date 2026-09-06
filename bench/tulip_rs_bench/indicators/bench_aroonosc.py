# Benchmark: aroonosc (Aroon Oscillator)
from __future__ import annotations

from typing import Any, List

import tulip_rs
from tulip_rs_bench.common import BenchmarkDef, OhlcvArrays, PdOhlcvArrays


import pandas as pd
import pandas_ta as pta
def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.aroonosc.indicator([data.high, data.low], options)


def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    result = pta.aroon(pd.Series(data.high), pd.Series(data.low), length=int(options[0]))
    return result.iloc[:, -1]


def _simd_assets(stocks: List[OhlcvArrays], options: List[float]) -> Any:
    """Process every loaded stock's series together via SIMD lanes."""
    inputs = [[stock.high, stock.low] for stock in stocks]
    return tulip_rs.indicators.aroonosc.simd_by_assets(inputs, options, None)


def _simd_options(data: OhlcvArrays, options_list: List[List[float]]) -> Any:
    """Process every option set together via SIMD lanes for one stock."""
    return tulip_rs.indicators.aroonosc.simd_by_options([data.high, data.low], options_list, None)


BENCHMARK = BenchmarkDef(
    name="aroonosc",
    options_list=[[25.0], [35.0], [50.0], [100.0]],
    tulip_fn=_tulip,
    ref_fn=None,
    extra_refs={"pandas_ta": _pta},
    simd_assets_fn=_simd_assets,
    simd_options_fn=_simd_options,
)
