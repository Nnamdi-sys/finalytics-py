Welcome to Finalytics Documentation
====================================

.. toctree::
   :maxdepth: 2
   :caption: Contents:

Symbols Module
--------------

This module provides functions related to symbols.

.. function:: get_symbols(query, asset_class) -> List[str]

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

   .. method:: __init__(symbol: str) -> Ticker

      Create a new Ticker object.

      :param symbol: The ticker symbol of the asset.
      :type symbol: str

      :returns: A Ticker object.
      :rtype: Ticker

      :example:

         Import the library and create a ticker:

         .. code-block:: python

            import finalytics

            ticker = finalytics.Ticker("AAPL")
            print(ticker.name, ticker.exchange, ticker.category, ticker.asset_class)

   .. method:: get_current_price() -> float

      Get the current price of the ticker.

      :returns: The current price of the ticker.
      :rtype: float

      :example:

         .. code-block:: python

            import finalytics

            ticker = finalytics.Ticker("AAPL")
            current_price = ticker.get_current_price()

   .. method:: get_summary_stats() -> dict

      Get summary technical and fundamental statistics for the ticker.

      :returns: A dictionary containing the summary statistics.
      :rtype: dict

      :example:

         .. code-block:: python

            import finalytics

            ticker = finalytics.Ticker("AAPL")
            summary_stats = ticker.get_summary_stats()

   .. method:: get_price_history(start: str, end: str, interval: str) -> DataFrame

      Get the ohlcv data for the ticker for a given time period.

      :param start: The start date of the time period in the format YYYY-MM-DD.
      :type start: str
      :param end: The end date of the time period in the format YYYY-MM-DD.
      :type end: str
      :param interval: The interval of the data (2m, 5m, 15m, 30m, 1h, 1d, 1wk, 1mo, 3mo).
      :type interval: str

      :returns: A Polars DataFrame containing the ohlcv data.
      :rtype: DataFrame

      :example:

         .. code-block:: python

            import finalytics

            ticker = finalytics.Ticker("AAPL")
            ohlcv = ticker.get_price_history("2020-01-01", "2020-12-31", "1d")

   .. method:: get_options_chain() -> DataFrame

      Get the options chain for the ticker.

      :returns: A Polars DataFrame containing the options chain.
      :rtype: DataFrame

      :example:

         .. code-block:: python

            import finalytics

            ticker = finalytics.Ticker("AAPL")
            options_chain = ticker.get_options_chain()

   .. method:: get_news(start: str, end: str, compute_sentiment: bool) -> dict

      Get the news for the ticker for a given time period.

      :param start: The start date of the time period in the format YYYY-MM-DD.
      :type start: str
      :param end: The end date of the time period in the format YYYY-MM-DD.
      :type end: str
      :param compute_sentiment: Whether to compute the sentiment of the news articles.
      :type compute_sentiment: bool

      :returns: A dictionary containing the news articles (and sentiment results if requested).
      :rtype: dict

      :example:

         .. code-block:: python

            import finalytics

            ticker = finalytics.Ticker("AAPL")
            news = ticker.get_news("2020-01-01", "2020-12-31", False)

   .. method:: get_income_statement() -> DataFrame

      Get the Income Statement for the ticker.

      :returns: A Polars DataFrame containing the Income Statement.
      :rtype: DataFrame

      :example:

         .. code-block:: python

            import finalytics

            ticker = finalytics.Ticker("AAPL")
            income_statement = ticker.get_income_statement()

   .. method:: get_balance_sheet() -> DataFrame

      Get the Balance Sheet for the ticker.

      :returns: A Polars DataFrame containing the Balance Sheet.
      :rtype: DataFrame

      :example:

         .. code-block:: python

            import finalytics

            ticker = finalytics.Ticker("AAPL")
            balance_sheet = ticker.get_balance_sheet()

   .. method:: get_cashflow_statement() -> DataFrame

      Get the Cashflow Statement for the ticker.

      :returns: A Polars DataFrame containing the Cashflow Statement.
      :rtype: DataFrame

      :example:

         .. code-block:: python

            import finalytics

            ticker = finalytics.Ticker("AAPL")
            cashflow_statement = ticker.get_cashflow_statement()

   .. method:: get_financial_ratios() -> DataFrame

      Get the Financial Ratios for the ticker.

      :returns: A Polars DataFrame containing the Financial Ratios.
      :rtype: DataFrame

      :example:

         .. code-block:: python

            import finalytics

            ticker = finalytics.Ticker("AAPL")
            financial_ratios = ticker.get_financial_ratios()

   .. method:: compute_performance_stats(start: str, end: str, interval: str, benchmark: str, confidence_level: float, risk_free_rate: float) -> dict

      Compute the performance statistics for the ticker.

      :param start: The start date of the time period in the format YYYY-MM-DD.
      :type start: str
      :param end: The end date of the time period in the format YYYY-MM-DD.
      :type end: str
      :param interval: The interval of the data (2m, 5m, 15m, 30m, 1h, 1d, 1wk, 1mo, 3mo).
      :type interval: str
      :param benchmark: The ticker symbol of the benchmark to compare against.
      :type benchmark: str
      :param confidence_level: The confidence level for the VaR and ES calculations.
      :type confidence_level: float
      :param risk_free_rate: The risk free rate to use in the calculations.
      :type risk_free_rate: float

      :returns: A dictionary containing the performance statistics.
      :rtype: dict

      :example:

         .. code-block:: python

            import finalytics

            ticker = finalytics.Ticker("AAPL")
            performance_stats = ticker.compute_performance_stats("2020-01-01", "2020-12-31", "1d", "^GSPC", 0.95, 0.02)

   .. method:: display_performance_chart(start: str, end: str, interval: str, benchmark: str, confidence_level: float, risk_free_rate: float, display_format: str) -> None

      Display the performance chart for the ticker.

      :param start: The start date of the time period in the format YYYY-MM-DD.
      :type start: str
      :param end: The end date of the time period in the format YYYY-MM-DD.
      :type end: str
      :param interval: The interval of the data (2m, 5m, 15m, 30m, 1h, 1d, 1wk, 1mo, 3mo).
      :type interval: str
      :param benchmark: The ticker symbol of the benchmark to compare against.
      :type benchmark: str
      :param confidence_level: The confidence level for the VaR and ES calculations.
      :type confidence_level: float
      :param risk_free_rate: The risk free rate to use in the calculations.
      :type risk_free_rate: float
      :param display_format: The format to display the chart in (png, html, notebook).
      :type display_format: str

      :example:

         .. code-block:: python

            import finalytics

            ticker = finalytics.Ticker("AAPL")
            ticker.display_performance_chart("2020-01-01", "2020-12-31", "1d", "^GSPC", 0.95, 0.02, "html")

   .. method:: display_candlestick_chart(start: str, end: str, interval: str, display_format: str) -> None

      Display the candlestick chart for the ticker.

      :param start: The start date of the time period in the format YYYY-MM-DD.
      :type start: str
      :param end: The end date of the time period in the format YYYY-MM-DD.
      :type end: str
      :param interval: The interval of the data (2m, 5m, 15m, 30m, 1h, 1d, 1wk, 1mo, 3mo).
      :type interval: str
      :param display_format: The format to display the chart in (png, html, notebook).
      :type display_format: str

      :example:

         .. code-block:: python

            import finalytics

            ticker = finalytics.Ticker("AAPL")
            ticker.display_candlestick_chart("2020-01-01", "2020-12-31", "1d", "html")

   .. method:: display_options_chart(risk_free_rate: float, display_format: str) -> None

      Display the options volatility surface, smile, and term structure charts for the ticker.

      :param risk_free_rate: The risk free rate to use in the calculations.
      :type risk_free_rate: float
      :param chart_type: The type of volatility chart to display (surface, smile, term_structure).
      :type chart_type: str
      :param display_format: The format to display the chart in (png, html, notebook).
      :type display_format: str

      :example:

         .. code-block:: python

            import finalytics

            ticker = finalytics.Ticker("AAPL")
            ticker.display_options_chart(0.02, "html")




