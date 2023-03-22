# User flow

`dorian.near` logs-in https://alpha.near.org.
They see our application. 

## Fist Step
This is the first time `dorian.near` interacts with our application, so the application asks Dorian to deploy a `hello-world` contract somewhere.

`dorian.near` deploys the contract in an account, for example `somewhere.near`.

:::warning
what if `dorian.near` is a hackzor, and they simply input `hello.near-examples.near`. They will pass the exam! 

How can we know that actually it is `dorian.near` that deployed the contract they are putting in the input???
:::

Dorian comes back to our application and inputs `somewhere.near` in the field.

Our smart contract calls `somewhere.near` and checks that it implements the right interface.

If it does, it saves

`dorian.near` -> {`hello-near`: true}

## Second Step
Our application asks dorian to deploy a collections contract in some account.

Dorian deploys the collection contract in `collections.dorian.near` (but could be anywhere, e.g. `somewhere.near`, `collections.near`, etc)

Dorian inputs the contract into our application.

The application calls `collections.dorian.near`, and checks if the interface is correct.

If it is, the smart contract now saves:

`dorian.near` -> { `hello-near`: true, `collections`: true }

## Final Step
Since `dorian.near` passed both tests, then our contract can mint and transfer an NFT to `dorian.near`. This NFT proves that `dorian.near` has passed our tests, thus being a great developer.