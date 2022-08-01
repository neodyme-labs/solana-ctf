package challenges

challenges: "bugchain": {
	enabled:        true
	displayName:    "🔥 Bugchain"
	category:       "Zoomer Crypto"
	difficulty:     "Hard"
	author:         "CherryWorm"
	broker:         "bugchain"
	brokerProtocol: "solana-explorer"

	description: """
	You've already dipped your toes into the water and exploited a couple of Solana smart contracts. 
	Now it's time to fully jump in and exploit Solana itself! This challenge is based on a real vulnerability we
	found and reported last year.
	Your goal again is to call the flag contract with proof that you have flag tokens. But there is a catch this 
	time: we didn't mint any flag tokens!

	If you enjoy this challenge, make sure to check out the [Solana Bug Bounty program](https://github.com/solana-labs/solana/security/policy).
	The bounties are very generous, paid out every month and the folks over at Solana are great to work with.

	**Hint**:
	- [https://docs.solana.com/implemented-proposals/durable-tx-nonces](https://docs.solana.com/implemented-proposals/durable-tx-nonces)
	- [https://docs.solana.com/offline-signing/durable-nonce](https://docs.solana.com/offline-signing/durable-nonce)
	"""

	flag:   "ALLES!{https://twitter.com/aeyakovenko/status/1426951281331998720}"
	points: 500
	files: [
		{
			name:      "bugchain.zip"
			sha256sum: "9c74fdf81541438f0209dccc5ab0f301a81e0503f52f2a4856ae0e3955bc5297"
		},
	]
}
