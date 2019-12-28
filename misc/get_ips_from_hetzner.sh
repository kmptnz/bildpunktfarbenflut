#!/bin/sh
curl -sH "Authorization: Bearer ${HCLOUD_TOKEN}" https://api.hetzner.cloud/v1/servers | jq '.servers | .[] | .public_net.ipv4.ip' | uniq | tr -d '"' > hosts
