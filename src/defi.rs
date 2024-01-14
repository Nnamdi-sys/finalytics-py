use finalytics::charts::defi::{DefiBalances, DefiPools};
use finalytics::data::defi::get_protocols;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use tokio::task;
use crate::ffi::{display_html, display_html_with_iframe, rust_df_to_py_df};

#[pyclass]
#[pyo3(name = "DefiPools")]
pub struct PyDefiPools {
    #[pyo3(get, set)]
    pub pools_data: PyObject,
    #[pyo3(get, set)]
    pub unique_pools: Vec<String>,
    #[pyo3(get, set)]
    pub unique_protocols: Vec<String>,
    #[pyo3(get, set)]
    pub unique_chains: Vec<String>,
    #[pyo3(get, set)]
    pub no_il_pools: Vec<String>,
    #[pyo3(get, set)]
    pub stable_coin_pools: Vec<String>,
    #[pyo3(get, set)]
    pub total_value_locked: f64,
}


#[pymethods]
impl PyDefiPools {
    #[new]
    /// Create a new DefiPools object
    ///
    /// # Returns
    ///
    /// `DefiPools` object
    ///
    /// # Example
    ///
    /// ```
    /// import finalytics
    ///
    /// defi_pools = finalytics.DefiPools()
    /// print(f"Total Value Locked: ${defi_pools.total_value_locked:,.0f}")
    /// print(defi_pools.pools_data)
    /// print(defi_pools.unique_pools)
    /// print(defi_pools.unique_protocols)
    /// print(defi_pools.unique_chains)
    /// print(defi_pools.no_il_pools)
    /// print(defi_pools.stable_coin_pools)
    /// ```
    pub fn new() -> Self {
        task::block_in_place(move || {
            let pools = tokio::runtime::Runtime::new().unwrap().block_on(
                DefiPools::new()).unwrap();
            PyDefiPools {
                pools_data: rust_df_to_py_df(&pools.pools_data).unwrap(),
                unique_pools: pools.unique_pools,
                unique_protocols: pools.unique_protocols,
                unique_chains: pools.unique_chains,
                no_il_pools: pools.no_il_pools,
                stable_coin_pools: pools.stable_coin_pools,
                total_value_locked: pools.total_value_locked,
            }
        })
    }

    /// Search the pools data for pools that match the search term
    ///
    /// # Arguments
    ///
    /// * `symbol` - `str` - cryptocurrency symbol e.g. "USDC"
    ///
    /// # Returns
    ///
    /// `list` - list of pools that match the search term
    ///
    /// # Example
    ///
    /// ```
    /// import finalytics
    ///
    /// defi_pools = finalytics.DefiPools()
    /// print(defi_pools.search_pools_by_symbol("USDC"))
    /// ```
    pub fn search_pools_by_symbol(&self, symbol: String) -> Vec<String> {
        task::block_in_place(move || {
            let pools = tokio::runtime::Runtime::new().unwrap().block_on(
                DefiPools::new()).unwrap();
            let filtered_pools = pools.search_pools(&symbol);
            filtered_pools
        })
    }

    /// Display the top protocols for a given symbol by total value locked
    ///
    /// # Arguments
    ///
    /// * `pool_symbol` - `str` - liquidity pool symbol e.g. "USDC-USDT"
    /// * `num_protocols` - `int` - number of protocols to display
    /// * `display_format` - `str` - display format for the chart (html, svg, notebook)
    ///
    /// # Example
    ///
    /// ```
    /// import finalytics
    ///
    /// defi_pools = finalytics.DefiPools()
    /// defi_pools.display_top_protocols_by_tvl("USDC-USDT", 20, "html")
    /// ```
    pub fn display_top_protocols_by_tvl(&self, pool_symbol: String, num_protocols: usize, display_format: String)  {
        task::block_in_place(move || {
            let pools = tokio::runtime::Runtime::new().unwrap().block_on(
                DefiPools::new()).unwrap();
            match display_format.as_str() {
                "html" => {
                    let _ = pools.display_top_protocols_by_tvl(&pool_symbol, num_protocols, &display_format, "top_tvl.html").unwrap();
                },
                "svg" => {
                    let _ = pools.display_top_protocols_by_tvl(&pool_symbol, num_protocols, &display_format, "top_tvl.svg").unwrap();
                },
                "notebook" => {
                    let file_path = "top_tvl.html";

                    if let Err(err) = std::fs::File::create(file_path) {
                        eprintln!("Error creating file: {:?}", err);
                    }

                    let _ = pools.display_top_protocols_by_tvl(&pool_symbol, num_protocols, "html", file_path).unwrap();

                    let _ = display_html_with_iframe(None, "top_tvl").unwrap();
                },
                "colab" => {
                    let file_path = "top_tvl.html";

                    if let Err(err) = std::fs::File::create(file_path) {
                        eprintln!("Error creating file: {:?}", err);
                    }

                    let _ = pools.display_top_protocols_by_tvl(&pool_symbol, num_protocols, "html", file_path).unwrap();

                    let _ = display_html(None, "top_tvl").unwrap();
                },
                _ => {
                    println!("Invalid display format. Please use 'html', 'svg', 'notebook', 'colab'.")
                }
            }
        })
    }

