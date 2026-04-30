#!/usr/bin/env python3
"""
Python example for the KVO indicator from tulip_rs_python.

This example demonstrates:
1. Basic KVO calculation with optional outputs
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
    # Sample data: high, low, close, volume (matching Rust example exactly)
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

    # Options for KVO: short_period=2, long_period=5 (matching Rust example)
    options = [2.0, 5.0]

    ################################################### Show Indicator Info First
    info = tulip_rs.indicators.kvo.info()
    print(f"=== {info['name'].upper()} ({info['full_name']}) ===")
    print(f"Type: {info['indicator_type']}")
    print(f"Inputs: {info['inputs']}")
    print(f"Options: {info['options']} (current: {options})")
    print(f"Outputs: {info['outputs']}")
    print(f"Optional outputs: {info['optional_outputs']}")

    # Show minimum data requirement
    min_data = tulip_rs.indicators.kvo.min_data(options)
    print(f"Minimum data required: {min_data}")

    # Show minimum data for accuracy
    min_data_accuracy = tulip_rs.indicators.kvo.min_data_accuracy(options, 6)
    print(f"Minimum data for accuracy (6 decimals): {min_data_accuracy}")
    print()

    ################################################### Full dataset calculation - Request ALL optional outputs
    high_vec = np.array(high, dtype=np.float64)
    low_vec = np.array(low, dtype=np.float64)
    close_vec = np.array(close, dtype=np.float64)
    volume_vec = np.array(volume, dtype=np.float64)
    inputs = [high_vec, low_vec, close_vec, volume_vec]

    # Get optional outputs count and request all
    optional_count = len(eval(info["optional_outputs"]))
    optional_outputs = [True] * optional_count if optional_count > 0 else None

    outputs, _ = tulip_rs.indicators.kvo.indicator(inputs, options, optional_outputs)

    print("Full dataset calculation:")
    print(f"Full KVO Line: {outputs[0]}")

    # Display optional outputs if they exist
    if optional_count > 0:
        for i in range(1, len(outputs)):
            print(f"Optional output {i}: {outputs[i]}")

    ################################################### Partial calculation for state continuation
    # Use partial data (all but last 5 elements, matching Rust example)
    partial_high = high[:-5]
    partial_low = low[:-5]
    partial_close = close[:-5]
    partial_volume = volume[:-5]

    high_vec_partial = np.array(partial_high, dtype=np.float64)
    low_vec_partial = np.array(partial_low, dtype=np.float64)
    close_vec_partial = np.array(partial_close, dtype=np.float64)
    volume_vec_partial = np.array(partial_volume, dtype=np.float64)
    inputs_partial = [
        high_vec_partial,
        low_vec_partial,
        close_vec_partial,
        volume_vec_partial,
    ]

    # For state continuation, use main outputs only (no optional_outputs parameter)
    outputs_partial, state = tulip_rs.indicators.kvo.indicator(inputs_partial, options)

    print(f"\nPartial calculation (first {len(partial_high)} elements):")
    print(f"Partial KVO Line: {outputs_partial[0]}")

    ################################################### State Continuation Demo
    print("\nDemonstrating state continuation...")

    # Get state info
    print(f"State info: KVO State - internal state for Klinger Volume Oscillator")

    # Continue with new data (last 5 elements, matching Rust example)
    new_high = high[-5:]
    new_low = low[-5:]
    new_close = close[-5:]
    new_volume = volume[-5:]

    new_high_vec = np.array(new_high, dtype=np.float64)
    new_low_vec = np.array(new_low, dtype=np.float64)
    new_close_vec = np.array(new_close, dtype=np.float64)
    new_volume_vec = np.array(new_volume, dtype=np.float64)
    new_inputs = [new_high_vec, new_low_vec, new_close_vec, new_volume_vec]

    # Use batch_indicator without optional_outputs for performance
    continued_outputs = state.batch_indicator(new_inputs)

    print("Continued calculation:")
    print(f"Remaining KVO Line: {continued_outputs[0]}")

    print(
        f"\nData split: {len(partial_high)} + {len(new_high)} = {len(high)} total elements"
    )


if __name__ == "__main__":
    main()
