# Benchmark: ad (Accumulation/Distribution)
from __future__ import annotations

from typing import Any, List

import ta.volume

import tulip_rs
from tulip_rs_bench.common import (
    BenchmarkDef,
    OhlcvArrays,
    PdOhlcvArrays,
)


def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.ad.indicator(
        [data.high, data.low, data.close, data.volume], options
    )


def _ref(data: PdOhlcvArrays, options: List[float]) -> Any:
    return ta.volume.AccDistIndexIndicator(
        high=data.high,
        low=data.low,
        close=data.close,
        volume=data.volume,
    ).acc_dist_index()


def _simd_assets(stocks: List[OhlcvArrays], options: List[float]) -> Any:
    """Process every loaded stock's series together via SIMD lanes."""
    inputs = [[stock.high, stock.low, stock.close, stock.volume] for stock in stocks]
    return tulip_rs.indicators.ad.simd_by_assets(inputs, options, None)


BENCHMARK = BenchmarkDef(
    name="ad",
    options_list=[[]],
    tulip_fn=_tulip,
    ref_fn=_ref,
    simd_assets_fn=_simd_assets,
)
