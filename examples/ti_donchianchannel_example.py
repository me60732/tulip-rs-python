#!/usr/bin/env python3
"""
Python example for the DONCHIANCHANNEL indicator from tulip_rs_python.

This example demonstrates:
1. Basic Donchian Channel calculation (lower, middle, upper bands)
2. Indicator info display
3. State continuation with new data
4. SIMD by assets and SIMD by options demonstrations

Donchian Channel identifies the highest-high and lowest-low over a rolling period:
  lower  = lowest low over 'period' bars
  middle = (upper + lower) / 2
  upper  = highest high over 'period' bars
"""

try:
    import numpy as np

    import tulip_rs
except ImportError as e:
    print(f"Import error: {e}")
    print("Please install numpy and build tulip_rs_python with maturin develop")
    exit(1)


def main():
    # Sample data: high, low (15 bars, matching Rust reference examples)
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

    # Options: period=5
    options = [5.0]

    ################################################### Show Indicator Info First
    info = tulip_rs.indicators.donchianchannel.info()
    print(f"=== {info['name'].upper()} ({info['full_name']}) ===")
    print(f"Type: {info['indicator_type']}")
    print(f"Inputs: {info['inputs']}")
    print(f"Options: {info['options']} (current: {options})")
    print(f"Outputs: {info['outputs']}")
    print(f"Optional outputs: {info['optional_outputs']}")

    min_data = tulip_rs.indicators.donchianchannel.min_data(options)
    print(f"Minimum data required: {min_data}")
    min_data_accuracy = tulip_rs.indicators.donchianchannel.min_data_accuracy(
        options, 6
    )
    print(f"Minimum data for accuracy (6 decimals): {min_data_accuracy}")
    output_len = tulip_rs.indicators.donchianchannel.output_length(len(high), options)
    print(f"Output length for {len(high)} bars: {output_len}")
    print()

    ################################################### Full dataset calculation
    high_vec = np.array(high, dtype=np.float64)
    low_vec = np.array(low, dtype=np.float64)
    inputs = [high_vec, low_vec]

    outputs, _ = tulip_rs.indicators.donchianchannel.indicator(inputs, options)

    print("Full dataset calculation:")
    print(f"Donchian Channel Lower:  {outputs[0]}")
    print(f"Donchian Channel Middle: {outputs[1]}")
    print(f"Donchian Channel Upper:  {outputs[2]}")

    ################################################### Partial calculation for state continuation
    partial_high = high[:-5]
    partial_low = low[:-5]
    high_vec_partial = np.array(partial_high, dtype=np.float64)
    low_vec_partial = np.array(partial_low, dtype=np.float64)
    inputs_partial = [high_vec_partial, low_vec_partial]

    outputs_partial, state = tulip_rs.indicators.donchianchannel.indicator(
        inputs_partial, options
    )

    print(f"\nPartial calculation (first {len(partial_high)} bars):")
    print(f"Donchian Channel Lower:  {outputs_partial[0]}")
    print(f"Donchian Channel Middle: {outputs_partial[1]}")
    print(f"Donchian Channel Upper:  {outputs_partial[2]}")

    ################################################### State Continuation Demo
    print("\nDemonstrating state continuation...")
    print(f"State info: {state.get_info()}")

    new_high = high[-5:]
    new_low = low[-5:]
    new_high_vec = np.array(new_high, dtype=np.float64)
    new_low_vec = np.array(new_low, dtype=np.float64)
    new_inputs = [new_high_vec, new_low_vec]

    continued_outputs = state.batch_indicator(new_inputs)

    print("Continued calculation:")
    print(f"New Donchian Channel Lower:  {continued_outputs[0]}")
    print(f"New Donchian Channel Middle: {continued_outputs[1]}")
    print(f"New Donchian Channel Upper:  {continued_outputs[2]}")
    print(
        f"\nData split: {len(partial_high)} + {len(new_high)} = {len(high)} total bars"
    )

    ################################################### SIMD by Assets Demo
    print("\n" + "=" * 60)
    print("SIMD BY ASSETS DEMONSTRATION")
    print("=" * 60)

    # Each asset has two input series: [high, low]
    asset1_high = np.array(high_vec, copy=True)
    asset1_low = np.array(low_vec, copy=True)
    asset2_high = high_vec * 1.2
    asset2_low = low_vec * 1.2
    asset3_high = np.array(
        [90.0 + i * 0.5 + v * 0.1 for i, v in enumerate(high_vec)], dtype=np.float64
    )
    asset3_low = np.array(
        [90.0 + i * 0.5 + v * 0.1 for i, v in enumerate(low_vec)], dtype=np.float64
    )
    asset4_high = np.array(
        [100.0 - i * 0.3 + v * 0.05 for i, v in enumerate(high_vec)], dtype=np.float64
    )
    asset4_low = np.array(
        [100.0 - i * 0.3 + v * 0.05 for i, v in enumerate(low_vec)], dtype=np.float64
    )

    simd_inputs = [
        [asset1_high, asset1_low],  # Asset 1: original
        [asset2_high, asset2_low],  # Asset 2: scaled up
        [asset3_high, asset3_low],  # Asset 3: upward trend
        [asset4_high, asset4_low],  # Asset 4: downward trend
    ]

    print(f"Processing {len(simd_inputs)} assets simultaneously using SIMD...")
    print()

    try:
        simd_outputs, simd_states = tulip_rs.indicators.donchianchannel.simd_by_assets(
            simd_inputs, options
        )

        print("SIMD Results (upper band):")
        for i, output in enumerate(simd_outputs):
            print(f"Asset {i + 1} Upper: {output[2]}")

        print("\nVerification - calculating each asset individually:")
        for i, asset_inputs in enumerate(simd_inputs):
            individual_output, _ = tulip_rs.indicators.donchianchannel.indicator(
                asset_inputs, options
            )
            print(f"Asset {i + 1} individual Upper: {individual_output[2]}")

            if np.allclose(
                simd_outputs[i][2], individual_output[2], rtol=1e-10, equal_nan=True
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
    #   period=9  → min_data=10 → 6 output values
    #   period=12 → min_data=13 → 3 output values
    # Note: period=14 gives only 1 output with 15 bars — expand data for larger periods.
    simd_options = [
        [5.0],  # period=5
        [7.0],  # period=7
        [9.0],  # period=9
        [12.0],  # period=12
    ]

    print(f"Processing {len(simd_options)} option sets simultaneously using SIMD...")
    for i, opt in enumerate(simd_options):
        print(f"Option set {i + 1}: period={opt[0]}")
    print()

    try:
        simd_opt_outputs, simd_opt_states = (
            tulip_rs.indicators.donchianchannel.simd_by_options(inputs, simd_options)
        )

        print("SIMD Results (upper band, first 5 values):")
        for i, output in enumerate(simd_opt_outputs):
            print(f"Option set {i + 1} Upper (first 5): {output[2][:5]}")

        print("\nVerification - calculating each option set individually:")
        for i, opt in enumerate(simd_options):
            individual_output, _ = tulip_rs.indicators.donchianchannel.indicator(
                inputs, opt
            )
            print(
                f"Option set {i + 1} individual Upper (first 5): {individual_output[2][:5]}"
            )

            if np.allclose(
                simd_opt_outputs[i][2], individual_output[2], rtol=1e-10, equal_nan=True
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
