# Benchmark: vosc (Volume Oscillator)
from __future__ import annotations

from typing import Any, List

import tulipy

import tulip_rs
from tulip_rs_bench.common import BenchmarkDef, OhlcvArrays, PdOhlcvArrays


def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.vosc.indicator([data.volume], options)


def _tulipy(data: OhlcvArrays, options: List[float]) -> Any:
    return tulipy.vosc(
        data.volume,
        short_period=int(options[0]),
        long_period=int(options[1]),
    )


BENCHMARK = BenchmarkDef(
    name="vosc",
    options_list=[[5.0, 20.0], [9.0, 26.0], [12.0, 26.0], [3.0, 10.0]],
    tulip_fn=_tulip,
    ref_fn=None,
    extra_refs={"tulipy": _tulipy},
)
