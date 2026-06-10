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


def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.ppo.indicator([data.close], options)


def _ref(data: PdOhlcvArrays, options: List[float]) -> Any:
    return ta.momentum.PercentagePriceOscillator(
        close=data.close,
        window_fast=int(options[0]),
        window_slow=int(options[1]),
    ).ppo()


BENCHMARK = BenchmarkDef(
    name="ppo",
    options_list=[[12.0, 26.0], [8.0, 18.0], [5.0, 13.0], [3.0, 9.0]],
    tulip_fn=_tulip,
    ref_fn=_ref,
)
