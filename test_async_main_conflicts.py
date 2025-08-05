# Test case 2: Async main function with __name__ == "__main__" block
import asyncio

async def main():
    print("Async main function")
    await asyncio.sleep(0.1)
    return "async_result"

def sync_helper():
    print("Sync helper function")

if __name__ == "__main__":
    result = asyncio.run(main())  # Call to user's async main
    sync_helper()
    print(f"Async result: {result}")