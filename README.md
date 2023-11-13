![Finalytics](https://github.com/Nnamdi-sys/finalytics-py/raw/main/examples/logo-color.png)

[![pypi](https://img.shields.io/pypi/v/finalytics)](https://pypi.org/project/finalytics/)
![License](https://img.shields.io/crates/l/finalytics)
[![Homepage](https://img.shields.io/badge/homepage-finalytics.rs-blue)](https://finalytics.rs/)

This is a python binding for [Finalytics Rust Library](https://github.com/Nnamdi-sys/finalytics) designed for retrieving financial data and performing security analysis and portfolio optimization.

## Installation

#### Windows and MacOS

```bash
pip install finalytics
```

#### Linux

Still in development

## Documentation

### Symbol Search

```python
from finalytics import get_symbols

print(get_symbols("Apple", "Equity"))
print(get_symbols("Bitcoin", "Crypto"))
print(get_symbols("S&P 500", "Index"))
print(get_symbols("EURUSD", "Currency"))
print(get_symbols("SPY", "ETF"))
```

### Security Analysis

```python
from finalytics import Ticker

ticker = Ticker("AAPL")
print(ticker.get_current_price())
print(ticker.get_summary_stats())
print(ticker.get_price_history("2023-01-01", "2023-10-31", "1d"))
print(ticker.get_options_chain())
print(ticker.get_news("2023-11-01", "2023-11-10", False))
print(ticker.get_income_statement())
print(ticker.get_balance_sheet())
print(ticker.get_cashflow_statement())
print(ticker.get_financial_ratios())
print(ticker.compute_performance_stats("2023-01-01", "2023-10-31", "1d", "^GSPC", 0.95, 0.02))
ticker.display_performance_chart("2023-01-01", "2023-10-31", "1d", "^GSPC", 0.95, 0.02, "html")
ticker.display_candlestick_chart("2023-01-01", "2023-10-31", "1d", "html")
ticker.display_options_chart(0.02, "png")
```

### Portfolio Optimization

```python
from finalytics import Portfolio

portfolio = Portfolio(["AAPL", "GOOG", "MSFT", "BTC-USD"], "^GSPC", "2020-01-01", "2022-01-01", "1d", 0.95, 0.02, 1000, "max_sharpe")
print(portfolio.get_optimization_results())
portfolio.display_portfolio_charts("html")
```

