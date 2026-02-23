# Rust Function Template for ExchangeOutpost

A baseline template for creating Rust-based financial functions that can be deployed and executed on [ExchangeOutpost](https://www.exchangeoutpost.com/).

## Overview

This project provides a foundational structure for developing financial analysis functions in Rust that compile to WebAssembly (WASM) and integrate seamlessly with the ExchangeOutpost platform. It includes pre-built data structures for handling financial market data, candlestick charts, and decimal precision calculations.

## Features

- **WebAssembly Compilation**: Functions compile to WASM for cross-platform execution
- **Financial Data Structures**: Built-in support for candlestick data, ticker information, and market data
- **Decimal Precision**: Integrated `rust_decimal` for accurate financial calculations
- **Extism Plugin Framework**: Uses Extism PDK for plugin development and execution
- **Type Safety**: Strongly typed financial data structures with serde serialization support

## Project Structure

```
src/
├── lib.rs                      # Main entry point and plugin function, edit this file to implement your function
└── exchange_outpost/           # Contains financial data structures and utility functions, you should not edit this directory
```

## Function Manifest

The `manifest.json` file is a configuration file that defines metadata and behavior for your function when deployed on ExchangeOutpost. This file is crucial for proper function registration and execution.

### Manifest Structure

```json
{
    "function_name": "Your Function Name",
    "description": "A brief description of what your function does",
    "financial_data_keys": ["symbol1", "symbol2"],
    "call_arguments_schema": {
        "type": "object",
        "properties": {
            "parameter_name": {
                "type": ["number", "string"],
                "description": "Parameter description"
            }
        }
    },
    "enforce_schemas": false
}
```

### Manifest Fields

- **`function_name`**: The display name for your function in the ExchangeOutpost interface
- **`description`**: A clear description of what your function analyzes or computes
- **`financial_data_keys`**: An array of strings specifying what are the labels for the financial data your function requires.
- **`call_arguments_schema`**: A JSON schema defining the parameters your function accepts. This creates a user interface for function configuration
- **`enforce_schemas`**: Boolean flag that determines whether the system should validate input against the defined schemas.

### Example Configurations

**Simple Moving Average Function:**
```json
{
    "function_name": "Simple Moving Average",
    "description": "Calculates simple moving average for specified periods",
    "financial_data_keys": ["symbol_data"],
    "call_arguments_schema": {
        "type": "object",
        "properties": {
            "period": {
                "type": ["integer", "string"],
                "description": "Number of periods for moving average calculation",
                "minimum": 1,
                "maximum": 200,
                "default": 20
            }
        },
        "required": ["period"]
    },
    "enforce_schemas": true
}
```


## Core Components

### FinData
The main data container that provides access to:
- **Ticker Data**: Market data for multiple trading symbols
- **Piped Data**: Additional data passed between functions
- **Call Arguments**: Runtime parameters for function execution

### Candle
Represents financial candlestick data with:
- Timestamp (Unix epoch)
- OHLCV data (Open, High, Low, Close, Volume)
- Support for both `f64` and `Decimal` precision

## Getting Started

### Prerequisites

- Rust 1.70+ with 2024 edition support
- `wasm32-unknown-unknown` target installed

### Installation

1. Clone this repository:
```bash
git clone https://github.com/ExchangeOutpost/rust-function-template.git
cd rust-function-template
```

2. Install the WebAssembly target:
```bash
rustup target add wasm32-unknown-unknown
```

3. Build the project:
```bash
cargo build --target wasm32-unknown-unknown --release
```

## Development

### Creating Your Function

The main entry point is in `src/lib.rs`. Modify the `run` function to implement your financial analysis logic:

```rust
#[plugin_fn]
pub fn run(fin_data: FinData) -> FnResult<Output> {
    // Access ticker data
    let labels = fin_data.get_labels();
    
    // Get candlestick data for a specific symbol
    let candles = fin_data.get_candles("symbol_data")?;
    
    // Use decimal precision for calculations
    let decimal_candles = fin_data.get_candles_decimal("symbol_data")?;
    
    // Your analysis logic here
    
    Ok(Output {
        // Return your results
    })
}
```

### Working with Financial Data

```rust
// Get available ticker symbols
let symbols = fin_data.get_labels();

// Access candlestick data
let candles = fin_data.get_candles("symbol_data")?;
for candle in candles {
    println!("Close price: {}", candle.close);
}

// Use decimal precision for accurate calculations
let decimal_candles = fin_data.get_candles_decimal("symbol_data")?;

// Access piped data from other functions (usage of this is discouraged)
let pipe_sources = fin_data.get_pipe_sources();
let data = fin_data.get_data_from_pipe("previous_analysis")?;

// Get function call arguments
let args = fin_data.get_call_arguments();
```

### Output Structure

Define your output structure by modifying the `Output` struct:

```rust
#[derive(Serialize, ToBytes)]
#[encoding(Json)]
pub struct Output {
    pub result: f64,
    pub signal: String,
    pub confidence: f64,
    // Add your fields here
}
```

### Sending webhooks 

You can send webhooks to external services by using the `schedule_webhook` function:

```rust
use crate::exchange_outpost::schedule_webhook;

schedule_webhook("/webhook", payload);
```
This will send a POST request to the specified webhook URL with the given payload. The base url will be set to the one you configured for webhooks in your Organization settings.

### Sending emails

You can send email notifications by using the `schedule_email` function:

```rust
use crate::exchange_outpost::schedule_email;

let email_body = "Alert: Price threshold reached!";
schedule_email("user@example.com", email_body);
```
This will send an email to the specified email address with the given body content.

## Building and Deployment

### Local Build

```bash
cargo build --target wasm32-unknown-unknown --release
```

The compiled WASM file will be located at:
`target/wasm32-unknown-unknown/release/rust-function-template.wasm`

### Automated Releases

This project includes GitHub Actions for automated releases. When you push a tag, it will:
1. Build the WASM binary
2. Create a GitHub release
3. Upload the binary as `finfunc.wasm`

To create a release:
```bash
git tag 1.0.0
git push origin 1.0.0
```
Tags must follow [semantic versioning](https://semver.org/).

### Testing Your Function
When pushing to the `master` branch, the CI will automatically build your function and create a preview release named `master`.
You can use this release to test your function on the ExchangeOutpost platform.

## Dependencies

- **extism-pdk** (1.4.0): Plugin development kit for WebAssembly functions
- **rust_decimal** (1.37.1): High-precision decimal arithmetic for financial calculations
- **serde** (1.0.219): Serialization/deserialization framework
- **serde_json** (1.0.140): JSON support for serde

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/new-feature`)
3. Commit your changes (`git commit -m 'Add new feature'`)
4. Push to the branch (`git push origin feature/new-feature`)
5. Open a Pull Request

## License

This project is licensed under the Apache License 2.0. See the [LICENSE](LICENSE) file for more details.


## Related Links

- [ExchangeOutpost Platform](https://www.exchangeoutpost.com/)
- [Extism Documentation](https://extism.org/)
- [Rust WebAssembly Book](https://rustwasm.github.io/docs/book/)
