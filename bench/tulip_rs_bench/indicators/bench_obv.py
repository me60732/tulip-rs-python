# Benchmark: obv (On Balance Volume)
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
    return tulip_rs.indicators.obv.indicator([data.close, data.volume], options)


def _ref(data: PdOhlcvArrays, options: List[float]) -> Any:
    return ta.volume.OnBalanceVolumeIndicator(
        close=data.close, volume=data.volume
    ).on_balance_volume()


def _tulipy(data: OhlcvArrays, options: List[float]) -> Any:
    return tulipy.obv(data.close, data.volume)


def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.obv(pd.Series(data.close), pd.Series(data.volume))

BENCHMARK = BenchmarkDef(
    name="obv",
    options_list=[[]],
    tulip_fn=_tulip,
    ref_fn=_ref,
    extra_refs={"tulipy": _tulipy, "pandas_ta": _pta},
)
