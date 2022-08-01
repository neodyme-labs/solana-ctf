# Workshop
To make this workshop hands-on, you will find bugs and develop exploits for them yourself.

We have prepared a few vulnerable Solana contracts.
As this workshop was intended for only 3h, we have simplified bugs we have found in the wild a lot.
This allows you to focus on finding and exploiting bugs over reverse-engineering functionality.

In the same vein, we have opted not to use the Anchor framework in our contracts, even though it usually leads to more secure contracts.
This is simply to save the extra time you'd need to learn anchor if you are not familiar yet.
The security fundamentals you learn here will apply just as well in anchor, they'll be just a bit easier to implement cleanly there.

In anchor, lots of checks are hidden away, and we often have to go diggin in anchor source to understand what exactly is being checked and what can be controlled. (have not found a bug inside anchor itself though... yet)

Each task can be solved without looking at the description here. But we have prepared some hints to help you.
