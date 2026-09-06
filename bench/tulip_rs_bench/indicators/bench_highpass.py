# Benchmark: highpass (High Pass Filter)
from __future__ import annotations

from typing import Any, List

import pandas as pd
import pandas_ta as pta

import tulip_rs
from tulip_rs_bench.common import BenchmarkDef, OhlcvArrays, PdOhlcvArrays


def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.highpass.indicator([data.close], options)


def _simd_assets(stocks: List[OhlcvArrays], options: List[float]) -> Any:
    """Process every loaded stock's close series together via SIMD lanes."""
    inputs = [[stock.close] for stock in stocks]
    return tulip_rs.indicators.highpass.simd_by_assets(inputs, options, None)


def _simd_options(data: OhlcvArrays, options_list: List[List[float]]) -> Any:
    """Process every option set together via SIMD lanes for one stock."""
    return tulip_rs.indicators.highpass.simd_by_options([data.close], options_list, None)


BENCHMARK = BenchmarkDef(
    name="highpass",
    options_list=[[20.0], [40.0], [60.0], [80.0]],
    tulip_fn=_tulip,
    ref_fn=None,
    extra_refs=None,
    simd_assets_fn=_simd_assets,
    simd_options_fn=_simd_options,
)
