provider "azurerm" {
  version = "=2.14.0"
  features {}
  subscription_id = "75173371-c161-447a-9731-f042213a19da"
}

provider "random" {
  version = "~> 2.2"
}

terraform {
  required_version = "0.12.25"
  backend "azurerm" {
    container_name       = "tfstate"
    key                  = "production.terraform.tfstate"
    resource_group_name  = "tfstate"
    storage_account_name = "fnhstfstateproduction"
  }
}

# This module should be applied first, using `terraform apply -target module.platform`
# so that the kubernetes cluster and postresql firewall rules are set up.
# These are needed before the `kubernetes` and `postgresql` terraform providers can plan
# anything.
module platform {
  source                  = "../../modules/platform"
  environment             = "production"
  location                = var.location
  ip_whitelist_postgresql = var.ip_whitelist_postgresql
}

module analytics {
  source              = "../../modules/analytics"
  environment         = "production"
  location            = var.location
  resource_group_name = module.platform.resource_group_name
  allowed_ips         = var.ip_whitelist_insights
  ad_username         = "FutureNHS Developers"
  ad_object_id        = "b06ebd00-f52c-4e82-ac88-0520f4320fee"
}

resource "azurerm_container_registry" "acr" {
  name                = "fnhsproduction"
  resource_group_name = module.platform.resource_group_name
  location            = var.location
  sku                 = "Basic"
}

resource "azurerm_role_assignment" "acrpull_cluster" {
  scope                            = azurerm_container_registry.acr.id
  role_definition_name             = "AcrPull"
  principal_id                     = module.platform.cluster_identity_principal_id
  skip_service_principal_aad_check = true
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
  environment              = "production"
  postgresql_server_name   = module.platform.postgresql_server_name
  eventgrid_topic_endpoint = module.platform.eventgrid_topic_endpoint
  eventgrid_topic_key      = module.platform.eventgrid_topic_key
}