    /// Display the top protocols for a given symbol by annual percentage yield
    ///
    /// # Arguments
    ///
    /// * `pool_symbol` - `str` - liquidity pool symbol e.g. "USDC-USDT"
    /// * `num_protocols` - `int` - number of protocols to display
    /// * `display_format` - `str` - display format for the chart (html, svg, notebook)
    ///
    /// # Example
    ///
    /// ```
    /// import finalytics
    ///
    /// defi_pools = finalytics.DefiPools()
    /// defi_pools.display_top_protocols_by_apy("USDC-USDT", 20, "html")
    /// ```
    pub fn display_top_protocols_by_apy(&self, pool_symbol: String, num_protocols: usize, display_format: String)  {
        task::block_in_place(move || {
            let pools = tokio::runtime::Runtime::new().unwrap().block_on(
                DefiPools::new()).unwrap();
            match display_format.as_str() {
                "html" => {
                    let _ = pools.display_top_protocols_by_apy(&pool_symbol, num_protocols, &display_format, "top_apy.html").unwrap();
                },
                "svg" => {
                    let _ = pools.display_top_protocols_by_apy(&pool_symbol, num_protocols, &display_format, "top_apy.svg").unwrap();
                },
                "notebook" => {
                    let file_path = "top_apy.html";

                    if let Err(err) = std::fs::File::create(file_path) {
                        eprintln!("Error creating file: {:?}", err);
                    }

                    let _ = pools.display_top_protocols_by_apy(&pool_symbol, num_protocols, "html", file_path).unwrap();

                    let _ = display_html_with_iframe(None, "top_apy").unwrap();
                },
                "colab" => {
                    let file_path = "top_apy.html";

                    if let Err(err) = std::fs::File::create(file_path) {
                        eprintln!("Error creating file: {:?}", err);
                    }

                    let _ = pools.display_top_protocols_by_apy(&pool_symbol, num_protocols, "html", file_path).unwrap();

                    let _ = display_html(None, "top_apy").unwrap();
                },
                _ => {
                    println!("Invalid display format. Please use 'html', 'svg', 'notebook' or 'colab'.")
                }
            }
        })
    }

    /// Display the the total value locked history for a given pool
    ///
    /// # Arguments
    ///
    /// * `pool_symbol` - `str` - liquidity pool symbol e.g. "USDC-USDT"
    /// * `protocol` - `str` - protocol e.g. "uniswap-v3"
    /// * `chain` - `str` - blockchain e.g. "ethereum"
    /// * `display_format` - `str` - display format for the chart (html, svg, notebook, colab)
    ///
    /// # Example
    ///
    /// ```
    /// import finalytics
    ///
    /// defi_pools = finalytics.DefiPools()
    /// defi_pools.display_pool_tvl_history("USDC-USDT", "uniswap-v3", "ethereum", "html")
    /// ```
    ///
    pub fn display_pool_tvl_history(&self, pool_symbol: String, protocol: String, chain: String, display_format: String) {
        task::block_in_place(move || {
            let pools = tokio::runtime::Runtime::new().unwrap().block_on(
                DefiPools::new()).unwrap();
            match display_format.as_str() {
                "html" => {
                    let _ = tokio::runtime::Runtime::new().unwrap().block_on(
                        pools.display_pool_tvl_history(&pool_symbol, &protocol, &chain, &display_format, "pool_tvl_history.html")
                    ).unwrap();
                },
                "svg" => {
                    let _ = tokio::runtime::Runtime::new().unwrap().block_on(
                        pools.display_pool_tvl_history(&pool_symbol, &protocol, &chain, &display_format, "pool_tvl_history.svg")
                    ).unwrap();
                },
                "notebook" => {
                    let file_path = "pool_tvl_history.html";

                    if let Err(err) = std::fs::File::create(file_path) {
                        eprintln!("Error creating file: {:?}", err);
                    }

                    let _ = tokio::runtime::Runtime::new().unwrap().block_on(
                        pools.display_pool_tvl_history(&pool_symbol, &protocol, &chain, "html", file_path)
                    ).unwrap();

                    let _ = display_html_with_iframe(None, "pool_tvl_history").unwrap();
                },
                "colab" => {
                    let file_path = "pool_tvl_history.html";

                    if let Err(err) = std::fs::File::create(file_path) {
                        eprintln!("Error creating file: {:?}", err);
                    }

                    let _ = tokio::runtime::Runtime::new().unwrap().block_on(
                        pools.display_pool_tvl_history(&pool_symbol, &protocol, &chain, "html", file_path)
                    ).unwrap();

                    let _ = display_html(None, "pool_tvl_history").unwrap();
                },
                _ => {
                    println!("Invalid display format. Please use 'html', 'svg', 'notebook' or 'colab'.")
                }
            }
        })
    }

