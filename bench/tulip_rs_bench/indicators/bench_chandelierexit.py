# Benchmark: chandelierexit (Chandelier Exit)
from __future__ import annotations

from typing import Any, List

import tulip_rs
from tulip_rs_bench.common import BenchmarkDef, OhlcvArrays, PdOhlcvArrays


import pandas as pd
import pandas_ta as pta
def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.chandelierexit.indicator(
        [data.high, data.low, data.close], options
    )


def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.chandelier_exit(pd.Series(data.high), pd.Series(data.low), pd.Series(data.close))


def _simd_assets(stocks: List[OhlcvArrays], options: List[float]) -> Any:
    """Process every loaded stock's series together via SIMD lanes."""
    inputs = [[stock.high, stock.low, stock.close] for stock in stocks]
    return tulip_rs.indicators.chandelierexit.simd_by_assets(inputs, options, None)


def _simd_options(data: OhlcvArrays, options_list: List[List[float]]) -> Any:
    """Process every option set together via SIMD lanes for one stock."""
    return tulip_rs.indicators.chandelierexit.simd_by_options([data.high, data.low, data.close], options_list, None)


BENCHMARK = BenchmarkDef(
    name="chandelierexit",
    options_list=[[14.0, 3.0], [20.0, 3.0], [22.0, 3.0], [22.0, 2.0]],
    tulip_fn=_tulip,
    ref_fn=None,
    extra_refs={"pandas_ta": _pta},
    simd_assets_fn=_simd_assets,
    simd_options_fn=_simd_options,
)
