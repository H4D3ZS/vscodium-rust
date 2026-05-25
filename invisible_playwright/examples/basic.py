"""Launch a patched Firefox with a random stealth profile and load example.com."""
from invisible_playwright import InvisiblePlaywright


def main() -> None:
    with InvisiblePlaywright() as browser:
        page = browser.new_page()
        page.goto("https://example.com")
        print("title:", page.title())


if __name__ == "__main__":
    main()
