"""
Shared utilities for tulip_rs_bench.

Loaded automatically when any bench_*.py imports from here.
.env is searched upward from CWD by python-dotenv — place it in bench/ when
running tulip-rs-bench from that directory.
"""

from __future__ import annotations

import os
import socket
import statistics
import sys
import timeit
from dataclasses import dataclass
from typing import Any, Callable, Dict, List, Optional

import numpy as np
import psycopg2
import psycopg2.extras
from dotenv import find_dotenv, load_dotenv

# python-dotenv walks upward from CWD — picks up bench/.env when run from there.
# Override location with the DOTENV_PATH environment variable if needed.
_dotenv_path = os.getenv("DOTENV_PATH") or find_dotenv(usecwd=True)
load_dotenv(_dotenv_path)

# Accept either STOCKS_DATABASE_URL (bench-specific) or the shared DATABASE_URL
# used by the Rust tulip_test .env so both point at the same file.
STOCKS_DB_URL = os.getenv("STOCKS_DATABASE_URL") or os.getenv(
    "DATABASE_URL", "postgresql://tulip:tulip@localhost:5432/stocks"
)
BENCH_DB_URL = os.getenv(
    "BENCHMARK_DATABASE_URL",
    "postgresql://tulip:tulip@localhost:5432/indicator_benchmark",
)
LOG_TO_DB = os.getenv("BENCHMARK_LOG_TO_DB", "0") == "1"
BENCH_NUMBER = int(os.getenv("BENCH_NUMBER", "10"))
BENCH_REPEAT = int(os.getenv("BENCH_REPEAT", "30"))
DATA_LIMIT = 6705  # same row-count as Rust benchmarks

STOCKS = [
    ("BHP", "ASX"),
    ("CBA", "ASX"),
    ("AAPL", "NYSE"),
    ("MSFT", "NYSE"),
]

# ---------------------------------------------------------------------------
# Data containers
# ---------------------------------------------------------------------------


@dataclass
class OhlcvArrays:
    """OHLCV as contiguous numpy float64 arrays — passed to tulip_rs."""

    symbol: str
    open: np.ndarray
    high: np.ndarray
    low: np.ndarray
    close: np.ndarray
    volume: np.ndarray

    @property
    def length(self) -> int:
        return len(self.close)


@dataclass
class PdOhlcvArrays:
    """Same data as pd.Series — built once before timing to exclude conversion cost."""

    symbol: str
    open: "pd.Series"
    high: "pd.Series"
    low: "pd.Series"
    close: "pd.Series"
    volume: "pd.Series"

    @property
    def length(self) -> int:
        return len(self.close)


@dataclass
class TimingResult:
    mean_ns: int
    stddev_ns: int
    min_ns: int
    max_ns: int
    sample_count: int


@dataclass
class BenchmarkDef:
    """
    Describes one indicator benchmark.

    tulip_fn(np_data: OhlcvArrays, options: List[float]) -> Any
        Calls tulip_rs.indicators.<name>.indicator(...) and returns.

    ref_fn(pd_data: PdOhlcvArrays, options: List[float]) -> Any
        Calls the `ta`-library equivalent (or a pandas/numpy reference for
        indicators not in `ta`). Set to None to skip the reference run.

    options_list: same 4 option sets used in the Rust benchmarks so results
        are directly comparable in the shared indicator_benchmark database.
        Use [[]] for indicators with no options.
    """

    name: str
    options_list: List[List[float]]
    tulip_fn: Callable[[OhlcvArrays, List[float]], Any]
    ref_fn: Optional[Callable[[PdOhlcvArrays, List[float]], Any]]


# ---------------------------------------------------------------------------
# Timing
# ---------------------------------------------------------------------------


