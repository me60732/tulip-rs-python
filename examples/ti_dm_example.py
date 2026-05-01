#!/usr/bin/env python3
"""
Python example for the DM indicator from tulip_rs_python.

This example demonstrates:
1. Basic DM calculation with optional outputs
2. Indicator info display
3. State continuation with new data
4. Exact match with Rust reference implementation
"""

try:
    import numpy as np
    import tulip_rs
except ImportError as e:
    print(f"Import error: {e}")
    print("Please install numpy and build tulip_rs_python with maturin develop")
    exit(1)


def main():
    # Sample data: high and low prices (matching Rust example exactly)
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

    # Options for DM: period=5 (matching Rust example)
    options = [5.0]

    ################################################### Show Indicator Info First
    info = tulip_rs.indicators.dm.info()
    print(f"=== {info['name'].upper()} ({info['full_name']}) ===")
    print(f"Type: {info['indicator_type']}")
    print(f"Inputs: {info['inputs']}")
    print(f"Options: {info['options']} (current: {options})")
    print(f"Outputs: {info['outputs']}")
    print(f"Optional outputs: {info['optional_outputs']}")

    # Show minimum data requirement
    min_data = tulip_rs.indicators.dm.min_data(options)
    print(f"Minimum data required: {min_data}")

    # Show minimum data for accuracy
    min_data_accuracy = tulip_rs.indicators.dm.min_data_accuracy(options, 6)
    print(f"Minimum data for accuracy (6 decimals): {min_data_accuracy}")
    print()

    ################################################### Partial calculation for state continuation
    # Use partial data (all but last 5 elements, matching Rust example)
    partial_high = high[:-5]
    partial_low = low[:-5]

    high_vec_partial = np.array(partial_high, dtype=np.float64)
    low_vec_partial = np.array(partial_low, dtype=np.float64)
    inputs_partial = [high_vec_partial, low_vec_partial]

    # For state continuation, use main outputs only (no optional_outputs parameter)
    outputs_partial, state = tulip_rs.indicators.dm.indicator(inputs_partial, options)

    print(f"Partial calculation (first {len(partial_high)} elements):")
    print(f"Plus DM: {outputs_partial[0]}")
    print(f"Minus DM: {outputs_partial[1]}")

    ################################################### State Continuation Demo
    print("\nDemonstrating state continuation...")

    # Get state info
    print(f"State info: DM State - internal state for Directional Movement")

    # Continue with new data (last 5 elements, matching Rust example)
    new_high = high[-5:]
    new_low = low[-5:]

    new_high_vec = np.array(new_high, dtype=np.float64)
    new_low_vec = np.array(new_low, dtype=np.float64)
    new_inputs = [new_high_vec, new_low_vec]

    # Use batch_indicator without optional_outputs for performance
    continued_outputs = state.batch_indicator(new_inputs)

    print("Continued calculation:")
    print(f"New Plus DM: {continued_outputs[0]}")
    print(f"New Minus DM: {continued_outputs[1]}")

    print(
        f"\nData split: {len(partial_high)} + {len(new_high)} = {len(high)} total elements"
    )

    ################################################### SIMD by Assets Demo
    print("\n" + "=" * 60)
    print("SIMD BY ASSETS DEMONSTRATION")
    print("=" * 60)

    # Create data for 4 assets (SIMD lane requirement: 2, 4, 8, or 16)
    # Asset 1: Original data
    asset1_high = np.array(high, dtype=np.float64)
    asset1_low = np.array(low, dtype=np.float64)

    # Asset 2: Scaled up data
    asset2_high = np.array([v * 1.2 for v in high], dtype=np.float64)
    asset2_low = np.array([v * 1.2 for v in low], dtype=np.float64)

    # Asset 3: Different trend
    asset3_high = np.array(
        [90 + i * 0.5 + v * 0.1 for i, v in enumerate(high)], dtype=np.float64
    )
    asset3_low = np.array(
        [90 + i * 0.5 + v * 0.1 for i, v in enumerate(low)], dtype=np.float64
    )

    # Asset 4: Inverted trend
    asset4_high = np.array(
        [100 - i * 0.3 + v * 0.05 for i, v in enumerate(high)], dtype=np.float64
    )
    asset4_low = np.array(
        [100 - i * 0.3 + v * 0.05 for i, v in enumerate(low)], dtype=np.float64
    )

    # Prepare SIMD inputs - must be exactly 2, 4, 8, or 16 assets
    simd_inputs = [
        [asset1_high, asset1_low],  # Asset 1
        [asset2_high, asset2_low],  # Asset 2
        [asset3_high, asset3_low],  # Asset 3
        [asset4_high, asset4_low],  # Asset 4
    ]

    print(f"Processing {len(simd_inputs)} assets simultaneously using SIMD...")
    print("Asset 1: Original data")
    print("Asset 2: Scaled up (+20% values)")
    print("Asset 3: Different upward trend")
    print("Asset 4: Downward trend")
    print()

    try:
        # Calculate DM for all assets using SIMD
        simd_outputs, simd_states = tulip_rs.indicators.dm.simd_by_assets(
            simd_inputs, options
        )

        print("SIMD Results:")
        for i, (output, state) in enumerate(zip(simd_outputs, simd_states)):
            print(f"Asset {i + 1} DM values: {output[0]}")

        print("\nVerification - calculating each asset individually:")
        for i, asset_inputs in enumerate(simd_inputs):
            individual_output, _ = tulip_rs.indicators.dm.indicator(
                asset_inputs, options
            )
            print(f"Asset {i + 1} individual: {individual_output[0]}")

            # Verify SIMD matches individual calculation
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

    # Expand inputs to ensure we have enough data for larger period options
    expanded_new_high = np.tile(new_high, 20).astype(np.float64)
    expanded_new_low = np.tile(new_low, 20).astype(np.float64)
    expanded_inputs = [expanded_new_high, expanded_new_low]

    simd_options = [
        [2.5],  # Option set 1
        [5.0],  # Option set 2 (Original)
        [7.5],  # Option set 3
        [10.0],  # Option set 4
    ]

    print(f"Processing {len(simd_options)} option sets simultaneously using SIMD...")
    for i, opt in enumerate(simd_options):
        print(f"Option set {i + 1}: {opt}")
    print()

    try:
        # Calculate DM for all option sets using SIMD
        simd_opt_outputs, simd_opt_states = tulip_rs.indicators.dm.simd_by_options(
            expanded_inputs, simd_options
        )

        print("SIMD Results:")
        for i, (output, state) in enumerate(zip(simd_opt_outputs, simd_opt_states)):
            print(f"Option set {i + 1} DM values (first 5): {output[0][:5]}")

        print("\nVerification - calculating each option set individually:")
        for i, opt in enumerate(simd_options):
            individual_output, _ = tulip_rs.indicators.dm.indicator(
                expanded_inputs, opt
            )
            print(
                f"Option set {i + 1} individual (first 5): {individual_output[0][:5]}"
            )

            # Verify SIMD matches individual calculation
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
