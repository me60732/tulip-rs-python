# Benchmark: nvi (Negative Volume Index)
from __future__ import annotations

from typing import Any, List

import ta.volume
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
    return tulip_rs.indicators.nvi.indicator([data.close, data.volume], options)


def _ref(data: PdOhlcvArrays, options: List[float]) -> Any:
    return ta.volume.NegativeVolumeIndexIndicator(
        close=data.close,
        volume=data.volume,
    ).negative_volume_index()


def _tulipy(data: OhlcvArrays, options: List[float]) -> Any:
    return tulipy.nvi(data.close, data.volume)


def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.nvi(pd.Series(data.close), pd.Series(data.volume))

BENCHMARK = BenchmarkDef(
    name="nvi",
    options_list=[[]],
    tulip_fn=_tulip,
    ref_fn=_ref,
    extra_refs={"tulipy": _tulipy, "pandas_ta": _pta},
)
