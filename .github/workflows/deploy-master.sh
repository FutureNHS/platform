#!/bin/bash

set -eux

COMPONENT=${1:?"Require components as first argument, e.g. frontend or workspace-service"}

# Install Kustomize
cd $HOME
curl -s "https://raw.githubusercontent.com/kubernetes-sigs/kustomize/master/hack/install_kustomize.sh" | bash

# Prepare manifests and copy to deployments repo
if [ "$COMPONENT" = "infrastructure" ]; then
	$GITHUB_WORKSPACE/futurenhs-platform/infrastructure/scripts/create-dev-overlays.py
	mkdir -p $GITHUB_WORKSPACE/futurenhs-deployments/cert-manager
	cp -r $GITHUB_WORKSPACE/futurenhs-platform/infrastructure/kubernetes/cert-manager/* $GITHUB_WORKSPACE/futurenhs-deployments/cert-manager
	mkdir -p $GITHUB_WORKSPACE/futurenhs-deployments/ingress
	cp -r $GITHUB_WORKSPACE/futurenhs-platform/infrastructure/kubernetes/ingress/* $GITHUB_WORKSPACE/futurenhs-deployments/ingress
else
	cd $GITHUB_WORKSPACE/futurenhs-platform/$COMPONENT/manifests/base
	$HOME/kustomize edit set image $DIGEST
	$GITHUB_WORKSPACE/futurenhs-platform/infrastructure/scripts/create-dev-overlays.py
	mkdir -p $GITHUB_WORKSPACE/futurenhs-deployments/$COMPONENT
	cp -r $GITHUB_WORKSPACE/futurenhs-platform/$COMPONENT/manifests/* $GITHUB_WORKSPACE/futurenhs-deployments/$COMPONENT
fi

# Commit and push changes to deployments repo
cd $GITHUB_WORKSPACE/futurenhs-deployments

git config --local user.email "futurenhs-devs@red-badger.com"
git config --local user.name "FutureNHS CI/CD"

git add -A
git diff-index --quiet HEAD || git commit -am "($COMPONENT-master) tag:${TAG:-"none"}"

declare -i n
n=0
until [ $n -ge 5 ]; do
	git push && break
	n+=1
	git pull --rebase
done

if [ $n -ge 5 ]; then
	echo "ran out of retries"
	exit 1
fi