def time_fn(
    fn: Callable[[], Any],
    number: int = BENCH_NUMBER,
    repeat: int = BENCH_REPEAT,
) -> TimingResult:
    """
    Time a zero-argument callable and return nanosecond statistics.

    number = back-to-back calls per sample  (amortises per-call overhead)
    repeat = independent samples             (source for mean/min/max/stddev)
    """
    timer = timeit.Timer(fn)
    raw = timer.repeat(repeat=repeat, number=number)
    # raw[i] = total seconds for `number` calls → ns per call
    ns = [t / number * 1e9 for t in raw]
    return TimingResult(
        mean_ns=int(statistics.mean(ns)),
        stddev_ns=int(statistics.stdev(ns)) if len(ns) > 1 else 0,
        min_ns=int(min(ns)),
        max_ns=int(max(ns)),
        sample_count=repeat,
    )


# ---------------------------------------------------------------------------
# Stock data loading
# ---------------------------------------------------------------------------


def load_stock_data() -> List[OhlcvArrays]:
    """
    Fetch OHLCV rows from the stocks DB for all benchmark tickers.
    Returns chronologically ordered arrays (oldest → newest), same as Rust.
    """
    query = """
        SELECT e.open, e.high, e.low, e.close, e.volume
        FROM listing l
        INNER JOIN adj_eod e ON l.listing_id = e.listing_id
        WHERE l.code = %s
          AND l.exchange_code = %s
          AND e.volume > 0
        ORDER BY e.ts ASC
        LIMIT %s
    """
    results: List[OhlcvArrays] = []
    conn = psycopg2.connect(STOCKS_DB_URL)
    try:
        with conn.cursor() as cur:
            for code, exchange in STOCKS:
                cur.execute(query, (code, exchange, DATA_LIMIT))
                rows = cur.fetchall()
                if not rows:
                    print(
                        f"[warn] no data for {code}/{exchange} — skipping",
                        file=sys.stderr,
                    )
                    continue
                arr = np.array(rows, dtype=np.float64)
                results.append(
                    OhlcvArrays(
                        symbol=f"{code}_{exchange}",
                        open=np.ascontiguousarray(arr[:, 0]),
                        high=np.ascontiguousarray(arr[:, 1]),
                        low=np.ascontiguousarray(arr[:, 2]),
                        close=np.ascontiguousarray(arr[:, 3]),
                        volume=np.ascontiguousarray(arr[:, 4]),
                    )
                )
                print(f"  loaded {len(rows):,} bars  {code}/{exchange}")
    finally:
        conn.close()
    return results


# ---------------------------------------------------------------------------
# Database logger
# ---------------------------------------------------------------------------


class BenchmarkLogger:
    """Writes timing results to the indicator_benchmark Postgres database."""

    def __init__(self) -> None:
        self.conn = psycopg2.connect(BENCH_DB_URL)
        self.run_id: Optional[int] = None
        self._cache: Dict[str, int] = {}
        self._load_indicators()

    def _load_indicators(self) -> None:
        with self.conn.cursor() as cur:
            cur.execute("SELECT id, name FROM indicators")
            for iid, name in cur.fetchall():
                self._cache[name] = iid

    def start_run(
        self, notes: str = "Python benchmarks — tulip_rs_python vs ta"
    ) -> None:
        import platform

        system_info = {
            "os": platform.system(),
            "arch": platform.machine(),
            "cpu_cores": os.cpu_count(),
            "hostname": socket.gethostname(),
            "python_version": sys.version.split()[0],
        }
        with self.conn.cursor() as cur:
            cur.execute(
                "INSERT INTO benchmark_runs (notes, system_info) VALUES (%s, %s) RETURNING id",
                (notes, psycopg2.extras.Json(system_info)),
            )
            self.run_id = cur.fetchone()[0]
        self.conn.commit()
        print(f"  benchmark run id: {self.run_id}")

    def log(
        self,
        indicator_name: str,
        impl_type: str,
        options: List[float],
        timing: TimingResult,
        symbol: str,
        input_size: int,
    ) -> None:
        iid = self._cache.get(indicator_name)
        if iid is None:
            print(
                f"[warn] '{indicator_name}' not in indicators table — skipping",
                file=sys.stderr,
            )
            return
        with self.conn.cursor() as cur:
            cur.execute(
                """
                INSERT INTO benchmark_results
                    (run_id, indicator_id, implementation_type, stock_symbol,
                     data_source, options, mean_time_ns, std_dev_ns,
                     min_time_ns, max_time_ns, sample_count, input_size)
                VALUES (%s,%s,%s,%s,'real_data',%s,%s,%s,%s,%s,%s,%s)
                """,
                (
                    self.run_id,
                    iid,
                    impl_type,
                    symbol,
                    psycopg2.extras.Json(options),
                    timing.mean_ns,
                    timing.stddev_ns,
                    timing.min_ns,
                    timing.max_ns,
                    timing.sample_count,
                    input_size,
                ),
            )
        self.conn.commit()

    def close(self) -> None:
        self.conn.close()


