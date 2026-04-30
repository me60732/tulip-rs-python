#!/usr/bin/env python3
"""
Python example for the FOSC indicator from tulip_rs_python.

This example demonstrates:
1. Basic FOSC calculation with optional outputs
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

    # Options for FOSC: period=5 (matching Rust example)
    options = [5.0]

    ################################################### Show Indicator Info First
    info = tulip_rs.indicators.fosc.info()
    print(f"=== {info['name'].upper()} ({info['full_name']}) ===")
    print(f"Type: {info['indicator_type']}")
    print(f"Inputs: {info['inputs']}")
    print(f"Options: {info['options']} (current: {options})")
    print(f"Outputs: {info['outputs']}")
    print(f"Optional outputs: {info['optional_outputs']}")

    # Show minimum data requirement
    min_data = tulip_rs.indicators.fosc.min_data(options)
    print(f"Minimum data required: {min_data}")

    # Show minimum data for accuracy
    min_data_accuracy = tulip_rs.indicators.fosc.min_data_accuracy(options, 6)
    print(f"Minimum data for accuracy (6 decimals): {min_data_accuracy}")
    print()

    ################################################### Full dataset calculation - Request ALL optional outputs
    close_vec = np.array(close, dtype=np.float64)
    inputs = [close_vec]

    # Get optional outputs count and request all (matching Rust example with [true, true, true, true])
    outputs, _ = tulip_rs.indicators.fosc.indicator(
        inputs, options, [True, True, True, True]
    )

    print("Full dataset calculation:")
    print(f"Full FOSC Line: {outputs[0]}")
    print(f"Full TSF Line: {outputs[1]}")
    print(f"Full Linreg Line: {outputs[2]}")
    print(f"Full Slope Line: {outputs[3]}")
    print(f"Full Intercept Line: {outputs[4]}")

    ################################################### Partial calculation for state continuation
    # Use partial data (all but last 5 elements, matching Rust example)
    partial_close = close[:-5]
    close_vec_partial = np.array(partial_close, dtype=np.float64)
    inputs_partial = [close_vec_partial]

    # For state continuation, use main outputs only (no optional_outputs parameter)
    outputs_partial, state = tulip_rs.indicators.fosc.indicator(inputs_partial, options)

    print(f"\nPartial calculation (first {len(partial_close)} elements):")
    print(f"Partial FOSC Line: {outputs_partial[0]}")

    ################################################### State Continuation Demo
    print("\nDemonstrating state continuation...")

    # Get state info
    print(f"State info: FOSC State - internal state for Forecast Oscillator")

    # Continue with new data (last 5 elements, matching Rust example)
    new_close = close[-5:]
    new_close_vec = np.array(new_close, dtype=np.float64)
    new_inputs = [new_close_vec]

    # Use batch_indicator without optional_outputs for performance
    continued_outputs = state.batch_indicator(new_inputs)

    print("Continued calculation:")
    print(f"Final FOSC Line: {continued_outputs[0]}")

    print(
        f"\nData split: {len(partial_close)} + {len(new_close)} = {len(close)} total elements"
    )


if __name__ == "__main__":
    main()
