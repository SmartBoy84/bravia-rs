# bravia-rs
Wrapper around the local REST API for the Sony Bravia TV series

# Example
```rust
    let tv = BraviaClient::new(PRK, "192.168.0.36");

    // for one-off/infrequent requests
    tv.set_vol(100).unwrap();

    // however, if calling many times
    let set_vol = (0..100usize)
        .map(|i| {
            make_payload(SetAudioVolume {
                volume: i.to_string(),
                target: "".to_string(),
            })
            .unwrap()
        })
        .collect::<Vec<_>>();

    // increase, decrease volume
    for v in set_vol.iter().chain(set_vol.iter().rev().skip(1)) {
        thread::sleep(Duration::from_millis(500));
        tv.write_cmd(v).unwrap();
    }
```
