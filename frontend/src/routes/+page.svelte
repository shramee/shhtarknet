<script>
	export let secrets_promise = getSecrets();

	let contract = "";
	let id = "";
	let secret = "";
	let postSecretResponse = "";
	let secretAddedMsg = "";

	async function getSecrets() {
		const res = await fetch(`http://localhost:8080/secret`);
		const resp = await res.json();

		if (res.ok) {
			return resp;
		} else {
			throw new Error("Could not connect...");
		}
	}

	async function save_secret( e ) {
		e.preventDefault();
		postSecretResponse = await fetch('http://localhost:8080/secret', {
			headers: {
				'Accept': 'application/json',
				'Content-Type': 'application/json'
			},
			method: "POST",
			body: JSON.stringify({ contract,id,secret })
		}).then( resp => resp.json() );
		secretAddedMsg = `Secret ${id} added.`
		secrets_promise = await getSecrets();
	}
</script>

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
			
			title="Your StarkNet contract address 0x followed by 64 hexadecimals"
			placeholder="Contract address"
			required
			bind:value={contract}
		/>

		<label for="secret_id">Secret ID</label>
		<input
			type="text"
			id="secret_id"
			name="secret_id"
			pattern="[0-zA-Z]+"
			title="Alphanumeric ID for your secret"
			placeholder="Contract address"
			required
			bind:value={id}
		/>

		<label for="secret">Secret</label>
		<input
			type="text"
			id="secret"
			name="secret"
			placeholder="Secret, comma separated for multiple values"
			required
			bind:value={secret}
		/>



		<!-- Button -->
		<button type="submit" on:click={save_secret}>Submit</button>
	</form>
</article>
<article>
	<header>
		<h3 style="margin: 0">Your secrets</h3>
	</header>
	<ul>
		<table>
			<thead>
			  <tr>
				<th scope="col">#</th>
				<th scope="col">Contract address</th>
				<th scope="col">Secret ID</th>
			  </tr>
			</thead>
			<tbody>
				{#await secrets_promise}
				<tr>
					<td colspan="3">Loading...</td>
				</tr>
			{:then secrets}
				{#each secrets as [contract, secret_id], i}
					<tr>
						<th scope="row">{i + 1}</th>
						<td>{contract}</td>
						<td>{secret_id}</td>
					</tr>
				{/each}
		  {:catch error}
				<p style="color: red">{error.message}</p>
			{/await}
			</tbody>
		  </table>
	</ul>

</article>
