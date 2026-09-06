# Benchmark: ppo (Percentage Price Oscillator)
from __future__ import annotations

from typing import Any, List

import ta.momentum

import tulip_rs
from tulip_rs_bench.common import (
    BenchmarkDef,
    OhlcvArrays,
    PdOhlcvArrays,
)


import pandas as pd
import pandas_ta as pta

def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.ppo.indicator([data.close], options)


def _simd_assets(stocks: List[OhlcvArrays], options: List[float]) -> Any:
    """Process every loaded stock's close series together via SIMD lanes."""
    inputs = [[stock.close] for stock in stocks]
    return tulip_rs.indicators.ppo.simd_by_assets(inputs, options, None)


def _simd_options(data: OhlcvArrays, options_list: List[List[float]]) -> Any:
    """Process every option set together via SIMD lanes for one stock."""
    return tulip_rs.indicators.ppo.simd_by_options([data.close], options_list, None)


def _ref(data: PdOhlcvArrays, options: List[float]) -> Any:
    return ta.momentum.PercentagePriceOscillator(
        close=data.close,
        window_fast=int(options[0]),
        window_slow=int(options[1]),
    ).ppo()


def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.ppo(pd.Series(data.close), fast=int(options[0]), slow=int(options[1]))

BENCHMARK = BenchmarkDef(
    name="ppo",
    options_list=[[12.0, 26.0], [8.0, 18.0], [5.0, 13.0], [3.0, 9.0]],
    tulip_fn=_tulip,
    ref_fn=_ref,
    extra_refs={"pandas_ta": _pta},
    simd_assets_fn=_simd_assets,
    simd_options_fn=_simd_options,
)
