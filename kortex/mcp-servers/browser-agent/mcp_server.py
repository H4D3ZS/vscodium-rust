import asyncio
import base64
from typing import Optional

from mcp.server.fastmcp import FastMCP
from invisible_playwright.async_api import InvisiblePlaywright, Page

# Create the FastMCP server
mcp = FastMCP("Antigravity Browser Agent")

# Global state for the browser
browser_instance = None
current_page: Optional[Page] = None
playwright_context = None


async def get_or_create_page() -> Page:
    global browser_instance, current_page, playwright_context
    if current_page is not None and not current_page.is_closed():
        return current_page

    if playwright_context is None:
        playwright_context = InvisiblePlaywright()
        await playwright_context.__aenter__()

    if browser_instance is None:
        browser_instance = await playwright_context.new_page()
    
    current_page = browser_instance
    return current_page


@mcp.tool()
async def browser_goto(url: str) -> str:
    """Navigate the invisible browser to a specific URL."""
    try:
        page = await get_or_create_page()
        await page.goto(url)
        # Wait a bit for dynamic content
        await page.wait_for_timeout(2000)
        title = await page.title()
        return f"Successfully navigated to {url}. Page title: {title}"
    except Exception as e:
        return f"Failed to navigate to {url}: {str(e)}"


@mcp.tool()
async def browser_click(selector: str) -> str:
    """Click an element on the current page using a CSS selector."""
    try:
        page = await get_or_create_page()
        await page.click(selector)
        await page.wait_for_timeout(1000)
        return f"Successfully clicked element: {selector}"
    except Exception as e:
        return f"Failed to click element {selector}: {str(e)}"


@mcp.tool()
async def browser_screenshot_base64() -> str:
    """Take a screenshot of the current page and return it as a base64 encoded string."""
    try:
        page = await get_or_create_page()
        # Take full page screenshot
        screenshot_bytes = await page.screenshot(full_page=True)
        b64 = base64.b64encode(screenshot_bytes).decode('utf-8')
        return f"data:image/png;base64,{b64}"
    except Exception as e:
        return f"Failed to take screenshot: {str(e)}"


@mcp.tool()
async def browser_evaluate(js_code: str) -> str:
    """Evaluate JavaScript code on the current page and return the result."""
    try:
        page = await get_or_create_page()
        result = await page.evaluate(js_code)
        return f"JavaScript evaluation result: {result}"
    except Exception as e:
        return f"Failed to evaluate JavaScript: {str(e)}"


if __name__ == "__main__":
    # Start the standard input/output MCP server
    mcp.run()
