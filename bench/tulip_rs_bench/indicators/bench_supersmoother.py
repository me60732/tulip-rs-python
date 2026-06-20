# Benchmark: supersmoother (Super Smoother)
from __future__ import annotations

from typing import Any, List

import pandas as pd
import pandas_ta as pta

import tulip_rs
from tulip_rs_bench.common import BenchmarkDef, OhlcvArrays, PdOhlcvArrays


def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.supersmoother.indicator([data.close], options)


def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.ssf(pd.Series(data.close), length=int(options[0]))


BENCHMARK = BenchmarkDef(
    name="supersmoother",
    options_list=[[10.0], [20.0], [30.0], [40.0]],
    tulip_fn=_tulip,
    ref_fn=None,
    extra_refs={"pandas_ta": _pta},
)
