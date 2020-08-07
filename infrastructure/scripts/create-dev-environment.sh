#!/bin/bash

set -eu

NAME="${1:?"Please enter your name as first argument"}"

if [ "$NAME" != "${NAME//[^a-z]/-}" ]; then
	echo "Name can only contain lowercase characters a-z"
	exit 1
fi

# We might want to write a OVERWRITE=full version later that does a terraform destroy etc.
OVERWRITE="${OVERWRITE:-forbidden}"
if [[ "$OVERWRITE" != "localfiles" && "$OVERWRITE" != "justdotterraform" && "$OVERWRITE" != "forbidden" ]]; then
	echo "Unexpected value for OVERWRITE environment variable: '$OVERWRITE'"
	echo "Use OVERWRITE=localfiles to clean your local terraform.tfvars and .terraform."
	echo "Use OVERWRITE=justdotterraform to clean just .terraform, but leave your terraform.tfvars alone."
	echo "Use OVERWRITE=forbidden (the default) if you are using a clean envionment and want to abort if anything is unclean."
	exit 1
fi

REPO_ROOT="$(git rev-parse --show-toplevel)"

setup_terraform() {
	DEV_CONFIG_FILE="$REPO_ROOT/infrastructure/environments/dev/terraform.tfvars"
	TERRAFORM_DIR="$REPO_ROOT/infrastructure/environments/dev/.terraform"

	SUBSCRIPTION_ID=4a4be66c-9000-4906-8253-6a73f09f418d
	RESOURCE_GROUP_NAME=tfstate$NAME
	STORAGE_ACCOUNT_NAME=fnhstfstatedev$NAME
	CONTAINER_NAME=tfstate

	if [ "$OVERWRITE" = "localfiles" ]; then
		rm -r $DEV_CONFIG_FILE $TERRAFORM_DIR
	elif [ "$OVERWRITE" = "justdotterraform" ]; then
		rm -r $TERRAFORM_DIR
	elif [ -f "$DEV_CONFIG_FILE" ]; then
		echo "File $DEV_CONFIG_FILE already exists."
		echo "If you want to initialize your environment again, please delete the file and rerun this script,"
		echo "or run:"
		echo ""
		echo "    OVERWRITE=localfiles $0 $NAME"
		echo "or:"
		echo "    OVERWRITE=justdotterraform $0 $NAME"
		echo ""
		exit 1
	elif [ -d "$TERRAFORM_DIR" ]; then
		echo "Folder $TERRAFORM_DIR already exists."
		echo "If you want to initialize your environment again, please delete that directory and rerun this script."
		echo "or run:"
		echo ""
		echo "    OVERWRITE=localfiles $0 $NAME"
		echo "or:"
		echo "    OVERWRITE=justdotterraform $0 $NAME"
		echo ""
		exit 1
	fi

	# Use non-production subscription
	az account set --subscription $SUBSCRIPTION_ID

	# Create resource group
	# TODO: do we want to limit this to england/wales, or is europe okay?
	az group create \
		--name $RESOURCE_GROUP_NAME \
		--location westeurope \
		--output none

	# Create storage account
	az storage account create \
		--kind StorageV2 \
		--resource-group $RESOURCE_GROUP_NAME \
		--name $STORAGE_ACCOUNT_NAME \
		--sku Standard_LRS \
		--encryption-services blob \
		--output none

	# Get storage account key
	ACCESS_KEY=$(az storage account keys list \
		--resource-group $RESOURCE_GROUP_NAME \
		--account-name $STORAGE_ACCOUNT_NAME \
		--query [0].value \
		--output tsv)

	# Create blob container
	az storage container create \
		--name $CONTAINER_NAME \
		--account-name $STORAGE_ACCOUNT_NAME \
		--account-key "$ACCESS_KEY" \
		--output none

	if [ ! -f "$DEV_CONFIG_FILE" ]; then
		cat >"$DEV_CONFIG_FILE" <<EOF
resource_group_name="$RESOURCE_GROUP_NAME"
storage_account_name="$STORAGE_ACCOUNT_NAME"
USERNAME="$NAME"
ip_whitelist_postgresql={"$NAME" = $(dig TXT +short o-o.myaddr.l.google.com @ns1.google.com)}
EOF
	fi

	# terraform init is idempotent, so we might as well run it for the user.
	cd $REPO_ROOT/infrastructure/environments/dev && terraform init -backend-config=terraform.tfvars

	echo "Your dev terraform environment is ready to go. To initialize run:"
	echo "("
	echo "    cd $REPO_ROOT/infrastructure/environments/dev &&"
	echo "    terraform apply -target module.platform &&"
	echo "    terraform apply"
	echo ")"
}

setup_terraform
