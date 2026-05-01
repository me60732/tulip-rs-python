#!/usr/bin/env python3
"""
Python example for the MACD indicator from tulip_rs_python.

This example demonstrates:
1. Basic MACD calculation with optional outputs
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
    # Sample data: close prices (matching Rust example exactly)
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

    # Options for MACD: short_period=2, long_period=5, signal_period=9 (matching Rust example)
    options = [2.0, 5.0, 9.0]

    ################################################### Show Indicator Info First
    info = tulip_rs.indicators.macd.info()
    print(f"=== {info['name'].upper()} ({info['full_name']}) ===")
    print(f"Type: {info['indicator_type']}")
    print(f"Inputs: {info['inputs']}")
    print(f"Options: {info['options']} (current: {options})")
    print(f"Outputs: {info['outputs']}")
    print(f"Optional outputs: {info['optional_outputs']}")

    # Show minimum data requirement
    min_data = tulip_rs.indicators.macd.min_data(options)
    print(f"Minimum data required: {min_data}")

    # Show minimum data for accuracy
    min_data_accuracy = tulip_rs.indicators.macd.min_data_accuracy(options, 6)
    print(f"Minimum data for accuracy (6 decimals): {min_data_accuracy}")
    print()

    ################################################### Full dataset calculation - Request ALL optional outputs
    close_vec = np.array(close, dtype=np.float64)
    inputs = [close_vec]

    # Get optional outputs count and request all
    optional_count = len(eval(info["optional_outputs"]))
    optional_outputs = [True] * optional_count if optional_count > 0 else None

    outputs, _ = tulip_rs.indicators.macd.indicator(inputs, options, optional_outputs)

    print("Full dataset calculation:")
    print(f"Full MACD Line: {outputs[0]}")
    print(f"Full Signal Line: {outputs[1]}")
    print(f"Full Histrogram: {outputs[2]}")

    # Display optional outputs if they exist
    if optional_count > 0:
        for i in range(3, len(outputs)):
            print(f"Optional output {i - 2}: {outputs[i]}")

    ################################################### Partial calculation for state continuation
    # Use partial data (all but last 1 element, matching Rust example)
    partial_close = close[:-1]
    close_vec_partial = np.array(partial_close, dtype=np.float64)
    inputs_partial = [close_vec_partial]

    # For state continuation, use specific optional outputs like Rust example
    outputs_partial, state = tulip_rs.indicators.macd.indicator(
        inputs_partial, options, [True, True]
    )

    print(f"\nPartial calculation (first {len(partial_close)} elements):")
    print(f"Partial MACD Line: {outputs_partial[0]}")
    print(f"Partial Signal Line: {outputs_partial[1]}")
    print(f"Partial Histrogram: {outputs_partial[2]}")

    ################################################### State Continuation Demo
    print("\nDemonstrating state continuation...")

    # Get state info
    print(
        f"State info: MACD State - internal state for Moving Average Convergence Divergence"
    )

    # Continue with new data (last 1 element, matching Rust example)
    new_close = close[-1:]
    new_close_vec = np.array(new_close, dtype=np.float64)
    new_inputs = [new_close_vec]

    # Use batch_indicator without optional_outputs for performance
    continued_outputs = state.batch_indicator(new_inputs)

    print("Continued calculation:")
    print(f"Final MACD Line: {continued_outputs[0]}")
    print(f"Final Signal Line: {continued_outputs[1]}")
    print(f"Final Histrogram: {continued_outputs[2]}")

    print(
        f"\nData split: {len(partial_close)} + {len(new_close)} = {len(close)} total elements"
    )

    ################################################### SIMD by Assets Demo
    print("\n" + "=" * 60)
    print("SIMD BY ASSETS DEMONSTRATION")
    print("=" * 60)

    # Create data for 4 assets (SIMD lane requirement: 2, 4, 8, or 16)
    # Asset 1: Original data
    asset1_close_vec = np.array(close_vec, copy=True)

    # Asset 2: Scaled up data
    asset2_close_vec = close_vec * 1.2

    # Asset 3: Different trend
    asset3_close_vec = np.array(
        [90 + i * 0.5 + v * 0.1 for i, v in enumerate(close_vec)], dtype=np.float64
    )

    # Asset 4: Inverted trend
    asset4_close_vec = np.array(
        [100 - i * 0.3 + v * 0.05 for i, v in enumerate(close_vec)], dtype=np.float64
    )

    # Prepare SIMD inputs - must be exactly 2, 4, 8, or 16 assets
    simd_inputs = [
        [asset1_close_vec],  # Asset 1
        [asset2_close_vec],  # Asset 2
        [asset3_close_vec],  # Asset 3
        [asset4_close_vec],  # Asset 4
    ]

    print(f"Processing {len(simd_inputs)} assets simultaneously using SIMD...")
    print("Asset 1: Original data")
    print("Asset 2: Scaled up (+20% values)")
    print("Asset 3: Different upward trend")
    print("Asset 4: Downward trend")
    print()

    try:
        # Calculate MACD for all assets using SIMD
        simd_outputs, simd_states = tulip_rs.indicators.macd.simd_by_assets(
            simd_inputs, options, optional_outputs
        )

        print("SIMD Results:")
        for i, (output, state) in enumerate(zip(simd_outputs, simd_states)):
            print(f"Asset {i + 1} MACD values: {output[0]}")

        print("\nVerification - calculating each asset individually:")
        for i, asset_inputs in enumerate(simd_inputs):
            individual_output, _ = tulip_rs.indicators.macd.indicator(
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
