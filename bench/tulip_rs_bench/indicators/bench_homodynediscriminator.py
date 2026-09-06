# Benchmark: homodynediscriminator (Homodyne Discriminator)
from __future__ import annotations

from typing import Any, List

import pandas as pd
import pandas_ta as pta

import tulip_rs
from tulip_rs_bench.common import BenchmarkDef, OhlcvArrays, PdOhlcvArrays


def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.homodynediscriminator.indicator([data.close], [])


def _simd_assets(stocks: List[OhlcvArrays], options: List[float]) -> Any:
    """Process every loaded stock's close series together via SIMD lanes."""
    inputs = [[stock.close] for stock in stocks]
    return tulip_rs.indicators.homodynediscriminator.simd_by_assets(inputs, options, None)


BENCHMARK = BenchmarkDef(
    name="homodynediscriminator",
    options_list=[[]],
    tulip_fn=_tulip,
    ref_fn=None,
    extra_refs=None,
    simd_assets_fn=_simd_assets,
)
