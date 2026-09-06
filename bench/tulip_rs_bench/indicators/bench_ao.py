# Benchmark: ao (Awesome Oscillator)
from __future__ import annotations

from typing import Any, List

import pandas as pd
import pandas_ta as pta
import ta.momentum

import tulip_rs
from tulip_rs_bench.common import (
    BenchmarkDef,
    OhlcvArrays,
    PdOhlcvArrays,
)


def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.ao.indicator([data.high, data.low], options)


def _ref(data: PdOhlcvArrays, options: List[float]) -> Any:
    return ta.momentum.AwesomeOscillatorIndicator(
        high=data.high, low=data.low
    ).awesome_oscillator()


def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.ao(pd.Series(data.high), pd.Series(data.low))


def _simd_assets(stocks: List[OhlcvArrays], options: List[float]) -> Any:
    """Process every loaded stock's series together via SIMD lanes."""
    inputs = [[stock.high, stock.low] for stock in stocks]
    return tulip_rs.indicators.ao.simd_by_assets(inputs, options, None)


BENCHMARK = BenchmarkDef(
    name="ao",
    options_list=[[]],
    tulip_fn=_tulip,
    ref_fn=_ref,
    extra_refs={"pandas_ta": _pta},
    simd_assets_fn=_simd_assets,
)
