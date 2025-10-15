import requests
import random
import json
import time
import unicodedata
from bs4 import BeautifulSoup
from urllib.parse import quote, unquote

HEADERS = {"User-Agent": "FeaturedFetcher/1.0 (your_email@example.com)"}


def get_featured_titles():
    url = "https://en.wikipedia.org/wiki/Wikipedia:Featured_articles"
    r = requests.get(url, headers=HEADERS)
    r.raise_for_status()
    soup = BeautifulSoup(r.text, "html.parser")

    titles = []
    rejected = []

    for a in soup.select("div.mw-parser-output a[href^='/wiki/']"):
        raw = a["href"].split("/wiki/")[-1]
        decoded = unquote(raw)

        if (
            any(decoded.startswith(prefix) for prefix in ["Wikipedia:", "Portal:", "Template:", "Help:", "Category:", "File:"])
            or ":" in decoded
        ):
            rejected.append((decoded, "namespace"))
            continue

        if not all(ord(c) < 128 for c in decoded):
            rejected.append((decoded, "non-ascii"))
            continue

        titles.append(decoded)

    titles = list(set(titles))

    print(f"Found {len(titles)} Featured Articles (after ASCII filter).")
    print(f"Rejected {len(rejected)} titles.\n")

    # summary of rejected titles
    if rejected:
        print("📛 Odrzucone tytuły:")
        for t, reason in rejected[:40]:
            print(f" - {t} ({reason})")
        if len(rejected) > 40:
            print(f"... i {len(rejected) - 40} więcej.\n")

    return titles



def clean_title_for_display(title: str) -> str:
    """Zamienia podkreślniki/myślniki na czytelne postaci do zapisu (display)."""
    replacements = {
        "−": "-",  # minus
        "–": "-",  # en dash
        "—": "-",  # em dash
        "_": " ",
    }
    for bad, good in replacements.items():
        title = title.replace(bad, good)
    return title.strip()


def clean_title_for_query(title: str) -> str:
    decoded = unquote(title)
    fixed = decoded.replace("–", "-").replace("−", "-").replace("—", "-")
    fixed = fixed.replace("_", " ")
    fixed = fixed.strip()
    return fixed


def normalize_text(text):
    normalized = unicodedata.normalize("NFD", text)
    cleaned = "".join(c for c in normalized if unicodedata.category(c) != "Mn")
    non_ascii = sum(1 for c in cleaned if ord(c) > 127)
    if non_ascii > 5:
        return None
    return cleaned


def fetch_summary(title):
    display_title = clean_title_for_display(unquote(title))
    query_title = clean_title_for_query(title)
    encoded = quote(query_title, safe="")

    # REST API
    url_rest = f"https://en.wikipedia.org/api/rest_v1/page/summary/{encoded}"
    try:
        r = requests.get(url_rest, headers=HEADERS, timeout=10)
    except Exception as e:
        print(f"❌ Network error for {display_title}: {e}")
        r = None

    if r and r.ok:
        try:
            data = r.json()
            return data.get("extract"), display_title
        except Exception:
            pass
    else:
        if r is not None:
            print(f"⚠️  REST failed for {display_title} ({r.status_code})")

    # fallback: classic action API (exintro + plaintext)
    url_fallback = (
        "https://en.wikipedia.org/w/api.php?action=query&prop=extracts"
        "&exintro=true&explaintext=true&format=json"
        f"&titles={quote(query_title, safe='')}"
    )
    try:
        r2 = requests.get(url_fallback, headers=HEADERS, timeout=10)
    except Exception as e:
        print(f"❌ Fallback network error for {display_title}: {e}")
        return None, display_title

    if not r2.ok:
        print(f"⚠️  Fallback failed for {display_title} ({r2.status_code})")
        return None, display_title

    data2 = r2.json()
    pages = data2.get("query", {}).get("pages", {})
    if not pages:
        return None, display_title

    page = next(iter(pages.values()))
    return page.get("extract"), display_title


def main():
    titles = get_featured_titles()
    random.shuffle(titles)
    sample = titles[:50]

    results = []

    for i, title in enumerate(sample, 1):
        print(f"[{i}/{len(sample)}] {title}")
        summary, display_title = fetch_summary(title)
        if not summary:
            # skipped
            time.sleep(0.25)
            continue

        normalized = normalize_text(summary)
        if not normalized:
            time.sleep(0.25)
            continue

        word_count = len(normalized.split())
        if 30 <= word_count <= 120:
            results.append({
                "title": display_title,
                "summary": normalized,
                "wordCount": word_count
            })

        time.sleep(0.25)  # pause

    with open("assets/featured_summaries.json", "w", encoding="utf-8") as f:
        json.dump(results, f, indent=2, ensure_ascii=False)

    print(f"\nSaved {len(results)} records to assets/featured_summaries.json")


if __name__ == "__main__":
    main()
