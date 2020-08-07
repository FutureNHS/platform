# This requires the postgresql and kubernetes providers to be set up properly.
# Run `terraform apply -target module.platform` to set up their requirements
# before attempting to apply this module.

resource "random_password" "kratos_postgresql_password" {
  length  = 50
  special = false
  upper   = true
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

resource "postgresql_role" "kratos" {
  name     = "kratos_user"
  login    = true
  password = random_password.kratos_postgresql_password.result
}

# we are using the postgresql provider here because azurerm_postgresql_database
# doesn't let us set the owner to a role other than the server admin.
resource "postgresql_database" "kratos" {
  name              = "kratos"
  owner             = postgresql_role.kratos.name
  lc_collate        = "C"
  connection_limit  = -1
  allow_connections = true
}

resource "kubernetes_namespace" "kratos" {
  metadata {
    name = "kratos"
    annotations = {
      "linkerd.io/inject" = "enabled"
    }
  }
}

resource "kubernetes_secret" "kratos_db_creds" {
  metadata {
    name      = "kratos"
    namespace = "kratos"
  }
  data = {
    secretsDefault = random_password.kratos_secrets_default.result
    secretsCookie  = random_password.kratos_secrets_cookie.result
    dsn = "postgres://${
      postgresql_role.kratos.name
      }@${
      var.postgresql_server_name
      }:${
      random_password.kratos_postgresql_password.result
      }@${
      var.postgresql_server_name
      }.postgres.database.azure.com:5432/${
      postgresql_database.kratos.name
    }"
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
