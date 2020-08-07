resource "azurerm_resource_group" "platform" {
  name     = "platform-${var.environment}"
  location = var.location

  tags = {
    environment = var.environment
  }
}

resource "azurerm_virtual_network" "platform" {
  name                = "platform-${var.environment}"
  address_space       = ["10.0.0.0/8"]
  location            = var.location
  resource_group_name = azurerm_resource_group.platform.name
  tags = {
    environment = var.environment
  }
}

resource "azurerm_subnet" "cluster_nodes" {
  name                 = "cluster-nodes-${var.environment}"
  resource_group_name  = azurerm_resource_group.platform.name
  virtual_network_name = azurerm_virtual_network.platform.name
  address_prefixes     = ["10.240.0.0/16"]
}

# The ingress controller needs to fiddle with azurerm_public_ip.cluster_outbound.
resource "azurerm_role_assignment" "managed_identity_operator_cluster" {
  scope                            = azurerm_resource_group.platform.id
  role_definition_name             = "Network Contributor"
  principal_id                     = azurerm_kubernetes_cluster.cluster.identity.0.principal_id
  skip_service_principal_aad_check = true
}

resource "azurerm_public_ip" "cluster_outbound" {
  name = "cluster_outbound"

  resource_group_name = azurerm_resource_group.platform.name
  location            = var.location
  allocation_method   = "Static"
  sku                 = "Standard"

  tags = {
    environment = var.environment
  }
}

resource "azurerm_kubernetes_cluster" "cluster" {
  name                = var.environment
  location            = var.location
  dns_prefix          = var.environment
  resource_group_name = azurerm_resource_group.platform.name

  default_node_pool {
    name                = "default"
    enable_auto_scaling = true
    max_count           = 2
    min_count           = 1
    vm_size             = "Standard_D2_v2"
    vnet_subnet_id      = azurerm_subnet.cluster_nodes.id
    availability_zones  = ["1", "2", "3"]
    tags = {
      environment = var.environment
    }
  }

  identity {
    type = "SystemAssigned"
  }

  role_based_access_control {
    enabled = true
  }

  network_profile {
    network_plugin = "kubenet"
    load_balancer_profile {
      outbound_ip_address_ids = [
        azurerm_public_ip.cluster_outbound.id
      ]
    }
  }

  addon_profile {
    aci_connector_linux {
      enabled = false
    }
    azure_policy {
      enabled = false
    }
    http_application_routing {
      enabled = false
    }
    kube_dashboard {
      enabled = false
    }
    oms_agent {
      enabled                    = true
      log_analytics_workspace_id = azurerm_log_analytics_workspace.cluster.id
    }
  }

  tags = {
    environment = var.environment
  }
}

resource "azurerm_log_analytics_workspace" "cluster" {
  name                = "cluster-${var.environment}"
  location            = var.location
  resource_group_name = azurerm_resource_group.platform.name
  sku                 = "PerGB2018"
  retention_in_days   = 30

  tags = {
    environment = var.environment
  }
}

resource "azurerm_application_insights" "app_insights" {
  name                = "cluster-${var.environment}"
  location            = var.location
  resource_group_name = azurerm_resource_group.platform.name
  application_type    = "other"
}

resource "random_password" "postgresql_password" {
  length  = 50
  special = false
  upper   = true
}

resource "azurerm_postgresql_server" "postgresql_server" {
  name                = "postgresql-${var.environment}"
  location            = azurerm_resource_group.platform.location
  resource_group_name = azurerm_resource_group.platform.name

  sku_name = "B_Gen5_2"

  storage_mb                   = 5120
  backup_retention_days        = 7
  geo_redundant_backup_enabled = false
  auto_grow_enabled            = true

  administrator_login          = "psqladminun"
  administrator_login_password = random_password.postgresql_password.result
  version                      = "9.5"
  ssl_enforcement_enabled      = true
}

resource "azurerm_postgresql_firewall_rule" "ip_whitelisted" {
  for_each            = var.ip_whitelist_postgresql
  name                = "ip-whitelisted-${each.key}"
  resource_group_name = azurerm_resource_group.platform.name
  server_name         = azurerm_postgresql_server.postgresql_server.name
  start_ip_address    = each.value
  end_ip_address      = each.value
}

resource "azurerm_postgresql_firewall_rule" "whitelist_cluster" {
  name                = "whitelist_cluster"
  resource_group_name = azurerm_resource_group.platform.name
  server_name         = azurerm_postgresql_server.postgresql_server.name
  start_ip_address    = azurerm_public_ip.cluster_outbound.ip_address
  end_ip_address      = azurerm_public_ip.cluster_outbound.ip_address
}

resource "azurerm_eventgrid_topic" "platform" {
  name                = "platform-${var.environment}"
  location            = var.location
  resource_group_name = azurerm_resource_group.platform.name

  tags = {
    environment = var.environment
  }
}

resource "azurerm_eventhub_namespace" "platform" {
  name                = "platform-${var.environment}"
  location            = var.location
  resource_group_name = azurerm_resource_group.platform.name
  sku                 = "Standard"
  capacity            = 1

  tags = {
    environment = var.environment
  }
}

resource "azurerm_eventhub" "analytics" {
  name                = "analytics"
  namespace_name      = azurerm_eventhub_namespace.platform.name
  resource_group_name = azurerm_resource_group.platform.name
  partition_count     = 1
  message_retention   = 1
}

resource "azurerm_eventgrid_event_subscription" "analytics" {
  name                 = "analytics-events"
  scope                = azurerm_eventgrid_topic.platform.id
  eventhub_endpoint_id = azurerm_eventhub.analytics.id
}
