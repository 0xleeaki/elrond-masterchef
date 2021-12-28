# Interaction

## On testnet

Create wallet:

```
erdpy --verbose wallet derive farmer.pem --mnemonic --index 0
```

Get HEX address from BECH32 address:

```
erdpy wallet bech32 --decode erd1q049n3qp2wc0jra5rd83za69u3ze0we0yqm7ax9hghjsde4jeeyqc78p2s
```

Deploy & interact with contract:

```
erdpy --verbose contract build contract