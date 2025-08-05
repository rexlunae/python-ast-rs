import asyncio

async def fetch_data():
    await asyncio.sleep(1)
    return "data"

def process_data(data):
    return f"Processed: {data}"

async def main():
    data = await fetch_data()
    result = process_data(data)
    print(result)

if __name__ == "__main__":
    asyncio.run(main())

async def another_async():
    return "another"

if __name__ == "__main__":
    print("Second main block")