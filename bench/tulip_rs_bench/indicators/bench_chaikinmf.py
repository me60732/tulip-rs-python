# Benchmark: chaikinmf (Chaikin Money Flow)
from __future__ import annotations

from typing import Any, List

import ta.volume

import tulip_rs
from tulip_rs_bench.common import (
    BenchmarkDef,
    OhlcvArrays,
    PdOhlcvArrays,
)


def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.chaikinmf.indicator(
        [data.high, data.low, data.close, data.volume], options
    )


def _ref(data: PdOhlcvArrays, options: List[float]) -> Any:
    return ta.volume.ChaikinMoneyFlowIndicator(
        high=data.high,
        low=data.low,
        close=data.close,
        volume=data.volume,
        window=int(options[0]),
    ).chaikin_money_flow()


BENCHMARK = BenchmarkDef(
    name="chaikinmf",
    options_list=[[14.0], [20.0], [25.0], [30.0]],
    tulip_fn=_tulip,
    ref_fn=_ref,
)
