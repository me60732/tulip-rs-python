# Benchmark: ao (Awesome Oscillator)
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
    return tulip_rs.indicators.ao.indicator([data.high, data.low], options)


def _ref(data: PdOhlcvArrays, options: List[float]) -> Any:
    return ta.momentum.AwesomeOscillatorIndicator(
        high=data.high, low=data.low
    ).awesome_oscillator()


BENCHMARK = BenchmarkDef(
    name="ao",
    options_list=[[]],
    tulip_fn=_tulip,
    ref_fn=_ref,
)
