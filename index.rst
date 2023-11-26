Welcome to Finalytics Documentation
====================================

Symbols Module
--------------

This module provides functions related to symbols.

**get_symbols(query, asset_class)**
    Fetches ticker symbols that closely match the specified query and asset class.

    - **Arguments:**
        - `query` (`str`): The query to search for.
        - `asset_class` (`str`): The asset class to search for.

    - **Returns:**
        - `List[str]`: A list of ticker symbols that closely match the query and asset class.

    **Example**

    ::

        import finalytics

        symbols = finalytics.get_symbols("Apple", "Equity")
        print(symbols)


Ticker Module
-------------

This module contains the `Ticker` class.

Ticker Class
------------
    A Python wrapper for the Ticker class in Finalytics.

Ticker Class Methods
---------------------

1. **new(symbol: str) -> Ticker**
    Create a new Ticker object.

    - **Arguments:**
        - `symbol` (`str`): The ticker symbol of the asset.

    - **Returns:**
        - `Ticker`: A Ticker object.

    - **Example:**
        ::

                import finalytics

                ticker = finalytics.Ticker("AAPL")
                print(ticker.name, ticker.exchange, ticker.category, ticker.asset_class)


2. **get_current_price() -> float**
    Get the current price of the ticker.

    - **Returns:**
        - `float`: The current price of the ticker.

    - **Example:**
        ::

                import finalytics

                ticker = finalytics.Ticker("AAPL")
                current_price = ticker.get_current_price()


3. **get_summary_stats() -> dict**
    Get summary technical and fundamental statistics for the ticker.

    - **Returns:**
        - `dict`: A dictionary containing the summary statistics.

    - **Example:**
        ::

                import finalytics

                ticker = finalytics.Ticker("AAPL")
                summary_stats = ticker.get_summary_stats()


4. **get_price_history(start: str, end: str, interval: str) -> DataFrame**
    Get the ohlcv data for the ticker for a given time period.

    - **Arguments:**
        - `start` (`str`): The start date of the time period in the format YYYY-MM-DD.
        - `end` (`str`): The end date of the time period in the format YYYY-MM-DD.
        - `interval` (`str`): The interval of the data (2m, 5m, 15m, 30m, 1h, 1d, 1wk, 1mo, 3mo).

    - **Returns:**
        - `DataFrame`: A Polars DataFrame containing the ohlcv data.

    - **Example:**
        ::

                import finalytics

                ticker = finalytics.Ticker("AAPL")
                ohlcv = ticker.get_price_history("2020-01-01", "2020-12-31", "1d")


5. **get_options_chain() -> DataFrame**
    Get the options chain for the ticker.

    - **Returns:**
        - `DataFrame`: A Polars DataFrame containing the options chain.

    - **Example:**
        ::

                import finalytics

                ticker = finalytics.Ticker("AAPL")
                options_chain = ticker.get_options_chain()


6. **get_news(start: str, end: str, compute_sentiment: bool) -> dict**
    Get the news for the ticker for a given time period.

    - **Arguments:**
        - `start` (`str`): The start date of the time period in the format YYYY-MM-DD.
        - `end` (`str`): The end date of the time period in the format YYYY-MM-DD.
        - `compute_sentiment` (`bool`): Whether to compute the sentiment of the news articles.

    - **Returns:**
        - `dict`: A dictionary containing the news articles (and sentiment results if requested).

    - **Example:**
        ::

                import finalytics

                ticker = finalytics.Ticker("AAPL")
                news = ticker.get_news("2020-01-01", "2020-12-31", False)


7. **get_income_statement() -> DataFrame**
    Get the Income Statement for the ticker.

    - **Returns:**
        - `DataFrame`: A Polars DataFrame containing the Income Statement.

    - **Example:**
        ::

                import finalytics

                ticker = finalytics.Ticker("AAPL")
                income_statement = ticker.get_income_statement()


8. **get_balance_sheet() -> DataFrame**
    Get the Balance Sheet for the ticker.

    - **Returns:**
        - `DataFrame`: A Polars DataFrame containing the Balance Sheet.

    - **Example:**
        ::

                import finalytics

                ticker = finalytics.Ticker("AAPL")
                balance_sheet = ticker.get_balance_sheet()


9. **get_cashflow_statement() -> DataFrame**
    Get the Cashflow Statement for the ticker.

    - **Returns:**
        - `DataFrame`: A Polars DataFrame containing the Cashflow Statement.

    - **Example:**
        ::

                import finalytics

                ticker = finalytics.Ticker("AAPL")
                cashflow_statement = ticker.get_cashflow_statement()


