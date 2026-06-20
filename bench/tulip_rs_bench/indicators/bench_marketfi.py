# Benchmark: marketfi (Market Facilitation Index)
from __future__ import annotations

from typing import Any, List

import tulipy

import tulip_rs
from tulip_rs_bench.common import BenchmarkDef, OhlcvArrays, PdOhlcvArrays


def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.marketfi.indicator(
        [data.high, data.low, data.volume], options
    )


def _tulipy(data: OhlcvArrays, options: List[float]) -> Any:
    return tulipy.marketfi(data.high, data.low, data.volume)


BENCHMARK = BenchmarkDef(
    name="marketfi",
    options_list=[[]],
    tulip_fn=_tulip,
    ref_fn=None,
    extra_refs={"tulipy": _tulipy},
)
