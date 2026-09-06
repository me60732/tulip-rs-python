# Benchmark: emv (Ease of Movement)
from __future__ import annotations

from typing import Any, List

import ta.volume

import tulip_rs
from tulip_rs_bench.common import (
    BenchmarkDef,
    OhlcvArrays,
    PdOhlcvArrays,
)


import pandas as pd
import pandas_ta as pta
def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.emv.indicator(
        [data.high, data.low, data.volume], options
    )


def _ref(data: PdOhlcvArrays, options: List[float]) -> Any:
    return ta.volume.EaseOfMovementIndicator(
        high=data.high,
        low=data.low,
        volume=data.volume,
    ).ease_of_movement()


def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.eom(pd.Series(data.high), pd.Series(data.low), pd.Series(data.close), pd.Series(data.volume))

def _simd_assets(stocks: List[OhlcvArrays], options: List[float]) -> Any:
    """Process every loaded stock's series together via SIMD lanes."""
    inputs = [[stock.high, stock.low, stock.volume] for stock in stocks]
    return tulip_rs.indicators.emv.simd_by_assets(inputs, options, None)


BENCHMARK = BenchmarkDef(
    name="emv",
    options_list=[[]],
    tulip_fn=_tulip,
    ref_fn=_ref,
    extra_refs={"pandas_ta": _pta},
    simd_assets_fn=_simd_assets,
)
