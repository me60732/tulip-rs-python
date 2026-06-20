# Benchmark: ao (Awesome Oscillator)
from __future__ import annotations

from typing import Any, List

import ta.momentum
import tulipy

import tulip_rs
from tulip_rs_bench.common import (
    BenchmarkDef,
    OhlcvArrays,
    PdOhlcvArrays,
)


import pandas as pd
import pandas_ta as pta
def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.ao.indicator([data.high, data.low], options)


def _ref(data: PdOhlcvArrays, options: List[float]) -> Any:
    return ta.momentum.AwesomeOscillatorIndicator(
        high=data.high, low=data.low
    ).awesome_oscillator()


def _tulipy(data: OhlcvArrays, options: List[float]) -> Any:
    return tulipy.ao(data.high, data.low)


def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.ao(pd.Series(data.high), pd.Series(data.low))

BENCHMARK = BenchmarkDef(
    name="ao",
    options_list=[[]],
    tulip_fn=_tulip,
    ref_fn=_ref,
    extra_refs={"tulipy": _tulipy, "pandas_ta": _pta},
)
