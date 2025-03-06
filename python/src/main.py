import asyncio
from typing import Iterable
import aiohttp
from result import Result, Ok, Err


async def download(urls: Iterable[str]) -> list[str]:
    async with aiohttp.ClientSession() as session:

        async def download_one(u: str) -> Result[str, Exception | str]:
            try:
                async with session.get(u) as resp:
                    if resp.status != 200:
                        return Err(f"HTTP error: {resp.status}")
                    return Ok(await resp.text())
            except Exception as e:
                return Err(e)

        res = await asyncio.gather(*map(download_one, urls))

        filtered: list[str] = []
        for r in res:
            match r:
                case Err(e):
                    print(f"Error: {e}")
                    pass
                case Ok(text):
                    filtered.append(text)
        return filtered


async def main():
    my_webpage = "http://127.0.0.1:8000/index.html"
    count = 2000
    pages = await download(map(lambda _: my_webpage, range(count)))
    assert len(pages) == count, len(pages)
    for page in pages:
        assert len(page) == 25


if __name__ == "__main__":
    asyncio.run(main())