Portfolio Module
----------------

This module contains the `Portfolio` class.

Portfolio Class
---------------
    A Python wrapper for the PortfolioCharts class in Finalytics.

   .. method:: __init__(ticker_symbols: List[str], benchmark_symbol: str, start_date: str, end_date: str, interval: str, confidence_level: float, risk_free_rate: float, max_iterations: int, objective_function: str) -> Portfolio

      Create a new Portfolio object.

      :param ticker_symbols: List of ticker symbols for the assets in the portfolio.
      :type ticker_symbols: List[str]
      :param benchmark_symbol: The ticker symbol of the benchmark to compare against.
      :type benchmark_symbol: str
      :param start_date: The start date of the time period in the format YYYY-MM-DD.
      :type start_date: str
      :param end_date: The end date of the time period in the format YYYY-MM-DD.
      :type end_date: str
      :param interval: The interval of the data (2m, 5m, 15m, 30m, 1h, 1d, 1wk, 1mo, 3mo).
      :type interval: str
      :param confidence_level: The confidence level for the VaR and ES calculations.
      :type confidence_level: float
      :param risk_free_rate: The risk-free rate to use in the calculations.
      :type risk_free_rate: float
      :param max_iterations: The maximum number of iterations to use in the optimization.
      :type max_iterations: int
      :param objective_function: The objective function to use in the optimization (max_sharpe, min_vol, max_return, nin_var, min_cvar, min_drawdown).
      :type objective_function: str

      :returns: A Portfolio object.
      :rtype: Portfolio

      :example:

         Import the library and create a portfolio:

         .. code-block:: python

            import finalytics

            portfolio = finalytics.Portfolio(["AAPL", "GOOG", "MSFT"], "^GSPC", "2020-01-01", "2021-01-01", "1d", 0.95, 0.02, 1000, "max_sharpe")

   .. method:: get_optimization_results() -> dict

      Get the portfolio optimization results.

      :returns: A dictionary containing optimization results.
      :rtype: dict

      :example:

         .. code-block:: python

            import finalytics

            portfolio = finalytics.Portfolio(["AAPL", "GOOG", "MSFT"], "^GSPC", "2020-01-01", "2021-01-01", "1d", 0.95, 0.02, 1000, "max_sharpe")
            optimization_results = portfolio.get_optimization_results()

   .. method:: display_portfolio_charts(display_format: str) -> None

      Display the portfolio optimization charts.

      :param chart_type: The type of chart to display (optimization, performance, asset_returns).
      :type chart_type: str
      :param display_format: The format to display the charts in (html, png, notebook).
      :type display_format: str

      :example:

         .. code-block:: python

            import finalytics

            portfolio = finalytics.Portfolio(["AAPL", "GOOG", "MSFT"], "^GSPC", "2020-01-01", "2021-01-01", "1d", 0.95, 0.02, 1000, "max_sharpe")
            portfolio.display_portfolio_charts("performance", "html")



