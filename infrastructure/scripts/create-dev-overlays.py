#!/usr/bin/env python3

import json
import os
from string import Template

repo_root = os.path.join(os.path.dirname(__file__), '../..')

# FIXME: This is duplicated in .github/workflows/infrastructure-branch.yaml
services = [
    'frontend/manifests',
    'hello-world/manifests',
    'infrastructure/kubernetes/argocd/apps',
    'infrastructure/kubernetes/cert-manager',
    'infrastructure/kubernetes/azure-service-operator',
    'infrastructure/kubernetes/ingress',
]

def create_overlays(name, variables):
    for service in  services:
        os.makedirs(f'{repo_root}/{service}/dev-{name}', exist_ok=True)
        for filename in os.listdir(f'{repo_root}/{service}/dev-template'):
            with open(f'{repo_root}/{service}/dev-template/{filename}') as f:
                content = f.read()
            content = Template(content).substitute(NAME=name, **variables)
            with open(f'{repo_root}/{service}/dev-{name}/{filename}', 'w') as f:
                f.write(content)

def main():
    with open(f'{repo_root}/infrastructure/dev-overlay-variables.json') as f:
        overlays = json.load(f)

    for name, variables in overlays.items():
        create_overlays(name, variables)

if __name__ == '__main__':
    main()
