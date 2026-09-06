# Benchmark: marketfi (Market Facilitation Index)
from __future__ import annotations

from typing import Any, List

import tulip_rs
from tulip_rs_bench.common import BenchmarkDef, OhlcvArrays, PdOhlcvArrays


def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.marketfi.indicator(
        [data.high, data.low, data.volume], options
    )


def _simd_assets(stocks: List[OhlcvArrays], options: List[float]) -> Any:
    """Process every loaded stock's series together via SIMD lanes."""
    inputs = [[stock.high, stock.low, stock.volume] for stock in stocks]
    return tulip_rs.indicators.marketfi.simd_by_assets(inputs, options, None)


BENCHMARK = BenchmarkDef(
    name="marketfi",
    options_list=[[]],
    tulip_fn=_tulip,
    ref_fn=None,
    simd_assets_fn=_simd_assets,
)
