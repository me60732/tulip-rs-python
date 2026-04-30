#!/usr/bin/env python3
"""
Python example for the CCI indicator from tulip_rs_python.

This example demonstrates:
1. Basic CCI calculation
2. Indicator info display
3. State continuation with new data
4. Full calculation verification
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

    high_vec = np.array(high, dtype=np.float64)
    low_vec = np.array(low, dtype=np.float64)
    close_vec = np.array(close, dtype=np.float64)
    inputs = [high_vec, low_vec, close_vec]

    # Options for the CCI calculation (matching Rust example)
    options = [5.0]

    ################################################### Show Indicator Info First
    info = tulip_rs.indicators.cci.info()
    print(f"=== {info['name'].upper()} ({info['full_name']}) ===")
    print(f"Type: {info['indicator_type']}")
    print(f"Inputs: {info['inputs']}")
    print(f"Options: {info['options']} (current: {options})")
    print(f"Outputs: {info['outputs']}")
    print(f"Optional outputs: {info['optional_outputs']}")

    # Show minimum data requirement
    min_data = tulip_rs.indicators.cci.min_data(options)
    print(f"Minimum data required: {min_data}")

    # Show minimum data for accuracy
    min_data_accuracy = tulip_rs.indicators.cci.min_data_accuracy(options, 6)
    print(f"Minimum data for accuracy (6 decimals): {min_data_accuracy}")
    print()

    ################################################### Calculating the Full CCI Line with Optional Outputs
    # Request optional outputs (matching Rust example with [true, false, false])
    optional_outputs = [True, False, False]
    outputs, _ = tulip_rs.indicators.cci.indicator(inputs, options, optional_outputs)
    print(f"Full CCI Line: {outputs[0]}")

    ################################################### Calculating the partial CCI Line
    # Use partial data for state demo (remove last 5 points, matching Rust example)
    high_vec2 = np.array(high[:-5], dtype=np.float64)
    low_vec2 = np.array(low[:-5], dtype=np.float64)
    close_vec2 = np.array(close[:-5], dtype=np.float64)
    inputs2 = [high_vec2, low_vec2, close_vec2]

    outputs2, state2 = tulip_rs.indicators.cci.indicator(
        inputs2, options, optional_outputs
    )
    print(f"Partial CCI Line: {outputs2[0]}")

    ################################################### State Continuation Demo
    print("Demonstrating state continuation...")

    # Get state info
    state_info = state2.get_info()
    print(f"State info: {state_info}")

    # Use the state to continue calculation with new data
    print("Adding new data to existing state...")
    # Continue with remaining data (last 5 points, matching Rust example)
    new_high_vec = np.array(high[-5:], dtype=np.float64)
    new_low_vec = np.array(low[-5:], dtype=np.float64)
    new_close_vec = np.array(close[-5:], dtype=np.float64)
    new_data = [new_high_vec, new_low_vec, new_close_vec]
    final_outputs = state2.batch_indicator(new_data)
    print(f"New CCI Line: {final_outputs[0]}")

    # Verify by calculating full sequence at once
    print("Verification - calculating full sequence:")
    # Verify with full data
    full_high_vec = np.array(high, dtype=np.float64)
    full_low_vec = np.array(low, dtype=np.float64)
    full_close_vec = np.array(close, dtype=np.float64)
    full_data = [full_high_vec, full_low_vec, full_close_vec]
    full_outputs, _ = tulip_rs.indicators.cci.indicator(
        full_data, options, optional_outputs
    )
    print(f"Full CCI Line: {full_outputs[0]}")


if __name__ == "__main__":
    main()
