# Tool

Decode Tool

```
https://www.convertstring.com/EncodeDecode/HexEncode
```

```
https://www.base64decode.org/
```

Create Wallet

```
erdpy --verbose wallet derive farmer.pem --mnemonic --index 0
```

Get HEX From BECH32

```
erdpy wallet bech32 --decode erd1q049n3qp2wc0jra5rd83za69u3ze0we0yqm7ax9hghjsde4jeeyqc78p2s
```

# Contract

```
Reward Fund: erd1qqqqqqqqqqqqqpgqurfayszn59q9yrxfnj0c63qqxef69g4jeeyq3acy5a
```

```
Masterchef: erd1qqqqqqqqqqqqqpgq0uc2a0wljaf3khvnedn66evq8zt0w8wweeyqr8fxyf
```

```
ICE: 0x4943452D616532633563 | ICE-ae2c5c
```

```
IRON: 0x49524F4E2D663539333630 | IRON-f59360
```

```
USDC: 0x555344432D313632626531 | USDC-162be1
```

# Interaction

## Build Contract

```
erdpy --verbose contract build contracts/masterchef
```

```
erdpy --verbose contract build contracts/fund
```

## Run Test:

```
erdpy contracts/masterchef test
```

```
erdpy contracts/fund test
```

## Deploy contract

```
erdpy --verbose contract deploy --project=contracts/fund --recall-nonce --pem="../farmer.pem" --gas-limit=60000000 --arguments 0x4943452D616532633563 --outfile="deploy-testnet.fund.json" --send --proxy="https://testnet-api.elrond.com" --chain=T
```

```
erdpy --verbose contract deploy --project=contracts/masterchef --recall-nonce --pem="../farmer.pem" --gas-limit=60000000 --arguments 0x00000000000000000500e0d3d24053a140520cc99c9f8d44003653a2a2b2ce48 --outfile="deploy-testnet.masterchef.json" --send --proxy="https://testnet-api.elrond.com" --chain=T
```

## Call Contract

### Reward Fund

```

```

### Masterchef

```
erdpy contract call erd1qqqqqqqqqqqqqpgq0uc2a0wljaf3khvnedn66evq8zt0w8wweeyqr8fxyf --recall-nonce --pem="../farmer.pem" --gas-limit=10000000 --function="add" --arguments 800000 0x49524F4E2D663539333630 --proxy="https://testnet-gateway.elrond.com" --chain=T --send
```

```
erdpy contract call erd1qqqqqqqqqqqqqpgq0uc2a0wljaf3khvnedn66evq8zt0w8wweeyqr8fxyf --recall-nonce --pem="../farmer.pem" --gas-limit=10000000 --function="add" --arguments 200000 0x555344432D313632626531 --proxy="https://testnet-gateway.elrond.com" --chain=T --send
```

```
erdpy contract call erd1qqqqqqqqqqqqqpgq0uc2a0wljaf3khvnedn66evq8zt0w8wweeyqr8fxyf --recall-nonce --pem="../farmer.pem" --gas-limit=10000000 --function="deposit" --arguments 8 "0x03EA59C40153B0F90FB41B4F117745E44597BB2F2037EE98B745E506E6B2CE48" --value=5 --proxy="https://testnet-gateway.elrond.com" --chain=T --send
```

## Query Contract

### Reward Fund

```
erdpy --verbose contract query erd1qqqqqqqqqqqqqpgqurfayszn59q9yrxfnj0c63qqxef69g4jeeyq3acy5a --function="getReward" --proxy="https://testnet-gateway.elrond.com"
```

### Masterchef

```
erdpy --verbose contract query erd1qqqqqqqqqqqqqpgq0uc2a0wljaf3khvnedn66evq8zt0w8wweeyqr8fxyf --function="getFund" --proxy="https://testnet-gateway.elrond.com"
```

```
erdpy --verbose contract query erd1qqqqqqqqqqqqqpgq0uc2a0wljaf3khvnedn66evq8zt0w8wweeyqr8fxyf --function="getTotalAllocPoint" --proxy="https://testnet-gateway.elrond.com"
```

```
erdpy --verbose contract query erd1qqqqqqqqqqqqqpgq0uc2a0wljaf3khvnedn66evq8zt0w8wweeyqr8fxyf --function="getPoolInfo" --arguments 1 --proxy="https://testnet-gateway.elrond.com"
```
