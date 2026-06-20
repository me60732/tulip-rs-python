#!/usr/bin/env python3
"""
Python example for the Candlestick indicator — mirrors the Rust reference example.

Data, options, and output are identical to:
    tulip_rs/examples/candlestick.rs

Two runs are demonstrated:
  Run 1 — forecast_type=None      (all patterns that match the current trend)
  Run 2 — forecast_type=BullishReversal  (only bullish-reversal signals)

Each detected pattern dict contains:
  name          — enum variant name  (e.g. "ThreeWhiteSoldiers")
  full_name     — human-readable     (e.g. "Three White Soldiers")
  japanese_name — Japanese name      (e.g. "akasankuusen")
  bars          — candle count       (e.g. "3")
  forecast      — signal direction   (e.g. "BullishReversal")
"""

try:
    import numpy as np

    import tulip_rs
except ImportError as e:
    print(f"Import error: {e}")
    print("Please build with:  maturin develop  (inside the activated .env)")
    exit(1)

cdl = tulip_rs.indicators.candlestick
ForecastType = cdl.ForecastType

# ---------------------------------------------------------------------------
# Exact data from the Rust example
# ---------------------------------------------------------------------------
open_ = [
    81.85,
    81.20,
    81.55,
    82.91,
    83.10,
    83.41,
    82.71,
    82.70,
    84.20,
    84.25,
    84.03,
    85.45,
    86.18,
    88.00,
    87.30,
    # pattern bars
    87.30,
    86.40,
    84.30,
    85.60,
]
high_ = [
    82.15,
    81.89,
    83.03,
    83.30,
    83.85,
    83.90,
    83.33,
    84.30,
    84.84,
    85.00,
    85.90,
    86.58,
    86.98,
    88.00,
    87.31,
    # pattern bars
    87.30,
    86.40,
    85.50,
    85.65,
]
low_ = [
    81.29,
    80.64,
    81.31,
    82.65,
    83.07,
    83.11,
    82.49,
    82.30,
    84.15,
    84.11,
    84.03,
    85.39,
    85.76,
    87.17,
    87.20,
    # pattern bars
    86.30,
    85.30,
    84.00,
    83.85,
]
close_ = [
    81.59,
    81.06,
    82.87,
    83.00,
    83.61,
    83.15,
    82.84,
    83.99,
    84.55,
    84.36,
    85.53,
    86.54,
    86.89,
    87.77,
    87.29,
    # pattern bars
    86.30,
    85.30,
    84.00,
    83.90,
]

options = [5.0, 1.0, 1.0]  # candle_period=5, trend_period=1, trend_signal_period=1

# Convert to numpy arrays — required by the binding (zero-copy, consistent with all other indicators)
open_np = np.array(open_, dtype=np.float64)
high_np = np.array(high_, dtype=np.float64)
low_np = np.array(low_, dtype=np.float64)
close_np = np.array(close_, dtype=np.float64)

# ---------------------------------------------------------------------------
# Pre-flight info
# ---------------------------------------------------------------------------
min_bars = cdl.min_data(options)
print(f"Bars in: {len(close_np)}  |  min_data: {min_bars}")

# ---------------------------------------------------------------------------
# Run 1 — no forecast filter (all patterns matching current trend)
# ---------------------------------------------------------------------------
print("\n" + "=" * 60)
print("Run 1: forecast_type = None  (all trend-matching patterns)")
print("=" * 60)

result, state = cdl.candlestick(open_np, high_np, low_np, close_np, options=options)

print(f"\nFull result ({len(result)} output bars):")
for i, entry in enumerate(result):
    if entry:
        names = [p["name"] for p in entry]
        print(f"  bar {i:2d}: {names}")

# Replicate Rust: inspect patterns on the last bar
last = result[-1]
if last:
    print("\nPatterns found on last bar:")
    for p in last:
        print(
            f"  - {p['full_name']} ({p['japanese_name']}),  bars: {p['bars']},  forecast: {p['forecast']}"
        )
else:
    print("\nNo patterns on last bar.")

# ---------------------------------------------------------------------------
# Run 2 — filter: BullishReversal only
# ---------------------------------------------------------------------------
print("\n" + "=" * 60)
print("Run 2: forecast_type = BullishReversal")
print("=" * 60)

result2, _ = cdl.candlestick(
    open_np,
    high_np,
    low_np,
    close_np,
    options=options,
    forecast_type=ForecastType.BullishReversal,
)

print(f"\nFull result ({len(result2)} output bars):")
for i, entry in enumerate(result2):
    if entry:
        names = [p["name"] for p in entry]
        print(f"  bar {i:2d}: {names}")

last2 = result2[-1]
if last2:
    print("\nPatterns found on last bar:")
    for p in last2:
        print(
            f"  - {p['full_name']} ({p['japanese_name']}),  bars: {p['bars']},  forecast: {p['forecast']}"
        )
else:
    print("\nNo patterns on last bar.")

# ---------------------------------------------------------------------------
# Streaming continuation from Run 1 state
# ---------------------------------------------------------------------------
print("\n" + "=" * 60)
print("Streaming: one new bar appended to Run 1 state")
print("=" * 60)

new_open = np.array([84.00], dtype=np.float64)
new_high = np.array([84.50], dtype=np.float64)
new_low = np.array([83.20], dtype=np.float64)
new_close = np.array([83.50], dtype=np.float64)

new_result = state.batch_indicator(new_open, new_high, new_low, new_close)
entry = new_result[0]
if entry:
    print("Patterns on new bar:")
    for p in entry:
        print(
            f"  - {p['full_name']} ({p['japanese_name']}),  bars: {p['bars']},  forecast: {p['forecast']}"
        )
else:
    print("No patterns on new bar.")
