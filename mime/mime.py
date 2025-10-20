"""Generate a JSON file with all available MIME types"""
import urllib.request
from urllib.error import HTTPError, URLError
import json
from bs4 import BeautifulSoup

def get_resources() -> dict[str,str]:
    with open('res.json', 'r') as f1:
        res_map = json.load(f1)
        return res_map

def get_all_mimes(res_map: dict[str, str]) -> set[str]:
    mimes = set()
    for url, css_select in res_map.items():
        if url != "extras":
            print(f"Retrieving MIME types from {url}.....")
            mimes |= retrieve_mimes(url, css_select)
        else:
            print(f"Retrieving extra MIME types.......")
            mimes |= set(css_select)
    return mimes


def retrieve_mimes(url: str, css_select: str) -> set[str]:
    headers = {
        "User-Agent": "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/134.0.0.0 Safari/537.36",
        "Accept": "text/html, application/xhtml+xml;q=0.8",
        "Accept-Language": "en, *;q=0.6"
    }
    req = urllib.request.Request(url, headers=headers)
    web_text = None
    try:
        with urllib.request.urlopen(req) as res:
            encoding = res.headers.get_content_charset() or "utf-8"
            web_text = res.read().decode(encoding,'replace')
    except (HTTPError, URLError) as e:
        print(f"!~ FAILED retrieving mimetypes from {url} ~!\nReason: {e.reason}")
        return set()
    soup = BeautifulSoup(web_text, 'html.parser')
    tags = soup.select(css_select)
    return {tag.string.strip() for tag in tags}


if __name__ == "__main__":
    """Grabs mime types from IANA database and friends"""
    all_mimes = get_all_mimes(get_resources())
    total = len(all_mimes)
    print("Completed Retrieval!\n")
    print("Exporting data to './mime.json' file path")
    with open('./mimes.json', 'w') as f:
        json.dump(sorted(all_mimes), f)

    print(f"+++| Total: {total} |+++")
    print("Export complete! Shutting down....")
