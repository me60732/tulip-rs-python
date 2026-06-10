# Benchmark: mass (Mass Index)
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
    return tulip_rs.indicators.mass.indicator([data.high, data.low], options)


def _ref(data: PdOhlcvArrays, options: List[float]) -> Any:
    return ta.trend.MassIndex(
        high=data.high, low=data.low, window_fast=9, window_slow=int(options[0])
    ).mass_index()


BENCHMARK = BenchmarkDef(
    name="mass",
    options_list=[[14.0], [20.0], [25.0], [30.0]],
    tulip_fn=_tulip,
    ref_fn=_ref,
)
