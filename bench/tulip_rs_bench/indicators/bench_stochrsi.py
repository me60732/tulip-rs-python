# Benchmark: stochrsi (Stochastic RSI)
from __future__ import annotations

from typing import Any, List

import ta.momentum

import tulip_rs
from tulip_rs_bench.common import (
    BenchmarkDef,
    OhlcvArrays,
    PdOhlcvArrays,
)


def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.stochrsi.indicator([data.close], options)


def _ref(data: PdOhlcvArrays, options: List[float]) -> Any:
    return ta.momentum.StochRSIIndicator(
        close=data.close, window=int(options[0])
    ).stochrsi()


BENCHMARK = BenchmarkDef(
    name="stochrsi",
    options_list=[[14.0], [20.0], [25.0], [30.0]],
    tulip_fn=_tulip,
    ref_fn=_ref,
)
