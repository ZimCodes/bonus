"""Generate a JSON file with all available MIME types"""
import json

import requests
from bs4 import BeautifulSoup


def get_resources() -> dict:
    with open('res.json', 'r') as f1:
        res_map = json.load(f1)
        return res_map


def get_all_mimes(res_map) -> set:
    mimes = set()
    for url, css_select in res_map.items():
        print(f"Retrieving MIME types from {url}.....")
        mimes |= retrieve_mimes(url, css_select)
    return mimes


def retrieve_mimes(url, css_select) -> set:
    r = requests.get(url)
    soup = BeautifulSoup(r.text, 'html.parser')
    tags = soup.select(css_select)
    return {tag.string.strip() for tag in tags}


if __name__ == "__main__":
    """Grabs mime types from IANA database and friends"""
    all_mimes = get_all_mimes(get_resources())
    print("Completed Retrieval!\n")
    print("Exporting data to './mime.json' file path")
    with open('./mimes.json', 'w') as f:
        json.dump(sorted(all_mimes), f)

    print("Export complete! Shutting down....")
