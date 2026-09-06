# Benchmark: vwma (Volume-Weighted Moving Average)
from __future__ import annotations

from typing import Any, List



import tulip_rs
from tulip_rs_bench.common import BenchmarkDef, OhlcvArrays, PdOhlcvArrays


import pandas as pd
import pandas_ta as pta
def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.vwma.indicator([data.close, data.volume], options)


def _simd_assets(stocks: List[OhlcvArrays], options: List[float]) -> Any:
    """Process every loaded stock's close and volume series together via SIMD lanes."""
    inputs = [[stock.close, stock.volume] for stock in stocks]
    return tulip_rs.indicators.vwma.simd_by_assets(inputs, options, None)


def _simd_options(data: OhlcvArrays, options_list: List[List[float]]) -> Any:
    """Process every option set together via SIMD lanes for one stock."""
    return tulip_rs.indicators.vwma.simd_by_options([data.close, data.volume], options_list, None)





def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.vwma(pd.Series(data.close), pd.Series(data.volume), length=int(options[0]))

BENCHMARK = BenchmarkDef(
    name="vwma",
    options_list=[[5.0], [14.0], [20.0], [50.0]],
    tulip_fn=_tulip,
    ref_fn=None,
    extra_refs={"pandas_ta": _pta},
    simd_assets_fn=_simd_assets,
    simd_options_fn=_simd_options,
)
