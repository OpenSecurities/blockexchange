# blockexchange
A blockchain-esque stock exchange.

## Security
Authentication is achieved via digital signatures and a CA infrastructure similar to the world wide web. When a new chain is initialized a CA certificate will be generated. This certificate will be used to sign any other client certificates. Those client certificates will be used to digitally sign any transactions. A transaction will need to be signed by both selling and buying parties and be verified by the rest of the network.
