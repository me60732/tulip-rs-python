# Benchmark: cybercycle (Cyber Cycle)
from __future__ import annotations

from typing import Any, List

import pandas as pd
import pandas_ta as pta

import tulip_rs
from tulip_rs_bench.common import BenchmarkDef, OhlcvArrays, PdOhlcvArrays


def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.cybercycle.indicator([data.close], options)


def _simd_assets(stocks: List[OhlcvArrays], options: List[float]) -> Any:
    """Process every loaded stock's close series together via SIMD lanes."""
    inputs = [[stock.close] for stock in stocks]
    return tulip_rs.indicators.cybercycle.simd_by_assets(inputs, options, None)


def _simd_options(data: OhlcvArrays, options_list: List[List[float]]) -> Any:
    """Process every option set together via SIMD lanes for one stock."""
    return tulip_rs.indicators.cybercycle.simd_by_options([data.close], options_list, None)


BENCHMARK = BenchmarkDef(
    name="cybercycle",
    options_list=[[0.05], [0.07], [0.10], [0.15]],
    tulip_fn=_tulip,
    ref_fn=None,
    extra_refs=None,
    simd_assets_fn=_simd_assets,
    simd_options_fn=_simd_options,
)
