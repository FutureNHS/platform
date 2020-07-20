#!/bin/bash
set -eu

# https://stackoverflow.com/a/1482133
cd $(dirname "$0")

ENVIRONMENT="${1:?"Please specify you environment name as the first parameter, e.g. dev-jane"}"
CURRENT_CONTEXT=$(kubectl config current-context)

if [ "$ENVIRONMENT" != "$CURRENT_CONTEXT" ]; then
	echo "You want to deploy to:   $ENVIRONMENT"
	echo "Your current content is: $CURRENT_CONTEXT"
	echo "Please change your current context (e.g. using 'kubectl config use-context' or a combination or 'az account set' and 'az aks get-credentials') and try again"
	exit 1
fi

echo "Installing Argo CD CLI"
brew install argoproj/tap/argocd || {
	echo 'Unable to install.'
	exit 1
}

echo "Installing Argo CD"
kustomize build ../kubernetes/argocd/install |
	kubectl apply -n argocd -f -
kubectl rollout status -n argocd deployment argocd-server

echo "Expanding dev overlyas"
./create-dev-overlays.py

echo "Creating applications"
if [ ! -d ../kubernetes/argocd/apps/$ENVIRONMENT ]; then
	echo "The overlay folder $ENVIRONMENT for Argo CD applications does not exist."
	echo "Did you run ./infrastructure/scripts/create-dev-overlays.py?"
	echo "Did you add the overlay in ./infrastructure/dev-overlay-variables.json?"
	exit 1
fi
kustomize build ../kubernetes/argocd/apps/$ENVIRONMENT |
	kubectl apply -n argocd -f -
