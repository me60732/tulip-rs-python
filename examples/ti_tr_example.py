#!/usr/bin/env python3
"""
Python example for the TR indicator from tulip_rs_python.

This example demonstrates:
1. Basic TR calculation
2. Indicator info display
3. State continuation with new data
4. Matches the Rust reference example exactly
"""

try:
    import numpy as np
    import tulip_rs
except ImportError as e:
    print(f"Import error: {e}")
    print("Please install numpy and build tulip_rs_python with maturin develop")
    exit(1)


def main():
    # Sample data from Rust example
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

    high_vec = np.array(high, dtype=np.float64)
    low_vec = np.array(low, dtype=np.float64)
    close_vec = np.array(close, dtype=np.float64)
    inputs = [high_vec, low_vec, close_vec]

    # Options from Rust example
    options = []

    ################################################### Show Indicator Info First
    info = tulip_rs.indicators.tr.info()
    print(f"=== {info['name'].upper()} ({info['full_name']}) ===")
    print(f"Type: {info['indicator_type']}")
    print(f"Inputs: {info['inputs']}")
    print(f"Options: {info['options']} (current: {options})")
    print(f"Outputs: {info['outputs']}")
    if "optional_outputs" in info:
        print(f"Optional Outputs: {info['optional_outputs']}")

    # Show minimum data requirement
    min_data = tulip_rs.indicators.tr.min_data(options)
    print(f"Minimum data required: {min_data}")

    # Show minimum data for accuracy
    print()

    ################################################### Calculating the Full TR Line
    # Full calculation
    optional_count = (
        len(info["optional_outputs"]) if info.get("optional_outputs") else 0
    )
    optional_outputs = [True] * optional_count if optional_count > 0 else None
    outputs, _ = tulip_rs.indicators.tr.indicator(inputs, options, optional_outputs)

    print(f"Full TR Line: {outputs[0]}")
    if optional_count > 0:
        for i in range(1, len(outputs)):
            print(f"Optional output {i}: {outputs[i]}")

    ################################################### Calculating the partial TR Line
    # Use partial data for state demo
    high_vec2 = np.array(high[:-5], dtype=np.float64)
    low_vec2 = np.array(low[:-5], dtype=np.float64)
    close_vec2 = np.array(close[:-5], dtype=np.float64)
    inputs2 = [high_vec2, low_vec2, close_vec2]

    # Partial calculation - main outputs only (no optional outputs for state continuation)
    outputs2, state2 = tulip_rs.indicators.tr.indicator(inputs2, options)
    print(f"TR Line: {outputs2[0]}")

    ################################################### State Continuation Demo
    print("Demonstrating state continuation...")

    # Get state info if available
    try:
        state_info = state2.get_info()
        print(f"State info: {state_info}")
    except AttributeError:
        print("State info: TR State - internal state for True Range")

    # Use the state to continue calculation with new data
    print("Adding new data to existing state...")
    # Continue with remaining data
    new_high_vec = np.array(high[-5:], dtype=np.float64)
    new_low_vec = np.array(low[-5:], dtype=np.float64)
    new_close_vec = np.array(close[-5:], dtype=np.float64)
    new_data = [new_high_vec, new_low_vec, new_close_vec]
    # State continuation - main outputs only
    final_outputs = state2.batch_indicator(new_data)
    print(f"New TR Line: {final_outputs[0]}")

    # Verify by calculating full sequence at once
    print("Verification - calculating full sequence:")
    # Verify with full data - use same optional outputs as full calculation
    full_outputs, _ = tulip_rs.indicators.tr.indicator(
        inputs, options, optional_outputs
    )
    print(f"Verification TR Line: {full_outputs[0]}")
    if optional_count > 0:
        for i in range(1, len(full_outputs)):
            print(f"Verification optional output {i}: {full_outputs[i]}")

    ################################################### SIMD by Assets Demo
    print("\n" + "=" * 60)
    print("SIMD BY ASSETS DEMONSTRATION")
    print("=" * 60)

    # Create data for 4 assets (SIMD lane requirement: 2, 4, 8, or 16)
    # Asset 1: Original data
    asset1_high_vec = np.array(high_vec, copy=True)
    asset1_low_vec = np.array(low_vec, copy=True)
    asset1_close_vec = np.array(close_vec, copy=True)

    # Asset 2: Scaled up data
    asset2_high_vec = high_vec * 1.2
    asset2_low_vec = low_vec * 1.2
    asset2_close_vec = close_vec * 1.2

    # Asset 3: Different trend
    asset3_high_vec = np.array(
        [90 + i * 0.5 + v * 0.1 for i, v in enumerate(high_vec)], dtype=np.float64
    )
    asset3_low_vec = np.array(
        [90 + i * 0.5 + v * 0.1 for i, v in enumerate(low_vec)], dtype=np.float64
    )
    asset3_close_vec = np.array(
        [90 + i * 0.5 + v * 0.1 for i, v in enumerate(close_vec)], dtype=np.float64
    )

    # Asset 4: Inverted trend
    asset4_high_vec = np.array(
        [100 - i * 0.3 + v * 0.05 for i, v in enumerate(high_vec)], dtype=np.float64
    )
    asset4_low_vec = np.array(
        [100 - i * 0.3 + v * 0.05 for i, v in enumerate(low_vec)], dtype=np.float64
    )
    asset4_close_vec = np.array(
        [100 - i * 0.3 + v * 0.05 for i, v in enumerate(close_vec)], dtype=np.float64
    )

    # Prepare SIMD inputs - must be exactly 2, 4, 8, or 16 assets
    simd_inputs = [
        [asset1_high_vec, asset1_low_vec, asset1_close_vec],  # Asset 1
        [asset2_high_vec, asset2_low_vec, asset2_close_vec],  # Asset 2
        [asset3_high_vec, asset3_low_vec, asset3_close_vec],  # Asset 3
        [asset4_high_vec, asset4_low_vec, asset4_close_vec],  # Asset 4
    ]

    print(f"Processing {len(simd_inputs)} assets simultaneously using SIMD...")
    print("Asset 1: Original data")
    print("Asset 2: Scaled up (+20% values)")
    print("Asset 3: Different upward trend")
    print("Asset 4: Downward trend")
    print()

    try:
        # Calculate TR for all assets using SIMD
        simd_outputs, simd_states = tulip_rs.indicators.tr.simd_by_assets(
            simd_inputs, options, optional_outputs
        )

        print("SIMD Results:")
        for i, (output, state) in enumerate(zip(simd_outputs, simd_states)):
            print(f"Asset {i + 1} TR values: {output[0]}")

        print("\nVerification - calculating each asset individually:")
        for i, asset_inputs in enumerate(simd_inputs):
            individual_output, _ = tulip_rs.indicators.tr.indicator(
                asset_inputs, options, optional_outputs
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
