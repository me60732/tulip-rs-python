# Benchmark: vosc (Volume Oscillator)
from __future__ import annotations

from typing import Any, List



import tulip_rs
from tulip_rs_bench.common import BenchmarkDef, OhlcvArrays, PdOhlcvArrays


def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.vosc.indicator([data.volume], options)


def _simd_assets(stocks: List[OhlcvArrays], options: List[float]) -> Any:
    """Process every loaded stock's volume series together via SIMD lanes."""
    inputs = [[stock.volume] for stock in stocks]
    return tulip_rs.indicators.vosc.simd_by_assets(inputs, options, None)


def _simd_options(data: OhlcvArrays, options_list: List[List[float]]) -> Any:
    """Process every option set together via SIMD lanes for one stock."""
    return tulip_rs.indicators.vosc.simd_by_options([data.volume], options_list, None)





BENCHMARK = BenchmarkDef(
    name="vosc",
    options_list=[[5.0, 20.0], [9.0, 26.0], [12.0, 26.0], [3.0, 10.0]],
    tulip_fn=_tulip,
    ref_fn=None,
    simd_assets_fn=_simd_assets,
    simd_options_fn=_simd_options,
)
