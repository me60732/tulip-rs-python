#!/usr/bin/env python3
"""
Python example for the ELDERRAY (Elder-ray) indicator from tulip_rs_python.

This example demonstrates:
1. Basic Elder-ray calculation (bull power and bear power)
2. Optional EMA output
3. Indicator info display
4. State continuation with new data
5. SIMD by assets and SIMD by options demonstrations
"""

try:
    import numpy as np

    import tulip_rs
except ImportError as e:
    print(f"Import error: {e}")
    print("Please install numpy and build tulip_rs_python with maturin develop")
    exit(1)


def main():
    # Sample data: high, low, close (matching Rust example exactly)
    high = [
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
        87.87,
    ]
    low = [
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
        87.01,
    ]
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

    # Options for Elder-ray: period=5
    options = [5.0]

    ################################################### Show Indicator Info First
    info = tulip_rs.indicators.elderray.info()
    print(f"=== {info['name'].upper()} ({info['full_name']}) ===")
    print(f"Type: {info['indicator_type']}")
    print(f"Inputs: {info['inputs']}")
    print(f"Options: {info['options']} (current: {options})")
    print(f"Outputs: {info['outputs']}")
    print(f"Optional outputs: {info['optional_outputs']}")

    # Show minimum data requirement
    min_data = tulip_rs.indicators.elderray.min_data(options)
    print(f"Minimum data required: {min_data}")

    # Show minimum data for accuracy
    print()

    ################################################### Full dataset calculation — bull and bear power only
    high_vec = np.array(high, dtype=np.float64)
    low_vec = np.array(low, dtype=np.float64)
    close_vec = np.array(close, dtype=np.float64)
    inputs = [high_vec, low_vec, close_vec]

    outputs, _ = tulip_rs.indicators.elderray.indicator(inputs, options)

    print("Full dataset calculation:")
    print(f"Bull Power (high - EMA): {outputs[0]}")
    print(f"Bear Power (low  - EMA): {outputs[1]}")

    ################################################### Full run with optional EMA output
    outputs_with_ema, _ = tulip_rs.indicators.elderray.indicator(
        inputs, options, [True]
    )

    print(f"\nFull dataset calculation (with optional EMA):")
    print(f"Bull Power: {outputs_with_ema[0]}")
    print(f"Bear Power: {outputs_with_ema[1]}")
    print(f"EMA line:   {outputs_with_ema[2]}")

    ################################################### Partial calculation for state continuation
    # Use partial data (all but last 5 elements)
    partial_high = high[:-5]
    partial_low = low[:-5]
    partial_close = close[:-5]

    high_vec_partial = np.array(partial_high, dtype=np.float64)
    low_vec_partial = np.array(partial_low, dtype=np.float64)
    close_vec_partial = np.array(partial_close, dtype=np.float64)
    inputs_partial = [high_vec_partial, low_vec_partial, close_vec_partial]

    outputs_partial, state = tulip_rs.indicators.elderray.indicator(
        inputs_partial, options
    )

    print(f"\nPartial calculation (first {len(partial_close)} elements):")
    print(f"Bull Power: {outputs_partial[0]}")
    print(f"Bear Power: {outputs_partial[1]}")

    ################################################### State Continuation Demo
    print("\nDemonstrating state continuation...")

    print(f"State info: {state.get_info()}")

    # Continue with new data (last 5 elements)
    new_high = high[-5:]
    new_low = low[-5:]
    new_close = close[-5:]

    new_high_vec = np.array(new_high, dtype=np.float64)
    new_low_vec = np.array(new_low, dtype=np.float64)
    new_close_vec = np.array(new_close, dtype=np.float64)
    new_inputs = [new_high_vec, new_low_vec, new_close_vec]

    continued_outputs = state.batch_indicator(new_inputs)

    print("Continued calculation:")
    print(f"New Bull Power: {continued_outputs[0]}")
    print(f"New Bear Power: {continued_outputs[1]}")

    print(
        f"\nData split: {len(partial_close)} + {len(new_close)} = {len(close)} total elements"
    )

    ################################################### SIMD by Assets Demo
    print("\n" + "=" * 60)
    print("SIMD BY ASSETS DEMONSTRATION")
    print("=" * 60)

    asset1_high_vec = np.array(high_vec, copy=True)
    asset1_low_vec = np.array(low_vec, copy=True)
    asset1_close_vec = np.array(close_vec, copy=True)

    asset2_high_vec = high_vec * 1.2
    asset2_low_vec = low_vec * 1.2
    asset2_close_vec = close_vec * 1.2

    asset3_high_vec = np.array(
        [90 + i * 0.5 + v * 0.1 for i, v in enumerate(high_vec)], dtype=np.float64
    )
    asset3_low_vec = np.array(
        [90 + i * 0.5 + v * 0.1 for i, v in enumerate(low_vec)], dtype=np.float64
    )
    asset3_close_vec = np.array(
        [90 + i * 0.5 + v * 0.1 for i, v in enumerate(close_vec)], dtype=np.float64
    )

    asset4_high_vec = np.array(
        [100 - i * 0.3 + v * 0.05 for i, v in enumerate(high_vec)], dtype=np.float64
    )
    asset4_low_vec = np.array(
        [100 - i * 0.3 + v * 0.05 for i, v in enumerate(low_vec)], dtype=np.float64
    )
    asset4_close_vec = np.array(
        [100 - i * 0.3 + v * 0.05 for i, v in enumerate(close_vec)], dtype=np.float64
    )

    simd_inputs = [
        [asset1_high_vec, asset1_low_vec, asset1_close_vec],
        [asset2_high_vec, asset2_low_vec, asset2_close_vec],
        [asset3_high_vec, asset3_low_vec, asset3_close_vec],
        [asset4_high_vec, asset4_low_vec, asset4_close_vec],
    ]

    print(f"Processing {len(simd_inputs)} assets simultaneously using SIMD...")
    print()

    try:
        simd_outputs, simd_states = tulip_rs.indicators.elderray.simd_by_assets(
            simd_inputs, options
        )

        print("SIMD Results:")
        for i, output in enumerate(simd_outputs):
            print(f"Asset {i + 1} Bull Power: {output[0]}")

        print("\nVerification - calculating each asset individually:")
        for i, asset_inputs in enumerate(simd_inputs):
            individual_output, _ = tulip_rs.indicators.elderray.indicator(
                asset_inputs, options
            )
            print(f"Asset {i + 1} individual Bull Power: {individual_output[0]}")

            if np.allclose(
                simd_outputs[i][0], individual_output[0], rtol=1e-10, equal_nan=True
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

    # period=5  → min_data=6  → 10 output values
    # period=7  → min_data=8  →  8 output values
    # period=9  → min_data=10 →  6 output values
    # period=12 → min_data=13 →  3 output values
    simd_options = [
        [5.0],
        [7.0],
        [9.0],
        [12.0],
    ]

    print(f"Processing {len(simd_options)} option sets simultaneously using SIMD...")
    for i, opt in enumerate(simd_options):
        print(f"Option set {i + 1}: period={opt[0]}")
    print()

    try:
        simd_opt_outputs, simd_opt_states = (
            tulip_rs.indicators.elderray.simd_by_options(inputs, simd_options)
        )

        print("SIMD Results:")
        for i, output in enumerate(simd_opt_outputs):
            print(f"Option set {i + 1} Bull Power: {output[0]}")

        print("\nVerification - calculating each option set individually:")
        for i, opt in enumerate(simd_options):
            individual_output, _ = tulip_rs.indicators.elderray.indicator(inputs, opt)
            print(f"Option set {i + 1} individual Bull Power: {individual_output[0]}")

            if np.allclose(
                simd_opt_outputs[i][0], individual_output[0], rtol=1e-10, equal_nan=True
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
