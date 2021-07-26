- How to determine the port number that AA listens to as a grpc service

In the container image decryption operation of kata cc V0 architecture, ocicrypt-rs parses the configuration file`/etc/containerd/ocicrypt/ocicrypt_keyprovider.conf` to obtain the port number of the keyprovider, for example:

```
{
        "key-providers": {
                "attestation-agent": {
                				"grpc": "127.0.0.1:$port"
                }
        }
}
```

At present, the implementation of AA adopts the scheme of listening to the fixed port number 44444

- gRPC or ttRPC

At present, AA uses gRPC protocol to implement the server of keyprovider for ocicrypt-rs. Compared with gRPC, ttRPC has the advantage of lighter weight. AA, as a memory resident service on the client side of kata cc V0 architecture, using lightweight ttRPC will save more resources.

