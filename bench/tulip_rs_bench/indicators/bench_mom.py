# Benchmark: mom (Momentum)
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
    return tulip_rs.indicators.mom.indicator([data.close], options)


def _simd_assets(stocks: List[OhlcvArrays], options: List[float]) -> Any:
    """Process every loaded stock's close series together via SIMD lanes."""
    inputs = [[stock.close] for stock in stocks]
    return tulip_rs.indicators.mom.simd_by_assets(inputs, options, None)


def _simd_options(data: OhlcvArrays, options_list: List[List[float]]) -> Any:
    """Process every option set together via SIMD lanes for one stock."""
    return tulip_rs.indicators.mom.simd_by_options([data.close], options_list, None)


def _ref(data: PdOhlcvArrays, options: List[float]) -> Any:
    return data.close.diff(int(options[0]))






def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.mom(pd.Series(data.close), length=int(options[0]))

BENCHMARK = BenchmarkDef(
    name="mom",
    options_list=[[25.0], [30.0], [50.0], [100.0]],
    tulip_fn=_tulip,
    ref_fn=_ref,
    extra_refs={"pandas_ta": _pta},
    simd_assets_fn=_simd_assets,
    simd_options_fn=_simd_options,
)
