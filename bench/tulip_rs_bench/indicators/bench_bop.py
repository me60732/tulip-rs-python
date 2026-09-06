# Benchmark: bop (Balance of Power)
from __future__ import annotations

from typing import Any, List

import tulip_rs
from tulip_rs_bench.common import BenchmarkDef, OhlcvArrays, PdOhlcvArrays


import pandas as pd
import pandas_ta as pta
def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.bop.indicator(
        [data.open, data.high, data.low, data.close], options
    )


def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.bop(pd.Series(data.open), pd.Series(data.high), pd.Series(data.low), pd.Series(data.close))

def _simd_assets(stocks: List[OhlcvArrays], options: List[float]) -> Any:
    """Process every loaded stock's series together via SIMD lanes."""
    inputs = [[stock.open, stock.high, stock.low, stock.close] for stock in stocks]
    return tulip_rs.indicators.bop.simd_by_assets(inputs, options, None)


BENCHMARK = BenchmarkDef(
    name="bop",
    options_list=[[]],
    tulip_fn=_tulip,
    ref_fn=None,
    extra_refs={"pandas_ta": _pta},
    simd_assets_fn=_simd_assets,
)
