terraform {
  # The configuration for this backend will be filled in by Terragrunt
  backend "local" {}
}

provider "hcloud" {
  token = var.hcloud_token
}
resource "hcloud_ssh_key" "default" {
  name       = "key"
  public_key = file("~/.ssh/terra.pub")
}
resource "hcloud_server" "node" {
  count       = var.node_count
  name        = "hcloud-${count.index + 1}"
  server_type = var.node_type
  location    = var.location
  image       = var.node_image
  ssh_keys    = ["${hcloud_ssh_key.default.name}"]
}
