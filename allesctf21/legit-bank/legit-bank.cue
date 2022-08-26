package challenges

challenges: "legit-bank": {
	enabled:        true
	displayName:    "Legit Bank"
	category:       "Zoomer Crypto"
	difficulty:     "Easy"
	author:         "CherryWorm, bennofs"
	broker:         "legit-bank"
	brokerProtocol: "solana-explorer"

	description: """
	You came across this very legit looking bank on the Solana blockchain. They swear they won't pull the rug! But can you pull it?

	The contract is deployed at: `Bank111111111111111111111111111111111111111`

	This challenge has the same setup as all Solana Smart Contract challenges: a validator running in a docker container that you have to interact with via RPC.
		
	We recommend using the [Solana PoC Framework](https://github.com/neodyme-labs/solana-poc-framework) which facilitates fast exploit development. Alternatively you can also use the official
	[rust api](https://docs.rs/solana-client/1.7.10/solana_client/rpc_client/struct.RpcClient.html), the official [js api](https://solana-labs.github.io/solana-web3.js/)
	or any other way you can think of interacting with the RPC server. Solana also has a multitude of [cli tools](https://docs.solana.com/cli/install-solana-cli-tools). Please note
	however that due to setup limitations, the TPU port of the validator is not exposed, which means the `solana program deploy` command will not work. The Solana PoC Framework has a 
	[function](https://docs.rs/poc-framework/0.1.0/poc_framework/trait.Environment.html#method.deploy_program) for this that only uses the rpc endpoint and will work.
	
	The [solana explorer](https://explorer.solana.com/) works with any cluster your browser can reach. Just click on the `Mainnet Beta` button and enter the url of the RPC 
	endpoint into the `Custom` text field. Checking the `Enable custom url param` checkbox might also be useful for collaboration. The explorer allows you to inspect accounts 
	and transactions and has a bunch of useful features.

	The goal of these challenges is to obtain a flag-token (mint `F1agMint11111111111111111111111111111111111`). After you got one, you have to call the flag contract 
	`F1ag111111111111111111111111111111111111111`. The instruction data is ignored, the first account has to be a spl-token account that contains a flag token and the second
	account has to be the owner of the token account. The second account needs to sign the transaction, to proof that you really got the flag.

	Alternatively, use the provided CLI after building with `cargo build`:

	`bank-cli -u http://localhost:1024 -k keys/rich-boi.json get-flag YOUR_FLAG_TOKEN_ACCOUNT -a keys/rich-boi.json`

	Note that the docker image uses old versions that might not be compatible with what you're running.
	If you're having issues, try rust version 1.59 and the following dependencies:

	```
	[dependencies]
	poc-framework = "=0.1.6"
	spl-token = "=3.2.0"
	```

	```
	rustup install 1.59
	rustup run 1.59 cargo build
	```


	A good starting point is the Solana documentation:
	- [https://docs.solana.com/developing/programming-model/overview](https://docs.solana.com/developing/programming-model/overview)
	- [https://spl.solana.com/token#operational-overview](https://docs.solana.com/developing/programming-model/overview)
	- [https://docs.solana.com/developing/clients/jsonrpc-api](https://docs.solana.com/developing/clients/jsonrpc-api)
	"""

	flag:   "ALLES!{Some Smart Contracts are not very smart :(}"
	points: 500
	files: [
		{
			name:      "legit-bank.zip"
			sha256sum: "d041723181d68d9d8ea2acb9471b28e6362b4a0f0660e66f6494c2b71f965fb1"
		},
	]
}
