# Benchmark: pvi (Positive Volume Index)
from __future__ import annotations

from typing import Any, List

import tulip_rs
from tulip_rs_bench.common import BenchmarkDef, OhlcvArrays, PdOhlcvArrays


import pandas as pd
import pandas_ta as pta
def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.pvi.indicator([data.close, data.volume], options)


def _simd_assets(stocks: List[OhlcvArrays], options: List[float]) -> Any:
    """Process every loaded stock's series together via SIMD lanes."""
    inputs = [[stock.close, stock.volume] for stock in stocks]
    return tulip_rs.indicators.pvi.simd_by_assets(inputs, options, None)


def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.pvi(pd.Series(data.close), pd.Series(data.volume))

BENCHMARK = BenchmarkDef(
    name="pvi",
    options_list=[[]],
    tulip_fn=_tulip,
    ref_fn=None,
    extra_refs={"pandas_ta": _pta},
    simd_assets_fn=_simd_assets,
)
