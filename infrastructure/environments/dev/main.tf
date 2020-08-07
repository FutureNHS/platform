provider "azurerm" {
  version = "=2.14.0"
  features {}
  subscription_id = "4a4be66c-9000-4906-8253-6a73f09f418d"
}

provider "random" {
  version = "~> 2.2"
}

terraform {
  required_version = "0.12.25"
  backend "azurerm" {
    container_name = "tfstate"
    key            = "dev.terraform.tfstate"
  }
}

# This module should be applied first, using `terraform apply -target module.platform`
# so that the kubernetes cluster and postresql firewall rules are set up.
# These are needed before the `kubernetes` and `postgresql` terraform providers can plan
# anything.
module platform {
  source                  = "../../modules/platform"
  environment             = "dev-${var.USERNAME}"
  location                = var.location
  ip_whitelist_postgresql = var.ip_whitelist_postgresql
}

provider "kubernetes" {
  load_config_file       = "false"
  host                   = module.platform.kube_config.0.host
  username               = module.platform.kube_config.0.username
  password               = module.platform.kube_config.0.password
  client_certificate     = base64decode(module.platform.kube_config.0.client_certificate)
  client_key             = base64decode(module.platform.kube_config.0.client_key)
  cluster_ca_certificate = base64decode(module.platform.kube_config.0.cluster_ca_certificate)
}

provider "postgresql" {
  host              = "${module.platform.postgresql_server_name}.postgres.database.azure.com"
  port              = 5432
  database          = "postgres"
  database_username = module.platform.postgresql_admin
  username          = "${module.platform.postgresql_admin}@${module.platform.postgresql_server_name}"
  password          = module.platform.postgresql_admin_password
  sslmode           = "require"
  connect_timeout   = 15
  superuser         = false
}

module databases {
  source                   = "../../modules/databases"
  environment              = "dev-${var.USERNAME}"
  postgresql_server_name   = module.platform.postgresql_server_name
  eventgrid_topic_endpoint = module.platform.eventgrid_topic_endpoint
  eventgrid_topic_key      = module.platform.eventgrid_topic_key
}
