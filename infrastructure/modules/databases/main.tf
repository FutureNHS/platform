# This requires the postgresql and kubernetes providers to be set up properly.
# Run `terraform apply -target module.platform` to set up their requirements
# before attempting to apply this module.
locals {
  databases = [
    "kratos",
    "workspace_service",
    "fusionauth",
  ]
}

resource "random_password" "postgresql_password" {
  for_each = toset(local.databases)
  length   = 50
  special  = false
  upper    = true
}
resource "random_password" "kratos_secrets_default" {
  length  = 32
  special = false
  upper   = true
}
resource "random_password" "kratos_secrets_cookie" {
  length  = 32
  special = false
  upper   = true
}

resource "postgresql_role" "service" {
  for_each = toset(local.databases)
  name     = "${each.value}_user"
  login    = true
  password = random_password.postgresql_password[each.value].result
}

# we are using the postgresql provider here because azurerm_postgresql_database
# doesn't let us set the owner to a role other than the server admin.
resource "postgresql_database" "service" {
  for_each          = toset(local.databases)
  name              = each.value
  owner             = postgresql_role.service[each.value].name
  lc_collate        = "C"
  connection_limit  = -1
  allow_connections = true
}

resource "postgresql_extension" "uuid-ossp" {
  name     = "uuid-ossp"
  database = "workspace_service"
}

resource "kubernetes_namespace" "service" {
  for_each = toset(local.databases)
  metadata {
    name = (replace(each.value, "_", "-"))
    annotations = {
      "linkerd.io/inject" = "enabled"
    }
  }
}

resource "kubernetes_secret" "kratos_db_creds" {
  metadata {
    name      = "kratos"
    namespace = kubernetes_namespace.service["kratos"].metadata[0].name
  }
  data = {
    secretsDefault = random_password.kratos_secrets_default.result
    secretsCookie  = random_password.kratos_secrets_cookie.result
    dsn = "postgres://${
      postgresql_role.service["kratos"].name
      }@${
      var.postgresql_server_name
      }:${
      random_password.postgresql_password["kratos"].result
      }@${
      var.postgresql_server_name
      }.postgres.database.azure.com:5432/${
      postgresql_database.service["kratos"].name
    }"
  }
}


resource "kubernetes_secret" "workspace_service_db_creds" {
  metadata {
    name      = "workspace-service"
    namespace = kubernetes_namespace.service["workspace_service"].metadata[0].name
  }
  data = {
    url = "postgres://${
      postgresql_role.service["workspace_service"].name
      }@${
      var.postgresql_server_name
      }:${
      random_password.postgresql_password["workspace_service"].result
      }@${
      var.postgresql_server_name
      }.postgres.database.azure.com:5432/${
      postgresql_database.service["workspace_service"].name
    }"
  }
}

resource "kubernetes_secret" "fusionauth_db_creds" {
  metadata {
    name      = "fusionauth"
    namespace = kubernetes_namespace.service["fusionauth"].metadata[0].name
  }
  # rootpassword == password for now
  data = {
    password = random_password.postgresql_password["fusionauth"].result
    rootpassword = random_password.postgresql_password["fusionauth"].result
  }
}

resource "kubernetes_namespace" "frontend" {
  metadata {
    name = "frontend"
    annotations = {
      "linkerd.io/inject" = "enabled"
    }
  }
}

resource "kubernetes_secret" "frontend" {
  metadata {
    name      = "frontend"
    namespace = kubernetes_namespace.frontend.metadata[0].name
  }
  data = {
    eventgrid_topic_endpoint = var.eventgrid_topic_endpoint
    eventgrid_topic_key      = var.eventgrid_topic_key
  }
}

resource "kubernetes_namespace" "hello_world" {
  metadata {
    name = "hello-world"
    annotations = {
      "linkerd.io/inject" = "enabled"
    }
  }
}

resource "kubernetes_config_map" "frontend_telemetry" {
  metadata {
    name      = "telemetry"
    namespace = kubernetes_namespace.frontend.metadata[0].name
  }
  data = {
    instrumentation_key = var.instrumentation_key
  }
}

resource "kubernetes_config_map" "hello_world_telemetry" {
  metadata {
    name      = "telemetry"
    namespace = kubernetes_namespace.hello_world.metadata[0].name
  }
  data = {
    instrumentation_key = var.instrumentation_key
  }
}

resource "kubernetes_config_map" "workspace_service_telemetry" {
  metadata {
    name      = "telemetry"
    namespace = kubernetes_namespace.service["workspace_service"].metadata[0].name
  }
  data = {
    instrumentation_key = var.instrumentation_key
  }
}