    /// Display the the annual percentage yield history for a given pool
    ///
    /// # Arguments
    ///
    /// * `pool_symbol` - `str` - liquidity pool symbol e.g. "USDC-USDT"
    /// * `protocol` - `str` - protocol e.g. "uniswap-v3"
    /// * `chain` - `str` - blockchain e.g. "ethereum"
    /// * `display_format` - `str` - display format for the chart (html, svg, notebook)
    ///
    /// # Example
    ///
    /// ```
    /// import finalytics
    ///
    /// defi_pools = finalytics.DefiPools()
    /// defi_pools.display_pool_apy_history("USDC-USDT", "uniswap-v3", "ethereum", "html")
    /// ```
    pub fn display_pool_apy_history(&self, pool_symbol: String, protocol: String, chain: String, display_format: String) {
        task::block_in_place(move || {
            let pools = tokio::runtime::Runtime::new().unwrap().block_on(
                DefiPools::new()).unwrap();
            match display_format.as_str() {
                "html" => {
                    let _ = tokio::runtime::Runtime::new().unwrap().block_on(
                        pools.display_pool_apy_history(&pool_symbol, &protocol, &chain, &display_format, "pool_apy_history.html")
                    ).unwrap();
                },
                "svg" => {
                    let _ = tokio::runtime::Runtime::new().unwrap().block_on(
                        pools.display_pool_apy_history(&pool_symbol, &protocol, &chain, &display_format, "pool_apy_history.svg")
                    ).unwrap();
                },
                "notebook" => {
                    let file_path = "pool_apy_history.html";

                    if let Err(err) = std::fs::File::create(file_path) {
                        eprintln!("Error creating file: {:?}", err);
                    }

                    let _ = tokio::runtime::Runtime::new().unwrap().block_on(
                        pools.display_pool_apy_history(&pool_symbol, &protocol, &chain, "html", file_path)
                    ).unwrap();

                    let _ = display_html_with_iframe(None, "pool_apy_history").unwrap();
                },
                "colab" => {
                    let file_path = "pool_apy_history.html";

                    if let Err(err) = std::fs::File::create(file_path) {
                        eprintln!("Error creating file: {:?}", err);
                    }

                    let _ = tokio::runtime::Runtime::new().unwrap().block_on(
                        pools.display_pool_apy_history(&pool_symbol, &protocol, &chain, "html", file_path)
                    ).unwrap();

                    let _ = display_html(None, "pool_apy_history").unwrap();
                },
                _ => {
                    println!("Invalid display format. Please use 'html', 'svg', 'notebook' or 'colab'.")
                }
            }
        })
    }

}


#[pyclass]
#[pyo3(name = "DefiBalances")]
pub struct PyDefiBalances {
    #[pyo3(get, set)]
    pub protocols: Vec<String>,
    #[pyo3(get, set)]
    pub chains: Vec<String>,
    #[pyo3(get, set)]
    pub address: String,
    #[pyo3(get, set)]
    pub balances: PyObject,
}

#[pymethods]

