# Benchmark: psar (Parabolic SAR)
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
    return tulip_rs.indicators.psar.indicator([data.high, data.low], options)


def _ref(data: PdOhlcvArrays, options: List[float]) -> Any:
    return ta.trend.PSARIndicator(
        high=data.high,
        low=data.low,
        close=data.close,
        step=options[0],
        max_step=options[1],
    ).psar_up()


BENCHMARK = BenchmarkDef(
    name="psar",
    options_list=[[0.02, 0.2], [0.01, 0.2], [0.02, 0.1], [0.04, 0.4]],
    tulip_fn=_tulip,
    ref_fn=_ref,
)
