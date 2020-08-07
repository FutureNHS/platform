output "resource_group_name" {
  value = azurerm_resource_group.platform.name
}

output "cluster_identity_principal_id" {
  value = azurerm_kubernetes_cluster.cluster.kubelet_identity.0.object_id
}

output "kube_config" {
  value = azurerm_kubernetes_cluster.cluster.kube_config
}

output "instrumentation_key" {
  value = azurerm_application_insights.app_insights.instrumentation_key
}

output "postgresql_server_name" {
  value = azurerm_postgresql_server.postgresql_server.name
}

output "postgresql_admin" {
  value = azurerm_postgresql_server.postgresql_server.administrator_login
}

output "postgresql_admin_password" {
  value     = azurerm_postgresql_server.postgresql_server.administrator_login_password
  sensitive = true
}

output "eventgrid_topic_endpoint" {
  value = azurerm_eventgrid_topic.platform.endpoint
}

output "eventgrid_topic_key" {
  value     = azurerm_eventgrid_topic.platform.primary_access_key
  sensitive = true
}
