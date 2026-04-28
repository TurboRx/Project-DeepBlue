import asyncio

async def test():
    import relay
    r = relay.Relay()
    # Test handling the message correctly handles the state updates
    r.handle_message("|turn|2")

if __name__ == "__main__":
    asyncio.run(test())