impl PyDefiBalances {
    #[new]
    /// Fetches the user's balances for the specified protocols and chains
    ///
    /// # Dependencies
    /// This function requires node.js and pnpm to be installed on the system
    /// for macos: brew install node && npm install -g pnpm
    /// for ubuntu: sudo apt install nodejs && npm install -g pnpm
    /// for windows: https://nodejs.org/en/download/ && npm install -g pnpm
    ///
    /// # Arguments
    ///
    /// * `protocols` - `list` - list of protocols to fetch balances for (include "wallet" for wallet balances)
    /// * `chains` - `list` - list of chains to fetch balances for
    /// * `address` - `str` - wallet address to fetch balances for
    /// * `display_format` - `str` - display format for the chart (html, svg, notebook, colab)
    ///
    /// # Returns
    ///
    /// `DefiBalances` object
    ///
    /// # Example
    ///
    /// ```
    /// import finalytics
    ///
    /// defi_balances = finalytics.DefiBalances(["wallet", "eigenlayer", "uniswap-v3", "gearbox", "ether.fi"],
    ///                                         ["ethereum", "arbitrum"],
    ///                                         "0x7ac34681f6aaeb691e150c43ee494177c0e2c183",
    ///                                         "html")
    /// print(defi_balances.balances)
    /// ```
    pub fn new(protocols: Vec<String>, chains: Vec<String>, address: String, display_format: String) -> Self {
        task::block_in_place(move || {
            let balances = DefiBalances::new(protocols, chains, address).unwrap();

            match display_format.as_str() {
                "html" => {
                    let _ = balances.display_wallet_balance(&display_format, "wallet_balances.html").unwrap();
                    let _ = balances.display_protocols_balance(&display_format, "protocols_balances.html").unwrap();

                },
                "svg" => {
                    let _ = balances.display_wallet_balance(&display_format, "wallet_balances.svg").unwrap();
                    let _ = balances.display_protocols_balance(&display_format, "protocols_balances.svg").unwrap();
                },
                "notebook" => {
                    let wallet_file_path = "wallet_balances.html";
                    let protocols_file_path = "protocols_balances.html";

                    if let Err(err) = std::fs::File::create(wallet_file_path) {
                        eprintln!("Error creating file: {:?}", err);
                    }

                    if let Err(err) = std::fs::File::create(protocols_file_path) {
                        eprintln!("Error creating file: {:?}", err);
                    }

                    let _ = balances.display_wallet_balance("html", wallet_file_path).unwrap();
                    let _ = balances.display_protocols_balance("html", protocols_file_path).unwrap();

                    let _ = display_html_with_iframe(None, "wallet_balances").unwrap();
                    let _ = display_html_with_iframe(None, "protocols_balances").unwrap();
                },
                "colab" => {
                    let wallet_file_path = "wallet_balances.html";
                    let protocols_file_path = "protocols_balances.html";

                    if let Err(err) = std::fs::File::create(wallet_file_path) {
                        eprintln!("Error creating file: {:?}", err);
                    }

                    if let Err(err) = std::fs::File::create(protocols_file_path) {
                        eprintln!("Error creating file: {:?}", err);
                    }

                    let _ = balances.display_wallet_balance("html", wallet_file_path).unwrap();
                    let _ = balances.display_protocols_balance("html", protocols_file_path).unwrap();

                    let _ = display_html(None, "wallet_balances").unwrap();
                    let _ = display_html(None, "protocols_balances").unwrap();
                },
                _ => {
                    println!("Invalid display format. Please use 'html', 'svg' or 'notebook'.")
                }
            }

            PyDefiBalances {
                protocols: balances.protocols,
                chains: balances.chains,
                address: balances.address,
                balances: rust_df_to_py_df(&balances.balances).unwrap(),
            }
        })
    }
}

#[pyfunction]
/// Fetches the supported protocols and chains for the DefiBalances class from the llamafolio-api
///
/// # Returns
///
/// `dict` - dictionary of protocols and chains
///
/// # Example
///
/// ```
/// import finalytics
///
/// supported_protocols = finalytics.get_supported_protocols()
/// print(supported_protocols)
/// ```
pub fn get_supported_protocols() -> Py<PyDict> {
    task::block_in_place(move || {
        let protocols = get_protocols().unwrap();
        Python::with_gil(|py| {
            let py_dict = PyDict::new(py);
            for (protocol, chains) in protocols {
                let py_list = PyList::new(py, &chains);
                py_dict.set_item(protocol, py_list).unwrap();
            }
            py_dict.into()
        })
    })
}