DeFi Module
-------------

This module contains the `DefiPools` and `DefiBalances` classes.

DefiPools Class
---------------

    This class is a Python wrapper for the `finalytics` DefiPools class.

    .. class:: DefiPools

       DefiPools is a class that provides information about decentralized finance (DeFi) liquidity pools.

       .. method:: __init__() -> DefiPools

          Create a new `DefiPools` object.

          :returns: A `DefiPools` object.
          :rtype: DefiPools

          :example:

             Import the library and create a `DefiPools` object:

             .. code-block:: python

                import finalytics

                defi_pools = finalytics.DefiPools()
                print(f"Total Value Locked: ${defi_pools.total_value_locked:,.0f}")
                print(defi_pools.pools_data)


       .. method:: search_pools_by_symbol(symbol: str) -> List[str]

          Search the pools data for pools that match the search term.

          :param symbol: Cryptocurrency symbol.
          :type symbol: str

          :returns: List of pools that match the search term.
          :rtype: List[str]

          :example:

             .. code-block:: python

                import finalytics

                defi_pools = finalytics.DefiPools()
                print(defi_pools.search_pools_by_symbol("USDC"))


       .. method:: display_top_protocols_by_tvl(pool_symbol: str, num_protocols: int, display_format: str)

          Display the top protocols for a given symbol by total value locked.

          :param pool_symbol: Liquidity pool symbol.
          :type pool_symbol: str
          :param num_protocols: Number of protocols to display.
          :type num_protocols: int
          :param display_format: Display format for the chart (html, svg, notebook).
          :type display_format: str

          :example:

             .. code-block:: python

                import finalytics

                defi_pools = finalytics.DefiPools()
                defi_pools.display_top_protocols_by_tvl("USDC-USDT", 20, "html")


       .. method:: display_top_protocols_by_apy(pool_symbol: str, num_protocols: int, display_format: str)

          Display the top protocols for a given symbol by APY.

          :param pool_symbol: Liquidity pool symbol.
          :type pool_symbol: str
          :param num_protocols: Number of protocols to display.
          :type num_protocols: int
          :param display_format: Display format for the chart (html, svg, notebook).
          :type display_format: str

          :example:

             .. code-block:: python

                import finalytics

                defi_pools = finalytics.DefiPools()
                defi_pools.display_top_protocols_by_apy("USDC-USDT", 20, "html")


       .. method:: display_pool_tvl_history(pool_symbol: str, protocol: str, chain: str, display_format: str)

          Display the total value locked history for a given pool.

          :param pool_symbol: Liquidity pool symbol.
          :type pool_symbol: str
          :param protocol: Protocol.
          :type protocol: str
          :param chain: Blockchain.
          :type chain: str
          :param display_format: Display format for the chart (html, svg, notebook).
          :type display_format: str

          :example:

             .. code-block:: python

                import finalytics

                defi_pools = finalytics.DefiPools()
                defi_pools.display_pool_tvl_history("USDC-USDT", "uniswap-v3", "ethereum", "html")


       .. method:: display_pool_apy_history(pool_symbol: str, protocol: str, chain: str, display_format: str)

          Display the APY history for a given pool.

          :param pool_symbol: Liquidity pool symbol.
          :type pool_symbol: str
          :param protocol: Protocol.
          :type protocol: str
          :param chain: Blockchain.
          :type chain: str
          :param display_format: Display format for the chart (html, svg, notebook).
          :type display_format: str

          :example:

             .. code-block:: python

                import finalytics

                defi_pools = finalytics.DefiPools()
                defi_pools.display_pool_apy_history("USDC-USDT", "uniswap-v3", "ethereum", "html")


