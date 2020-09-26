# Yasashii Unified Number Authority - OIDs

## Linting

```
cargo run -- lint ./registry.toml
```

## Decoding

```
% target/debug/yuna-oid decode ./registry.toml 1.3.6.1.4.1.28900.0
iso(1) identified-organization(3) dod(6) internet(1) private(4) enterprise(1) yasashii-syndicate(28900) snmp(0)
% target/debug/yuna-oid decode ./registry.toml 1.3.6.1.4.1.28900.1.2.3
iso(1) identified-organization(3) dod(6) internet(1) private(4) enterprise(1) yasashii-syndicate(28900) x509(1) hash-algorithms(2) keccak-512(3)
```

## Other

Please add OIDs in order, as if they are composed of sorted vectors of numbers in lexicographical order.
