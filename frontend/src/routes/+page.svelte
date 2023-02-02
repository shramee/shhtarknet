<h1>Welcome to Shh-tarknet</h1>
<article>
	<header>
		<h3 style="margin: 0">Create a secret</h3>
	</header>
	<form>
		<label for="contract_addr">Contract address</label>
		<input
			type="text"
			id="contract_addr"
			name="contract_addr"
			pattern="0x[0-9A-Za-z]{64}"
			title="Your StarkNet contract address 0x followed by 64 char code"
			placeholder="Contract address"
			required
		/>

		<label for="secret">Secret</label>
		<input
			type="text"
			id="secret"
			name="secret"
			placeholder="Secret, comma separated for multiple values"
			required
		/>

		<!-- Button -->
		<button type="submit">Submit</button>
	</form>
</article>
