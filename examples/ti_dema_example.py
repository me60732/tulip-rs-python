#!/usr/bin/env python3
"""
Python example for the DEMA indicator from tulip_rs_python.

This example demonstrates:
1. Basic DEMA calculation with optional outputs
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
    # Close prices - matches Rust example
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
    options = [5.0]  # Period

    # Show indicator info
    info = tulip_rs.indicators.dema.info()
    print(f"=== {info['name'].upper()} ({info['full_name']}) ===")
    print(f"Type: {info['indicator_type']}")
    print(f"Inputs: {info['inputs']}")
    print(f"Options: {info['options']} (current: {options})")
    print(f"Outputs: {info['outputs']}")
    print(f"Optional outputs: {info['optional_outputs']}")

    # Show minimum data requirement
    min_data = tulip_rs.indicators.dema.min_data(options)
    print(f"Minimum data required: {min_data}")

    # Show minimum data for accuracy
    min_data_accuracy = tulip_rs.indicators.dema.min_data_accuracy(options, 6)
    print(f"Minimum data for accuracy (6 decimals): {min_data_accuracy}")
    print()

    # Partial calculation (remove last 1 point) - matches Rust example
    partial_close = close[:-1]  # close.len() - 1
    partial_close_vec = np.array(partial_close, dtype=np.float64)
    inputs = [partial_close_vec]

    # Full calculation with optional outputs
    optional_count = len(eval(info["optional_outputs"]))
    optional_outputs = [True] * optional_count if optional_count > 0 else None

    outputs, state = tulip_rs.indicators.dema.indicator(
        inputs, options, optional_outputs
    )
    print(f"DEMA Line: {outputs[0]}")
    print(f"EMA Line: {outputs[1]}")

    # State info
    state_info = state.get_info()
    print(f"State info: {state_info}")

    # Continuation with remaining data - matches Rust example
    continuation_close = close[-1:]  # last 1 point
    continuation_close_vec = np.array(continuation_close, dtype=np.float64)
    new_inputs = [continuation_close_vec]

    new_outputs = state.batch_indicator(new_inputs)
    print(f"\nNew DEMA Line: {new_outputs[0]}")


if __name__ == "__main__":
    main()
