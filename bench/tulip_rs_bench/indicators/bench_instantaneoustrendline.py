# Benchmark: instantaneoustrendline (Instantaneous Trendline)
from __future__ import annotations

from typing import Any, List

import pandas as pd
import pandas_ta as pta

import tulip_rs
from tulip_rs_bench.common import BenchmarkDef, OhlcvArrays, PdOhlcvArrays


def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.instantaneoustrendline.indicator([data.close], [])


def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.ht_trendline(pd.Series(data.close))


def _simd_assets(stocks: List[OhlcvArrays], options: List[float]) -> Any:
    """Process every loaded stock's close series together via SIMD lanes."""
    inputs = [[stock.close] for stock in stocks]
    return tulip_rs.indicators.instantaneoustrendline.simd_by_assets(inputs, options, None)


BENCHMARK = BenchmarkDef(
    name="instantaneoustrendline",
    options_list=[[]],
    tulip_fn=_tulip,
    ref_fn=None,
    extra_refs={"pandas_ta": _pta},
    simd_assets_fn=_simd_assets,
)
