# Benchmark: adosc (Chaikin A/D Oscillator)
from __future__ import annotations

from typing import Any, List

import tulip_rs
from tulip_rs_bench.common import BenchmarkDef, OhlcvArrays, PdOhlcvArrays


def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.adosc.indicator(
        [data.high, data.low, data.close, data.volume], options
    )


def _simd_assets(stocks: List[OhlcvArrays], options: List[float]) -> Any:
    """Process every loaded stock's series together via SIMD lanes."""
    inputs = [[stock.high, stock.low, stock.close, stock.volume] for stock in stocks]
    return tulip_rs.indicators.adosc.simd_by_assets(inputs, options, None)


def _simd_options(data: OhlcvArrays, options_list: List[List[float]]) -> Any:
    """Process every option set together via SIMD lanes for one stock."""
    return tulip_rs.indicators.adosc.simd_by_options([data.high, data.low, data.close, data.volume], options_list, None)


BENCHMARK = BenchmarkDef(
    name="adosc",
    options_list=[[2.0, 5.0], [3.0, 10.0], [5.0, 20.0], [10.0, 30.0]],
    tulip_fn=_tulip,
    ref_fn=None,
    simd_assets_fn=_simd_assets,
    simd_options_fn=_simd_options,
)
