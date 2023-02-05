# ShhtarkNet

Secrets for Starknet contracts. Create and manage secrets for StarkNet smart contracts.

## Contents

- [ShhtarkNet](#shhtarknet)
	- [Contents](#contents)
	- [Under the hood](#under-the-hood)
		- [Secret structure](#secret-structure)
				- [contract](#contract)
				- [id](#id)
				- [hash](#hash)
		- [Creating a secret](#creating-a-secret)
		- [Consuming a secret](#consuming-a-secret)
		- [Job structure](#job-structure)
				- [candidate\_len `felt`](#candidate_len-felt)
				- [candidate `felt*`](#candidate-felt)
				- [callback\_contract `felt`](#callback_contract-felt)
				- [callback\_entrypoint `felt`](#callback_entrypoint-felt)
				- [callback\_param `felt`](#callback_param-felt)
		- [Processing the job](#processing-the-job)
	- [Usage](#usage)
			- [🛠️ *TODO*](#️-todo)
	- [Contribute](#contribute)
		- [Is their a better way?](#is-their-a-better-way)

## Under the hood

### Secret structure

A secret is represented by these attributes,

##### contract
Secrets are bound to a contract address, contracts can only consume secret matching the contract address.

##### id
The identifier for the secret, contracts use the ID to refer to the secret.

##### hash
The value of the secret is never saved but only it's hash

### Creating a secret

* You create a secret with the UI.
* Secret hash is generated and secret is saved to the DB.
* 

### Consuming a secret

* Cairo contracts create a queue of jobs
* Endpoint `shhtarknet_jobs` returns an array of jobs
* Rust probes the contract at regular intervals to find jobs

### Job structure

A job represents a request to consume the secret, with a bunch of passwords to compare against.

**:warning: Notice:** multiple candidates to make it harder to identify the value that matched. [Is their a better way?](is-their-a-better-way-)

A job is defined by,

##### candidate_len `felt`
Number of candidates to compare

##### candidate `felt*`
Pointer to the first candidate

##### callback_contract `felt`
Contract to invoke when request is processed

##### callback_entrypoint `felt`
Contract entrypoint to call when request is processed

##### callback_param `felt`
An optional custom parameter to identify what needs to
be done if the job succeeds.

🟢 Invoke will not include matched value, so use this param to indentify what needs to be done (or what/who for).

### Processing the job

Executing the job involes,

1. Looping all candidates and matching against the saved hash.
2. Invoke `callback_entrypoint` in `callback_entrypoint` with `callback_param` and,
   - `TRUE` If one of the values matched the secret or
   - `FALSE` If none of the values matched the secret
3. Job report is created and available via the API.

## Usage

#### 🛠️ *TODO*

## Contribute

Please have a look around at the issues, comment on the issue if you find something you'd be interested in working on!

#### Something not right?

Found a bug? Something doesn't seem right? Please create an issue. Thanks in advance! :bow:

#### Is their a better way?

There is always a better way to do something. We would love to hear from you! Please create an issue on the repo.
