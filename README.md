# ISO 3166 Proto

This is a generator and upstream host of protobuf definitions for [ISO 3166]
country codes. The codes are generated from [IBAN].

## Generating codes

There is no automatic generation of codes but the crate in this repository can be used to manually generate new codes whenever needed.

```sh
cargo run > iso-3166.proto
```

## Using the proto

Once you've downloaded the proto in whatever way you want just use it like any other proto to include in your message.

```proto
syntax = 'proto3';

package my.package;

import 'iso3166.v1';

message User {
    string            name    = 1;
    iso3166.v1.Alpha2 country = 2;
}
```

  [IBAN]: https://www.iban.com/country-codes
  [ISO 3166]: https://www.iso.org/iso-3166-country-codes.html
