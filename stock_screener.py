import requests
import json

class StockScreener:
    """
    Поиск акций по параметрам: EPS, P/E, прибыль, объем, дивиденды.
    Поддержка фильтров по рынкам и секторам. API — demo (можно подключить реальные).
    """
    def __init__(self):
        self.stocks = []
        self.filters = {}

    def load_stocks(self, api_url="https://mock-stock-api.com/api/stocks"):
        response = requests.get(api_url)
        self.stocks = response.json()
        print(f"Загружено {len(self.stocks)} акций.")

    def filter_by_pe(self, min_pe=None, max_pe=None):
        filtered = []
        for s in self.stocks:
            pe = s.get("pe")
            if pe is None:
                continue
            if min_pe and pe < min_pe:
                continue
            if max_pe and pe > max_pe:
                continue
            filtered.append(s)
        print(f"По P/E фильтровано: {len(filtered)}")
        return filtered

    def filter_sector(self, sector):
        filtered = [s for s in self.stocks if s.get("sector") == sector]
        print(f"Сектор: {sector}. Найдено: {len(filtered)}")
        return filtered

    def top_dividend_stocks(self, min_yield=0.05):
        filtered = [s for s in self.stocks if s.get("div_yield", 0) >= min_yield]
        print(f"Высокие дивиденды фильтр: {len(filtered)}")
        return filtered

    def export_filtered(self, stocks, filename):
        with open(filename, "w", encoding="utf-8") as f:
            json.dump(stocks, f, ensure_ascii=False, indent=3)
        print(f"Результаты сохранены в {filename}")

if __name__ == "__main__":
    screener = StockScreener()
    screener.load_stocks()
    pe_stocks = screener.filter_by_pe(min_pe=5, max_pe=15)
    screener.export_filtered(pe_stocks, "pe_stocks.json")
