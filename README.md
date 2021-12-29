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
```

```
erdpy --verbose contract deploy --project=contract --recall-nonce --pem="../farmer.pem" --gas-limit=60000000 --arguments 0x45474c44 --outfile="deploy-testnet.masterchef.json" --send --proxy="https://testnet-api.elrond.com" --chain=T
```

Call contract

```
erdpy contract call erd1qqqqqqqqqqqqqpgqvcwzhdulmds7vrch8amq6x0sax0spvsveeyqnmzdkq --recall-nonce --pem="../farmer.pem" --gas-limit=10000000 --function="add" --arguments 80 0x45474c44 --proxy="https://testnet-gateway.elrond.com" --chain=T --send
```

```
erdpy contract call erd1qqqqqqqqqqqqqpgqvcwzhdulmds7vrch8amq6x0sax0spvsveeyqnmzdkq --recall-nonce --pem="../farmer.pem" --gas-limit=10000000 --function="add" --arguments 20 0x45474c44 --proxy="https://testnet-gateway.elrond.com" --chain=T --send
```

```
erdpy contract call erd1qqqqqqqqqqqqqpgqvcwzhdulmds7vrch8amq6x0sax0spvsveeyqnmzdkq --recall-nonce --pem="../farmer.pem" --gas-limit=10000000 --function="deposit" --arguments "u64:1" "0x03EA59C40153B0F90FB41B4F117745E44597BB2F2037EE98B745E506E6B2CE48" --value=5 --proxy="https://testnet-gateway.elrond.com" --chain=T --send
```

Read contract:

```
erdpy --verbose contract query erd1qqqqqqqqqqqqqpgqvcwzhdulmds7vrch8amq6x0sax0spvsveeyqnmzdkq --function="getReward" --proxy="https://testnet-gateway.elrond.com"
```

```
erdpy --verbose contract query erd1qqqqqqqqqqqqqpgqvcwzhdulmds7vrch8amq6x0sax0spvsveeyqnmzdkq --function="totalAllocPoint" --proxy="https://testnet-gateway.elrond.com"
```

```
erdpy --verbose contract query erd1qqqqqqqqqqqqqpgqvcwzhdulmds7vrch8amq6x0sax0spvsveeyqnmzdkq --function="getPoolInfo" --arguments 0 --proxy="https://testnet-gateway.elrond.com"
```

```
erdpy --verbose contract query erd1qqqqqqqqqqqqqpgqvcwzhdulmds7vrch8amq6x0sax0spvsveeyqnmzdkq --function="getPoolInfo" --arguments 1 --proxy="https://testnet-gateway.elrond.com"
```
