#!/bin/bash

cp build.tpl build.rs

echo "Starting Vault..."
if pgrep -x "vault" > /dev/null
then
    echo "Vault is already running. Aborting."
    exit 1
fi

vault server -dev -dev-root-token-id=root &
sleep 5
VAULT_PID=$!

defer_stop_vault() {
    echo "Stopping Vault..."
    kill $VAULT_PID
    sleep 1
}
trap defer_stop_vault INT TERM EXIT

token=root
secrets="ad alicloud aws azure consul database gcp gcpkms kv kubernetes mongodbatlas nomad ldap pki rabbitmq ssh terraform totp transit"
export VAULT_ADDR=http://127.0.0.1:8200
export VAULT_TOKEN=$token
echo "\n\n"  | tee -a build.rs
echo "Start to generate auth spec"
for secret in $secrets; do
  vault secrets enable $secret
  curl -H"X-Vault-Token: $token" http://127.0.0.1:8200/v1/$secret?help=1 | jq .openapi > spec/secrets/$secret.json
done

vault secrets enable kv-v2
curl -H"X-Vault-Token: $token" http://127.0.0.1:8200/v1/kv-v2?help=1 | jq .openapi > spec/secrets/kv-v2.json

echo "fn main() {" | tee -a build.rs

for secret in $secrets; do
    echo "    build(\"spec/secrets/$secret.json\", \"src/secrets/$secret.rs\");" | tee -a build.rs
done
echo "    build(\"spec/secrets/kv-v2.json\", \"src/secrets/kv2.rs\");" | tee -a build.rs

echo "\n\n"  | tee -a build.rs
echo "Start to generate auth spec"
auths="alicloud approle aws azure cf github gcp jwt kerberos kubernetes ldap oci okta radius cert userpass"
for auth in $auths; do
  vault auth enable $auth
  curl -H"X-Vault-Token: $token" http://127.0.0.1:8200/v1/auth/$auth?help=1 | jq .openapi > spec/auth/$auth.json
done

curl -H"X-Vault-Token: $token" http://127.0.0.1:8200/v1/auth/token?help=1 | jq .openapi > spec/auth/token.json
for auth in $auths; do
    echo "    build(\"spec/auth/$auth.json\", \"src/auth/$auth.rs\");" | tee -a build.rs
done
echo "    build(\"spec/auth/token.json\", \"src/auth/token.rs\");" | tee -a build.rs

echo "\n\n"  | tee -a build.rs
echo "Start to generate sys api spec"
curl -H"X-Vault-Token: $token" http://127.0.0.1:8200/v1/sys?help=1 | jq .openapi > spec/sys.json
echo "    build(\"spec/sys.json\", \"src/sys/mod.rs\");" | tee -a build.rs

echo "}" | tee -a build.rs
