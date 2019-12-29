# setup

## export secrets

```bash
export HCLOUD_TOKEN=XXX
```

## terraform

```bash
cd terraform
curl -Lo para "https://github.com/paraterraform/para/releases/latest/download/$(curl \
-L https://github.com/paraterraform/para/releases/latest/download/SHA256SUMS | grep -i $(uname -s) | awk '{ print $2 }')"
chmod +x para
terraform init -input=false
terraform plan -input=false -var-file=pixelflut.tfvars -var=hcloud_token=${HCLOUD_TOKEN}
terraform apply -input=false -var-file=pixelflut.tfvars -var=hcloud_token=${HCLOUD_TOKEN}
```

## ansible

```
cd ansible
pipenv install
ansible-galaxy install -r requirements.yml
ansible-playbook bootstrap.yml  -i inventory/hcloud.yml
```
