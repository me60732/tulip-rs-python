#!/usr/bin/env python3
"""
Python example for the EMV indicator from tulip_rs_python.

This example demonstrates:
1. Basic EMV calculation with optional outputs
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
    # Sample data: high, low, volume (matching Rust example exactly)
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
    volume = [
        5653100.0,
        6447400.0,
        7690900.0,
        3831400.0,
        4455100.0,
        3798000.0,
        3936200.0,
        4732000.0,
        4841300.0,
        3915300.0,
        6830800.0,
        6694100.0,
        5293600.0,
        7985800.0,
        4807900.0,
    ]

    # Options for EMV: none (matching Rust example)
    options = []

    ################################################### Show Indicator Info First
    info = tulip_rs.indicators.emv.info()
    print(f"=== {info['name'].upper()} ({info['full_name']}) ===")
    print(f"Type: {info['indicator_type']}")
    print(f"Inputs: {info['inputs']}")
    print(f"Options: {info['options']} (current: {options})")
    print(f"Outputs: {info['outputs']}")
    print(f"Optional outputs: {info['optional_outputs']}")

    # Show minimum data requirement
    min_data = tulip_rs.indicators.emv.min_data(options)
    print(f"Minimum data required: {min_data}")

    # Show minimum data for accuracy
    min_data_accuracy = tulip_rs.indicators.emv.min_data_accuracy(options, 6)
    print(f"Minimum data for accuracy (6 decimals): {min_data_accuracy}")
    print()

    ################################################### Partial calculation for state continuation
    # Use partial data (all but last 5 elements, matching Rust example)
    partial_high = high[:-5]
    partial_low = low[:-5]
    partial_volume = volume[:-5]

    high_vec_partial = np.array(partial_high, dtype=np.float64)
    low_vec_partial = np.array(partial_low, dtype=np.float64)
    volume_vec_partial = np.array(partial_volume, dtype=np.float64)
    inputs_partial = [high_vec_partial, low_vec_partial, volume_vec_partial]

    # For initial calculation, use specific optional outputs (matching Rust example with [true])
    outputs_partial, state = tulip_rs.indicators.emv.indicator(
        inputs_partial, options, [True]
    )

    print(f"Partial calculation (first {len(partial_high)} elements):")
    print(f"EMV: {outputs_partial[0]}")
    print(f"MEDPRICE: {outputs_partial[1]}")

    ################################################### State Continuation Demo
    print("\nDemonstrating state continuation...")

    # Get state info
    print(f"State info: EMV State - internal state for Ease of Movement")

    # Continue with new data (last 5 elements, matching Rust example)
    new_high = high[-5:]
    new_low = low[-5:]
    new_volume = volume[-5:]

    new_high_vec = np.array(new_high, dtype=np.float64)
    new_low_vec = np.array(new_low, dtype=np.float64)
    new_volume_vec = np.array(new_volume, dtype=np.float64)
    new_inputs = [new_high_vec, new_low_vec, new_volume_vec]

    # Use batch_indicator without optional_outputs for performance
    continued_outputs = state.batch_indicator(new_inputs)

    print("Continued calculation:")
    print(f"New EMV Line: {continued_outputs[0]}")

    print(
        f"\nData split: {len(partial_high)} + {len(new_high)} = {len(high)} total elements"
    )

    ################################################### SIMD by Assets Demo
    print("\n" + "=" * 60)
    print("SIMD BY ASSETS DEMONSTRATION")
    print("=" * 60)

    # Create data for 4 assets (SIMD lane requirement: 2, 4, 8, or 16)
    # Asset 1: Original data
    asset1_new_high_vec = np.array(new_high_vec, copy=True)
    asset1_new_low_vec = np.array(new_low_vec, copy=True)
    asset1_new_volume_vec = np.array(new_volume_vec, copy=True)

    # Asset 2: Scaled up data
    asset2_new_high_vec = new_high_vec * 1.2
    asset2_new_low_vec = new_low_vec * 1.2
    asset2_new_volume_vec = new_volume_vec * 1.2

    # Asset 3: Different trend
    asset3_new_high_vec = np.array(
        [90 + i * 0.5 + v * 0.1 for i, v in enumerate(new_high_vec)], dtype=np.float64
    )
    asset3_new_low_vec = np.array(
        [90 + i * 0.5 + v * 0.1 for i, v in enumerate(new_low_vec)], dtype=np.float64
    )
    asset3_new_volume_vec = np.array(
        [90 + i * 0.5 + v * 0.1 for i, v in enumerate(new_volume_vec)], dtype=np.float64
    )

    # Asset 4: Inverted trend
    asset4_new_high_vec = np.array(
        [100 - i * 0.3 + v * 0.05 for i, v in enumerate(new_high_vec)], dtype=np.float64
    )
    asset4_new_low_vec = np.array(
        [100 - i * 0.3 + v * 0.05 for i, v in enumerate(new_low_vec)], dtype=np.float64
    )
    asset4_new_volume_vec = np.array(
        [100 - i * 0.3 + v * 0.05 for i, v in enumerate(new_volume_vec)], dtype=np.float64
    )

    # Prepare SIMD inputs - must be exactly 2, 4, 8, or 16 assets
    simd_inputs = [
        [asset1_new_high_vec, asset1_new_low_vec, asset1_new_volume_vec],  # Asset 1
        [asset2_new_high_vec, asset2_new_low_vec, asset2_new_volume_vec],  # Asset 2
        [asset3_new_high_vec, asset3_new_low_vec, asset3_new_volume_vec],  # Asset 3
        [asset4_new_high_vec, asset4_new_low_vec, asset4_new_volume_vec],  # Asset 4
    ]

    print(f"Processing {len(simd_inputs)} assets simultaneously using SIMD...")
    print("Asset 1: Original data")
    print("Asset 2: Scaled up (+20% values)")
    print("Asset 3: Different upward trend")
    print("Asset 4: Downward trend")
    print()

    try:
        # Calculate EMV for all assets using SIMD
        simd_outputs, simd_states = tulip_rs.indicators.emv.simd_by_assets(
            simd_inputs, options
        )

        print("SIMD Results:")
        for i, (output, state) in enumerate(zip(simd_outputs, simd_states)):
            print(f"Asset {i + 1} EMV values: {output[0]}")

        print("\nVerification - calculating each asset individually:")
        for i, asset_inputs in enumerate(simd_inputs):
            individual_output, _ = tulip_rs.indicators.emv.indicator(
                asset_inputs, options
            )
            print(f"Asset {i + 1} individual: {individual_output[0]}")

            # Verify SIMD matches individual calculation
            if np.allclose(simd_outputs[i][0], individual_output[0], rtol=1e-10, equal_nan=True):
                print(f"✓ Asset {i + 1} SIMD matches individual calculation")
            else:
                print(f"✗ Asset {i + 1} SIMD does not match individual calculation")

        print("\nSIMD by Assets demonstration completed successfully!")

    except Exception as e:
        print(f"SIMD by Assets error: {e}")

if __name__ == "__main__":
    main()
