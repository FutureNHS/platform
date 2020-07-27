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
