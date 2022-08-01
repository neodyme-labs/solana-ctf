package challenges

challenges: "legit-bank": {
	enabled:        true
	displayName:    "Legit Bank"
	category:       "Zoomer Crypto"
	difficulty:     "Easy"
	author:         "CherryWorm, bennofs"
	broker:         "legit-bank",
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
			sha256sum: "7d94a2ec1271af5dcdf36e45980908c78a3edea5a777afd335a1709bf4b91890"
		},
	]
}
