#!/usr/bin/env python3
"""
Python example for the SMAENVELOPE indicator from tulip_rs_python.

This example demonstrates:
1. Basic SMA Envelope calculation (lower, middle, upper bands)
2. Indicator info display
3. State continuation with new data
4. SIMD by assets and SIMD by options demonstrations

SMA Envelope plots three bands around a Simple Moving Average:
  lower  = SMA - SMA * (percentage / 100)
  middle = SMA
  upper  = SMA + SMA * (percentage / 100)
"""

try:
    import numpy as np

    import tulip_rs
except ImportError as e:
    print(f"Import error: {e}")
    print("Please install numpy and build tulip_rs_python with maturin develop")
    exit(1)


def main():
    # Sample data: close prices (15 bars, matching Rust reference examples)
    close = [
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
    ]

    # Options: period=5, percentage=2.0 (envelope width as % of SMA)
    options = [5.0, 2.0]

    ################################################### Show Indicator Info First
    info = tulip_rs.indicators.smaenvelope.info()
    print(f"=== {info['name'].upper()} ({info['full_name']}) ===")
    print(f"Type: {info['indicator_type']}")
    print(f"Inputs: {info['inputs']}")
    print(f"Options: {info['options']} (current: {options})")
    print(f"Outputs: {info['outputs']}")
    print(f"Optional outputs: {info['optional_outputs']}")

    min_data = tulip_rs.indicators.smaenvelope.min_data(options)
    print(f"Minimum data required: {min_data}")
    print()

    ################################################### Full dataset calculation
    close_vec = np.array(close, dtype=np.float64)
    inputs = [close_vec]

    outputs, _ = tulip_rs.indicators.smaenvelope.indicator(inputs, options)

    print("Full dataset calculation:")
    print(f"SMA Envelope Lower:  {outputs[0]}")
    print(f"SMA Envelope Middle: {outputs[1]}")
    print(f"SMA Envelope Upper:  {outputs[2]}")

    ################################################### Partial calculation for state continuation
    partial_close = close[:-5]
    close_vec_partial = np.array(partial_close, dtype=np.float64)
    inputs_partial = [close_vec_partial]

    outputs_partial, state = tulip_rs.indicators.smaenvelope.indicator(
        inputs_partial, options
    )

    print(f"\nPartial calculation (first {len(partial_close)} bars):")
    print(f"SMA Envelope Lower:  {outputs_partial[0]}")
    print(f"SMA Envelope Middle: {outputs_partial[1]}")
    print(f"SMA Envelope Upper:  {outputs_partial[2]}")

    ################################################### State Continuation Demo
    print("\nDemonstrating state continuation...")
    print(f"State info: {state.get_info()}")

    new_close = close[-5:]
    new_close_vec = np.array(new_close, dtype=np.float64)
    new_inputs = [new_close_vec]

    continued_outputs = state.batch_indicator(new_inputs)

    print("Continued calculation:")
    print(f"New SMA Envelope Lower:  {continued_outputs[0]}")
    print(f"New SMA Envelope Middle: {continued_outputs[1]}")
    print(f"New SMA Envelope Upper:  {continued_outputs[2]}")
    print(
        f"\nData split: {len(partial_close)} + {len(new_close)} = {len(close)} total bars"
    )

    ################################################### SIMD by Assets Demo
    print("\n" + "=" * 60)
    print("SIMD BY ASSETS DEMONSTRATION")
    print("=" * 60)

    # Each asset has a single 'real' (price) input series
    asset1_vec = np.array(close_vec, copy=True)
    asset2_vec = close_vec * 1.2
    asset3_vec = np.array(
        [90.0 + i * 0.5 + v * 0.1 for i, v in enumerate(close_vec)], dtype=np.float64
    )
    asset4_vec = np.array(
        [100.0 - i * 0.3 + v * 0.05 for i, v in enumerate(close_vec)], dtype=np.float64
    )

    simd_inputs = [
        [asset1_vec],  # Asset 1: original
        [asset2_vec],  # Asset 2: scaled up
        [asset3_vec],  # Asset 3: upward trend
        [asset4_vec],  # Asset 4: downward trend
    ]

    print(f"Processing {len(simd_inputs)} assets simultaneously using SIMD...")
    print()

    try:
        simd_outputs, simd_states = tulip_rs.indicators.smaenvelope.simd_by_assets(
            simd_inputs, options
        )

        print("SIMD Results (middle band):")
        for i, output in enumerate(simd_outputs):
            print(f"Asset {i + 1} Middle: {output[1]}")

        print("\nVerification - calculating each asset individually:")
        for i, asset_inputs in enumerate(simd_inputs):
            individual_output, _ = tulip_rs.indicators.smaenvelope.indicator(
                asset_inputs, options
            )
            print(f"Asset {i + 1} individual Middle: {individual_output[1]}")

            if np.allclose(
                simd_outputs[i][1], individual_output[1], rtol=1e-10, equal_nan=True
            ):
                print(f"✓ Asset {i + 1} SIMD matches individual calculation")
            else:
                print(f"✗ Asset {i + 1} SIMD does not match individual calculation")

        print("\nSIMD by Assets demonstration completed successfully!")

    except Exception as e:
        print(f"SIMD by Assets error: {e}")

    ################################################### SIMD by Options Demo
    print("\n" + "=" * 60)
    print("SIMD BY OPTIONS DEMONSTRATION")
    print("=" * 60)

    # Option sets chosen to all produce meaningful output with 15 bars:
    #   period=5  → min_data=6  → 10 output values
    #   period=7  → min_data=8  → 8 output values
    #   period=10 → min_data=11 → 5 output values
    #   period=12 → min_data=13 → 3 output values
    # Note: period=14 would give only 1 output with 15 bars — expand data for larger periods.
    simd_options = [
        [5.0, 2.0],  # period=5, 2% envelope
        [7.0, 3.0],  # period=7, 3% envelope
        [10.0, 2.0],  # period=10, 2% envelope
        [12.0, 5.0],  # period=12, 5% envelope
    ]

    print(f"Processing {len(simd_options)} option sets simultaneously using SIMD...")
    for i, opt in enumerate(simd_options):
        print(f"Option set {i + 1}: period={opt[0]}, percentage={opt[1]}%")
    print()

    try:
        simd_opt_outputs, simd_opt_states = (
            tulip_rs.indicators.smaenvelope.simd_by_options(inputs, simd_options)
        )

        print("SIMD Results (middle band, first 5 values):")
        for i, output in enumerate(simd_opt_outputs):
            print(f"Option set {i + 1} Middle (first 5): {output[1][:5]}")

        print("\nVerification - calculating each option set individually:")
        for i, opt in enumerate(simd_options):
            individual_output, _ = tulip_rs.indicators.smaenvelope.indicator(
                inputs, opt
            )
            print(
                f"Option set {i + 1} individual Middle (first 5): {individual_output[1][:5]}"
            )

            if np.allclose(
                simd_opt_outputs[i][1], individual_output[1], rtol=1e-10, equal_nan=True
            ):
                print(f"✓ Option set {i + 1} SIMD matches individual calculation")
            else:
                print(
                    f"✗ Option set {i + 1} SIMD does not match individual calculation"
                )

        print("\nSIMD by Options demonstration completed successfully!")

    except Exception as e:
        print(f"SIMD by Options error: {e}")


if __name__ == "__main__":
    main()
