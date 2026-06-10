# Benchmark: sma (Simple Moving Average)
from __future__ import annotations

from typing import Any, List

import ta.trend

import tulip_rs
from tulip_rs_bench.common import (
    BenchmarkDef,
    OhlcvArrays,
    PdOhlcvArrays,
)


def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.sma.indicator([data.close], options)


def _ref(data: PdOhlcvArrays, options: List[float]) -> Any:
    return ta.trend.SMAIndicator(
        close=data.close, window=int(options[0])
    ).sma_indicator()


BENCHMARK = BenchmarkDef(
    name="sma",
    options_list=[[50.0], [100.0], [200.0], [300.0]],
    tulip_fn=_tulip,
    ref_fn=_ref,
)