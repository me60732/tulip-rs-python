# Benchmark: donchianchannel (Donchian Channel)
from __future__ import annotations

from typing import Any, List

import ta.volatility

import tulip_rs
from tulip_rs_bench.common import (
    BenchmarkDef,
    OhlcvArrays,
    PdOhlcvArrays,
)


import pandas as pd
import pandas_ta as pta
def _tulip(data: OhlcvArrays, options: List[float]) -> Any:
    return tulip_rs.indicators.donchianchannel.indicator([data.high, data.low], options)


def _ref(data: PdOhlcvArrays, options: List[float]) -> Any:
    # DonchianChannel requires close even though tulip-rs only uses high/low.
    return ta.volatility.DonchianChannel(
        high=data.high, low=data.low, close=data.close, window=int(options[0])
    ).donchian_channel_mband()


def _pta(data: OhlcvArrays, options: List[float]) -> Any:
    return pta.donchian(pd.Series(data.high), pd.Series(data.low), lower_length=int(options[0]), upper_length=int(options[0]))


def _simd_assets(stocks: List[OhlcvArrays], options: List[float]) -> Any:
    """Process every loaded stock's series together via SIMD lanes."""
    inputs = [[stock.high, stock.low] for stock in stocks]
    return tulip_rs.indicators.donchianchannel.simd_by_assets(inputs, options, None)


def _simd_options(data: OhlcvArrays, options_list: List[List[float]]) -> Any:
    """Process every option set together via SIMD lanes for one stock."""
    return tulip_rs.indicators.donchianchannel.simd_by_options([data.high, data.low], options_list, None)


BENCHMARK = BenchmarkDef(
    name="donchianchannel",
    options_list=[[14.0], [20.0], [25.0], [30.0]],
    tulip_fn=_tulip,
    ref_fn=_ref,
    extra_refs={"pandas_ta": _pta},
    simd_assets_fn=_simd_assets,
    simd_options_fn=_simd_options,
)
