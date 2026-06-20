# Benchmark: msw (Mesa Sine Wave)
from __future__ import annotations

from typing import Any, List

import tulipy

import tulip_rs
from tulip_rs_bench.common import BenchmarkDef, OhlcvArrays, PdOhlcvArrays


def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.msw.indicator([data.close], options)


def _tulipy(data: OhlcvArrays, options: List[float]) -> Any:
    return tulipy.msw(data.close, period=int(options[0]))


BENCHMARK = BenchmarkDef(
    name="msw",
    options_list=[[5.0], [8.0], [14.0], [20.0]],
    tulip_fn=_tulip,
    ref_fn=None,
    extra_refs={"tulipy": _tulipy},
)
