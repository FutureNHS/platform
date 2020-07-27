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

module platform {
  source      = "../../modules/platform"
  environment = "production"
  location    = var.location
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
