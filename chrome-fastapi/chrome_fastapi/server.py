from typing import cast

import uvicorn
from fastapi import FastAPI, HTTPException
from pydantic import HttpUrl, BaseModel
from seleniumbase import Driver
from seleniumbase.undetected import Chrome
from selenium.common.exceptions import WebDriverException

app = FastAPI(
  title="Web Scraper API",
  description="Simple web scraping API using Selenium for content fetching",
  version="1.0.0",
)


class UrlRequest(BaseModel):
  url: HttpUrl

  class Config:
    json_schema_extra = {"example": {"url": "https://example.com"}}


class HtmlResponse(BaseModel):
  html: str
  url: HttpUrl


@app.post(
  "/fetch-html", response_model=HtmlResponse, summary="Fetch HTML content from a URL", response_description="Returns the HTML content from the requested URL"
)
def fetch_html(request: UrlRequest) -> HtmlResponse:
  driver = cast(Chrome, Driver(headless=True, uc=True))
  try:
    for _ in range(3):
      try:
        driver.get(str(request.url))
      except Exception:
        driver.quit()
        driver = cast(Chrome, Driver(headless=True, uc=True))
      else:
        break
    return HtmlResponse(html=driver.page_source, url=request.url)
  except WebDriverException as e:
    raise HTTPException(status_code=400, detail=str(e))
  except Exception as e:
    raise HTTPException(status_code=500, detail=str(e))
  finally:
    driver.quit()


@app.get("/health")
async def health_check() -> dict[str, str]:
  return {"status": "healthy"}


if __name__ == "__main__":
  uvicorn.run(app, host="0.0.0.0", port=8231)
