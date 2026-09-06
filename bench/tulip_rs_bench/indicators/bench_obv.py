# Benchmark: obv (On Balance Volume)
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
    return tulip_rs.indicators.obv.indicator([data.close, data.volume], options)


def _simd_assets(stocks: List[OhlcvArrays], options: List[float]) -> Any:
    """Process every loaded stock's series together via SIMD lanes."""
    inputs = [[stock.close, stock.volume] for stock in stocks]
    return tulip_rs.indicators.obv.simd_by_assets(inputs, options, None)


def _ref(data: PdOhlcvArrays, options: List[float]) -> Any:
    return ta.volume.OnBalanceVolumeIndicator(
        close=data.close, volume=data.volume
    ).on_balance_volume()


def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.obv(pd.Series(data.close), pd.Series(data.volume))

BENCHMARK = BenchmarkDef(
    name="obv",
    options_list=[[]],
    tulip_fn=_tulip,
    ref_fn=_ref,
    extra_refs={"pandas_ta": _pta},
    simd_assets_fn=_simd_assets,
)
