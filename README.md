# ShhtarkNet

Secrets for Starknet contracts. Create and manage secrets for StarkNet smart contracts.

## Contents

* [Under the hood](#under-the-hood)
  * [What is a secret](#secret-structure)
  * [Creating a secret](#creating-a-secret)
  * [What is a job](#what-is-a-job)
  * [What is a secret](#)
  * [What is a secret](#)

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

### Using a secret in a smart contract

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

#### Is their a better way?

Is there a better way to do something? We would love to hear from you! Please create an issue on the repo.
