# Benchmark: adosc (Chaikin A/D Oscillator)
from __future__ import annotations

from typing import Any, List

import tulipy

import tulip_rs
from tulip_rs_bench.common import BenchmarkDef, OhlcvArrays, PdOhlcvArrays


def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.adosc.indicator(
        [data.high, data.low, data.close, data.volume], options
    )


def _tulipy(data: OhlcvArrays, options: List[float]) -> Any:
    return tulipy.adosc(
        data.high,
        data.low,
        data.close,
        data.volume,
        short_period=int(options[0]),
        long_period=int(options[1]),
    )


BENCHMARK = BenchmarkDef(
    name="adosc",
    options_list=[[2.0, 5.0], [3.0, 10.0], [5.0, 20.0], [10.0, 30.0]],
    tulip_fn=_tulip,
    ref_fn=None,
    extra_refs={"tulipy": _tulipy},
)
