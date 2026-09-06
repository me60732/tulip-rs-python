# Benchmark: vwap (Volume-Weighted Average Price)
from __future__ import annotations

from typing import Any, List

import pandas as pd
import pandas_ta as pta

import tulip_rs
from tulip_rs_bench.common import BenchmarkDef, OhlcvArrays, PdOhlcvArrays


def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.vwap.indicator(
        [data.high, data.low, data.close, data.volume], []
    )


def _simd_assets(stocks: List[OhlcvArrays], options: List[float]) -> Any:
    """Process every loaded stock's series together via SIMD lanes."""
    inputs = [[stock.high, stock.low, stock.close, stock.volume] for stock in stocks]
    return tulip_rs.indicators.vwap.simd_by_assets(inputs, options, None)


def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    idx = pd.date_range("2020-01-01", periods=len(data.high), freq="D")
    return pta.vwap(
        pd.Series(data.high, index=idx),
        pd.Series(data.low, index=idx),
        pd.Series(data.close, index=idx),
        pd.Series(data.volume, index=idx),
    )


BENCHMARK = BenchmarkDef(
    name="vwap",
    options_list=[[]],
    tulip_fn=_tulip,
    ref_fn=None,
    extra_refs={"pandas_ta": _pta},
    simd_assets_fn=_simd_assets,
)