10. **get_financial_ratios() -> DataFrame**
    Get the Financial Ratios for the ticker.

    - **Returns:**
        - `DataFrame`: A Polars DataFrame containing the Financial Ratios.

    - **Example:**
        ::

                import finalytics

                ticker = finalytics.Ticker("AAPL")
                financial_ratios = ticker.get_financial_ratios()


11. **compute_performance_stats(start: str, end: str, interval: str, benchmark: str, confidence_level: float, risk_free_rate: float) -> dict**
    Compute the performance statistics for the ticker.

    - **Arguments:**
        - `start` (`str`): The start date of the time period in the format YYYY-MM-DD.
        - `end` (`str`): The end date of the time period in the format YYYY-MM-DD.
        - `interval` (`str`): The interval of the data (2m, 5m, 15m, 30m, 1h, 1d, 1wk, 1mo, 3mo).
        - `benchmark` (`str`): The ticker symbol of the benchmark to compare against.
        - `confidence_level` (`float`): The confidence level for the VaR and ES calculations.
        - `risk_free_rate` (`float`): The risk free rate to use in the calculations.

    - **Returns:**
        - `dict`: A dictionary containing the performance statistics.

    - **Example:**
        ::

               import finalytics

               ticker = finalytics.Ticker("AAPL")
               performance_stats = ticker.compute_performance_stats("2020-01-01", "2020-12-31", "1d", "^GSPC", 0.95, 0.02)


12. **display_performance_chart(start: str, end: str, interval: str, benchmark: str, confidence_level: float, risk_free_rate: float, display_format: str)**
    Display the performance chart for the ticker.

    - **Arguments:**
        - `start` (`str`): The start date of the time period in the format YYYY-MM-DD.
        - `end` (`str`): The end date of the time period in the format YYYY-MM-DD.
        - `interval` (`str`): The interval of the data (2m, 5m, 15m, 30m, 1h, 1d, 1wk, 1mo, 3mo).
        - `benchmark` (`str`): The ticker symbol of the benchmark to compare against.
        - `confidence_level` (`float`): The confidence level for the VaR and ES calculations.
        - `risk_free_rate` (`float`): The risk free rate to use in the calculations.
        - `display_format` (`str`): The format to display the chart in (png, html).

    - **Example:**
        ::

                import finalytics

                ticker = finalytics.Ticker("AAPL")
                ticker.display_performance_chart("2020-01-01", "2020-12-31", "1d", "^GSPC", 0.95, 0.02, "html")


13. **display_candlestick_chart(start: str, end: str, interval: str, display_format: str)**
    Display the candlestick chart for the ticker.

    - **Arguments:**
        - `start` (`str`): The start date of the time period in the format YYYY-MM-DD.
        - `end` (`str`): The end date of the time period in the format YYYY-MM-DD.
        - `interval` (`str`): The interval of the data (2m, 5m, 15m, 30m, 1h, 1d, 1wk, 1mo, 3mo).
        - `display_format` (`str`): The format to display the chart in (png, html).

    - **Example:**
        ::

                import finalytics

                ticker = finalytics.Ticker("AAPL")
                ticker.display_candlestick_chart("2020-01-01", "2020-12-31", "1d", "html")


14. **display_options_chart(risk_free_rate: float, display_format: str)**
    Display the options volatility surface, smile, and term structure charts for the ticker.

    - **Arguments:**
        - `risk_free_rate` (`float`): The risk free rate to use in the calculations.
        - `display_format` (`str`): The format to display the chart in (png, html).

    - **Example:**
        ::

                import finalytics

                ticker = finalytics.Ticker("AAPL")
                ticker.display_options_chart(0.02, "html")



Portfolio Module
----------------

This module contains the `Portfolio` class.

Portfolio Class
---------------
    A Python wrapper for the PortfolioCharts class in Finalytics.

Portfolio Class Methods
-------------------------

1. **new(ticker_symbols: List[str], benchmark_symbol: str, start_date: str, end_date: str, interval: str, confidence_level: float, risk_free_rate: float, max_iterations: int, objective_function: str) -> Portfolio**
    Create a new Portfolio object.

    - **Arguments:**
        - `ticker_symbols` (`List[str]`): List of ticker symbols for the assets in the portfolio.
        - `benchmark_symbol` (`str`): The ticker symbol of the benchmark to compare against.
        - `start_date` (`str`): The start date of the time period in the format YYYY-MM-DD.
        - `end_date` (`str`): The end date of the time period in the format YYYY-MM-DD.
        - `interval` (`str`): The interval of the data (2m, 5m, 15m, 30m, 1h, 1d, 1wk, 1mo, 3mo).
        - `confidence_level` (`float`): The confidence level for the VaR and ES calculations.
        - `risk_free_rate` (`float`): The risk-free rate to use in the calculations.
        - `max_iterations` (`int`): The maximum number of iterations to use in the optimization.
        - `objective_function` (`str`): The objective function to use in the optimization (max_sharpe, min_vol, max_return, nin_var, min_cvar, min_drawdown).

    - **Returns:**
        - `Portfolio`: A Portfolio object.

    - **Example:**
        ::

                import finalytics

                portfolio = finalytics.Portfolio(["AAPL", "GOOG", "MSFT"], "^GSPC", "2020-01-01", "2021-01-01", "1d", 0.95, 0.02, 1000, "max_sharpe")