DefiBalances Class
------------------

    This class is a Python wrapper for the `finalytics` DefiBalances class.

    .. class:: DefiBalances

       DefiBalances is a class that provides information about decentralized finance (DeFi) wallet balances.

       .. method:: __init__(protocols: List[str], chains: List[str], address: str, display_format: str) -> DefiBalances

          Initializes a new `DefiBalances` object.

          :param protocols: List of protocols to fetch balances for.
          :type protocols: List[str]
          :param chains: List of chains to fetch balances for.
          :type chains: List[str]
          :param address: Wallet address to fetch balances for.
          :type address: str
          :param display_format: Display format for the chart (html, svg, notebook).
          :type display_format: str

          :returns: A `DefiBalances` object.
          :rtype: DefiBalances

          :example:

             .. code-block:: python

                import finalytics

                defi_balances = finalytics.DefiBalances(["wallet", "eigenlayer", "blast", "ether.fi"],
                                                           ["ethereum", "arbitrum"],
                                                           "0x7ac34681f6aaeb691e150c43ee494177c0e2c183",
                                                           "html")
                print(defi_balances.balances)


    .. function:: get_supported_protocols() -> Dict[str, List[str]]

      Fetches the supported protocols and chains for the `DefiBalances` class.

      :returns: Dictionary of protocols and chains.
      :rtype: Dict[str, List[str]]

      :example:

         .. code-block:: python

            import finalytics

            supported_protocols = finalytics.get_supported_protocols()
            print(supported_protocols)
