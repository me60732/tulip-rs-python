# Benchmark: mfi (Money Flow Index)
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
    return tulip_rs.indicators.mfi.indicator(
        [data.high, data.low, data.close, data.volume], options
    )


def _ref(data: PdOhlcvArrays, options: List[float]) -> Any:
    return ta.volume.MFIIndicator(
        high=data.high,
        low=data.low,
        close=data.close,
        volume=data.volume,
        window=int(options[0]),
    ).money_flow_index()


def _tulipy(data: OhlcvArrays, options: List[float]) -> Any:
    return tulipy.mfi(
        data.high, data.low, data.close, data.volume, period=int(options[0])
    )


def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.mfi(pd.Series(data.high), pd.Series(data.low), pd.Series(data.close), pd.Series(data.volume), length=int(options[0]))

BENCHMARK = BenchmarkDef(
    name="mfi",
    options_list=[[14.0], [20.0], [25.0], [30.0]],
    tulip_fn=_tulip,
    ref_fn=_ref,
    extra_refs={"tulipy": _tulipy, "pandas_ta": _pta},
)
