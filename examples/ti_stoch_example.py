#!/usr/bin/env python3
"""
Python example for the STOCH indicator from tulip_rs_python.

This example demonstrates:
1. Basic STOCH calculation with optional outputs
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

    # Options for STOCH: k_period=5, k_slow_period=3, d_period=3 (matching Rust example)
    options = [5.0, 3.0, 3.0]

    ################################################### Show Indicator Info First
    info = tulip_rs.indicators.stoch.info()
    print(f"=== {info['name'].upper()} ({info['full_name']}) ===")
    print(f"Type: {info['indicator_type']}")
    print(f"Inputs: {info['inputs']}")
    print(f"Options: {info['options']} (current: {options})")
    print(f"Outputs: {info['outputs']}")
    print(f"Optional outputs: {info['optional_outputs']}")

    # Show minimum data requirement
    min_data = tulip_rs.indicators.stoch.min_data(options)
    print(f"Minimum data required: {min_data}")

    # Show minimum data for accuracy
    min_data_accuracy = tulip_rs.indicators.stoch.min_data_accuracy(options, 6)
    print(f"Minimum data for accuracy (6 decimals): {min_data_accuracy}")
    print()

    ################################################### Full dataset calculation - Request ALL optional outputs
    high_vec = np.array(high, dtype=np.float64)
    low_vec = np.array(low, dtype=np.float64)
    close_vec = np.array(close, dtype=np.float64)
    inputs = [high_vec, low_vec, close_vec]

    # Get optional outputs count and request all
    optional_count = len(eval(info["optional_outputs"]))
    optional_outputs = [True] * optional_count if optional_count > 0 else None

    outputs, _ = tulip_rs.indicators.stoch.indicator(inputs, options, optional_outputs)

    print("Full dataset calculation:")
    print(f"Full Stochastic Oscillator %K Line: {outputs[0]}")
    print(f"Full Stochastic Oscillator %D Line: {outputs[1]}")

    # Display optional outputs if they exist
    if optional_count > 0:
        for i in range(2, len(outputs)):
            print(f"Optional output {i - 1}: {outputs[i]}")

    ################################################### Partial calculation for state continuation
    # Use partial data (all but last 1 element, matching Rust example)
    partial_high = high[:-1]
    partial_low = low[:-1]
    partial_close = close[:-1]

    high_vec_partial = np.array(partial_high, dtype=np.float64)
    low_vec_partial = np.array(partial_low, dtype=np.float64)
    close_vec_partial = np.array(partial_close, dtype=np.float64)
    inputs_partial = [high_vec_partial, low_vec_partial, close_vec_partial]

    # For state continuation, use main outputs only (no optional_outputs parameter)
    outputs_partial, state = tulip_rs.indicators.stoch.indicator(
        inputs_partial, options
    )

    print(f"\nPartial calculation (first {len(partial_close)} elements):")
    print(f"Stochastic Oscillator %K Line: {outputs_partial[0]}")
    print(f"Stochastic Oscillator %D Line: {outputs_partial[1]}")

    ################################################### State Continuation Demo
    print("\nDemonstrating state continuation...")

    # Get state info
    print(f"State info: STOCH State - internal state for Stochastic Oscillator")

    # Continue with new data (last 1 element, matching Rust example)
    new_high = high[-1:]
    new_low = low[-1:]
    new_close = close[-1:]

    new_high_vec = np.array(new_high, dtype=np.float64)
    new_low_vec = np.array(new_low, dtype=np.float64)
    new_close_vec = np.array(new_close, dtype=np.float64)
    new_inputs = [new_high_vec, new_low_vec, new_close_vec]

    # Use batch_indicator without optional_outputs for performance
    continued_outputs = state.batch_indicator(new_inputs)

    print("Continued calculation:")
    print(f"New Stochastic Oscillator %K Line: {continued_outputs[0]}")
    print(f"New Stochastic Oscillator %D Line: {continued_outputs[1]}")

    print(
        f"\nData split: {len(partial_close)} + {len(new_close)} = {len(close)} total elements"
    )


if __name__ == "__main__":
    main()
