from fastapi import FastAPI
from pydantic import BaseModel

app = FastAPI()


class PostRequest(BaseModel):
    key: str
    value: str


class PostResponse(BaseModel):
    result: str


class GetResponse(BaseModel):
    key: str
    value: str


@app.get("/get/{key}", response_model=GetResponse)
def get_endpoint(key: str):
    # For simplicity, we are returning a static value for any key
    return GetResponse(key=key, value="Hello, World!")


@app.post("/post", response_model=PostResponse)
def post_endpoint(request: PostRequest):
    # For simplicity, we are returning a success message without storing the key-value pair
    return PostResponse(result=f"Received key: {request.key}, value: {request.value}")
