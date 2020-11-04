#!/bin/bash

set -eux

COMPONENT=${1:?"Require components as first argument, e.g. frontend or workspace-service"}

# Checkout correct branch in the deployments repo
cd $GITHUB_WORKSPACE/futurenhs-deployments
git config --local user.email "futurenhs-devs@red-badger.com"
git config --local user.name "FutureNHS CI/CD"
if git fetch --unshallow origin pr-$GITHUB_HEAD_REF master; then
	git checkout origin/pr-$GITHUB_HEAD_REF -b pr-$GITHUB_HEAD_REF
	git merge origin/master --strategy=recursive --strategy-option=ours
else
	git checkout HEAD -b pr-$GITHUB_HEAD_REF
fi

# Install Kustomize
cd $HOME
curl -s "https://raw.githubusercontent.com/kubernetes-sigs/kustomize/master/hack/install_kustomize.sh" | bash

# Prepare manifests and copy to deployments repo
cd $GITHUB_WORKSPACE/futurenhs-platform/$COMPONENT/manifests/dev-template
if [ "$COMPONENT" = "infrastructure" ]; then
	mkdir -p $GITHUB_WORKSPACE/futurenhs-deployments/cert-manager
	cp -r $GITHUB_WORKSPACE/futurenhs-platform/infrastructure/kubernetes/cert-manager/* $GITHUB_WORKSPACE/futurenhs-deployments/cert-manager
	mkdir -p $GITHUB_WORKSPACE/futurenhs-deployments/ingress
	cp -r $GITHUB_WORKSPACE/futurenhs-platform/infrastructure/kubernetes/ingress/* $GITHUB_WORKSPACE/futurenhs-deployments/ingress
else
	$HOME/kustomize edit set image $DIGEST
	$GITHUB_WORKSPACE/futurenhs-platform/infrastructure/scripts/create-dev-overlays.py
	mkdir -p $GITHUB_WORKSPACE/futurenhs-deployments/$COMPONENT
	cp -r $GITHUB_WORKSPACE/futurenhs-platform/$COMPONENT/manifests/* $GITHUB_WORKSPACE/futurenhs-deployments/$COMPONENT
fi

# Get commit message
cd $GITHUB_WORKSPACE/futurenhs-platform
git fetch origin HEAD --deepen=2
COMMIT_MESSAGE=$(git log --format=%B -n 1 $GITHUB_SHA)

# Commit and push changes to deployments repo
cd $GITHUB_WORKSPACE/futurenhs-deployments

git add -A
git diff-index --quiet HEAD || git commit -am "$COMPONENT-branch: $COMMIT_MESSAGE ${TAG:-""}"

declare -i n
n=0
until [ $n -ge 5 ]; do
	git push origin "HEAD:pr-$GITHUB_HEAD_REF" && break
	n+=1
	git fetch origin pr-$GITHUB_HEAD_REF
	git merge origin/pr-$GITHUB_HEAD_REF --strategy=recursive --strategy-option=ours
done

if [ $n -ge 5 ]; then
	echo "ran out of retries"
	exit 1
fi