2. **get_optimization_results() -> dict**
    Get the portfolio optimization results.

    - **Returns:**
        - `dict`: A dictionary containing optimization results.

    - **Example:**
        ::

                import finalytics

                portfolio = finalytics.Portfolio(["AAPL", "GOOG", "MSFT"], "^GSPC", "2020-01-01", "2021-01-01", "1d", 0.95, 0.02, 1000, "max_sharpe")
                optimization_results = portfolio.get_optimization_results()


3. **display_portfolio_charts(display_format: str)**
    Display the portfolio optimization charts.

    - **Arguments:**
        - `display_format` (`str`): The format to display the charts in (html, png).

    - **Example:**
        ::

                import finalytics

                portfolio = finalytics.Portfolio(["AAPL", "GOOG", "MSFT"], "^GSPC", "2020-01-01", "2021-01-01", "1d", 0.95, 0.02, 1000, "max_sharpe")
                portfolio.display_portfolio_charts("html")


DeFi Module
-------------

.. _defi_pools:

DefiPools Class
---------------

This class is a Python wrapper for the `finalytics` DefiPools class.

DefiPools Class Methods
-------------------------

1. **new() -> DefiPools**
    Create a new `DefiPools` object.

    - **Returns:**
        - `DefiPools`: A `DefiPools` object.

    - **Example:**
        ::

            import finalytics

            defi_pools = finalytics.DefiPools()
            print(f"Total Value Locked: ${defi_pools.total_value_locked:,.0f}")
            print(defi_pools.pools_data)
            print(defi_pools.unique_pools)
            print(defi_pools.unique_protocols)
            print(defi_pools.unique_chains)
            print(defi_pools.no_il_pools)
            print(defi_pools.stable_coin_pools)


2. **search_pools_by_symbol(symbol: str) -> List[str]**
    Search the pools data for pools that match the search term.

    - **Arguments:**
        - `symbol` (`str`): Cryptocurrency symbol.

    - **Returns:**
        - `List[str]`: List of pools that match the search term.

    - **Example:**
        ::

            import finalytics

            defi_pools = finalytics.DefiPools()
            print(defi_pools.search_pools_by_symbol("USDC"))


3. **display_top_protocols_by_tvl(pool_symbol: str, num_protocols: int, display_format: str)**
    Display the top protocols for a given symbol by total value locked.

    - **Arguments:**
        - `pool_symbol` (`str`): Liquidity pool symbol.
        - `num_protocols` (`int`): Number of protocols to display.
        - `display_format` (`str`): Display format for the chart (html or svg).

    - **Example:**
        ::

            import finalytics

            defi_pools = finalytics.DefiPools()
            defi_pools.display_top_protocols_by_tvl("USDC-USDT", 20, "html")

.. _defi_balances:

DefiBalances Class
------------------

This class is a Python wrapper for the `finalytics` DefiBalances class.

DefiBalances Class Methods
-----------------------------

1. **new(protocols: List[str], chains: List[str], address: str, display_format: str) -> PyDefiBalances**
    Initializes a new `DefiBalances` object.

    - **Arguments:**
        - `protocols` (`List[str]`): List of protocols to fetch balances for.
        - `chains` (`List[str]`): List of chains to fetch balances for.
        - `address` (`str`): Wallet address to fetch balances for.
        - `display_format` (`str`): Display format for the chart (html or svg).

    - **Returns:**
        - `DefiBalances`: A `DefiBalances` object.

    - **Example:**
        ::

            import finalytics

            defi_balances = finalytics.DefiBalances(["wallet", "eigenlayer", "uniswap-v3", "gearbox", "ether.fi"],
                                                       ["ethereum", "arbitrum"],
                                                       "0x7ac34681f6aaeb691e150c43ee494177c0e2c183",
                                                       "html")
            print(defi_balances.balances)


2. **get_supported_protocols() -> Dict[str, List[str]]**
    Fetches the supported protocols and chains for the `DefiBalances` class.

    - **Returns:**
        - `Dict[str, List[str]]`: Dictionary of protocols and chains.

    - **Example:**
        ::

            import finalytics

            supported_protocols = finalytics.get_supported_protocols()
            print(supported_protocols)
