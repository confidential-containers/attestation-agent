 This guide shows how to Use skopeo and containerd to verify the basic functions of attestation agent.

## Dependence

- [skopeo](https://github.com/containers/skopeo)
  `skopeo` is a command line utility that performs various operations on container images and image repositories.

- [containerd v1.5.3](https://github.com/containerd/containerd/tree/v1.5.3)

  containerd is an industry-standard container runtime with an emphasis on simplicity, robustness and portability. It also provides the following tools.

  - `crictl`: `crictl` provides a CLI for CRI-compatible container runtimes. This allows the CRI runtime developers to debug their runtime without needing to set up Kubernetes components.
  - `ctd-decoder`: `ctd-decoder` is use by containerd to decrypt encrypted container images.

## Instructions

The following steps describe how to use skopeo with sample KBS to encrypt your container image. When deploying a container image using crictl, contact AA through containerd to decrypt your container image, and finally complete the deployment.

clone attestation-agent repository to your local repository：

```
git clone https://github.com/containers/attestation-agent
cd attestation-agent
```

### Encrypt the image

Build and run the sample KBS program from the source code as follows.

```
cd sample/sample_kbs
cargo run --bin sample_kbs
```

It may take a long time to compile and run for the first time. After running and listening to the port successfully, please open `/etc/containerd/ocicrypt/ocicrypt_keyprovider.conf`，and write the following into it:

```
{
        "key-providers": {
                "sample_kbs": {
                				"grpc": "127.0.0.1:46666"
                }
        }
}
```

After completing the above configuration, you can use skopeo to encrypt your image locally. You need to prepare a sample image for testing. You can use the following command to pull it to the local repository. The image should be oci complaint.

```
skopeo --insecure-policy copy docker://docker.io/library/alpine:latest oci:alpine
```

Encrypt the image as follows.

```
OCICRYPT_KEYPROVIDER_CONFIG=/etc/containerd/ocicrypt/ocicrypt_keyprovider.conf skopeo \
  copy --insecure-policy --encryption-key provider:sample_kbs:test \
  oci:alpine oci:alpine-encrypted
```

Push encryption images to docker hub.

```
# login to docker hub
skopeo login docker.io --username <$username> --password <$password>

# push the images to docker hub
skopeo copy --insecure-policy --dest-tls-verify=false \
  oci:alpine-encrypted docker://<$username>/alpine-encrypted:latest
```

### Deploy and decrypt the image

After completing the encryption and upload of the container image according to the above steps, you can exit the sample KBS program and build and run the attestation agent program from the source code:

```
cd attestation-agent
cargo run --bin attestation_agent
```

It may take a long time to compile and run for the first time. After running and listening to the port successfully, please open `/etc/containerd/ocicrypt/ocicrypt_keyprovider.conf`，and write the following into it:

```
{
        "key-providers": {
                "attestation-agent": {
                				"grpc": "127.0.0.1:44444"
                }
        }
}
```

After completing the above configuration, you can deploy and decrypt the encrypted container image uploaded to the docker hub locally

Firstly, start containerd

```
containerd &
```

Generate pod configuration file

```
cat << EOF >pod.yaml
metadata:
  attempt: 1
  name: my-podsandbox
  namespace: default
  uid: hdishd83djaidwnduwk28bcsb
log_directory: /tmp/eaa_test
linux:
  namespaces:
    options: {}
EOF
```

Generate container configuration file

```
cat << EOF >container.yaml
metadata:
  name: alpine.enc
image:
  image: <$TARGET_IMAGE>
command:
- top
log_path: busybox.0.log
EOF
```

The variable `<$TARGET_IMAGE>` specifies the image address. You can use image `<$username>/alpine-encrypted:latest`.

Run pod and container

```
crictl run container.yaml pod.yaml
```

Test the pod

```
crictl ps
CONTAINER           IMAGE                                            CREATED             STATE               NAME                ATTEMPT             POD ID
2d0414e875912       <$user_name>/alpine-encrypted:latest           4 seconds ago       Running             alpine.enc          0                   cc584e4876a66

crictl exec 2d0414e875912 echo "Hello, world!!"
Hello, world
```

