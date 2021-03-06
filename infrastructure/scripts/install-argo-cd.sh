#!/bin/bash
set -eu

# https://stackoverflow.com/a/1482133
cd $(dirname "$0")

ENVIRONMENT="${1:?"Please specify your environment name as the first parameter, e.g. dev-jane"}"
BRANCH="${2:-""}"

CURRENT_CONTEXT=$(kubectl config current-context)

if [ "$ENVIRONMENT" != "$CURRENT_CONTEXT" ]; then
	echo "You want to deploy to:   $ENVIRONMENT"
	echo "Your current content is: $CURRENT_CONTEXT"
	echo "Please change your current context using:"
	echo "    kubectl config use-context $ENVIRONMENT"
	echo "or"
	echo "    az account set --subscription \$SUBSCRIPTION_ID && az aks get-credentials --resource-group=platform-$ENVIRONMENT --name=$ENVIRONMENT"
	echo "and try again."
	exit 1
fi

if [ "$BRANCH" = "MINE" ]; then
	BRANCH=pr-$(git branch --show-current)
fi
if [ -z "$BRANCH" ]; then
	echo "Expanding dev overlays."
	echo ""
	echo "To force argocd to point at the pr-build for your current branch, try:"
	echo "    $0 $* MINE"
	echo ""
	echo "To force argocd to point at a pr-build for someone else's branch, try:"
	echo "    $0 $* pr-\$BRANCHNAME"
	echo ""
	./create-dev-overlays.py
else
	echo "Expanding dev overlays to point at $BRANCH"
	./create-dev-overlays.py --set-branch=$BRANCH
fi

echo "Installing Argo CD CLI"
brew install argoproj/tap/argocd || brew upgrade argocd || {
	echo 'Unable to install.'
	exit 1
}

echo "Installing Argo CD"
kustomize build ../kubernetes/argocd/install |
	kubectl apply -n argocd -f -
kubectl rollout status -n argocd deployment argocd-server

echo "Creating applications"
if [ ! -d ../kubernetes/argocd/apps/$ENVIRONMENT ]; then
	echo "The overlay folder $ENVIRONMENT for Argo CD applications does not exist."
	echo "Did you add the overlay in ./infrastructure/dev-overlay-variables.json?"
	exit 1
fi
kustomize build ../kubernetes/argocd/apps/$ENVIRONMENT |
	kubectl apply -n argocd -f -

# The user should set this in their .profile, but let's set it here just in case it's not already set.
export ARGOCD_OPTS='--port-forward --port-forward-namespace argocd'

POD_NAME=$(kubectl get pods -n argocd | grep --only-matching 'argocd-server-[^ ]*')
if ! argocd login --username admin --password $POD_NAME; then
	if [ ${RESET_PASSWORD:-0} = 1 ]; then
		echo "It looks like argocd has been upgraded. Resetting the password."
		KUBE_EDITOR='sed -i "" "/admin.password/d"' kubectl edit secret -n argocd argocd-secret -o json
		kubectl rollout restart -n argocd deployment argocd-server
		POD_NAME=$(kubectl get pods -n argocd | grep --only-matching 'argocd-server-[^ ]*')
		argocd login --username admin --password $POD_NAME
	elif argocd app list >/dev/null; then
		echo "argocd has been upgraded at some point, but has kept its old password."
		echo "if you have trouble logging in in the browser, re-run this script with"
		echo "RESET_PASSWORD=1 to reset and print the password."
		exit 0
	else
		echo "argocd has been upgraded at some point, and we no longer know the password."
		echo "Please re-run this script with RESET_PASSWORD=1 to reset and print the password."
		exit 1
	fi
fi

for app in $(argocd app list -o name); do
	argocd app sync $app
done

echo "Your argocd username for the web ui is 'admin' and password is '$POD_NAME'"