# ---------------------------------------------------------------------------
# Core runner
# ---------------------------------------------------------------------------


def run_benchmark(
    defn: BenchmarkDef,
    stocks: List[OhlcvArrays],
    logger: Optional[BenchmarkLogger],
) -> None:
    """
    Run one BenchmarkDef against every stock and every option set.
    Prints µs timings to stdout; optionally writes ns rows to the DB.

    pd.Series conversion is done once per stock before the timed region
    so conversion overhead is excluded from measurements.
    """
    import pandas as pd

    print(f"\n{'─' * 64}")
    print(f"  {defn.name.upper()}")
    print(f"{'─' * 64}")

    for np_data in stocks:  # type: ignore[assignment]
        # Build pd.Series versions once — outside the timed loop
        pd_data = PdOhlcvArrays(
            symbol=np_data.symbol,
            open=pd.Series(np_data.open, dtype="float64"),
            high=pd.Series(np_data.high, dtype="float64"),
            low=pd.Series(np_data.low, dtype="float64"),
            close=pd.Series(np_data.close, dtype="float64"),
            volume=pd.Series(np_data.volume, dtype="float64"),
        )

        for options in defn.options_list:
            # Use default-arg capture to snapshot loop variables for the lambda
            tulip_result = time_fn(lambda _d=np_data, _o=options: defn.tulip_fn(_d, _o))
            _print_row("tulip_rs_python", np_data.symbol, options, tulip_result)
            if logger:
                logger.log(
                    defn.name,
                    "tulip_rs_python",
                    options,
                    tulip_result,
                    np_data.symbol,
                    np_data.length,
                )

            if defn.ref_fn is not None:
                ref_result = time_fn(lambda _d=pd_data, _o=options: defn.ref_fn(_d, _o))
                ratio = (
                    ref_result.mean_ns / tulip_result.mean_ns
                    if tulip_result.mean_ns
                    else float("inf")
                )
                _print_row("ta", np_data.symbol, options, ref_result, ratio=ratio)
                if logger:
                    logger.log(
                        defn.name,
                        "ta",
                        options,
                        ref_result,
                        np_data.symbol,
                        np_data.length,
                    )


def _fmt_opts(options: List[float]) -> str:
    """Format an option list as a bare comma-separated string (no brackets)."""
    if not options:
        return "\u2014"
    return ", ".join(str(int(o) if o == int(o) else o) for o in options)


def _print_row(
    impl: str,
    symbol: str,
    options: List[float],
    t: TimingResult,
    ratio: Optional[float] = None,
) -> None:
    opts = _fmt_opts(options)
    if ratio is not None and ratio < 1.0:
        ratio_s = f"  \u00d7{1 / ratio:.1f} faster than tulip_rs"
    elif ratio is not None:
        ratio_s = f"  \u00d7{ratio:.1f} slower than tulip_rs"
    else:
        ratio_s = ""
    print(
        f"    {impl:<20} {symbol:<14} [{opts}]  {t.mean_ns:>10,} ns \u00b1 {t.stddev_ns:,}{ratio_s}"
    )
